#!/usr/bin/env python3
"""Deterministic, standard-library-only reader for docs/agent/context-map.json.

Resolves task-class routes into their referenced context-map entries so an
agent (or script) does not have to hand-parse the JSON. Prints one JSON
document to stdout on success; diagnostics go to stderr; failures use a
nonzero exit status. Does not call an LLM, build a dependency graph, or
guess -- an unmatched task class returns the map's declared fallback route,
never a silently invented one.
"""

import argparse
import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
CONTEXT_MAP_PATH = REPO_ROOT / "docs" / "agent" / "context-map.json"


def load_context_map(path=CONTEXT_MAP_PATH):
    with open(path, "r", encoding="utf-8") as source:
        return json.load(source)


def _entries_by_id(context_map):
    entries = {}
    for entry in context_map.get("entries", []):
        entries[entry["id"]] = entry
    return entries


def _resolve_refs(refs, entries_by_id, label):
    resolved = []
    for ref in refs:
        entry_id = ref["ref"]
        if entry_id not in entries_by_id:
            raise ValueError(f"{label} references unknown entry id: {entry_id}")
        resolved_entry = dict(entries_by_id[entry_id])
        if "reason" in ref:
            resolved_entry["reason"] = ref["reason"]
        resolved.append(resolved_entry)
    return resolved


def resolve_route(route, context_map):
    entries_by_id = _entries_by_id(context_map)
    resolved = dict(route)
    for field in ("required_context", "optional_context", "likely_code_areas", "do_not_modify_casually"):
        resolved[field] = _resolve_refs(route.get(field, []), entries_by_id, f"route '{route['id']}'.{field}")
    return resolved


def cmd_list_routes(args, context_map):
    return {
        "task_routes": [
            {"id": route["id"], "description": route["description"]}
            for route in context_map.get("task_routes", [])
        ]
    }


def cmd_route(args, context_map):
    for route in context_map.get("task_routes", []):
        if route["id"] == args.task_class:
            return {"matched": True, "route": resolve_route(route, context_map)}
    fallback = context_map["unknown_task_fallback"]
    return {
        "matched": False,
        "requested_task_class": args.task_class,
        "route": resolve_route(fallback, context_map),
    }


def cmd_entry_documents(args, context_map):
    entries = [e for e in context_map.get("entries", []) if e.get("role") == "canonical_entry_document"]
    entries += [e for e in context_map.get("entries", []) if e.get("role") == "source_of_truth"]
    return {"entry_documents": entries}


def cmd_validation(args, context_map):
    return {"validation_commands": context_map.get("validation_commands", [])}


def cmd_validate(args, context_map):
    errors = []
    seen_ids = set()
    for entry in context_map.get("entries", []):
        if entry["id"] in seen_ids:
            errors.append(f"duplicate entry id: {entry['id']}")
        seen_ids.add(entry["id"])
        path = REPO_ROOT / entry["path"]
        if entry.get("role") != "generated_local_artifact" and not path.exists():
            errors.append(f"entry '{entry['id']}' path does not exist: {entry['path']}")

    entries_by_id = _entries_by_id(context_map)
    route_ids = set()
    for route in context_map.get("task_routes", []):
        if route["id"] in route_ids:
            errors.append(f"duplicate task_routes id: {route['id']}")
        route_ids.add(route["id"])
        for field in ("required_context", "optional_context", "likely_code_areas", "do_not_modify_casually"):
            for ref in route.get(field, []):
                if ref["ref"] not in entries_by_id:
                    errors.append(f"route '{route['id']}'.{field} references unknown entry id: {ref['ref']}")

    fallback = context_map.get("unknown_task_fallback")
    if fallback is None:
        errors.append("missing unknown_task_fallback")
    else:
        for field in ("required_context", "optional_context", "likely_code_areas", "do_not_modify_casually"):
            for ref in fallback.get(field, []):
                if ref["ref"] not in entries_by_id:
                    errors.append(f"unknown_task_fallback.{field} references unknown entry id: {ref['ref']}")

    if "schema_version" not in context_map:
        errors.append("missing schema_version")

    return {"valid": len(errors) == 0, "errors": errors}


def build_parser():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--context-map",
        type=Path,
        default=CONTEXT_MAP_PATH,
        help="Path to context-map.json (default: docs/agent/context-map.json)",
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    subparsers.add_parser("list-routes", help="List known task classes.")

    route_parser = subparsers.add_parser("route", help="Resolve required/optional context for a task class.")
    route_parser.add_argument("task_class", help="A task_routes id, e.g. structural-audit-research.")

    subparsers.add_parser("entry-documents", help="List canonical entry documents and sources of truth.")
    subparsers.add_parser("validation", help="List required validation commands.")
    subparsers.add_parser("validate", help="Check the context map's internal consistency and path references.")

    return parser


COMMANDS = {
    "list-routes": cmd_list_routes,
    "route": cmd_route,
    "entry-documents": cmd_entry_documents,
    "validation": cmd_validation,
    "validate": cmd_validate,
}


def main(argv=None):
    args = build_parser().parse_args(argv)
    try:
        context_map = load_context_map(args.context_map)
        result = COMMANDS[args.command](args, context_map)
    except (OSError, ValueError, KeyError) as error:
        print(f"agent_context: {error}", file=sys.stderr)
        return 1

    print(json.dumps(result, indent=2, sort_keys=True))
    if args.command == "validate" and not result["valid"]:
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
