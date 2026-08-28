# Legends (`leg`) structural audit

- Status: **open — replacement adjudicator reauthorized 2026-08-27; two
  independent annotation passes may begin; no empirical findings yet**
- Preregistration: `docs/findings/leg-structural-audit-preregistration.md`
- Gate readiness: `docs/gates/gate-1-readiness-matrix.md`
- Entry checklist record: `docs/gates/legends-entry-record.md`

The preregistration §12 entry checklist passed and program owner Avidiyah
authorized opening on 2026-08-27. No individual Legends card had been inspected,
quoted, queried, or annotated when the aggregate blocks below were populated.
No empirical conclusion is recorded below.

## Frozen inputs and audit status

- Measurement freeze commit:
  `2e5173570077dab43cdfde2dc33d5a0e0831bd89`
- Frozen-input package commit:
  `48bbd42bda377c420ea7d31d9cdba3fb16a67dfb`
- Protocol v1.0 SHA-256:
  `1bc05d357b24006a2eecc692f9bed5b86d1d828f116c2d741fb75662df4913bf`
- Annotation guide, frozen Legends v1.0 SHA-256:
  `d31dee0a3b06494bd7ba0238be65b330e2366edb1b8bcf4e5e6a6f865de5d84b`
- Retained export: `docs/audits/leg/units-export.tsv`, SHA-256
  `c39a2d695b94ce33a2e16356dd93bc6dc614b7c83becfb2b2f72ad5cb298d2e3`
- Final experiment manifest:
  `docs/manifests/experiment-legends-freeze-2026-08-27.json`, SHA-256
  `dfe57656f3a81b172cc76806c7afa2ebe487328ff599d9cde171b3698a433f23`
- Pass 1: `claude-fable-5-pass1-2026-08-27`
- Pass 2: `gpt-5.6-pass2-2026-08-27`
- Adjudicator: `fresh-legends-adjudicator-2026-08-27` (replacement after the
  opening-control incident recorded in the entry record)

## Scope and exclusions

The development partition is the 293 Legends-first-printing cards remaining
after protocol §6.3 exclusion at the database boundary; 273 have Oracle text.
The aggregate-only verifier selected 310 cards before exclusion, excluded 17
held-out identities, and found zero held-out records in the export. Both passes
must cover all 426 frozen units. Held-out evaluation, implementation proposals,
Gate 1 review, semantic IR, and engine work remain out of scope.

## Preregistered hypotheses

The hypotheses remain exactly as frozen in
`docs/findings/leg-structural-audit-preregistration.md` §5. This document does
not restate or modify them.

## Pre-audit baseline

Recorded before either pass opened a row:

- cards / cards with text: 293 / 273;
- printed / rules-supplied units: 426 / 0;
- top-level / child units: 399 / 27;
- face histogram: face 0 = 426;
- non-null / null `prefix`: 0 / 426;
- distinct / singleton templates: 315 / 285;
- multi-sentence units: 60;
- kinds: activated 97, ante 2, cast restriction 4, CDA 2, keyword 91,
  prevention 9, replacement 8, static/spell 133, triggered 80;
- roles: ability 399, delayed trigger 5, granted 12, mode 10;
- source: printed 426.

## Verified findings

<!-- Empty until both annotation passes and adjudication support a finding. -->

## Bounded observations

<!-- Empty. Separate descriptive measurements from verified findings. -->

## Unsupported and ambiguous cases

<!-- Empty. Preserve gap:<class>, competing readings, and denominators. -->

## D14 independent-trigger observations

<!-- Empty. Do not merge with D19. -->

## D19 effect-created delayed-trigger observations

<!-- Empty. Follow docs/findings/d19-attachment-research-design.md. -->

## Proposed changes

<!-- Empty. S10 items 1-3 only; no implementation in this report. -->

## Measurements and agreement

| Field | Numerator | Denominator | Value |
|---|---:|---:|---:|
| Boundary precision |  |  |  |
| Missed boundaries |  | n/a |  |
| Boundary recall |  |  |  |
| Kind accuracy |  |  |  |
| Role accuracy |  |  |  |
| Source accuracy |  |  |  |
| Structural exact-card correctness |  |  |  |
| Independent row agreement |  |  |  |
| Independent exact-card agreement |  |  |  |
| Unsupported |  |  |  |
| Ambiguous |  |  |  |
| Unit novelty |  |  |  |
| Template novelty |  |  |  |

## Reproduction

<!-- Empty. Record every command only after the audit opens. -->

## Decision record

<!-- Empty. Evidence · options · decision · reversal evidence · affected docs. -->
