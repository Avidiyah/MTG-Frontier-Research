#!/usr/bin/env python3
"""Compute the structural-audit measurements defined in
docs/protocol/structural-investigation-protocol.md from an annotated unit
table (docs/audits/<set>/units-annotated.tsv).

    python scripts/python/audit_metrics.py docs/audits/lea/units-annotated.tsv
    python scripts/python/audit_metrics.py docs/audits/arn/units-annotated.tsv \
        --earlier docs/audits/lea/units-export.tsv --export docs/audits/arn/units-export.tsv

--earlier (repeatable) supplies unit exports of earlier sets for template and
unit novelty. --export supplies the current export to detect drift between the
annotated text and what the segmenter emits now (a changed segmenter
invalidates unit-keyed annotations). Every ratio is printed with its
numerator and denominator. Standard library only; prints one JSON document.
"""

import argparse
import json
import sys
from collections import Counter


def read_tsv(path):
    # Strip CRLF as well as LF: git autocrlf checkouts must not turn the last
    # column into "value\r" and fake template novelty or drift.
    with open(path, encoding="utf-8", newline="") as f:
        rows = [line.rstrip("\r\n").split("\t") for line in f if line.strip()]
    header, body = rows[0], rows[1:]
    return [dict(zip(header, r)) for r in body]


def ratio(num, den):
    return {"numerator": num, "denominator": den, "value": (round(num / den, 4) if den else None)}


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("annotated")
    ap.add_argument("--earlier", action="append", default=[], help="unit export TSV of an earlier set (repeatable)")
    ap.add_argument("--export", help="current unit export TSV of the same set, for drift detection")
    args = ap.parse_args()

    rows = read_tsv(args.annotated)
    printed = [r for r in rows if r["source"] == "printed"]
    supplied = [r for r in rows if r["source"] != "printed"]
    top = [r for r in printed if r["parent_index"] == ""]
    children = [r for r in printed if r["parent_index"] != ""]

    def judged(rs, field):
        return [r for r in rs if r[field] not in ("unsure", "n/a", "")]

    b_ok = [r for r in printed if r["boundary"] == "ok"]
    b_judged = judged(printed, "boundary")
    missed = sum(int(r["missed"] or 0) for r in printed)
    kind_scope = [r for r in b_ok if r["kind_ok"] in ("yes", "no")]
    role_scope = [r for r in b_ok if r["role_ok"] in ("yes", "no")]
    src_scope = [r for r in rows if r["source_ok"] in ("yes", "no")]

    out = {
        "annotated_file": args.annotated,
        "cards": len({r["oracle_id"] for r in rows}),
        "units": {"all": len(rows), "printed": len(printed), "rules_supplied": len(supplied),
                  "printed_top_level": len(top), "printed_children": len(children)},
        "boundary": {
            "precision": ratio(len(b_ok), len(b_judged)),
            "recall": ratio(len(b_ok), len(b_ok) + missed),
            "by_value": dict(Counter(r["boundary"] for r in printed)),
            "missed_boundaries": missed,
            "unsure": len(printed) - len(b_judged),
        },
        "kind_accuracy": ratio(sum(r["kind_ok"] == "yes" for r in kind_scope), len(kind_scope)),
        "kind_not_applicable_or_unsure": len(b_ok) - len(kind_scope),
        "role_accuracy": ratio(sum(r["role_ok"] == "yes" for r in role_scope), len(role_scope)),
        "source_accuracy": ratio(sum(r["source_ok"] == "yes" for r in src_scope), len(src_scope)),
        "dispositions": dict(Counter(r["disposition"] for r in rows)),
        "context_required": dict(Counter(r["context"] for r in rows)),
        "structure_tags": dict(Counter(t for r in rows for t in r["structure_tags"].split(";") if t)),
        "normalization_flags": dict(Counter(t for r in rows for t in r["norm_issue"].split(";") if t)),
        "kind_expected_when_wrong": dict(Counter(r["kind_expected"] for r in rows if r["kind_ok"] == "no")),
        "annotators": dict(Counter(r["annotator"] for r in rows)),
    }

    templates = Counter(r["normalized"] for r in printed)
    out["templates"] = {"distinct": len(templates), "singletons": sum(1 for c in templates.values() if c == 1)}
    if args.earlier:
        earlier = set()
        for path in args.earlier:
            earlier |= {r["normalized"] for r in read_tsv(path) if r["source"] == "printed"}
        novel_units = [r for r in printed if r["normalized"] not in earlier]
        novel_templates = [t for t in templates if t not in earlier]
        out["novelty"] = {
            "earlier_files": args.earlier,
            "earlier_distinct_templates": len(earlier),
            "unit_novelty": ratio(len(novel_units), len(printed)),
            "template_novelty": ratio(len(novel_templates), len(templates)),
        }
    if args.export:
        exported = read_tsv(args.export)
        key_fields = ("oracle_id", "face", "index")
        structural_fields = (
            "set", "oracle_id", "name", "type_line", "index", "parent_index",
            "depth", "face", "line", "kind", "role", "source", "rule", "text",
            "normalized",
        )

        def keyed(rs):
            result = {}
            for row in rs:
                key = tuple(row[field] for field in key_fields)
                if key in result:
                    raise ValueError(f"duplicate unit key in audit data: {key}")
                result[key] = row
            return result

        annotated_by_key = keyed(rows)
        exported_by_key = keyed(exported)
        drift = []
        for key in sorted(set(annotated_by_key) | set(exported_by_key)):
            annotated_row = annotated_by_key.get(key)
            exported_row = exported_by_key.get(key)
            if annotated_row is None:
                drift.append(f"added:{exported_row['name']}#{exported_row['face']}:{exported_row['index']}")
                continue
            if exported_row is None:
                drift.append(f"missing:{annotated_row['name']}#{annotated_row['face']}:{annotated_row['index']}")
                continue
            changed = [
                field for field in structural_fields
                if annotated_row.get(field, "") != exported_row.get(field, "")
            ]
            if changed:
                drift.append(
                    f"changed:{annotated_row['name']}#{annotated_row['face']}:{annotated_row['index']}"
                    f"[{','.join(changed)}]"
                )
        out["drift"] = {
            "export": args.export,
            "changed_or_missing_units": len(drift),
            "examples": drift[:20],
        }

    json.dump(out, sys.stdout, indent=2)
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
