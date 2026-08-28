#!/usr/bin/env python3
"""Tests for the AI-facing metadata validator (validate_agent_context.py)."""

import tempfile
import unittest
from pathlib import Path

from scripts.python import validate_agent_context as vac


def _minimal_index(entries):
    return {
        "schema_version": 1,
        "last_updated": "2026-08-28",
        "purpose": "test",
        "entries": entries,
    }


def _entry(entry_id, path="docs/findings/x.md", **overrides):
    entry = {
        "id": entry_id,
        "path": path,
        "type": "finding",
        "scope": None,
        "date": "2026-08-28",
        "status": "closed",
        "summary": "summary",
        "supersedes": [],
        "superseded_by": [],
    }
    entry.update(overrides)
    return entry


class FindingsIndexValidationTests(unittest.TestCase):
    def test_committed_findings_index_is_internally_valid(self):
        index = vac.load_json(vac.FINDINGS_INDEX_PATH)
        errors = vac.validate_findings_index(index, vac.REPO_ROOT)
        self.assertEqual(errors, [])

    def test_committed_findings_index_has_full_coverage(self):
        index = vac.load_json(vac.FINDINGS_INDEX_PATH)
        errors = vac.validate_findings_coverage(index, vac.REPO_ROOT)
        self.assertEqual(errors, [])

    def test_duplicate_id_is_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo_root = Path(tmp)
            (repo_root / "docs" / "findings").mkdir(parents=True)
            (repo_root / "docs" / "findings" / "a.md").write_text("a", encoding="utf-8")
            index = _minimal_index(
                [
                    _entry("dup", path="docs/findings/a.md"),
                    _entry("dup", path="docs/findings/a.md"),
                ]
            )
            errors = vac.validate_findings_index(index, repo_root)
            self.assertTrue(any("duplicate id" in e for e in errors))

    def test_missing_path_is_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo_root = Path(tmp)
            index = _minimal_index([_entry("ghost", path="docs/findings/does-not-exist.md")])
            errors = vac.validate_findings_index(index, repo_root)
            self.assertTrue(any("path does not exist" in e for e in errors))

    def test_broken_supersession_reference_is_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo_root = Path(tmp)
            (repo_root / "docs" / "findings").mkdir(parents=True)
            (repo_root / "docs" / "findings" / "a.md").write_text("a", encoding="utf-8")
            index = _minimal_index(
                [_entry("a", path="docs/findings/a.md", supersedes=["nonexistent-id"])]
            )
            errors = vac.validate_findings_index(index, repo_root)
            self.assertTrue(any("supersedes unknown id" in e for e in errors))

    def test_asymmetric_supersession_is_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo_root = Path(tmp)
            (repo_root / "docs" / "findings").mkdir(parents=True)
            for name in ("a.md", "b.md"):
                (repo_root / "docs" / "findings" / name).write_text("x", encoding="utf-8")
            index = _minimal_index(
                [
                    # a claims to supersede b, but b does not record superseded_by a.
                    _entry("a", path="docs/findings/a.md", supersedes=["b"]),
                    _entry("b", path="docs/findings/b.md"),
                ]
            )
            errors = vac.validate_findings_index(index, repo_root)
            self.assertTrue(any("does not list 'a' back" in e for e in errors))

    def test_symmetric_supersession_is_accepted(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo_root = Path(tmp)
            (repo_root / "docs" / "findings").mkdir(parents=True)
            for name in ("a.md", "b.md"):
                (repo_root / "docs" / "findings" / name).write_text("x", encoding="utf-8")
            index = _minimal_index(
                [
                    _entry("a", path="docs/findings/a.md", supersedes=["b"]),
                    _entry("b", path="docs/findings/b.md", superseded_by=["a"]),
                ]
            )
            errors = vac.validate_findings_index(index, repo_root)
            self.assertEqual(errors, [])

    def test_path_traversal_is_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo_root = Path(tmp)
            index = _minimal_index([_entry("escape", path="../../etc/passwd")])
            errors = vac.validate_findings_index(index, repo_root)
            self.assertTrue(any("unsafe or non-relative path" in e for e in errors))

    def test_unsupported_schema_version_is_rejected(self):
        index = _minimal_index([])
        index["schema_version"] = 999
        errors = vac.validate_findings_index(index, Path("."))
        self.assertTrue(any("unsupported schema_version" in e for e in errors))

    def test_missing_required_field_is_rejected(self):
        broken_entry = _entry("incomplete")
        del broken_entry["summary"]
        index = _minimal_index([broken_entry])
        errors = vac.validate_findings_index(index, Path("."))
        self.assertTrue(any("missing required fields" in e and "summary" in e for e in errors))

    def test_coverage_flags_unindexed_document(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo_root = Path(tmp)
            (repo_root / "docs" / "findings").mkdir(parents=True)
            (repo_root / "docs" / "findings" / "indexed.md").write_text("x", encoding="utf-8")
            (repo_root / "docs" / "findings" / "forgotten.md").write_text("x", encoding="utf-8")
            index = _minimal_index([_entry("indexed", path="docs/findings/indexed.md")])
            errors = vac.validate_findings_coverage(index, repo_root)
            self.assertTrue(any("forgotten.md" in e for e in errors))

    def test_coverage_does_not_look_outside_declared_directories(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo_root = Path(tmp)
            (repo_root / "docs" / "findings").mkdir(parents=True)
            (repo_root / "docs" / "audits").mkdir(parents=True)
            (repo_root / "docs" / "audits" / "not-a-finding.md").write_text("x", encoding="utf-8")
            index = _minimal_index([])
            errors = vac.validate_findings_coverage(index, repo_root)
            self.assertEqual(errors, [])


class ContextMapExtraValidationTests(unittest.TestCase):
    def test_committed_context_map_extra_checks_pass(self):
        context_map = vac.agent_context.load_context_map()
        errors = vac.validate_context_map_extra(context_map, vac.REPO_ROOT)
        self.assertEqual(errors, [])

    def test_malformed_validation_ref_is_rejected(self):
        context_map = {
            "schema_version": 1,
            "entries": [],
            "validation_commands": [{"id": "real-check", "command": "echo ok"}],
            "task_routes": [
                {
                    "id": "broken-route",
                    "description": "x",
                    "required_context": [],
                    "optional_context": [],
                    "likely_code_areas": [],
                    "validation": [{"ref": "no-such-command"}],
                    "do_not_modify_casually": [],
                }
            ],
        }
        errors = vac.validate_context_map_extra(context_map, Path("."))
        self.assertTrue(any("unknown validation_commands id" in e for e in errors))

    def test_valid_minimal_future_extension_is_accepted(self):
        """A brand-new task class with a well-formed validation ref must pass
        without any validator code change -- this is the extensibility the
        objective calls for."""
        context_map = {
            "schema_version": 1,
            "entries": [],
            "validation_commands": [{"id": "future-check", "command": "echo future"}],
            "task_routes": [
                {
                    "id": "brand-new-future-task-class",
                    "description": "Invented after this validator was written.",
                    "required_context": [],
                    "optional_context": [],
                    "likely_code_areas": [],
                    "validation": [{"ref": "future-check"}],
                    "do_not_modify_casually": [],
                }
            ],
        }
        errors = vac.validate_context_map_extra(context_map, Path("."))
        self.assertEqual(errors, [])

    def test_generated_artifact_not_gitignored_is_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo_root = Path(tmp)
            (repo_root / ".gitignore").write_text("/target/\n", encoding="utf-8")
            context_map = {
                "entries": [
                    {"id": "gen-x", "path": "some-generated-file.db", "role": "generated_local_artifact"}
                ]
            }
            errors = vac.validate_context_map_extra(context_map, repo_root)
            self.assertTrue(any("not covered by .gitignore" in e for e in errors))

    def test_tracked_source_accidentally_gitignored_is_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo_root = Path(tmp)
            (repo_root / ".gitignore").write_text("/docs/\n", encoding="utf-8")
            context_map = {
                "entries": [
                    {"id": "current-state", "path": "docs/current-state.md", "role": "source_of_truth"}
                ]
            }
            errors = vac.validate_context_map_extra(context_map, repo_root)
            self.assertTrue(any("cannot be tracked as claimed" in e for e in errors))


class EntryPointValidationTests(unittest.TestCase):
    def test_committed_entry_points_are_valid(self):
        errors = vac.validate_entry_points(vac.REPO_ROOT)
        self.assertEqual(errors, [])

    def test_entry_point_missing_contract_reference_is_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo_root = Path(tmp)
            (repo_root / "docs" / "agent").mkdir(parents=True)
            (repo_root / "docs" / "agent" / "agent-contract.md").write_text(
                "See docs/current-state.md and docs/findings/index.json.", encoding="utf-8"
            )
            (repo_root / ".github").mkdir()
            (repo_root / ".github" / "copilot-instructions.md").write_text("stale content", encoding="utf-8")
            (repo_root / "CLAUDE.md").write_text("Read docs/agent/agent-contract.md first.", encoding="utf-8")
            (repo_root / "AGENTS.md").write_text("Read docs/agent/agent-contract.md first.", encoding="utf-8")
            errors = vac.validate_entry_points(repo_root)
            self.assertTrue(
                any(".github/copilot-instructions.md" in e and "canonical contract" in e for e in errors)
            )


class FullRepositoryValidationTests(unittest.TestCase):
    def test_committed_repository_state_is_fully_valid(self):
        errors = vac.run_all_checks(vac.REPO_ROOT)
        self.assertEqual(errors, [])


if __name__ == "__main__":
    unittest.main()
