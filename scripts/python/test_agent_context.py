#!/usr/bin/env python3
"""Tests for the context-map routing helper."""

import unittest

from scripts.python import agent_context


class ContextMapValidationTests(unittest.TestCase):
    def test_committed_context_map_is_internally_valid(self):
        context_map = agent_context.load_context_map()
        result = agent_context.cmd_validate(None, context_map)
        self.assertEqual(result["errors"], [])
        self.assertTrue(result["valid"])

    def test_committed_context_map_has_known_task_classes(self):
        context_map = agent_context.load_context_map()
        route_ids = {route["id"] for route in context_map["task_routes"]}
        for expected in [
            "repository-orientation",
            "documentation-handoff-maintenance",
            "structural-segmentation-normalization-change",
            "structural-audit-research",
            "corpus-data-ingestion-change",
            "rules-comprehensive-rules-investigation",
            "cli-query-behavior-change",
            "research-protocol-gate-work",
            "literature-external-research",
            "future-semantic-parser-ir-work",
        ]:
            self.assertIn(expected, route_ids)


class RouteResolutionTests(unittest.TestCase):
    def setUp(self):
        self.context_map = {
            "entries": [
                {"id": "a", "path": "docs/agent/agent-contract.md", "role": "canonical_entry_document"},
                {"id": "b", "path": "docs/current-state.md", "role": "source_of_truth"},
            ],
            "task_routes": [
                {
                    "id": "known-class",
                    "description": "A known task class.",
                    "required_context": [{"ref": "a"}],
                    "optional_context": [{"ref": "b"}],
                    "likely_code_areas": [],
                    "validation": [],
                    "do_not_modify_casually": [],
                }
            ],
            "unknown_task_fallback": {
                "id": "generic-orientation",
                "description": "fallback",
                "required_context": [{"ref": "a"}],
                "optional_context": [],
                "likely_code_areas": [],
                "validation": [],
                "do_not_modify_casually": [],
            },
        }

    def test_known_task_class_resolves_and_matches(self):
        args = argparse_namespace(task_class="known-class")
        result = agent_context.cmd_route(args, self.context_map)
        self.assertTrue(result["matched"])
        self.assertEqual(result["route"]["required_context"][0]["path"], "docs/agent/agent-contract.md")

    def test_unknown_task_class_fails_safe_to_fallback(self):
        args = argparse_namespace(task_class="totally-unclassified-task")
        result = agent_context.cmd_route(args, self.context_map)
        self.assertFalse(result["matched"])
        self.assertEqual(result["route"]["id"], "generic-orientation")
        self.assertEqual(result["requested_task_class"], "totally-unclassified-task")

    def test_validate_catches_dangling_reference(self):
        broken = dict(self.context_map)
        broken["task_routes"] = [
            {
                "id": "broken-class",
                "description": "x",
                "required_context": [{"ref": "does-not-exist"}],
                "optional_context": [],
                "likely_code_areas": [],
                "validation": [],
                "do_not_modify_casually": [],
            }
        ]
        result = agent_context.cmd_validate(None, broken)
        self.assertFalse(result["valid"])
        self.assertTrue(any("does-not-exist" in error for error in result["errors"]))

    def test_new_future_task_class_can_be_added_without_touching_helper_code(self):
        extended = dict(self.context_map)
        extended["task_routes"] = list(self.context_map["task_routes"]) + [
            {
                "id": "brand-new-future-category",
                "description": "A task class invented after this test was written.",
                "required_context": [{"ref": "a"}],
                "optional_context": [],
                "likely_code_areas": [],
                "validation": [],
                "do_not_modify_casually": [],
            }
        ]
        args = argparse_namespace(task_class="brand-new-future-category")
        result = agent_context.cmd_route(args, extended)
        self.assertTrue(result["matched"])


def argparse_namespace(**kwargs):
    class Namespace:
        pass

    namespace = Namespace()
    for key, value in kwargs.items():
        setattr(namespace, key, value)
    return namespace


if __name__ == "__main__":
    unittest.main()
