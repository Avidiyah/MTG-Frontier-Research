# Reproducibility manifests

These committed JSON files bind small research-control records to generated
data without committing Scryfall bulk files, SQLite databases, or
auditor-visible development exports.

- `snapshot-scryfall-2026-08-25.json` records source/database/rules byte
  identities and the frozen held-out partition as an aggregate count plus a
  digest of sorted Oracle IDs. It deliberately does not disclose the IDs.
- `experiment-pre-legends-export-gate-2026-08-26.json` binds the T7 export
  verification to the snapshot, exact executable source files, commands,
  environment, output hashes, stable-key counts, and aggregate exclusion
  counts. The row-bearing outputs were not retained or displayed.

Validate the committed records and every locally available bound artifact:

```powershell
python scripts/python/verify_manifests.py `
  docs/manifests/snapshot-scryfall-2026-08-25.json `
  docs/manifests/experiment-pre-legends-export-gate-2026-08-26.json
```

Use `--allow-missing-artifacts` in a checkout that has not fetched the ignored
bulk files or built `cards.sqlite`; schema and committed-file hashes are still
checked. Manifest SHA-256 values are over the exact committed UTF-8 JSON bytes.
The snapshot records Scryfall's documented drop date because the original
fetch did not preserve the bulk object's upstream ID or exact `updated_at`;
those fields remain explicit `null` values rather than inferred metadata.

These manifests close technical provenance and export-safety evidence only.
They do not populate the Legends findings file, assign annotators, select a
held-out Gate 1 sample, or authorize the Legends audit to open.
