#!/usr/bin/env python3
"""Tests for lightweight provenance-manifest validation."""

import hashlib
from contextlib import closing
import sqlite3
import unittest

from scripts.python import verify_manifests


class ManifestTests(unittest.TestCase):
    def test_committed_manifests_validate(self):
        for relative_path in [
            "docs/manifests/snapshot-scryfall-2026-08-25.json",
            "docs/manifests/experiment-pre-legends-export-gate-2026-08-26.json",
        ]:
            result = verify_manifests.validate_manifest(
                verify_manifests.REPO_ROOT / relative_path, allow_missing=True
            )
            self.assertEqual(result["path"].replace("\\", "/"), relative_path)

    def test_heldout_digest_is_sorted_and_respects_protocol_exceptions(self):
        with closing(sqlite3.connect(":memory:")) as connection:
            connection.execute(
                """CREATE TABLE cards (
                    oracle_id TEXT, first_set TEXT,
                    first_is_fallback INTEGER, oracle_text TEXT
                )"""
            )
            connection.executemany(
                "INSERT INTO cards VALUES (?, ?, ?, ?)",
                [
                    ("f-two", "dev", 0, "Trample"),
                    ("a-safe", "dev", 0, "Flying"),
                    ("F-one", "dev", 0, "Haste"),
                    ("f-exception", "lea", 0, "Reach"),
                    ("f-fallback", "dev", 1, "Vigilance"),
                ],
            )
            count, digest = verify_manifests.heldout_identity_digest_from_connection(
                connection
            )
        self.assertEqual(count, 2)
        self.assertEqual(
            digest,
            hashlib.sha256(b"f-one\nf-two\n").hexdigest(),
        )


if __name__ == "__main__":
    unittest.main()
