#!/usr/bin/env python3
"""Export every structural unit of one first-printing set as a flat TSV.

Interim research tool: it drives the existing ``mtg-discover`` CLI (``cards
--set`` to enumerate the set, ``segment --card`` per card) and flattens the
segment tree so that each unit is one row with a parent pointer. It exists so
that a set's audit inventory is reproducible and diffable. It adds no
segmentation logic of its own; when the CLI grows a native export, prefer that.

Usage (from the repository root, after ``cargo build --release``):

    python scripts/python/export_units.py lea > docs/audits/lea/units-export.tsv
    python scripts/python/export_units.py lea --mtg path/to/mtg-discover.exe

Rows are sorted by card name, then pre-order unit index, so the output is
deterministic for a fixed database and segmenter. Standard library only.
"""

import argparse
import json
import os
import subprocess
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
DEFAULT_MTG = os.path.join(REPO_ROOT, "target", "release", "mtg-discover.exe")
COLUMNS = [
    "set", "oracle_id", "name", "type_line", "index", "parent_index", "depth",
    "face", "line", "kind", "role", "source", "rule", "text", "normalized",
]


def run_json(mtg, db, args):
    cmd = [mtg]
    if db:
        cmd += ["--db", db]
    cmd += args
    proc = subprocess.run(cmd, capture_output=True, text=True, encoding="utf-8")
    if proc.returncode != 0:
        sys.stderr.write(proc.stderr)
        raise SystemExit(f"command failed: {' '.join(cmd)}")
    return json.loads(proc.stdout)


def list_set(mtg, db, set_code, page=200):
    cards, offset = [], 0
    while True:
        batch = run_json(mtg, db, ["cards", "", "--set", set_code, "--limit", str(page), "--offset", str(offset)])
        cards.extend(batch["cards"])
        if batch["count"] < page:
            return cards
        offset += page


def flatten(card, set_code, segments, parent_index=None, depth=0):
    for seg in segments:
        yield {
            "set": set_code,
            "oracle_id": card["oracle_id"],
            "name": card["name"],
            "type_line": card["type_line"],
            "index": seg["index"],
            "parent_index": "" if parent_index is None else parent_index,
            "depth": depth,
            "face": seg["face"],
            "line": seg["line"],
            "kind": seg["kind"],
            "role": seg["role"],
            "source": seg["source"],
            "rule": seg.get("rule", ""),
            "text": seg["text"],
            "normalized": seg["normalized"],
        }
        yield from flatten(card, set_code, seg.get("children", []), seg["index"], depth + 1)


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("set_code", help="first-printing set code, e.g. lea")
    parser.add_argument("--mtg", default=DEFAULT_MTG, help="path to the mtg-discover binary")
    parser.add_argument("--db", default=None, help="path to cards.sqlite (default: CLI default)")
    args = parser.parse_args()

    cards = sorted(list_set(args.mtg, args.db, args.set_code), key=lambda c: (c["name"], c["oracle_id"]))
    rows = []
    for card in cards:
        if not card.get("oracle_text"):
            continue
        result = run_json(args.mtg, args.db, ["segment", "--card", card["name"]])
        rows.extend(flatten(card, args.set_code, result["segments"]))

    # Oracle text contains em dashes and curly quotes; never let the console
    # codepage decide how they are written.
    sys.stdout.reconfigure(encoding="utf-8", newline="\n")
    out = sys.stdout
    out.write("\t".join(COLUMNS) + "\n")
    for row in rows:
        out.write("\t".join(str(row[c]).replace("\t", " ").replace("\n", " ") for c in COLUMNS) + "\n")
    sys.stderr.write(f"{len(cards)} cards, {len(rows)} units\n")


if __name__ == "__main__":
    main()
