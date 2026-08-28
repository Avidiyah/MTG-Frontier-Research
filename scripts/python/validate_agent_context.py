#!/usr/bin/env python3
"""Deterministic, standard-library-only validator for the repository's
AI-facing metadata: docs/findings/index.json, docs/agent/context-map.json,
and the routing pointers in the agent entry-point files.

This checks STRUCTURE, not today's research conclusions: it enforces that the
index, the context map, and the entry points stay internally consistent and
in sync with the files they describe. It does not, and must not, encode any
claim about which research phase is active, which source directories are
"the only valid ones", or anything else that is current-state rather than
structural. See docs/agent/README-validate-agent-context.md for the full
scope statement.

Prints one JSON document ({"valid": bool, "errors": [...]}) to stdout;
diagnostics go to stderr; exit status is nonzero when invalid or on a
load/parse failure.
"""

import argparse
import json
import sys
from pathlib import Path

_REPO_ROOT_FOR_IMPORT = Path(__file__).resolve().parent.parent.parent
if str(_REPO_ROOT_FOR_IMPORT) not in sys.path:
    sys.path.insert(0, str(_REPO_ROOT_FOR_IMPORT))

from scripts.python import agent_context  # noqa: E402  (see sys.path setup above)

REPO_ROOT = agent_context.REPO_ROOT
FINDINGS_INDEX_PATH = REPO_ROOT / "docs" / "findings" / "index.json"
CONTEXT_MAP_PATH = agent_context.CONTEXT_MAP_PATH
GITIGNORE_PATH = REPO_ROOT / ".gitignore"

# Schema versions this validator understands. Bump when the index/map gains a
# field the validator must actively account for; a document declaring a
# newer version than this set fails closed rather than being silently
# accepted or silently mis-checked.
SUPPORTED_FINDINGS_SCHEMA_VERSIONS = {1}
SUPPORTED_CONTEXT_MAP_SCHEMA_VERSIONS = {1}

FINDINGS_INDEX_REQUIRED_FIELDS = (
    "id",
    "path",
    "type",
    "scope",
    "date",
    "status",
    "summary",
    "supersedes",
    "superseded_by",
)

# Directories docs/findings/index.json's own "purpose" field declares it
# covers. Kept in sync with that declared scope, not invented independently:
# widening this without the index also declaring it would make the coverage
# check enforce a scope the index itself does not claim.
FINDINGS_INDEX_COVERED_DIRS = ("docs/findings", "docs/gates", "docs/protocol")

# Agent entry-point files that must point at the canonical contract rather
# than re-duplicating its content. Checked for a literal substring only --
# no prose equality, so legitimate wording changes never trip this.
ENTRY_POINT_FILES = ("CLAUDE.md", "AGENTS.md", ".github/copilot-instructions.md")
AGENT_CONTRACT_PATH = "docs/agent/agent-contract.md"
CURRENT_STATE_PATH = "docs/current-state.md"
FINDINGS_INDEX_REL_PATH = "docs/findings/index.json"


def load_json(path):
    with open(path, "r", encoding="utf-8") as source:
        return json.load(source)


def _is_safe_relative_path(path_str):
    """Reject absolute paths, drive letters, and path traversal.

    A findings-index or context-map path is meant to be a repository-relative
    pointer; anything else is either a mistake or an attempt to reference
    something outside the repository.
    """
    if not isinstance(path_str, str) or not path_str:
        return False
    if path_str.startswith("/") or path_str.startswith("\\"):
        return False
    if ":" in path_str:  # Windows drive letter (C:\...) or a URL-shaped value
        return False
    parts = Path(path_str).parts
    return ".." not in parts


def _load_gitignore_patterns(path=GITIGNORE_PATH):
    patterns = []
    if not path.exists():
        return patterns
    with open(path, "r", encoding="utf-8") as source:
        for line in source:
            line = line.strip()
            if line and not line.startswith("#"):
                patterns.append(line)
    return patterns


def _gitignore_matches(rel_path, patterns):
    """Minimal, non-recursive .gitignore matcher for this repo's own simple
    patterns (leading-slash-anchored literals/globs and trailing-slash
    directory patterns). Not a general gitignore implementation -- it only
    needs to answer "is this specific declared entry ignored", not resolve
    arbitrary rule precedence.
    """
    import fnmatch

    normalized = rel_path.replace("\\", "/")
    top_component = normalized.split("/", 1)[0]
    for pattern in patterns:
        anchored = pattern.startswith("/")
        is_dir_pattern = pattern.endswith("/")
        stripped = pattern.strip("/")
        if is_dir_pattern:
            if anchored:
                if top_component == stripped:
                    return True
            elif stripped in Path(normalized).parts[:-1] or top_component == stripped:
                return True
            continue
        if anchored:
            if fnmatch.fnmatch(normalized, stripped) or fnmatch.fnmatch(top_component, stripped):
                return True
        else:
            if fnmatch.fnmatch(normalized, pattern) or fnmatch.fnmatch(Path(normalized).name, pattern):
                return True
    return False


def validate_findings_index(index, repo_root=REPO_ROOT):
    errors = []

    if "schema_version" not in index:
        errors.append("findings index: missing schema_version")
    elif index["schema_version"] not in SUPPORTED_FINDINGS_SCHEMA_VERSIONS:
        errors.append(
            f"findings index: unsupported schema_version {index['schema_version']!r} "
            f"(supported: {sorted(SUPPORTED_FINDINGS_SCHEMA_VERSIONS)})"
        )

    entries = index.get("entries")
    if not isinstance(entries, list):
        errors.append("findings index: 'entries' must be a list")
        return errors

    seen_ids = {}
    seen_paths = {}
    for position, entry in enumerate(entries):
        label = f"findings index entry[{position}]"
        if not isinstance(entry, dict):
            errors.append(f"{label}: not an object")
            continue

        missing = [field for field in FINDINGS_INDEX_REQUIRED_FIELDS if field not in entry]
        if missing:
            errors.append(f"{label}: missing required fields: {', '.join(missing)}")
            continue

        entry_id = entry["id"]
        label = f"findings index entry '{entry_id}'"

        if entry_id in seen_ids:
            errors.append(f"{label}: duplicate id (also entry[{seen_ids[entry_id]}])")
        else:
            seen_ids[entry_id] = position

        path_value = entry["path"]
        if not _is_safe_relative_path(path_value):
            errors.append(f"{label}: unsafe or non-relative path: {path_value!r}")
        else:
            if path_value in seen_paths:
                errors.append(f"{label}: duplicate path {path_value!r} (also used by '{seen_paths[path_value]}')")
            else:
                seen_paths[path_value] = entry_id
            if not (repo_root / path_value).exists():
                errors.append(f"{label}: path does not exist: {path_value}")

        for field in ("supersedes", "superseded_by"):
            if not isinstance(entry[field], list):
                errors.append(f"{label}: '{field}' must be a list")

        for field in ("summary", "status", "type"):
            if not isinstance(entry[field], str) or not entry[field]:
                errors.append(f"{label}: '{field}' must be a non-empty string")

    entries_by_id = {entry["id"]: entry for entry in entries if isinstance(entry, dict) and "id" in entry}
    for entry in entries:
        if not isinstance(entry, dict) or "id" not in entry:
            continue
        entry_id = entry["id"]
        label = f"findings index entry '{entry_id}'"

        for target_id in entry.get("supersedes", []) or []:
            if target_id not in entries_by_id:
                errors.append(f"{label}: supersedes unknown id: {target_id}")
            else:
                target = entries_by_id[target_id]
                if entry_id not in (target.get("superseded_by") or []):
                    errors.append(
                        f"{label}: supersedes '{target_id}', but '{target_id}'.superseded_by "
                        f"does not list '{entry_id}' back"
                    )

        for target_id in entry.get("superseded_by", []) or []:
            if target_id not in entries_by_id:
                errors.append(f"{label}: superseded_by unknown id: {target_id}")
            else:
                target = entries_by_id[target_id]
                if entry_id not in (target.get("supersedes") or []):
                    errors.append(
                        f"{label}: superseded_by '{target_id}', but '{target_id}'.supersedes "
                        f"does not list '{entry_id}' back"
                    )

    return errors


def validate_findings_coverage(index, repo_root=REPO_ROOT):
    """Every Markdown file directly inside a directory the index's own
    'purpose' field declares it covers must have an index entry. Does not
    look outside those directories, and does not recurse into subdirectories
    (docs/audits/<set>/ artifact packages are a different, un-indexed
    category, and this must not assume otherwise).
    """
    errors = []
    indexed_paths = {
        entry["path"] for entry in index.get("entries", []) if isinstance(entry, dict) and "path" in entry
    }
    for covered_dir in FINDINGS_INDEX_COVERED_DIRS:
        directory = repo_root / covered_dir
        if not directory.is_dir():
            continue
        for md_file in sorted(directory.glob("*.md")):
            rel_path = md_file.relative_to(repo_root).as_posix()
            if rel_path not in indexed_paths:
                errors.append(
                    f"findings coverage: {rel_path} exists under a covered directory "
                    f"({covered_dir}) but has no docs/findings/index.json entry"
                )
    return errors


def validate_context_map_extra(context_map, repo_root=REPO_ROOT):
    """Checks beyond agent_context.cmd_validate: validation_commands
    structural validity, route validation refs resolving to them, and
    generated-vs-tracked role/.gitignore consistency.
    """
    errors = []

    if "schema_version" in context_map and context_map["schema_version"] not in SUPPORTED_CONTEXT_MAP_SCHEMA_VERSIONS:
        errors.append(
            f"context map: unsupported schema_version {context_map['schema_version']!r} "
            f"(supported: {sorted(SUPPORTED_CONTEXT_MAP_SCHEMA_VERSIONS)})"
        )

    validation_commands = context_map.get("validation_commands", [])
    command_ids = set()
    for position, command in enumerate(validation_commands):
        label = f"context map validation_commands[{position}]"
        if not isinstance(command, dict):
            errors.append(f"{label}: not an object")
            continue
        missing = [field for field in ("id", "command") if not command.get(field)]
        if missing:
            errors.append(f"{label}: missing non-empty fields: {', '.join(missing)}")
            continue
        if command["id"] in command_ids:
            errors.append(f"{label}: duplicate validation_commands id: {command['id']}")
        command_ids.add(command["id"])

    routes = list(context_map.get("task_routes", []))
    fallback = context_map.get("unknown_task_fallback")
    if fallback is not None:
        routes = routes + [fallback]
    for route in routes:
        route_label = route.get("id", "<unnamed route>")
        for ref in route.get("validation", []) or []:
            ref_id = ref.get("ref") if isinstance(ref, dict) else None
            if ref_id is None:
                errors.append(f"context map route '{route_label}'.validation: entry missing 'ref'")
            elif ref_id not in command_ids:
                errors.append(
                    f"context map route '{route_label}'.validation references unknown "
                    f"validation_commands id: {ref_id}"
                )

    patterns = _load_gitignore_patterns(repo_root / ".gitignore")
    for entry in context_map.get("entries", []):
        entry_id = entry.get("id", "<unnamed entry>")
        path_value = entry.get("path")
        role = entry.get("role")
        if not _is_safe_relative_path(path_value):
            errors.append(f"context map entry '{entry_id}': unsafe or non-relative path: {path_value!r}")
            continue
        ignored = _gitignore_matches(path_value, patterns)
        if role == "generated_local_artifact" and not ignored:
            errors.append(
                f"context map entry '{entry_id}': declared generated_local_artifact but "
                f"{path_value} is not covered by .gitignore -- it would be tracked as source"
            )
        if role != "generated_local_artifact" and ignored:
            errors.append(
                f"context map entry '{entry_id}': declared role '{role}' (not a generated "
                f"local artifact) but {path_value} is covered by .gitignore -- it cannot be "
                f"tracked as claimed"
            )

    return errors


def validate_entry_points(repo_root=REPO_ROOT):
    """Lightweight structural-drift check: agent entry points must still
    point at the canonical contract, and the contract must still point at
    current-state and the findings index. Substring presence only -- this
    intentionally cannot and does not check that the prose reads well.
    """
    errors = []

    for rel_path in ENTRY_POINT_FILES:
        path = repo_root / rel_path
        if not path.exists():
            errors.append(f"agent entry points: expected entry point file missing: {rel_path}")
            continue
        text = path.read_text(encoding="utf-8")
        if AGENT_CONTRACT_PATH not in text:
            errors.append(
                f"agent entry points: {rel_path} does not reference the canonical contract "
                f"({AGENT_CONTRACT_PATH})"
            )

    contract_path = repo_root / AGENT_CONTRACT_PATH
    if not contract_path.exists():
        errors.append(f"agent entry points: canonical contract missing: {AGENT_CONTRACT_PATH}")
    else:
        text = contract_path.read_text(encoding="utf-8")
        for required_ref in (CURRENT_STATE_PATH, FINDINGS_INDEX_REL_PATH):
            if required_ref not in text:
                errors.append(
                    f"agent entry points: {AGENT_CONTRACT_PATH} does not reference {required_ref}"
                )

    return errors


def run_all_checks(repo_root=REPO_ROOT):
    errors = []

    try:
        index = load_json(repo_root / "docs" / "findings" / "index.json")
    except (OSError, ValueError) as error:
        return [f"findings index: failed to load/parse: {error}"]
    errors.extend(validate_findings_index(index, repo_root))
    errors.extend(validate_findings_coverage(index, repo_root))

    try:
        context_map = agent_context.load_context_map(repo_root / "docs" / "agent" / "context-map.json")
    except (OSError, ValueError) as error:
        return errors + [f"context map: failed to load/parse: {error}"]
    base_result = agent_context.cmd_validate(None, context_map)
    errors.extend(f"context map: {message}" for message in base_result["errors"])
    errors.extend(validate_context_map_extra(context_map, repo_root))

    errors.extend(validate_entry_points(repo_root))

    return errors


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--repo-root",
        type=Path,
        default=REPO_ROOT,
        help="Repository root to validate (default: this script's repository).",
    )
    args = parser.parse_args(argv)

    errors = run_all_checks(args.repo_root)
    result = {"valid": len(errors) == 0, "errors": errors}
    print(json.dumps(result, indent=2, sort_keys=True))
    return 0 if result["valid"] else 1


if __name__ == "__main__":
    sys.exit(main())
