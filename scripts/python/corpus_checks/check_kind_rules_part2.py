import json, os, re, sqlite3, sys
from collections import Counter
sys.stdout.reconfigure(encoding="utf-8")
SP = os.path.dirname(os.path.abspath(__file__))
WT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
units = [json.loads(l) for l in open(sys.argv[1], encoding="utf-8")]
db = sqlite3.connect("file:" + os.path.join(WT, "cards.sqlite") + "?mode=ro", uri=True)
tl = {oid: (t or "") for oid, t in db.execute("SELECT oracle_id, type_line FROM cards")}
def pool(u): return u["oracle_id"].startswith("f")
def face_tl(u):
    parts = [p.strip() for p in tl.get(u["oracle_id"], "").split(" // ")]
    return parts[u["face"]] if u["face"] < len(parts) else parts[0]
def is_is(t): return any(w in ("Instant", "Sorcery") for w in re.split(r"[\s—/]+", t.split("—")[0]))
def yr(u): return (u["first_released_at"] or "????")[:4]
def show(u): return f"[{u['first_set']} {yr(u)} | {face_tl(u)[:26]}] {u['card_name']}: {u['unit_text'][:150]}"
out = []; P = out.append
P("\n## A2. prevention_effect role=ability misfire subclasses (of 161 role=ability units)")
prev = [u for u in units if u["kind"] == "prevention_effect" and u["role"] == "ability"]
cant = [u for u in prev if re.search(r"(?i)can't be prevented", u["unit_text"])]
prefixed = [u for u in prev if re.match(r"^[^—.:]{1,45} — ", u["unit_text"])]
trig_word = [u for u in prev if re.match(r"(?i)^(when|whenever|at the beginning)", u["unit_text"])]
P(f"'can't be prevented' statics (not prevention effects; rule-modifying statics): {len(cant)} (pool {sum(map(pool, cant))})")
for u in cant:
    if not pool(u): P("  " + show(u))
P(f"ability-word / chapter / named-mode prefixed units ('X — ...'): {len(prefixed)} (pool {sum(map(pool, prefixed))})")
for u in prefixed:
    if not pool(u): P("  " + show(u) + f"  || signals={u.get('signals')}")
P(f"trigger-word-initial units: {len(trig_word)}")
ok = [u for u in prev if u not in cant and u not in prefixed and u not in trig_word]
P(f"remaining role=ability prevention units judged CR 615.1a statics: {len(ok)} / 161")
# manual scan of remaining for non-prevention wording
odd = [u for u in ok if not re.search(r"(?i)\bprevent", u["unit_text"])]
P(f"  of which lacking the word 'prevent': {len(odd)}")
for u in odd[:10]: P("  " + show(u))

P("\n## B2. I/S-face top-level triggered_ability subclasses (of 111)")
isu = [u for u in units if is_is(face_tl(u)) and u["source"] == "printed" and u["role"] == "ability" and u["parent_index"] is None and u["kind"] == "triggered_ability"]
def cls(u):
    t = u["unit_text"]
    if re.search(r"(?i)\b(this turn|this combat)\b|\bnext\b", t) and not re.search(r"(?i)\bcycle\b|graveyard|discard this card|exiled|suspended|haunts", t): return "delayed trigger created by the spell (603.7d)"
    if re.search(r"(?i)^when you cast this spell|cast this spell from|^whenever .* is countered|^when this spell resolves", t): return "cast/resolve trigger of the spell (triggered ability)"
    return "off-stack triggered ability of the card (113.6b: cycle/discard/graveyard/exile/haunt/suspend)"
c = Counter(cls(u) for u in isu); P(str(c))
for k in c:
    P(f"### {k}")
    for u in [x for x in isu if cls(x) == k][:12]:
        if not pool(u): P("  " + show(u))
open(sys.argv[2], "a", encoding="utf-8").write("\n".join(out) + "\n")
print("\n".join(out))
