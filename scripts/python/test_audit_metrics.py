#!/usr/bin/env python3
"""Unit tests for the structural-audit measurement helpers."""

import io
import os
import sys
import unittest


SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
if SCRIPT_DIR not in sys.path:
    sys.path.insert(0, SCRIPT_DIR)

import audit_metrics


def annotation_row(oracle_id="card-a", index="0", **changes):
    row = {
        "set": "tst",
        "oracle_id": oracle_id,
        "name": oracle_id,
        "type_line": "Creature — Test",
        "index": index,
        "parent_index": "",
        "depth": "0",
        "face": "0",
        "line": "1",
        "kind": "keyword_ability",
        "role": "ability",
        "source": "printed",
        "rule": "",
        "prefix": "",
        "text": "Flying",
        "normalized": "Flying",
        "boundary": "ok",
        "missed": "0",
        "kind_expected": "keyword_ability",
        "kind_ok": "yes",
        "role_ok": "yes",
        "source_ok": "yes",
        "context": "none",
        "cr_ref": "702.9",
        "structure_tags": "keyword",
        "norm_issue": "",
        "disposition": "accept",
        "annotator": "test-pass",
        "note": "",
    }
    row.update(changes)
    return row


class ExactCardCorrectnessTests(unittest.TestCase):
    def test_one_bad_unit_invalidates_only_its_card(self):
        rows = [
            annotation_row("card-a", "0"),
            annotation_row("card-a", "1", text="Trample", normalized="Trample"),
            annotation_row(
                "card-b",
                "0",
                boundary="under",
                missed="1",
                kind_expected="n/a",
                kind_ok="n/a",
                role_ok="unsure",
                disposition="defect",
            ),
        ]

        self.assertEqual(
            audit_metrics.structural_exact_card_correctness(rows),
            {"numerator": 1, "denominator": 2, "value": 0.5},
        )

    def test_every_applicable_failure_prevents_exact_correctness(self):
        cases = (
            {"boundary": "unsure"},
            {"missed": "1"},
            {"kind_ok": "no"},
            {"kind_ok": "unsure"},
            {"role_ok": "no"},
            {"role_ok": "unsure"},
            {"source_ok": "no"},
        )
        for changes in cases:
            with self.subTest(changes=changes):
                result = audit_metrics.structural_exact_card_correctness(
                    [annotation_row(**changes)]
                )
                self.assertEqual(result["numerator"], 0)
                self.assertEqual(result["denominator"], 1)

    def test_explicit_non_applicable_kind_is_allowed(self):
        result = audit_metrics.structural_exact_card_correctness(
            [annotation_row(kind_expected="n/a", kind_ok="n/a")]
        )
        self.assertEqual(result, {"numerator": 1, "denominator": 1, "value": 1.0})

    def test_invalid_missed_value_is_rejected(self):
        with self.assertRaisesRegex(ValueError, "invalid missed value"):
            audit_metrics.structural_exact_card_correctness(
                [annotation_row(missed="not-an-integer")]
            )


class PassComparisonTests(unittest.TestCase):
    def compare(self, left, right):
        return audit_metrics.compare_annotation_passes(
            left, right, "left.tsv", "right.tsv"
        )

    def test_identical_passes_agree_by_row_and_card(self):
        left = [
            annotation_row("card-a", "0"),
            annotation_row("card-a", "1", text="Trample", normalized="Trample"),
            annotation_row("card-b", "0"),
        ]
        right = [dict(row, annotator="other-pass") for row in left]

        result = self.compare(left, right)

        self.assertTrue(result["valid"])
        self.assertEqual(
            result["row_agreement"],
            {"numerator": 3, "denominator": 3, "value": 1.0},
        )
        self.assertEqual(
            result["exact_card_agreement"],
            {"numerator": 2, "denominator": 2, "value": 1.0},
        )
        self.assertEqual(result["disagreements"], [])

    def test_one_row_disagreement_invalidates_its_whole_card(self):
        left = [
            annotation_row("card-a", "0"),
            annotation_row("card-a", "1", text="Trample", normalized="Trample"),
            annotation_row("card-b", "0"),
        ]
        right = [dict(row, annotator="other-pass") for row in left]
        right[1].update(boundary="under", missed="1", disposition="defect")

        result = self.compare(left, right)

        self.assertEqual(result["row_agreement"]["numerator"], 2)
        self.assertEqual(result["row_agreement"]["denominator"], 3)
        self.assertEqual(result["exact_card_agreement"]["numerator"], 1)
        self.assertEqual(result["exact_card_agreement"]["denominator"], 2)
        self.assertEqual(len(result["disagreements"]), 1)
        self.assertEqual(
            set(result["disagreements"][0]["fields"]),
            {"boundary", "missed", "disposition"},
        )
        boundary_pairs = {
            (record["left"], record["right"]): record["count"]
            for record in result["confusion"]["boundary"]
        }
        self.assertEqual(boundary_pairs, {("ok", "ok"): 2, ("ok", "under"): 1})

    def test_missing_key_invalidates_comparison_without_agreement(self):
        left = [annotation_row("card-a", "0"), annotation_row("card-b", "0")]
        right = [annotation_row("card-a", "0")]

        result = self.compare(left, right)

        self.assertFalse(result["valid"])
        self.assertNotIn("row_agreement", result)
        self.assertEqual(
            result["key_integrity"]["missing_from_right"],
            [{"oracle_id": "card-b", "face": "0", "index": "0"}],
        )

    def test_duplicate_key_invalidates_comparison(self):
        duplicate = annotation_row("card-a", "0")
        result = self.compare([duplicate, dict(duplicate)], [dict(duplicate)])

        self.assertFalse(result["valid"])
        self.assertEqual(
            result["key_integrity"]["left_duplicate_keys"],
            [{"oracle_id": "card-a", "face": "0", "index": "0", "occurrences": 2}],
        )

    def test_structural_drift_invalidates_comparison(self):
        left = [annotation_row()]
        right = [annotation_row(text="Changed printed text")]

        result = self.compare(left, right)

        self.assertFalse(result["valid"])
        self.assertEqual(result["structural_drift"]["count"], 1)
        self.assertEqual(
            set(result["structural_drift"]["records"][0]["fields"]), {"text"}
        )

    def test_supplemental_differences_do_not_change_primary_agreement(self):
        left = [annotation_row(cr_ref="702.9;702.9a", structure_tags="keyword;flying")]
        right = [
            annotation_row(
                cr_ref="702.9;702.9b",
                structure_tags="keyword;evasion",
                note="review separately",
                annotator="other-pass",
            )
        ]

        result = self.compare(left, right)

        self.assertTrue(result["valid"])
        self.assertEqual(result["row_agreement"]["value"], 1.0)
        self.assertEqual(result["exact_card_agreement"]["value"], 1.0)
        self.assertEqual(result["supplemental"]["cr_ref"]["left_only_values"], 1)
        self.assertEqual(result["supplemental"]["cr_ref"]["right_only_values"], 1)
        self.assertEqual(
            result["supplemental"]["note"]["by_category"],
            {"right_only_nonblank": 1},
        )


class TsvReaderTests(unittest.TestCase):
    def test_crlf_input_does_not_leak_carriage_returns(self):
        header = "\t".join(audit_metrics.ANNOTATION_FIELDS)
        row = annotation_row()
        body = "\t".join(row[field] for field in audit_metrics.ANNOTATION_FIELDS)
        rows = audit_metrics.parse_tsv_lines(
            io.StringIO(header + "\r\n" + body + "\r\n"), "annotations.tsv"
        )

        self.assertEqual(rows[0]["note"], "")
        self.assertEqual(rows[0]["normalized"], "Flying")

    def test_current_export_requires_prefix_in_annotation(self):
        annotated = annotation_row()
        del annotated["prefix"]
        with self.assertRaisesRegex(ValueError, "prefix"):
            audit_metrics.compute_metrics(
                [annotated],
                "annotations.tsv",
                exported=[annotation_row()],
                export_path="export.tsv",
            )


if __name__ == "__main__":
    unittest.main()
