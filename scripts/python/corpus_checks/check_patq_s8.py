"""Protocol S8 counterexample search for P-ATQ-3 (prefix stripping) and
P-ATQ-4 (spell-created delayed triggers) over the full corpus dump.

    python scripts/python/corpus_checks/check_patq_s8.py corpus-units.jsonl out.md \
        [--pre corpus-units-pre.jsonl] [--mtg target/release/mtg-discover.exe]
    python scripts/python/corpus_checks/check_patq_s8.py --self-test

The Python rules below re-state the production patterns of `src/main.rs`
(`extract_prefix`, `is_saga_chapter_prefix`, `has_delayed_trigger_temporal_scope`,
`is_cast_or_resolve_trigger`, `has_off_stack_evidence`) so the search can be
run over a dump; `--mtg` verifies the re-statement against the binary on every
unit it fires on, and the P-ATQ-4 predicate is checked against the dumped
role. `--pre` supplies a dump from the pre-change binary for before/after
transitions. Held-out pool cards (oracle_id prefix `f`) are counted, never listed.
"""
import argparse, json, os, re, sqlite3, subprocess, sys
from collections import Counter, defaultdict

WT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
PREFIX = re.compile(r"^([^.:]{1,45}?) — (\S.*)$")
ROMAN = re.compile(r"^[IVXLCDM]+$")
TEMPORAL = re.compile(r"(?i)\bthis turn\b|\bthis combat\b|\bnext\b")
CAST_OR_RESOLVE = re.compile(r"(?i)^when you cast ~|\bcast ~ from\b|~ is countered\b|^when ~ resolves\b")
OFF_STACK_KEYWORD = re.compile(r"(?i)\bcycl(?:e|ing)\b|\bsuspended\b|\bhaunts?\b")
OFF_STACK_ZONE = re.compile(r"(?i)~[^.]{0,30}\b(?:graveyard|exiled?|discard(?:ed|s)?)\b|\b(?:graveyard|exiled?|discard(?:ed|s)?)\b[^.]{0,30}~")
TRIGGER_WORD = re.compile(r"(?i)\b(when|whenever|at the beginning of|at end of combat)\b")
OTHER_DURATION = re.compile(r"(?i)\buntil\b|\beach\b|\bthis (game|phase|step)\b|\bduring\b|\bas long as\b|\bfor the rest\b")
ZONE_WORDS = re.compile(r"(?i)\b(library|hand|graveyard|exile[d]?|command zone|in exile)\b")


def extract_prefix(normalized):
    m = PREFIX.match(normalized)
    return (m.group(1), m.group(2)) if m else None

def is_saga_chapter_prefix(prefix):
    parts = [p.strip() for p in prefix.split(",")]
    return all(p and ROMAN.match(p) for p in parts)

def is_saga(type_line): return "Saga" in re.split(r"[\s—/]+", type_line or "")
def is_is(type_line): return any(w in ("Instant", "Sorcery") for w in re.split(r"[\s—/]+", (type_line or "").split("—")[0]))
def classification_text(normalized):
    p = extract_prefix(normalized); return p[1] if p else normalized
def is_spell_created_delayed_trigger(text, type_line):
    return is_is(type_line) and bool(TEMPORAL.search(text)) and not CAST_OR_RESOLVE.search(text) \
        and not (OFF_STACK_KEYWORD.search(text) or OFF_STACK_ZONE.search(text))
def mask(t): return re.sub(r'"[^"]*"', lambda m: "_" * len(m.group(0)), t)

def ability_words(rules_path):
    """CR 207.2c's enumerated ability words, normalized like unit text (integers -> N)."""
    for line in open(rules_path, encoding="utf-8"):
        if line.startswith("207.2c"):
            tail = line.split("The ability words are", 1)[1]
            words = [w.strip(" .\n") for w in re.split(r",\s*(?:and\s+)?", tail)]
            return {re.sub(r"\d+", "N", w.replace("’", "'")).lower() for w in words if w}
    return set()


def self_test():
    assert extract_prefix("Heroic — Whenever you cast a spell, draw a card.") == ("Heroic", "Whenever you cast a spell, draw a card.")
    assert extract_prefix("I, II — Prevent all damage.")[0] == "I, II"
    assert extract_prefix("Choose one —") is None                     # empty body
    assert extract_prefix("{M}: Draw a card — then discard.") is None  # colon before dash
    assert extract_prefix("Draw a card. Then — discard.") is None      # period before dash
    assert extract_prefix("x" * 46 + " — body") is None                # over 45
    assert extract_prefix("Suspend N—{M}") is None                     # no spaces around dash
    assert is_saga_chapter_prefix("I, II") and is_saga_chapter_prefix("III") and not is_saga_chapter_prefix("N") and not is_saga_chapter_prefix("Immune")
    assert is_saga("Enchantment — Saga") and not is_saga("Creature — Human")
    assert is_is("Instant — Arcane") and is_is("Kindred Sorcery — Elf") and not is_is("Creature — Sorcerer")
    assert is_spell_created_delayed_trigger("whenever a creature blocks this turn, it gets +N/+N until end of turn.", "Instant")
    assert not is_spell_created_delayed_trigger("whenever a creature blocks this turn, it gets +N/+N.", "Enchantment")
    assert not is_spell_created_delayed_trigger("when you cast ~, copy it for each spell you've cast this turn.", "Instant")
    assert not is_spell_created_delayed_trigger("when you cycle ~, target creature gets +N/+N until end of turn this turn.", "Instant")
    assert not is_spell_created_delayed_trigger("at the beginning of your upkeep, if ~ is in your graveyard, return it next turn.", "Instant")
    assert is_spell_created_delayed_trigger("whenever an opponent discards cards this turn, return those cards from your graveyard to your hand.", "Instant")
    assert not is_spell_created_delayed_trigger("at the beginning of each of that player's upkeeps, remove a counter.", "Instant")
    assert "landfall" in ability_words(os.path.join(WT, "Magic-Comprehensive_Rules.md"))
    assert "descend n" in ability_words(os.path.join(WT, "Magic-Comprehensive_Rules.md"))
    print("self-test ok")


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("dump", nargs="?"); ap.add_argument("out", nargs="?")
    ap.add_argument("--pre"); ap.add_argument("--mtg"); ap.add_argument("--self-test", action="store_true")
    a = ap.parse_args()
    if a.self_test: self_test(); return
    sys.stdout.reconfigure(encoding="utf-8")
    units = [json.loads(l) for l in open(a.dump, encoding="utf-8") if l.strip()]
    pre = [json.loads(l) for l in open(a.pre, encoding="utf-8") if l.strip()] if a.pre else None
    db = sqlite3.connect("file:" + os.path.join(WT, "cards.sqlite") + "?mode=ro", uri=True)
    tl = {oid: (t or "") for oid, t in db.execute("SELECT oracle_id, type_line FROM cards")}
    AW = ability_words(os.path.join(WT, "Magic-Comprehensive_Rules.md"))
    def face_tl(u):
        parts = [p.strip() for p in tl.get(u["oracle_id"], "").split(" // ")]
        return parts[u["face"]] if u["face"] < len(parts) else parts[0]
    def pool(u): return u["oracle_id"].startswith("f")
    def yr(u): return (u["first_released_at"] or "????")[:4]
    def dec(u): return yr(u)[:3] + "0s"
    def show(u): return f"[{u['first_set']} {yr(u)} | {face_tl(u)[:30]}] {u['card_name']} #{u['face']}:{u['unit_index']} ({u['kind']}/{u['role']}): {u['unit_text'][:150]}"
    def key(u): return (u["oracle_id"], u["face"], u["source_line"], u["unit_text"])
    pre_by_key = {key(u): u for u in pre} if pre else {}
    out = []; P = out.append
    printed = [u for u in units if u["source"] == "printed"]
    P("# S8 counterexample search: P-ATQ-3 prefix rule and P-ATQ-4 spell-created delayed triggers\n")
    P(f"Units: {len(units)} (printed {len(printed)}); pre-change dump: {'yes, ' + str(len(pre)) + ' units' if pre else 'none'}")

    # ---------------- P-ATQ-3 ----------------
    fired = [(u, *extract_prefix(u["normalized"])) for u in units if extract_prefix(u["normalized"])]
    P(f"\n## P3.1 Prefix rule firing inventory: {len(fired)} units (pool {sum(pool(u) for u, _, _ in fired)})")
    P("by source: " + str(Counter(u["source"] for u, _, _ in fired)))
    P("by role: " + str(Counter(u["role"] for u, _, _ in fired)))
    P("by kind (post): " + str(Counter(u["kind"] for u, _, _ in fired)))
    P("by decade: " + str(dict(sorted(Counter(dec(u) for u, _, _ in fired).items()))))
    P("by set_type: " + str(Counter(u["set_type"] for u, _, _ in fired)))
    def cat(u, prefix):
        if is_saga_chapter_prefix(prefix): return "chapter (Saga face)" if is_saga(face_tl(u)) else "roman numerals on non-Saga face"
        if prefix.lower() in AW: return "ability word (CR 207.2c)"
        if u["role"] == "mode": return "named mode (role = mode)"
        return "other (flavor word CR 207.2d, label, or false positive)"
    cats = defaultdict(list)
    for u, prefix, body in fired: cats[cat(u, prefix)].append((u, prefix, body))
    P("\n## P3.2 Categories")
    for c in ("chapter (Saga face)", "ability word (CR 207.2c)", "named mode (role = mode)", "roman numerals on non-Saga face", "other (flavor word CR 207.2d, label, or false positive)"):
        v = cats[c]; dist = Counter(p for _, p, _ in v)
        P(f"\n### {c}: {len(v)} units, {len(dist)} distinct prefixes (pool {sum(pool(u) for u, _, _ in v)})")
        P("kinds: " + str(Counter(u["kind"] for u, _, _ in v)))
        limit = None if c.startswith(("roman", "other")) else 40
        for p, n in dist.most_common(limit):
            ex = next((u for u, pp, _ in v if pp == p and not pool(u)), None)
            P(f"  {n:4d}  {p!r}" + (f"  e.g. {show(ex)}" if ex and (n <= 2 or c.startswith(('roman', 'other'))) else ""))
    P("\n## P3.3 Anomaly flags on fired prefixes (every occurrence; the S11 rare-result inspection list)")
    flags = []
    for u, prefix, body in fired:
        f = []
        if prefix[:1].islower(): f.append("lowercase-initial")
        if re.search(r"\d|\bN\b|\bX\b", prefix) and not is_saga_chapter_prefix(prefix): f.append("numeral")
        if re.search(r"[\"()~{}•]", prefix): f.append("punct/symbol")
        if "," in prefix and not is_saga_chapter_prefix(prefix): f.append("comma")
        if len(prefix) > 25: f.append("long>25")
        if re.match(r"(?i)^(if|when|whenever|at|as|you|target|each|choose|the next|until)\b", prefix) and prefix.lower() not in AW: f.append("clause-like")
        if f: flags.append((u, prefix, f))
    P(f"flagged: {len(flags)} (pool {sum(pool(u) for u, _, _ in flags)}); by flag: {Counter(x for _, _, fs in flags for x in fs)}")
    for u, prefix, fs in flags:
        if not pool(u): P(f"  {fs}: prefix={prefix!r} :: {show(u)}")
    P("\n## P3.4 Non-firing em-dash units (false-negative search)")
    dash = [u for u in units if "—" in u["normalized"] and not extract_prefix(u["normalized"])]
    spaced = [u for u in dash if " — " in u["normalized"]]
    P(f"units containing an em dash but no prefix: {len(dash)}; with a spaced ` — `: {len(spaced)}; unspaced only (keyword `Suspend N—{{M}}`-style): {len(dash) - len(spaced)}")
    P("unspaced top templates: " + str(Counter(re.sub(r"\{M\}", "{M}", u["normalized"])[:40] for u in dash if " — " not in u["normalized"]).most_common(8)))
    reasons = defaultdict(list)
    for u in spaced:
        head = u["normalized"].split(" — ", 1)[0]; tail = u["normalized"].split(" — ", 1)[1]
        if not tail.strip(): reasons["empty body (mode header)"].append((u, head))
        elif "." in head or ":" in head: reasons["period/colon before the dash"].append((u, head))
        elif len(head) > 45: reasons["pre-dash text longer than 45"].append((u, head))
        else: reasons["other"].append((u, head))
    for r, v in reasons.items():
        P(f"\n### {r}: {len(v)} (pool {sum(pool(u) for u, _ in v)})")
        heads = Counter(h for _, h in v)
        for h, n in heads.most_common(None if r != "empty body (mode header)" else 12):
            ex = next((u for u, hh in v if hh == h and not pool(u)), None)
            P(f"  {n:4d}  {h[:90]!r}" + (f"  e.g. {show(ex)}" if ex and r != "empty body (mode header)" else ""))
    P("\n## P3.5 Chapter units on Saga faces not classified triggered_ability")
    bad = [u for u, p, _ in cats["chapter (Saga face)"] if u["kind"] != "triggered_ability"]
    P(f"count: {len(bad)}")
    for u in bad:
        if not pool(u): P("  " + show(u))
    if pre:
        P("\n## P3.6 Before/after kind transitions on fired units (matched by oracle_id, face, source_line, unit_text)")
        trans = Counter(); changed = []
        unmatched = 0
        for u, prefix, body in fired:
            p = pre_by_key.get(key(u))
            if p is None: unmatched += 1; continue
            trans[(p["kind"], u["kind"])] += 1
            if p["kind"] != u["kind"]: changed.append((p, u, prefix))
        P(f"matched {len(fired) - unmatched}, unmatched {unmatched}; transitions: " + str({f'{a}->{b}': n for (a, b), n in trans.most_common()}))
        routine = sum(1 for p, u, _ in changed if p["kind"] == "spell_or_static_text" and u["kind"] == "triggered_ability")
        P(f"changed: {len(changed)}; of which the routine hidden-trigger-word recovery spell_or_static_text -> triggered_ability: {routine} (not listed); every other change (non-pool):")
        for p, u, prefix in changed:
            if not pool(u) and not (p["kind"] == "spell_or_static_text" and u["kind"] == "triggered_ability"): P(f"  {p['kind']} -> {u['kind']} :: {show(u)}")
        P(f"changed in pool (count only): {sum(pool(u) for _, u, _ in changed)}")

    # ---------------- P-ATQ-4 ----------------
    isu = [u for u in printed if is_is(face_tl(u))]
    top = [u for u in isu if u["parent_index"] is None]
    P(f"\n## P4.1 Instant/sorcery-face population: {len(isu)} printed units; top-level {len(top)}; roles {Counter(u['role'] for u in isu)}; top-level kinds {Counter(u['kind'] for u in top)}")
    trig = [u for u in top if u["kind"] == "triggered_ability"]
    pos = [u for u in trig if u["role"] == "delayed_trigger"]; neg = [u for u in trig if u["role"] != "delayed_trigger"]
    P(f"top-level triggered_ability: {len(trig)} = role delayed_trigger {len(pos)} (pool {sum(map(pool, pos))}) + role ability {len(neg)} (pool {sum(map(pool, neg))})")
    disagree = [u for u in trig if is_spell_created_delayed_trigger(classification_text(u["normalized"]), face_tl(u)) != (u["role"] == "delayed_trigger")]
    P(f"Python predicate vs dumped role disagreements: {len(disagree)}")
    for u in disagree: P("  " + show(u))
    def tform(t):
        m = TEMPORAL.search(t); return m.group(0).lower() if m else None
    P(f"\n## P4.2 Positives (all {len(pos)}; non-pool listed) by temporal form: {Counter(tform(classification_text(u['normalized'])) for u in pos)}; by decade {dict(sorted(Counter(dec(u) for u in pos).items()))}; multi-face {sum(' // ' in tl.get(u['oracle_id'], '') for u in pos)}")
    for u in sorted(pos, key=lambda r: (yr(r), r["card_name"])):
        if not pool(u):
            z = ZONE_WORDS.findall(u["unit_text"]); P(f"  [{tform(classification_text(u['normalized']))}]{' zone-words=' + str(z) if z else ''} {show(u)}")
    P(f"\n## P4.3 Negatives (top-level I/S triggered_ability keeping role = ability): {len(neg)}")
    def nclass(u):
        t = classification_text(u["normalized"])
        if not TEMPORAL.search(t): return "no stated duration (this turn/this combat/next)"
        if CAST_OR_RESOLVE.search(t): return "excluded: cast/resolve trigger of the spell"
        return "excluded: off-stack evidence (keyword or ~ near zone word)"
    nc = defaultdict(list)
    for u in neg: nc[nclass(u)].append(u)
    for c, v in nc.items():
        P(f"\n### {c}: {len(v)} (pool {sum(map(pool, v))})")
        for u in sorted(v, key=lambda r: (yr(r), r["card_name"])):
            if not pool(u):
                od = OTHER_DURATION.findall(u["unit_text"]); P(f"  {'other-duration=' + str(sorted(set(x if isinstance(x, str) else x[0] for x in od))) + ' ' if od and c.startswith('no') else ''}{show(u)}")
    P("\n## P4.4 False-negative sweep over every I/S-face unit not already role = delayed_trigger")
    cand = []
    for u in isu:
        if u["role"] == "delayed_trigger": continue
        m = mask(u["unit_text"])
        if not TRIGGER_WORD.search(m) or not TEMPORAL.search(m): continue
        if any(c["parent_index"] == u["unit_index"] and c["oracle_id"] == u["oracle_id"] and c["face"] == u["face"] and c["role"] == "delayed_trigger" for c in isu): continue
        if u["parent_index"] is None and u["kind"] == "triggered_ability": continue  # already in P4.3
        cand.append(u)
    def cclass(u):
        m = mask(u["unit_text"]); first = TRIGGER_WORD.search(m)
        if u["role"] == "mode": return "mode child"
        if u["role"] == "granted": return "granted quoted ability"
        if u["parent_index"] is not None: return "child unit"
        if re.match(r"(?i)^(until|this turn|during)", m): return "duration-first trigger (`Until end of turn, whenever ...`)"
        if first and m[:first.start()].rstrip().endswith((",", ";")): return "trigger word after a comma/semicolon mid-sentence"
        if first and m[:first.start()].rstrip().endswith("."): return "sentence-initial trigger word not split by P-ARN-2 (no `this turn`/`this way` in that sentence)"
        return "trigger word elsewhere in spell text"
    cc = defaultdict(list)
    for u in cand: cc[cclass(u)].append(u)
    P(f"candidates: {len(cand)} (pool {sum(map(pool, cand))}); by class: {dict(Counter(cclass(u) for u in cand))}")
    for c, v in cc.items():
        P(f"\n### {c}: {len(v)}; kinds {Counter(u['kind'] for u in v)}")
        v = sorted([u for u in v if not pool(u)], key=lambda r: (yr(r), r["card_name"]))
        step = max(1, len(v) // 25)
        for u in v[::step][:25]: P("  " + show(u))
    P(f"\n## P4.5 I/S-face units carrying the delayed_trigger_unattached_candidate signal: {sum('delayed_trigger_unattached_candidate' in u['signals'] for u in isu)}")
    if pre:
        P("\n## P4.6 Before/after for top-level I/S triggered_ability units")
        t2 = Counter(); moved = []
        for u in trig:
            p = pre_by_key.get(key(u))
            t2[((p or {}).get("kind"), (p or {}).get("role"), u["kind"], u["role"])] += 1
            if p and (p["role"] != u["role"] or p["kind"] != u["kind"]): moved.append((p, u))
        P("transitions (pre kind, pre role) -> (post kind, post role): " + str({f'{a}/{b}->{c}/{d}': n for (a, b, c, d), n in t2.most_common()}))
        for p, u in moved:
            if not pool(u) and (p["kind"] != u["kind"]): P(f"  kind changed {p['kind']}->{u['kind']}: {show(u)}")
        pre_is_trig = [u for u in pre if u["source"] == "printed" and u["parent_index"] is None and u["kind"] == "triggered_ability" and u["role"] == "ability" and is_is(face_tl(u))]
        P(f"pre-change top-level I/S triggered role=ability: {len(pre_is_trig)}; post: {len(neg)} ability + {len(pos)} delayed_trigger")

    # ---------------- binary cross-check ----------------
    if a.mtg:
        P("\n## X. Binary cross-check of the Python prefix rule (`segment --text` on every fired unit and every spaced-em-dash non-firing unit)")
        mism = 0; n = 0
        for u, prefix, _ in fired + [(u, None, None) for u in spaced]:
            if u["source"] != "printed" or u["parent_index"] is not None: continue
            args = [a.mtg, "--db", os.path.join(WT, "cards.sqlite"), "segment", "--text", u["unit_text"], "--name", u["card_name"], "--type-line", face_tl(u)]
            r = subprocess.run(args, capture_output=True, text=True, encoding="utf-8")
            if r.returncode != 0: mism += 1; P(f"  ERROR {u['card_name']}: {r.stderr[:80]}"); continue
            segs = json.loads(r.stdout)["segments"]; n += 1
            got = segs[0].get("prefix") if segs else None
            if got != prefix or (segs and segs[0]["kind"] != u["kind"]):
                mism += 1
                if not pool(u): P(f"  MISMATCH binary prefix={got!r} kind={segs[0]['kind'] if segs else None} vs python prefix={prefix!r} dump kind={u['kind']} :: {show(u)}")
        P(f"checked {n} top-level printed units; mismatches {mism}")

    open(a.out, "w", encoding="utf-8").write("\n".join(out) + "\n")
    print("\n".join(out))


if __name__ == "__main__":
    main()
