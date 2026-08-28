#!/usr/bin/env python3
"""Compute reproducible structural-audit measurements from annotated TSVs.

    python scripts/python/audit_metrics.py docs/audits/lea/units-annotated.tsv
    python scripts/python/audit_metrics.py docs/audits/arn/units-annotated.tsv \
        --earlier docs/audits/lea/units-export.tsv \
        --export docs/audits/arn/units-export.tsv
    python scripts/python/audit_metrics.py docs/audits/atq/units-annotated.tsv \
        --compare docs/audits/atq/units-annotated-pass2.tsv

``--earlier`` (repeatable) supplies earlier unit exports for template and unit
novelty. ``--export`` detects structural drift between an annotation and the
current export. ``--compare`` aligns two independent annotation passes and
reports row-level and exact-card agreement. Standard library only; successful
runs print one JSON document. Every ratio includes its numerator and
denominator.
"""

import argparse
import json
import sys
from collections import Counter, defaultdict


KEY_FIELDS = ("oracle_id", "face", "index")
LEGACY_STRUCTURAL_FIELDS = (
    "set",
    "oracle_id",
    "name",
    "type_line",
    "index",
    "parent_index",
    "depth",
    "face",
    "line",
    "kind",
    "role",
    "source",
    "rule",
    "text",
    "normalized",
)
STRUCTURAL_FIELDS = (
    *LEGACY_STRUCTURAL_FIELDS[:-2],
    "prefix",
    *LEGACY_STRUCTURAL_FIELDS[-2:],
)
JUDGEMENT_FIELDS = (
    "boundary",
    "missed",
    "kind_expected",
    "kind_ok",
    "role_ok",
    "source_ok",
    "context",
    "disposition",
)
SUPPLEMENTAL_SET_FIELDS = ("cr_ref", "structure_tags")
SUPPLEMENTAL_TEXT_FIELDS = ("note",)
ANNOTATION_FIELDS = (
    *STRUCTURAL_FIELDS,
    *JUDGEMENT_FIELDS,
    *SUPPLEMENTAL_SET_FIELDS,
    "norm_issue",
    "annotator",
    *SUPPLEMENTAL_TEXT_FIELDS,
)
LEGACY_ANNOTATION_FIELDS = (
    *LEGACY_STRUCTURAL_FIELDS,
    *JUDGEMENT_FIELDS,
    *SUPPLEMENTAL_SET_FIELDS,
    "norm_issue",
    "annotator",
    *SUPPLEMENTAL_TEXT_FIELDS,
)


def parse_tsv_lines(lines, label):
    """Parse strict TSV lines while accepting either LF or CRLF."""
    raw_rows = [line.rstrip("\r\n").split("\t") for line in lines if line.strip()]
    if not raw_rows:
        raise ValueError(f"empty TSV: {label}")

    header, body = raw_rows[0], raw_rows[1:]
    duplicate_columns = sorted(name for name, count in Counter(header).items() if count > 1)
    if duplicate_columns:
        raise ValueError(f"duplicate TSV columns in {label}: {duplicate_columns}")

    rows = []
    for line_number, values in enumerate(body, start=2):
        if len(values) != len(header):
            raise ValueError(
                f"TSV row width mismatch in {label}:{line_number}: "
                f"expected {len(header)} columns, found {len(values)}"
            )
        rows.append(dict(zip(header, values)))
    return rows


def read_tsv(path):
    """Read a strict UTF-8 TSV while accepting either LF or CRLF."""
    with open(path, encoding="utf-8", newline="") as file:
        return parse_tsv_lines(file, path)


def require_fields(rows, fields, label):
    if not rows:
        raise ValueError(f"no annotation rows in {label}")
    missing = sorted(set(fields) - set(rows[0]))
    if missing:
        raise ValueError(f"missing required columns in {label}: {missing}")


def ratio(num, den):
    return {
        "numerator": num,
        "denominator": den,
        "value": (round(num / den, 4) if den else None),
    }


def parse_missed(row):
    raw = row["missed"] or "0"
    try:
        value = int(raw)
    except ValueError as exc:
        raise ValueError(
            f"invalid missed value {raw!r} for stable key {stable_key(row)}"
        ) from exc
    if value < 0:
        raise ValueError(f"negative missed value {raw!r} for stable key {stable_key(row)}")
    return value


def stable_key(row):
    return tuple(row[field] for field in KEY_FIELDS)


def key_sort_value(key):
    def numeric(value):
        try:
            return (0, int(value))
        except ValueError:
            return (1, value)

    return (key[0], numeric(key[1]), numeric(key[2]))


def key_record(key):
    return dict(zip(KEY_FIELDS, key))


def index_rows(rows):
    indexed = {}
    counts = Counter(stable_key(row) for row in rows)
    for row in rows:
        indexed.setdefault(stable_key(row), row)
    duplicates = [
        {**key_record(key), "occurrences": count}
        for key, count in sorted(counts.items(), key=lambda item: key_sort_value(item[0]))
        if count > 1
    ]
    return indexed, duplicates


def card_is_structurally_exact(rows):
    """Apply the preregistered exact-card structural correctness definition."""
    for row in rows:
        if row["source"] == "printed":
            if row["boundary"] != "ok" or parse_missed(row) != 0:
                return False

        # Kind and role are evaluated only when the emitted boundary itself is
        # usable. ``n/a`` is an explicit non-applicable kind judgement, not an
        # incorrect one. Role has no n/a value in the frozen schema.
        if row["boundary"] == "ok":
            if row["kind_ok"] not in ("yes", "n/a"):
                return False
            if row["role_ok"] != "yes":
                return False

        # Source correctness applies to printed and rules-supplied units.
        if row["source_ok"] != "yes":
            return False
    return True


def structural_exact_card_correctness(rows):
    by_card = defaultdict(list)
    for row in rows:
        by_card[row["oracle_id"]].append(row)
    exact = sum(card_is_structurally_exact(card_rows) for card_rows in by_card.values())
    return ratio(exact, len(by_card))


def keyed_rows(rows, label):
    indexed, duplicates = index_rows(rows)
    if duplicates:
        examples = duplicates[:5]
        raise ValueError(f"duplicate unit keys in {label}: {examples}")
    return indexed


def drift_report(annotated, exported, export_path):
    annotated_by_key = keyed_rows(annotated, "annotated data")
    exported_by_key = keyed_rows(exported, export_path)
    drift = []
    for key in sorted(set(annotated_by_key) | set(exported_by_key), key=key_sort_value):
        annotated_row = annotated_by_key.get(key)
        exported_row = exported_by_key.get(key)
        if annotated_row is None:
            drift.append(
                f"added:{exported_row['name']}#{exported_row['face']}:{exported_row['index']}"
            )
            continue
        if exported_row is None:
            drift.append(
                f"missing:{annotated_row['name']}#{annotated_row['face']}:{annotated_row['index']}"
            )
            continue
        changed = [
            field
            for field in STRUCTURAL_FIELDS
            if annotated_row.get(field, "") != exported_row.get(field, "")
        ]
        if changed:
            drift.append(
                f"changed:{annotated_row['name']}#{annotated_row['face']}:{annotated_row['index']}"
                f"[{','.join(changed)}]"
            )
    return {
        "export": export_path,
        "changed_or_missing_units": len(drift),
        "examples": drift[:20],
    }


def compute_metrics(rows, annotated_path, earlier_paths=None, exported=None, export_path=None):
    export_has_prefix = bool(exported and "prefix" in exported[0])
    required_fields = ANNOTATION_FIELDS if export_has_prefix else LEGACY_ANNOTATION_FIELDS
    require_fields(rows, required_fields, annotated_path)
    earlier_paths = earlier_paths or []
    printed = [row for row in rows if row["source"] == "printed"]
    supplied = [row for row in rows if row["source"] != "printed"]
    top = [row for row in printed if row["parent_index"] == ""]
    children = [row for row in printed if row["parent_index"] != ""]

    def judged(selected, field):
        return [row for row in selected if row[field] not in ("unsure", "n/a", "")]

    boundary_ok = [row for row in printed if row["boundary"] == "ok"]
    boundary_judged = judged(printed, "boundary")
    missed = sum(parse_missed(row) for row in printed)
    kind_scope = [row for row in boundary_ok if row["kind_ok"] in ("yes", "no")]
    role_scope = [row for row in boundary_ok if row["role_ok"] in ("yes", "no")]
    source_scope = [row for row in rows if row["source_ok"] in ("yes", "no")]

    out = {
        "annotated_file": annotated_path,
        "cards": len({row["oracle_id"] for row in rows}),
        "units": {
            "all": len(rows),
            "printed": len(printed),
            "rules_supplied": len(supplied),
            "printed_top_level": len(top),
            "printed_children": len(children),
        },
        "boundary": {
            "precision": ratio(len(boundary_ok), len(boundary_judged)),
            "recall": ratio(len(boundary_ok), len(boundary_ok) + missed),
            "by_value": dict(Counter(row["boundary"] for row in printed)),
            "missed_boundaries": missed,
            "unsure": len(printed) - len(boundary_judged),
        },
        "kind_accuracy": ratio(
            sum(row["kind_ok"] == "yes" for row in kind_scope), len(kind_scope)
        ),
        "kind_not_applicable_or_unsure": len(boundary_ok) - len(kind_scope),
        "role_accuracy": ratio(
            sum(row["role_ok"] == "yes" for row in role_scope), len(role_scope)
        ),
        "source_accuracy": ratio(
            sum(row["source_ok"] == "yes" for row in source_scope), len(source_scope)
        ),
        "structural_exact_card_correctness": structural_exact_card_correctness(rows),
        "dispositions": dict(Counter(row["disposition"] for row in rows)),
        "context_required": dict(Counter(row["context"] for row in rows)),
        "structure_tags": dict(
            Counter(
                tag
                for row in rows
                for tag in row["structure_tags"].split(";")
                if tag
            )
        ),
        "normalization_flags": dict(
            Counter(
                tag for row in rows for tag in row["norm_issue"].split(";") if tag
            )
        ),
        "kind_expected_when_wrong": dict(
            Counter(row["kind_expected"] for row in rows if row["kind_ok"] == "no")
        ),
        "annotators": dict(Counter(row["annotator"] for row in rows)),
    }

    templates = Counter(row["normalized"] for row in printed)
    out["templates"] = {
        "distinct": len(templates),
        "singletons": sum(1 for count in templates.values() if count == 1),
    }
    if earlier_paths:
        earlier = set()
        for path in earlier_paths:
            earlier |= {
                row["normalized"]
                for row in read_tsv(path)
                if row["source"] == "printed"
            }
        novel_units = [row for row in printed if row["normalized"] not in earlier]
        novel_templates = [template for template in templates if template not in earlier]
        out["novelty"] = {
            "earlier_files": earlier_paths,
            "earlier_distinct_templates": len(earlier),
            "unit_novelty": ratio(len(novel_units), len(printed)),
            "template_novelty": ratio(len(novel_templates), len(templates)),
        }
    if exported is not None:
        out["drift"] = drift_report(rows, exported, export_path)
    return out


def structural_differences(left_by_key, right_by_key, shared_keys):
    differences = []
    for key in sorted(shared_keys, key=key_sort_value):
        left = left_by_key[key]
        right = right_by_key[key]
        changed = {
            field: {"left": left.get(field, ""), "right": right.get(field, "")}
            for field in STRUCTURAL_FIELDS
            if left.get(field, "") != right.get(field, "")
        }
        if changed:
            differences.append(
                {
                    "key": key_record(key),
                    "name": left.get("name", right.get("name", "")),
                    "fields": changed,
                }
            )
    return differences


def confusion_records(left_by_key, right_by_key, keys, field):
    counts = Counter((left_by_key[key][field], right_by_key[key][field]) for key in keys)
    return [
        {"left": left, "right": right, "count": count}
        for (left, right), count in sorted(counts.items())
    ]


def compare_set_field(left_by_key, right_by_key, keys, field):
    exact = 0
    shared_values = 0
    left_only_values = 0
    right_only_values = 0
    differences = []
    for key in keys:
        left = {value for value in left_by_key[key][field].split(";") if value}
        right = {value for value in right_by_key[key][field].split(";") if value}
        if left == right:
            exact += 1
        else:
            differences.append(
                {
                    "key": key_record(key),
                    "name": left_by_key[key]["name"],
                    "left_only": sorted(left - right),
                    "right_only": sorted(right - left),
                }
            )
        shared_values += len(left & right)
        left_only_values += len(left - right)
        right_only_values += len(right - left)
    return {
        "exact_row_agreement": ratio(exact, len(keys)),
        "shared_values": shared_values,
        "left_only_values": left_only_values,
        "right_only_values": right_only_values,
        "differences": differences,
    }


def compare_text_field(left_by_key, right_by_key, keys, field):
    categories = Counter()
    differences = []
    exact = 0
    for key in keys:
        left = left_by_key[key][field]
        right = right_by_key[key][field]
        if left == right:
            exact += 1
            categories["both_blank" if not left else "equal_nonblank"] += 1
            continue
        if left and right:
            category = "different_nonblank"
        elif left:
            category = "left_only_nonblank"
        else:
            category = "right_only_nonblank"
        categories[category] += 1
        differences.append(
            {
                "key": key_record(key),
                "name": left_by_key[key]["name"],
                "category": category,
            }
        )
    return {
        "exact_row_agreement": ratio(exact, len(keys)),
        "by_category": dict(sorted(categories.items())),
        "differences": differences,
    }


def compare_annotation_passes(left_rows, right_rows, left_path, right_path):
    require_fields(left_rows, ANNOTATION_FIELDS, left_path)
    require_fields(right_rows, ANNOTATION_FIELDS, right_path)
    left_by_key, left_duplicates = index_rows(left_rows)
    right_by_key, right_duplicates = index_rows(right_rows)
    left_keys = set(left_by_key)
    right_keys = set(right_by_key)
    shared_keys = left_keys & right_keys
    missing_from_left = sorted(right_keys - left_keys, key=key_sort_value)
    missing_from_right = sorted(left_keys - right_keys, key=key_sort_value)
    structural_drift = structural_differences(left_by_key, right_by_key, shared_keys)

    result = {
        "left_file": left_path,
        "right_file": right_path,
        "judgement_fields": list(JUDGEMENT_FIELDS),
        "valid": not (
            left_duplicates
            or right_duplicates
            or missing_from_left
            or missing_from_right
            or structural_drift
        ),
        "key_integrity": {
            "left_rows": len(left_rows),
            "right_rows": len(right_rows),
            "shared_keys": len(shared_keys),
            "left_duplicate_keys": left_duplicates,
            "right_duplicate_keys": right_duplicates,
            "missing_from_left": [key_record(key) for key in missing_from_left],
            "missing_from_right": [key_record(key) for key in missing_from_right],
        },
        "structural_drift": {
            "count": len(structural_drift),
            "records": structural_drift,
        },
    }
    if not result["valid"]:
        result["invalid_reason"] = (
            "Agreement was not computed because duplicate or missing stable keys, "
            "or structural drift, invalidated the frozen-row denominator."
        )
        return result

    keys = sorted(shared_keys, key=key_sort_value)
    disagreements = []
    disagreement_keys = set()
    for key in keys:
        left = left_by_key[key]
        right = right_by_key[key]
        changed = {
            field: {"left": left[field], "right": right[field]}
            for field in JUDGEMENT_FIELDS
            if left[field] != right[field]
        }
        if changed:
            disagreement_keys.add(key)
            disagreements.append(
                {
                    "key": key_record(key),
                    "name": left["name"],
                    "fields": changed,
                }
            )

    cards = defaultdict(list)
    for key in keys:
        cards[key[0]].append(key)
    exact_cards = sum(
        all(key not in disagreement_keys for key in card_keys)
        for card_keys in cards.values()
    )
    result.update(
        {
            "row_agreement": ratio(len(keys) - len(disagreement_keys), len(keys)),
            "exact_card_agreement": ratio(exact_cards, len(cards)),
            "confusion": {
                field: confusion_records(left_by_key, right_by_key, keys, field)
                for field in JUDGEMENT_FIELDS
            },
            "disagreements": disagreements,
            "supplemental": {
                field: compare_set_field(left_by_key, right_by_key, keys, field)
                for field in SUPPLEMENTAL_SET_FIELDS
            }
            | {
                field: compare_text_field(left_by_key, right_by_key, keys, field)
                for field in SUPPLEMENTAL_TEXT_FIELDS
            },
        }
    )
    return result


def parse_args(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("annotated")
    parser.add_argument(
        "--earlier",
        action="append",
        default=[],
        help="unit export TSV of an earlier set (repeatable)",
    )
    parser.add_argument(
        "--export", help="current unit export TSV of the same set, for drift detection"
    )
    parser.add_argument(
        "--compare",
        help="second annotated TSV produced from the same frozen export",
    )
    return parser.parse_args(argv)


def main(argv=None):
    args = parse_args(argv)
    try:
        rows = read_tsv(args.annotated)
        exported = read_tsv(args.export) if args.export else None
        out = compute_metrics(
            rows,
            args.annotated,
            earlier_paths=args.earlier,
            exported=exported,
            export_path=args.export,
        )
        exit_code = 0
        if args.compare:
            comparison = compare_annotation_passes(
                rows, read_tsv(args.compare), args.annotated, args.compare
            )
            out["comparison"] = comparison
            if not comparison["valid"]:
                exit_code = 1
    except (KeyError, ValueError) as exc:
        sys.stderr.write(f"error: {exc}\n")
        return 2

    json.dump(out, sys.stdout, indent=2)
    sys.stdout.write("\n")
    return exit_code


if __name__ == "__main__":
    raise SystemExit(main())
