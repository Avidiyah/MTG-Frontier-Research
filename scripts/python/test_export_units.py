#!/usr/bin/env python3
"""Tests for deterministic, held-out-safe structural export validation."""

import copy
from contextlib import closing
import sqlite3
import unittest

from scripts.python import export_units
from scripts.python import verify_export_safety


def record(oracle_id="a-safe", index=0, **changes):
    value = {
        "oracle_id": oracle_id,
        "card_name": "Synthetic",
        "type_line": "Creature",
        "first_set": "dev",
        "first_released_at": "2000-01-01",
        "first_is_fallback": False,
        "face": 0,
        "source_line": 1,
        "unit_index": index,
        "parent_index": None,
        "depth": 0,
        "source_line_text": "Flying",
        "unit_text": "Flying",
        "prefix": None,
        "normalized": "Flying",
        "kind": "keyword_ability",
        "role": "ability",
        "source": "printed",
        "rule": None,
        "signals": [],
    }
    value.update(changes)
    return value


def payload(records, excluded=True):
    return {
        "schema_version": "audit-export-v1",
        "set": "dev",
        "ordering": "card name, oracle_id, face, pre-order unit_index",
        "stable_key": ["oracle_id", "face", "unit_index"],
        "heldout_exclusion": {"enabled": excluded},
        "cards": len({item["oracle_id"] for item in records}),
        "cards_with_text": len({item["oracle_id"] for item in records}),
        "count": len(records),
        "records": records,
    }


class ExportValidationTests(unittest.TestCase):
    def test_tsv_projection_preserves_stable_key_and_parent(self):
        records = [
            record(),
            record(index=1, parent_index=0, depth=1, unit_text="Trample"),
        ]
        rows = export_units.tsv_rows(payload(records), True)
        self.assertEqual(
            [(row["oracle_id"], row["face"], row["index"]) for row in rows],
            [("a-safe", 0, 0), ("a-safe", 0, 1)],
        )
        self.assertEqual(rows[1]["parent_index"], 0)

    def test_tsv_projection_preserves_prefix_and_uses_empty_string_when_absent(self):
        rows = export_units.tsv_rows(
            payload(
                [
                    record(prefix="Heroic", unit_text="Heroic \u2014 Whenever something happens."),
                    record(index=1),
                ]
            ),
            True,
        )
        self.assertEqual(rows[0]["prefix"], "Heroic")
        self.assertEqual(rows[1]["prefix"], "")

    def test_duplicate_stable_key_is_rejected(self):
        duplicate = record()
        with self.assertRaisesRegex(ValueError, "duplicate stable keys"):
            export_units.validate_native_export(
                payload([duplicate, copy.deepcopy(duplicate)]), True
            )

    def test_cross_identity_parent_is_rejected(self):
        rows = [
            record("a-parent"),
            record("b-child", index=1, parent_index=0, depth=1),
        ]
        with self.assertRaisesRegex(ValueError, "cross-identity parent"):
            export_units.validate_native_export(payload(rows), True)

    def test_held_out_identity_is_rejected_when_exclusion_is_required(self):
        with self.assertRaisesRegex(ValueError, "exposed a held-out identity"):
            export_units.validate_native_export(payload([record("f-held")]), True)

    def test_historical_exception_and_fallback_are_not_held_out(self):
        self.assertFalse(
            export_units.is_held_out_record(record("f-exception", first_set="lea"))
        )
        self.assertFalse(
            export_units.is_held_out_record(
                record("f-fallback", first_is_fallback=True)
            )
        )

    def test_database_aggregates_measure_exclusion_without_returning_rows(self):
        with closing(sqlite3.connect(":memory:")) as connection:
            connection.execute(
                """CREATE TABLE cards (
                    oracle_id TEXT, first_set TEXT,
                    first_is_fallback INTEGER, oracle_text TEXT
                )"""
            )
            connection.executemany(
                "INSERT INTO cards VALUES (?, 'dev', ?, ?)",
                [
                    ("a-safe", 0, "Flying"),
                    ("f-held", 0, "Trample"),
                    ("f-fallback", 1, "Haste"),
                    ("f-empty", 0, None),
                ],
            )
            result = verify_export_safety.database_aggregates_from_connection(
                connection, "dev"
            )
        self.assertEqual(result["selected_cards_before_exclusion"], 4)
        self.assertEqual(result["heldout_cards_excluded"], 1)
        self.assertEqual(result["fallback_cards_before_exclusion"], 1)
        self.assertEqual(result["expected_cards_after_exclusion"], 3)
        self.assertEqual(result["global_heldout_pool_cards"], 1)


if __name__ == "__main__":
    unittest.main()
