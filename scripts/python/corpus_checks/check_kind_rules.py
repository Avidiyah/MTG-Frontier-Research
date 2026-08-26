import json, os, re, sqlite3, sys
from collections import Counter, defaultdict
sys.stdout.reconfigure(encoding="utf-8")
SP = os.path.dirname(os.path.abspath(__file__))
WT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
units = [json.loads(l) for l in open(sys.argv[1], encoding="utf-8")]
db = sqlite3.connect("file:" + os.path.join(WT, "cards.sqlite") + "?mode=ro", uri=True)
tl = {oid: (t or "") for oid, t in db.execute("SELECT oracle_id, type_line FROM cards")}
def pool(u): return u["oracle_id"].startswith("f")
def face_tl(u):
    t = tl.get(u["oracle_id"], "")
    parts = [p.strip() for p in t.split(" // ")]
    return parts[u["face"]] if u["face"] < len(parts) else parts[0]
def is_is(t): return any(w in ("Instant", "Sorcery") for w in re.split(r"[\s—/]+", t.split("—")[0]))
def yr(u): return (u["first_released_at"] or "????")[:4]
def show(u): return f"[{u['first_set']} {yr(u)} | {face_tl(u)[:28]}] {u['card_name']}: {u['unit_text'][:160]}"
def sample(rows, k):
    rows = sorted([r for r in rows if not pool(r)], key=lambda r: (yr(r), r["card_name"]))
    step = max(1, len(rows) // k); return rows[::step][:k]
out = []
P = out.append
by_key = {(u["oracle_id"], u["face"], u["unit_index"]): u for u in units}
def parent(u):
    return by_key.get((u["oracle_id"], u["face"], u["parent_index"])) if u["parent_index"] is not None else None

# ---------- A ----------
P("# S11 check of P-ARN-2/3/4 (commit af150b0)\n")
prev = [u for u in units if u["kind"] == "prevention_effect"]
P(f"## A. prevention_effect units: {len(prev)} (held-out pool {sum(map(pool, prev))})")
P("### by role/source/face-type")
P(str(Counter((u['role'], u['source'], 'I/S' if is_is(face_tl(u)) else 'perm') for u in prev)))
P("### distinct templates")
for t, n in Counter(u["normalized"] for u in prev).most_common(): P(f"  {n:3d}  {t}")
# heuristics for judging: static prevention = role ability, printed, not I/S face, not activated/triggered wording
def judge_prev(u):
    t = u["unit_text"]
    if u["role"] == "mode": return "mode (kind n/a)"
    if u["role"] == "granted": return "granted quoted (ok if quoted text is static)"
    if u["source"] != "printed": return "rules_supplied"
    if is_is(face_tl(u)): return "spell text on I/S face"
    if re.match(r"^\s*(\{[^}]+\}|[A-Z][^:]{0,60}):", t) and ":" in t.split(".")[0]: return "activated wording"
    if re.match(r"(?i)^(when|whenever|at the beginning)", t): return "triggered wording"
    if re.search(r"(?i)\binstead\b", t) and not re.search(r"(?i)\bprevent", t): return "replacement not prevention"
    if not re.search(r"(?i)\bprevent", t): return "no 'prevent' word"
    return "ok"
jc = Counter(judge_prev(u) for u in prev)
P("### judgement classes"); P(str(jc))
P("### non-ok examples (non-pool)")
for u in prev:
    j = judge_prev(u)
    if j != "ok" and not pool(u): P(f"  {j}: {show(u)}")
# misses: static residual containing 'prevent'
miss = [u for u in units if u["kind"] == "spell_or_static_text" and u["role"] == "ability" and u["source"] == "printed"
        and re.search(r"(?i)\bprevent", u["unit_text"]) and not is_is(face_tl(u))]
P(f"\n### residual statics on permanents containing 'prevent' (possible misses): {len(miss)} (pool {sum(map(pool, miss))})")
for u in sample(miss, 20): P("  " + show(u))

# ---------- B ----------
P("\n## B. Instant/Sorcery faces")
isu = [u for u in units if is_is(face_tl(u)) and u["source"] == "printed"]
P(f"units on I/S faces: {len(isu)}; kinds by role=ability top-level: {Counter(u['kind'] for u in isu if u['role']=='ability' and u['parent_index'] is None)}")
P(f"all roles: {Counter(u['role'] for u in isu)}")
odd = [u for u in isu if u["role"] == "ability" and u["parent_index"] is None and u["kind"] in ("replacement_effect", "prevention_effect", "characteristic_defining_ability", "triggered_ability")]
P(f"### top-level I/S units with replacement/prevention/CDA/triggered kinds: {len(odd)} (pool {sum(map(pool, odd))})")
P(str(Counter(u['kind'] for u in odd)))
for u in sorted(odd, key=lambda r: (r["kind"], yr(r), r["card_name"])):
    if not pool(u): P(f"  {u['kind']}: {show(u)}")
mf = [u for u in isu if " // " in tl.get(u["oracle_id"], "")]
P(f"\n### I/S faces on multi-face cards: {len(mf)} units; kinds {Counter(u['kind'] for u in mf if u['role']=='ability')}")
for u in sample([u for u in mf if u["role"] == "ability"], 20): P(f"  face{u['face']} {u['kind']}: {show(u)}  || full type: {tl.get(u['oracle_id'])}")
# instant/sorcery faces with kind replacement despite rule = check whether whole-card type line contains I/S at all
# also: permanent faces on I/S-fronted DFC classified as spell? count units whose whole type mentions I/S but face is permanent and kind replacement
perm_face_on_is_card = [u for u in units if " // " in tl.get(u["oracle_id"], "") and not is_is(face_tl(u)) and is_is(tl[u["oracle_id"]]) and u["kind"] in ("replacement_effect", "prevention_effect")]
P(f"permanent faces on cards with an I/S face keeping replacement/prevention kinds: {len(perm_face_on_is_card)}")
for u in sample(perm_face_on_is_card, 6): P("  " + show(u))

# ---------- C ----------
P("\n## C. delayed_trigger children starting with When/Whenever")
dt = [u for u in units if u["role"] == "delayed_trigger"]
wh = [u for u in dt if re.match(r"(?i)^(when|whenever)\b", u["unit_text"])]
P(f"delayed_trigger children total: {len(dt)}; When/Whenever-initial: {len(wh)} (pool {sum(map(pool, wh))})")
P("forms: " + str(Counter("when you do" if re.match(r"(?i)^when you do", u["unit_text"]) else ("this way" if "this way" in u["unit_text"] else ("this turn" if "this turn" in u["unit_text"] else "other")) for u in wh)))
P("by decade: " + str(Counter(yr(u)[:3] + "0s" for u in wh)))
P("parent kinds: " + str(Counter((parent(u) or {}).get("kind") for u in wh)))
P("### sample 40 (parent | child)")
for u in sample(wh, 40):
    p = parent(u)
    P(f"  [{u['first_set']} {yr(u)}] {u['card_name']}: PARENT<{(p or {}).get('unit_text','?')[:90]}> CHILD<{u['unit_text'][:120]}>")
# other-form children (not When): decade + sample of comma-fragment parents
frag = [u for u in dt if (parent(u) or {}).get("unit_text", "").rstrip().endswith((",", ":"))]
P(f"\n(non-scope note) delayed children whose parent ends with ',' or ':' : {len(frag)} of {len(dt)}")

# ---------- D ----------
P("\n## D. unsplit top-level units with '. When you do' or '. When ... this turn' outside quotes")
def mask(t): return re.sub(r'"[^"]*"', lambda m: "_" * len(m.group(0)), t)
res = [u for u in units if u["role"] == "ability" and u["source"] == "printed" and u["parent_index"] is None
       and re.search(r"\. When (you do|[^.]*\bthis (turn|way)\b)", mask(u["unit_text"]))]
P(f"count: {len(res)} (pool {sum(map(pool, res))})")
for u in sample(res, 10): P("  " + show(u))
open(sys.argv[2], "w", encoding="utf-8").write("\n".join(out) + "\n")
print("\n".join(out))
