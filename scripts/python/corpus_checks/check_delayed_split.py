import json, re, sys, os
from collections import Counter, defaultdict
sys.stdout.reconfigure(encoding="utf-8")
SP = os.path.dirname(os.path.abspath(__file__))
recs = [json.loads(l) for l in open(sys.argv[1], encoding="utf-8") if l.strip()]
printed = [r for r in recs if r["source"] == "printed"]
by_key = {(r["oracle_id"], r["face"], r["unit_index"]): r for r in recs}
def pool(r): return r["oracle_id"].startswith("f")
def dec(r): return (r["first_released_at"] or "????")[:3] + "0s"
def lab(r): return f"[{r['first_set']} {(r['first_released_at'] or '????')[:4]}] {r['card_name']}"
out = []
def P(s=""): out.append(s)

dt = [r for r in printed if r["role"] == "delayed_trigger"]
LABEL = sys.argv[3] if len(sys.argv) > 3 else "HEAD"
P(f"# Over-segmentation check: delayed-trigger splits (commit {LABEL})\n")
P(f"Printed units: {len(printed)}; delayed_trigger children: {len(dt)}/{len(printed)}")
pairs = []
for c in dt:
    p = by_key.get((c["oracle_id"], c["face"], c["parent_index"]))
    pairs.append((p, c))
missing_parent = sum(1 for p, c in pairs if p is None)
P(f"Children with resolvable parent: {len(pairs)-missing_parent}/{len(pairs)}")

def cls(p, c):
    pt = p["unit_text"].rstrip()
    ct = c["unit_text"].lstrip()
    if pt.endswith(","): return "comma"
    if pt.endswith(":"): return "colon"
    if pt.endswith((".", '"', ")", "!", "?")) and ct[:1].isupper(): return "sentence"
    return "other"
classes = defaultdict(list)
for p, c in pairs:
    if p: classes[cls(p, c)].append((p, c))
P("\n## 1. Split classes")
for k in ("sentence", "comma", "colon", "other"):
    v = classes[k]
    P(f"- {k}: {len(v)}/{len(dt)}; by decade: {dict(sorted(Counter(dec(c) for _, c in v).items()))}")
P("\nOther-class examples (non-pool, up to 10):")
for p, c in [x for x in classes["other"] if not pool(x[1])][:10]:
    P(f"  - {lab(c)}: PARENT `{p['unit_text'][-80:]}` | CHILD `{c['unit_text'][:100]}`")

P("\n## 2. Comma/colon-level parents")
cc = classes["comma"] + classes["colon"]
tmpl = Counter(p["normalized"] for p, c in cc)
P(f"Distinct parent templates: {len(tmpl)} over {len(cc)} splits")
for t, n in tmpl.most_common(60): P(f"  {n:3d}  {t}")
def pclass(p, c):
    pt = p["unit_text"].strip()
    if re.match(r"(?i)^(when|whenever|at)\b", pt) and pt.endswith(","):
        # condition only if no effect verb after the first comma? treat 'When/Whenever/At ... ,' with a single clause as condition-only
        inner = pt[:-1]
        # condition-only when there is no earlier comma-separated effect (heuristic: no ', ' after a trigger-word-led clause that contains a verb like 'you may'/'put'/'create'...)
        return "condition_only" if inner.count(", ") == 0 or re.match(r"(?i)^(when|whenever|at the beginning of|at end of) [^,]*,( if [^,]*,)?$", pt) else "condition_plus"
    if re.match(r"^\{[^}]+\}(, ?\{[^}]+\})*:$", pt) or re.match(r"^[^:]{0,60}:$", pt): return "cost_only"
    return "effect_fragment"
pc = defaultdict(list)
for p, c in cc: pc[pclass(p, c)].append((p, c))
for k in ("condition_only", "condition_plus", "cost_only", "effect_fragment"):
    v = pc[k]
    P(f"\n### {k}: {len(v)}/{len(cc)}")
    for p, c in [x for x in v if not pool(x[1])][:10]:
        P(f"  - {lab(c)}: PARENT `{p['unit_text'][:110]}` | CHILD `{c['unit_text'][:110]}`")

P("\n## 3. Judgement sample (40 comma/colon splits)")
srt = sorted([x for x in cc if not pool(x[1])], key=lambda x: (x[1]["card_name"], x[1]["unit_index"]))
k = max(1, len(srt)//40)
sample = srt[::k][:40]
for p, c in sample:
    P(f"  - {lab(c)}: PARENT `{p['unit_text'][:120]}` | CHILD `{c['unit_text'][:140]}`")

P("\n## 4. Children beginning lowercase or with and/or/then")
low = [(p, c) for p, c in pairs if p and re.match(r"^(and |or |then |[a-z])", c["unit_text"].lstrip())]
P(f"Count: {len(low)}/{len(dt)}; starting with and/or/then: {sum(1 for p,c in low if re.match(r'^(and |or |then )', c['unit_text'].lstrip()))}")
lowt = Counter(c["normalized"][:60] for p, c in low)
for t, n in lowt.most_common(12): P(f"  {n:3d}  {t}")
P("Examples (non-pool, up to 15, preferring and/or/then):")
ex = sorted([x for x in low if not pool(x[1])], key=lambda x: (0 if re.match(r'^(and |or |then )', x[1]['unit_text'].lstrip()) else 1, x[1]['card_name']))
for p, c in ex[:15]:
    P(f"  - {lab(c)}: PARENT `{p['unit_text'][-90:]}` | CHILD `{c['unit_text'][:110]}`")

P("\n## 5. Sentence-level sample (30 across decades)")
sent = sorted([x for x in classes["sentence"] if not pool(x[1])], key=lambda x: (x[1]["first_released_at"] or "", x[1]["card_name"]))
k = max(1, len(sent)//30)
for p, c in sent[::k][:30]:
    P(f"  - {lab(c)}: CHILD `{c['unit_text'][:150]}`")

P("\n## 6. Residual misses")
pat = re.compile(r"(?i)at the beginning of the next|at end of combat|when you do")
def has_dt_child(r):
    return any(x["parent_index"] == r["unit_index"] and x["oracle_id"] == r["oracle_id"] and x["face"] == r["face"] and x["role"] == "delayed_trigger" for x in children[(r["oracle_id"], r["face"])])
children = defaultdict(list)
for r in recs:
    if r["parent_index"] is not None: children[(r["oracle_id"], r["face"])].append(r)
res = [r for r in printed if r["role"] != "delayed_trigger" and pat.search(r["unit_text"]) and not has_dt_child(r)]
P(f"Count: {len(res)}/{len(printed)}")
res_start = [r for r in res if re.match(r"(?i)^(at the beginning of the next|at end of combat)", r["unit_text"].strip())]
P(f"  of which phrase at unit start (top-level trigger, expected not split): {len(res_start)}")
P(f"  by kind: {dict(Counter(r['kind'] for r in res))}; by role: {dict(Counter(r['role'] for r in res))}")
for r in [x for x in res if not pool(x) and not re.match(r'(?i)^(at the beginning of the next|at end of combat)', x['unit_text'].strip())][:12]:
    P(f"  - {lab(r)} [{r['kind']}/{r['role']}]: `{r['unit_text'][:160]}`")

P("\n## 7. Recurring `At end of combat, if ...` top-level triggers")
rec = [r for r in printed if re.match(r"(?i)^at end of combat, if", r["unit_text"].strip())]
P(f"Count: {len(rec)}; role=ability: {sum(1 for r in rec if r['role']=='ability')}; with delayed child: {sum(1 for r in rec if has_dt_child(r))}")
for r in [x for x in rec if not pool(x)][:5]: P(f"  - {lab(r)}: `{r['unit_text'][:120]}`")
rec2 = [r for r in printed if re.match(r"(?i)^at end of combat", r["unit_text"].strip()) and r["role"] == "ability"]
P(f"All top-level units starting `At end of combat`: {len(rec2)}")

open(sys.argv[2], "w", encoding="utf-8").write("\n".join(out) + "\n")
print("\n".join(out))
