#!/usr/bin/env python3
"""Export every structural unit of one first-printing set as a flat TSV.

This script is the protocol TSV view over the native ``audit export`` JSON.
The native query performs optional held-out exclusion before segmentation;
this layer validates the stable keys and parent references before writing any
auditor-visible row. It adds no segmentation logic of its own.

Usage (from the repository root, after ``cargo build --release``):

    python scripts/python/export_units.py lea > docs/audits/lea/units-export.tsv
    python scripts/python/export_units.py lea --mtg path/to/mtg-discover.exe
    python scripts/python/export_units.py leg --exclude-heldout > output.tsv

Rows are sorted by card name, Oracle ID, face, then pre-order unit index, so
the output is deterministic for a fixed database and segmenter. Standard
library only.
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
    "face", "line", "kind", "role", "source", "rule", "prefix", "text", "normalized",
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


def stable_key(record):
    return record["oracle_id"], record["face"], record["unit_index"]


def is_held_out_record(record):
    return (
        record["oracle_id"][:1].lower() == "f"
        and not record["first_is_fallback"]
        and record["first_set"].lower() not in {"lea", "leb", "arn"}
    )


def validate_native_export(payload, require_heldout_exclusion=False):
    if payload.get("schema_version") != "audit-export-v1":
        raise ValueError("unsupported native audit export schema")
    if payload.get("stable_key") != ["oracle_id", "face", "unit_index"]:
        raise ValueError("native audit export declares an unexpected stable key")
    exclusion = payload.get("heldout_exclusion", {})
    if require_heldout_exclusion and not exclusion.get("enabled"):
        raise ValueError("held-out exclusion was requested but not attested")

    records = payload.get("records", [])
    keys = [stable_key(record) for record in records]
    if len(keys) != len(set(keys)):
        raise ValueError("native audit export contains duplicate stable keys")
    expected_order = sorted(
        records,
        key=lambda record: (
            record["card_name"].lower(),
            record["card_name"],
            record["oracle_id"],
            record["face"],
            record["unit_index"],
        ),
    )
    if records != expected_order:
        raise ValueError("native audit export violates its declared ordering")
    key_set = set(keys)
    for record in records:
        parent_index = record.get("parent_index")
        if parent_index is None:
            continue
        if parent_index >= record["unit_index"]:
            raise ValueError("native audit export contains a non-preorder parent")
        if (record["oracle_id"], record["face"], parent_index) not in key_set:
            raise ValueError("native audit export contains a cross-identity parent")
    if require_heldout_exclusion and any(is_held_out_record(record) for record in records):
        raise ValueError("native audit export exposed a held-out identity")
    if payload.get("count") != len(records):
        raise ValueError("native audit export row count does not match its metadata")
    return records


def tsv_rows(payload, require_heldout_exclusion=False):
    records = validate_native_export(payload, require_heldout_exclusion)
    set_code = payload["set"]
    return [
        {
            "set": set_code,
            "oracle_id": record["oracle_id"],
            "name": record["card_name"],
            "type_line": record.get("type_line") or "",
            "index": record["unit_index"],
            "parent_index": "" if record.get("parent_index") is None else record["parent_index"],
            "depth": record["depth"],
            "face": record["face"],
            "line": record["source_line"],
            "kind": record["kind"],
            "role": record["role"],
            "source": record["source"],
            "rule": record.get("rule") or "",
            "prefix": record.get("prefix") or "",
            "text": record["unit_text"],
            "normalized": record["normalized"],
        }
        for record in records
    ]


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("set_code", help="first-printing set code, e.g. lea")
    parser.add_argument("--mtg", default=DEFAULT_MTG, help="path to the mtg-discover binary")
    parser.add_argument("--db", default=None, help="path to cards.sqlite (default: CLI default)")
    parser.add_argument(
        "--exclude-heldout",
        action="store_true",
        help="exclude protocol 6.3 held-out identities before segmentation",
    )
    args = parser.parse_args()

    export_args = ["audit", "export", args.set_code]
    if args.exclude_heldout:
        export_args.append("--exclude-heldout")
    payload = run_json(args.mtg, args.db, export_args)
    rows = tsv_rows(payload, args.exclude_heldout)

    # Oracle text contains em dashes and curly quotes; never let the console
    # codepage decide how they are written.
    sys.stdout.reconfigure(encoding="utf-8", newline="\n")
    out = sys.stdout
    out.write("\t".join(COLUMNS) + "\n")
    for row in rows:
        out.write("\t".join(str(row[c]).replace("\t", " ").replace("\n", " ") for c in COLUMNS) + "\n")
    sys.stderr.write(f"{payload['cards']} cards, {len(rows)} units\n")


if __name__ == "__main__":
    main()
