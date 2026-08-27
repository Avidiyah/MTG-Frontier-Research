#!/usr/bin/env python3
"""Verify deterministic held-out-safe JSON and TSV exports without printing rows."""

import argparse
from contextlib import closing
import csv
import hashlib
import io
import json
import os
from pathlib import Path
import sqlite3
import subprocess
import sys

try:
    from scripts.python import export_units
except ModuleNotFoundError:  # Direct execution from scripts/python.
    import export_units


def sha256_bytes(data):
    return hashlib.sha256(data).hexdigest()


def run_bytes(command):
    proc = subprocess.run(command, capture_output=True)
    if proc.returncode != 0:
        stderr = proc.stderr.decode("utf-8", errors="replace")
        raise RuntimeError(
            f"export command failed with exit {proc.returncode}: {stderr.strip()}"
        )
    return proc.stdout


def assert_repeated_bytes(command, runs):
    outputs = [run_bytes(command) for _ in range(runs)]
    if any(output != outputs[0] for output in outputs[1:]):
        raise ValueError("repeated export bytes differ")
    return outputs[0]


def parse_tsv(data):
    text = data.decode("utf-8")
    reader = csv.DictReader(io.StringIO(text), delimiter="\t")
    if reader.fieldnames != export_units.COLUMNS:
        raise ValueError("TSV export columns do not match the protocol contract")
    rows = list(reader)
    keys = [(row["oracle_id"], int(row["face"]), int(row["index"])) for row in rows]
    if len(keys) != len(set(keys)):
        raise ValueError("TSV export contains duplicate stable keys")
    return rows, keys


HELD_OUT_SQL = """
    oracle_text IS NOT NULL AND oracle_text != ''
    AND lower(substr(oracle_id, 1, 1)) = 'f'
    AND first_is_fallback = 0
    AND lower(coalesce(first_set, '')) NOT IN ('lea', 'leb', 'arn')
"""


def database_aggregates(db_path, set_code):
    uri = Path(db_path).resolve().as_uri() + "?mode=ro"
    with closing(sqlite3.connect(uri, uri=True)) as connection:
        return database_aggregates_from_connection(connection, set_code)


def database_aggregates_from_connection(connection, set_code):
    selected = connection.execute(
        f"""
        SELECT count(*),
               sum(CASE WHEN oracle_text IS NOT NULL AND oracle_text != '' THEN 1 ELSE 0 END),
               sum(CASE WHEN {HELD_OUT_SQL} THEN 1 ELSE 0 END),
               sum(CASE WHEN first_is_fallback != 0 THEN 1 ELSE 0 END)
        FROM cards WHERE lower(first_set) = ?
        """,
        (set_code.lower(),),
    ).fetchone()
    pool_size = connection.execute(
        f"SELECT count(*) FROM cards WHERE {HELD_OUT_SQL}"
    ).fetchone()[0]
    cards, cards_with_text, held_out_cards, fallback_cards = (
        int(value or 0) for value in selected
    )
    return {
        "selected_cards_before_exclusion": cards,
        "selected_cards_with_text_before_exclusion": cards_with_text,
        "heldout_cards_excluded": held_out_cards,
        "fallback_cards_before_exclusion": fallback_cards,
        "expected_cards_after_exclusion": cards - held_out_cards,
        "expected_cards_with_text_after_exclusion": cards_with_text - held_out_cards,
        "global_heldout_pool_cards": pool_size,
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("set_code", help="first-printing set code")
    parser.add_argument("--mtg", default=export_units.DEFAULT_MTG)
    parser.add_argument("--db", default=None)
    parser.add_argument("--runs", type=int, default=2)
    args = parser.parse_args()
    if args.runs < 2:
        parser.error("--runs must be at least 2")

    native_command = [args.mtg]
    if args.db:
        native_command += ["--db", args.db]
    native_command += ["audit", "export", args.set_code, "--exclude-heldout"]
    native_bytes = assert_repeated_bytes(native_command, args.runs)
    native_payload = json.loads(native_bytes.decode("utf-8"))
    native_records = export_units.validate_native_export(native_payload, True)
    native_keys = [export_units.stable_key(record) for record in native_records]
    db_path = args.db or os.path.join(export_units.REPO_ROOT, "cards.sqlite")
    aggregates = database_aggregates(db_path, args.set_code)
    if native_payload["cards"] != aggregates["expected_cards_after_exclusion"]:
        raise ValueError("filtered card count disagrees with the source aggregate")
    if (
        native_payload["cards_with_text"]
        != aggregates["expected_cards_with_text_after_exclusion"]
    ):
        raise ValueError("filtered text-card count disagrees with the source aggregate")

    tsv_command = [
        sys.executable,
        os.path.join(export_units.REPO_ROOT, "scripts", "python", "export_units.py"),
        args.set_code,
        "--mtg",
        args.mtg,
        "--exclude-heldout",
    ]
    if args.db:
        tsv_command += ["--db", args.db]
    tsv_bytes = assert_repeated_bytes(tsv_command, args.runs)
    tsv_rows, tsv_keys = parse_tsv(tsv_bytes)
    if native_keys != tsv_keys:
        raise ValueError("native JSON and protocol TSV stable-key sequences differ")

    result = {
        "schema_version": "export-safety-verification-v1",
        "set": native_payload["set"],
        "heldout_exclusion": {
            "enabled": True,
            "policy": native_payload["heldout_exclusion"]["policy"],
            "matching_export_records": sum(
                export_units.is_held_out_record(record) for record in native_records
            ),
            **aggregates,
        },
        "stable_keys": {
            "declared": native_payload["stable_key"],
            "rows": len(native_keys),
            "unique": len(set(native_keys)),
            "json_tsv_sequence_identical": True,
        },
        "native_json": {
            "runs": args.runs,
            "byte_identical": True,
            "bytes": len(native_bytes),
            "sha256": sha256_bytes(native_bytes),
            "cards": native_payload["cards"],
            "cards_with_text": native_payload["cards_with_text"],
            "records": len(native_records),
        },
        "protocol_tsv": {
            "runs": args.runs,
            "byte_identical": True,
            "bytes": len(tsv_bytes),
            "sha256": sha256_bytes(tsv_bytes),
            "records": len(tsv_rows),
        },
    }
    print(json.dumps(result, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
