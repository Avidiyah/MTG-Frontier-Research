#!/usr/bin/env python3
"""Validate lightweight snapshot and experiment manifests and bound hashes."""

import argparse
from contextlib import closing
import hashlib
import json
from pathlib import Path
import re
import sqlite3


REPO_ROOT = Path(__file__).resolve().parent.parent.parent
SHA256_RE = re.compile(r"^[0-9a-f]{64}$")
HELD_OUT_SQL = """
    oracle_text IS NOT NULL AND oracle_text != ''
    AND lower(substr(oracle_id, 1, 1)) = 'f'
    AND first_is_fallback = 0
    AND lower(coalesce(first_set, '')) NOT IN ('lea', 'leb', 'arn')
"""


def file_sha256(path):
    digest = hashlib.sha256()
    with open(path, "rb") as source:
        for chunk in iter(lambda: source.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def require_fields(value, fields, label):
    missing = [field for field in fields if field not in value]
    if missing:
        raise ValueError(f"{label} is missing required fields: {', '.join(missing)}")


def validate_sha256(value, label):
    if not isinstance(value, str) or not SHA256_RE.fullmatch(value):
        raise ValueError(f"{label} is not a lowercase SHA-256 value")


def validate_bound_file(entry, label, allow_missing=False):
    require_fields(entry, ["path", "bytes", "sha256"], label)
    validate_sha256(entry["sha256"], f"{label}.sha256")
    path = REPO_ROOT / entry["path"]
    if not path.is_file():
        if allow_missing:
            return False
        raise ValueError(f"{label} bound file is missing: {entry['path']}")
    if path.stat().st_size != entry["bytes"]:
        raise ValueError(f"{label} byte length does not match")
    if file_sha256(path) != entry["sha256"]:
        raise ValueError(f"{label} content hash does not match")
    return True


def heldout_identity_digest(db_path):
    uri = db_path.resolve().as_uri() + "?mode=ro"
    with closing(sqlite3.connect(uri, uri=True)) as connection:
        return heldout_identity_digest_from_connection(connection)


def heldout_identity_digest_from_connection(connection):
    digest = hashlib.sha256()
    count = 0
    rows = connection.execute(
        f"SELECT lower(oracle_id) FROM cards WHERE {HELD_OUT_SQL} ORDER BY lower(oracle_id)"
    )
    for (oracle_id,) in rows:
        digest.update(oracle_id.encode("utf-8"))
        digest.update(b"\n")
        count += 1
    return count, digest.hexdigest()


def validate_snapshot(manifest, allow_missing=False):
    require_fields(
        manifest,
        [
            "schema_version",
            "manifest_type",
            "snapshot_id",
            "created_at_utc",
            "sources",
            "database",
            "comprehensive_rules",
            "heldout_partition",
            "producing_command",
            "repository_commit",
        ],
        "snapshot manifest",
    )
    if manifest["schema_version"] != "mtg-snapshot-manifest-v1":
        raise ValueError("unsupported snapshot manifest schema")
    if manifest["manifest_type"] != "dataset_snapshot":
        raise ValueError("snapshot manifest has the wrong manifest_type")
    if not manifest["sources"]:
        raise ValueError("snapshot manifest has no sources")

    checked = 0
    for index, source in enumerate(manifest["sources"]):
        require_fields(
            source,
            ["role", "bulk_type", "source_url", "retrieved_at_utc", "mtime_utc"],
            f"sources[{index}]",
        )
        checked += validate_bound_file(source, f"sources[{index}]", allow_missing)
    checked += validate_bound_file(manifest["database"], "database", allow_missing)
    checked += validate_bound_file(
        manifest["comprehensive_rules"], "comprehensive_rules", allow_missing
    )

    partition = manifest["heldout_partition"]
    require_fields(
        partition,
        ["policy", "identity_count", "identity_sha256", "digest_serialization"],
        "heldout_partition",
    )
    validate_sha256(partition["identity_sha256"], "heldout_partition.identity_sha256")
    db_path = REPO_ROOT / manifest["database"]["path"]
    if db_path.is_file():
        count, digest = heldout_identity_digest(db_path)
        if count != partition["identity_count"]:
            raise ValueError("held-out identity count does not match the database")
        if digest != partition["identity_sha256"]:
            raise ValueError("held-out identity digest does not match the database")
    elif not allow_missing:
        raise ValueError("cannot validate held-out partition without the database")
    return checked


def validate_experiment(manifest, allow_missing=False):
    require_fields(
        manifest,
        [
            "schema_version",
            "manifest_type",
            "experiment_id",
            "question",
            "snapshot_manifest",
            "partition",
            "commands",
            "repository",
            "environment",
            "started_at_utc",
            "ended_at_utc",
            "exit_status",
            "outputs",
            "verification",
            "reviewer",
            "interpretation_document",
        ],
        "experiment manifest",
    )
    if manifest["schema_version"] != "mtg-experiment-manifest-v1":
        raise ValueError("unsupported experiment manifest schema")
    if manifest["manifest_type"] != "experiment":
        raise ValueError("experiment manifest has the wrong manifest_type")
    if manifest["exit_status"] != 0:
        raise ValueError("experiment manifest records a failed run")
    checked = 0
    checked += validate_bound_file(
        manifest["snapshot_manifest"], "snapshot_manifest", allow_missing
    )
    for index, source in enumerate(manifest["repository"].get("source_files", [])):
        checked += validate_bound_file(source, f"source_files[{index}]", allow_missing)
    for index, command in enumerate(manifest["commands"]):
        require_fields(command, ["argv", "runs", "exit_status"], f"commands[{index}]")
        if not isinstance(command["argv"], list) or not command["argv"]:
            raise ValueError(f"commands[{index}].argv must be a non-empty list")
        if command["runs"] < 1 or command["exit_status"] != 0:
            raise ValueError(f"commands[{index}] records an invalid run")
    for index, output in enumerate(manifest["outputs"]):
        require_fields(
            output,
            ["format", "retained", "bytes", "sha256", "records"],
            f"outputs[{index}]",
        )
        validate_sha256(output["sha256"], f"outputs[{index}].sha256")
    verification = manifest["verification"]
    if not verification.get("byte_identical_repeated_exports"):
        raise ValueError("experiment does not attest byte-identical repeated exports")
    if verification.get("heldout_export_records") != 0:
        raise ValueError("experiment records held-out rows in auditor-visible output")
    if verification.get("stable_key_rows") != verification.get("stable_key_unique"):
        raise ValueError("experiment records duplicate stable keys")
    return checked


def validate_manifest(path, allow_missing=False):
    with open(path, encoding="utf-8") as source:
        manifest = json.load(source)
    manifest_type = manifest.get("manifest_type")
    if manifest_type == "dataset_snapshot":
        checked = validate_snapshot(manifest, allow_missing)
    elif manifest_type == "experiment":
        checked = validate_experiment(manifest, allow_missing)
    else:
        raise ValueError(f"unknown manifest_type in {path}")
    return {
        "path": str(Path(path).relative_to(REPO_ROOT)),
        "manifest_type": manifest_type,
        "schema_version": manifest["schema_version"],
        "sha256": file_sha256(path),
        "bound_files_checked": checked,
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("manifests", nargs="*")
    parser.add_argument(
        "--allow-missing-artifacts",
        action="store_true",
        help="validate schema and available files in a checkout without generated data",
    )
    parser.add_argument(
        "--heldout-digest",
        metavar="DB",
        help="print only the aggregate held-out count and identity digest",
    )
    args = parser.parse_args()
    if args.heldout_digest:
        count, digest = heldout_identity_digest(REPO_ROOT / args.heldout_digest)
        print(json.dumps({"identity_count": count, "identity_sha256": digest}, indent=2))
        return
    if not args.manifests:
        parser.error("provide at least one manifest or --heldout-digest")
    results = [
        validate_manifest(REPO_ROOT / path, args.allow_missing_artifacts)
        for path in args.manifests
    ]
    print(json.dumps({"valid": True, "manifests": results}, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
