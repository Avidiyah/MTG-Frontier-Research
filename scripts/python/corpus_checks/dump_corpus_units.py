"""Dump every structural unit of every first-printing set via `audit export` into one JSONL."""
import json, subprocess, sys, os, time
WT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
M = os.path.join(WT, "target", "release", "mtg-discover.exe")
OUT = sys.argv[1] if len(sys.argv) > 1 else os.path.join(WT, "corpus-units.jsonl")
os.chdir(WT)
def run(*args):
    p = subprocess.run([M, *args], capture_output=True, text=True, encoding="utf-8")
    if p.returncode != 0:
        sys.stderr.write(f"ERR {args}: {p.stderr[:200]}\n"); return None
    return json.loads(p.stdout)
sets = run("sets")["sets"]
t0 = time.time(); n = 0
with open(OUT, "w", encoding="utf-8") as f:
    for s in sets:
        d = run("audit", "export", s["set"])
        if d is None: continue
        recs = next(v for v in d.values() if isinstance(v, list))
        for r in recs:
            r["set_type"] = s["type"]
            f.write(json.dumps(r, ensure_ascii=False) + "\n"); n += 1
print("sets", len(sets), "units", n, "seconds", round(time.time() - t0), "->", OUT)
