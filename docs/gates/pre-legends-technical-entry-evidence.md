# Pre-Legends technical entry evidence

- Date verified: 2026-08-26 (America/Chicago)
- Scope: protocol T7, deterministic structural export, stable-key integrity,
  and lightweight provenance
- Status: **technical export gate satisfied; Legends audit remains closed**
- Snapshot: `scryfall-2026-08-25-cr-2026-08-07`
- Base repository commit: `2355b6c3d84bfc47eceea1013417945d19fd2c05`

## Non-observation statement

No eligible Legends row was printed, opened, quoted, or added to the Legends
findings document during this verification. The verifier captured row-bearing
JSON and TSV bytes internally, validated them, discarded them, and printed
only aggregate counts and SHA-256 values. No held-out identity was printed.

This record establishes technical capability and a repeatable aggregate-only
check. It does not assign annotators, populate the preregistration's final
freeze block, retain the annotation export, or authorize the audit to open.

## Guarantees now enforced

1. `--exclude-heldout` is available on `cards`, database-backed `segment
   --card`, and the native audit export family. The protocol 6.3 predicate is
   applied in SQLite before a card row reaches segmentation or serialization.
   The export also checks the same policy as a pre-serialization postcondition.
2. `scripts/python/export_units.py --exclude-heldout` now consumes the native
   filtered export rather than first enumerating unfiltered card rows. It
   validates exclusion metadata before writing the first TSV row.
3. Native export validates uniqueness of `(oracle_id, face, unit_index)`,
   parent existence within the same Oracle identity and face, preorder
   parentage, and declared sort order. The TSV layer independently repeats
   these checks and preserves the stable-key sequence exactly.
4. Both JSON and TSV order by lowercase card name, exact card name, Oracle ID,
   face, and preorder unit index. Reversing input-card order in a Rust test
   produces byte-identical serialized JSON.

The default unfiltered behavior remains available when the flag is omitted.
Fallback-first-printing behavior is unchanged. The aggregate Legends check
found zero fallback records, so that separate development-partition exclusion
does not alter this set's verified counts.

## Aggregate-only verification result

| Check | Result |
|---|---:|
| Cards before held-out exclusion | 310 |
| Cards with text before exclusion | 290 |
| Held-out Oracle identities excluded | 17 |
| Fallback-first-printing cards | 0 |
| Cards after exclusion | 293 |
| Cards with text after exclusion | 273 |
| Export records matching held-out definition | 0 |
| Stable-key rows / unique keys | 426 / 426 |
| JSON/TSV stable-key sequence | identical |
| Native JSON repeated runs | byte-identical (2 / 2) |
| Protocol TSV repeated runs | byte-identical (2 / 2) |
| Native JSON SHA-256 | `c2675a5837e5085e942584d3b9f777a18f70123092420b9e0ba70abcede1cf2a` |
| Protocol TSV SHA-256 | `ca7f95e55a20916dd15e1107a2951b4138e93edb3c2a1babf90e4e5dba8a1202` |

The global held-out query returned the frozen 2,096-card count. Its sorted,
lowercase Oracle-ID stream is bound without disclosure by SHA-256
`377e12bdf80e0263c361f48ff2be241f600efd854b6eaa4f916d239a83067fc7`.

## Provenance artifacts

- `docs/manifests/snapshot-scryfall-2026-08-25.json` binds the three ignored
  Scryfall inputs, `cards.sqlite`, the Comprehensive Rules file, the pool
  count/digest, producing command, and base commit.
- `docs/manifests/experiment-pre-legends-export-gate-2026-08-26.json` binds
  the question, snapshot, partition policy, exact commands, executable source
  hashes, toolchain, output hashes, and verification aggregates.
- `scripts/python/verify_manifests.py` validates schemas, every available bound
  file, and the held-out count/digest. No third-party dependency was added.

## Reproduction

```powershell
cargo build --release
cargo test
python -m unittest scripts.python.test_audit_metrics
python -m unittest scripts.python.test_export_units scripts.python.test_manifests
python scripts/python/verify_export_safety.py leg `
  --mtg .\target\release\mtg-discover.exe --runs 2
python scripts/python/verify_manifests.py `
  docs/manifests/snapshot-scryfall-2026-08-25.json `
  docs/manifests/experiment-pre-legends-export-gate-2026-08-26.json
```

Observed status: all commands passed; `cargo test` reported 87 passed, 0
failed; the required audit-metrics suite reported 11 passed; the added export
and manifest suites reported 8 passed combined. The export verifier reported
the aggregate table above and no row data.

## Remaining non-technical entry items

The actual annotation export must still be generated and retained at the
chosen clean freeze commit after annotator/adjudicator assignment and
independence attestations, then bound into the preregistration block. The
annotation-guide declaration, program-owner authorization, and final clean
build/fmt/clippy record also remain governed by
`docs/gates/legends-entry-record.md`. Gate 1's cross-era gold and sampled
held-out evaluation artifacts remain out of scope and incomplete.
