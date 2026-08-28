# The Dark (`drk`) entry checklist completion record

- Date prepared: 2026-08-27
- Prepared by: Fable (Claude Code), technical validator
- Status: **pass 1 sealed and preserved; the preregistration §12 assignment-
  timing deviation is recorded below; pass 2 is assigned, attested, and
  authorized to open; the separate adjudicator is assigned and barred from
  Dark rows and both passes until pass 2 is sealed**
- Basis: repository state at measurement-freeze commit
  `70fa956515123b80d33ab08a13e938d54c6b66f8` ("Post-legends disco"), clean
  working tree
- Governs: `docs/findings/drk-structural-audit-preregistration.md` §12
- Non-observation statement: this record was prepared from control documents,
  committed Legends/earlier-audit artifacts, aggregate commands (`git`,
  `sha256sum`, `cargo`, `mtg-discover info`/`sets`), and the empty Dark outline
  only. No eligible Dark card was queried, inspected, quoted, or annotated. No
  held-out card was inspected. `docs/findings/drk-structural-audit.md` is an
  empty outline.

This is a governance artifact, not a findings document. It contains no Dark
measurement and authorizes nothing by itself: the audit opens only when every
§1 item reads **satisfied** and the program-owner sign-off in §4.3 is recorded
here.

## 1. Entry checklist status (preregistration §12)

Statuses: **satisfied** — committed evidence meets the item as written;
**partial** — evidence exists but the item is not fully demonstrated;
**pending** — the artifact or act does not yet exist (Phase 3 or later).

| # | Checklist item | Evidence | Status |
|---|---|---|---|
| 1 | Legends is adjudicated, closed, committed; final artifact hashes match its findings report. | `docs/findings/leg-structural-audit.md`; final annotation `678fcb58…`, export `c39a2d69…`, pass1 `89e314dc…`, pass2 `0615077043…`, metrics `91c0fc03…`, manifest `dfe57656…` — all verified equal to the findings report; final annotation has 426 unique keys, drift 0, dispositions {accept 409, defect 16, unsupported 1}, zero `adjudicate`/`ambiguous`. | **satisfied** |
| 2 | `docs/current-state.md` reflects the accepted live baseline. | `docs/current-state.md` "Last verified 2026-08-27"; Legends closure and "next eligible set is The Dark (`drk`)" recorded. Re-confirm at the freeze commit. | **satisfied** |
| 3 | Measurement-freeze commit, data snapshots, CR, protocol, guide, preregistration, and five earlier export hashes recorded. | Freeze commit `70fa956`. Protocol `1bc05d357b…`, guide `d31dee0a3b…`. Preregistration §3 populated from live output; pre-population control-text hash `b1c31c3b4d…`. Snapshot manifest `aa67c0c9…`; Dark experiment manifest `38d86700…` (validated). Snapshot hashes: oracle `9611b5d9…`, rulings `3064689880…`, default `d65608b4…`, cards.sqlite `d1c88cb9…`, CR `dc01ca54…`. Earlier export hashes: lea `aabc1bd5…`, leb `4cb90170…`, arn `4827f5be…`, atq `8ec1047b…`, leg `c39a2d69…`. | **satisfied** |
| 4 | Build and tests pass at the measurement-freeze commit. | At `70fa956`: `cargo test` 89 passed / 0 failed; `cargo fmt -- --check` clean; `cargo clippy --all-targets -- -D warnings` clean; `python -m unittest discover scripts/python -p "test_*.py"` 21 passed. | **satisfied** |
| 5 | A held-out-safe deterministic development export exists and is verified by aggregate counts only. | `verify_export_safety.py drk --runs 2`: 119→113 cards (6 held-out excluded), 110 with text, 163 records, byte-identical repeated JSON+TSV, 163 unique `(oracle_id,face,unit_index)` keys, JSON/TSV key sequences identical, **0** held-out export records. Retained `docs/audits/drk/units-export.tsv` SHA-256 `4460c2de…` equals the verifier's expected TSV hash. Aggregate integrity re-check: 161 top-level, 2 children, 0 parent-integrity violations. No row displayed. | **satisfied** |
| 6 | The cumulative held-out exclusion registry, including the four named incident exclusions, is bound to the audit. | Protocol §6.3 pool (count 2,096, digest `377e12bd…`, snapshot manifest) plus Combust, Malignus, Lava Burst, Wild Slash and every other logged incident; bound in `docs/manifests/experiment-dark-freeze-2026-08-27.json` (`partition.incident_registry`) and §2 below. | **satisfied** |
| 7 | Both independent annotators and the adjudicator are assigned. | §3 role blocks and §3.4 reconciliation. Pass 1 `claude-fable-5-pass1-2026-08-27`; pass 2 `gpt-5.3-codex-pass2-2026-08-27`; adjudicator `fresh-dark-adjudicator-2026-08-27`. The latter two assignments were made after pass 1 opened, which is the explicit preregistration §12 deviation recorded in §3.4. | **satisfied by owner-authorized reconciliation; timing deviation recorded** |
| 8 | Neither annotator has inspected eligible Dark text before the freeze. | §3.3 pass-1 attestation and §3.5 pass-2 attestation. Pass 2 additionally attests that it has not read pass 1 and opened no Dark row before the reconciliation and authorization were recorded. | **satisfied** |
| 9 | `docs/findings/drk-structural-audit.md` remains an empty outline until the baseline block is written verbatim. | File created as an empty outline; verified placeholders only. | **satisfied** |
| 10 | The program owner authorizes the audit to begin. | §4.3 preserves Avidiyah's original pass-1 authorization; §4.4 records Avidiyah's narrow governance reconciliation and authorization for pass 2 to open. | **satisfied** |

**Readiness statement:** all ten items now read **satisfied**, subject to the
explicit governance deviation and preservation decision in §3.4. Pass 1 opened
and sealed before the full roster was assigned, contrary to preregistration
§12. Program owner Avidiyah has preserved that sealed pass, assigned pass 2 and
the separate adjudicator, and authorized pass 2 to open. The audit is **not
adjudicated/closed**; the adjudicator may not inspect Dark rows, either pass, or
an agreement report until pass 2 is sealed.

## 2. Held-out exclusion registry bound to this audit

- Protocol §6.3 frozen pool: Oracle text present; `oracle_id` begins `f`;
  `first_is_fallback = 0`; `first_set` not `lea`/`leb`/`arn`. Identity count and
  non-disclosing digest bound via
  `docs/manifests/snapshot-scryfall-2026-08-25.json`
  (`377e12bdf80e0263c361f48ff2be241f600efd854b6eaa4f916d239a83067fc7`).
- Cumulative incident registry (additive): Combust, Malignus, Lava Burst, Wild
  Slash, and every other exclusion already recorded in the cumulative held-out
  incident registry. Any future accidental exposure is logged here and the
  exposed identity and linked faces/variants are excluded from later held-out
  sampling.
- The Dark's own `oracle_id`-prefix-`f` non-fallback cards remain held-out and
  are excluded from the development export by `--exclude-heldout`; the excluded
  count is reported as an aggregate by the Phase 3 verifier.

## 3. Role assignments and attestations

The preregistration required identities, hashes, and attestations to be filled
before any retained Dark export was opened. Pass 1 was instead authorized and
opened while pass 2 and the adjudicator remained pending. That history is not
rewritten: §3.4 records the deviation and the owner-directed reconciliation.
Pass 2 remains a separate independent annotator that cannot read pass 1. The
adjudicator remains a third identity that cannot read Dark rows or either pass
until both passes seal. The research lead and program owner are Avidiyah.

### 3.1 Annotator independence attestation (one per annotator)

```text
THE DARK ANNOTATOR INDEPENDENCE ATTESTATION
Pass:                      <1 | 2>
Annotator identity:        <id as it will appear in units-annotated-passN.tsv>
Date:                      <YYYY-MM-DD>
Protocol:                  structural-investigation-protocol.md v1.0, sha256 1bc05d357b24006a2eecc692f9bed5b86d1d828f116c2d741fb75662df4913bf
Annotation guide binding:  frozen Legends v1.0 / sha256 d31dee0a3b06494bd7ba0238be65b330e2366edb1b8bcf4e5e6a6f865de5d84b
Preregistration:           drk-structural-audit-preregistration.md, sha256 <…>
Development export:         docs/audits/drk/units-export.tsv, sha256 <…>

I attest that, as of the date above:
1. I have not queried, read, quoted, segmented, or annotated any Dark (drk)
   card's Oracle text in a heuristic-design, proposal, or review context before
   the baseline freeze.
2. I have not read, and will not read before both passes are sealed, the other
   annotator's annotations, notes, row-level hypotheses, or candidate proposal
   list.
3. I will annotate only the frozen held-out-safe development export identified
   by the sha256 above, will not run unfiltered card searches over the set, and
   will report any held-out identity that appears in my view immediately rather
   than continue.
4. I will use unsure / unsupported / ambiguous / adjudicate as defined in the
   preregistration rather than guess, and will not discuss or implement
   classifier changes (including P-LEG-1..3) during the pass.
5. My sealed pass will be delivered with a content hash and timestamp.
Exceptions or prior exposure to declare (write "none" if none): <…>

Signed: <identity>        Received by research lead: <id, YYYY-MM-DD>
```

### 3.2 Adjudicator assignment note

```text
THE DARK ADJUDICATOR ASSIGNMENT
Adjudicator identity:      <id, distinct from both annotators>
Assigned by:               research lead <id>, <YYYY-MM-DD>; approved by program owner <id>, <YYYY-MM-DD>
Independence:              neither pass-1 nor pass-2 annotator; has not inspected
                           eligible Dark text or rows before the freeze: <yes | exception>
Inputs the adjudicator may open, only after both passes are sealed:
  - sealed pass 1 <path, sha256, timestamp>
  - sealed pass 2 <path, sha256, timestamp>
  - the alignment/agreement report (preregistration §7.3)
Authority order: Comprehensive Rules and Oracle text control; rulings clarify.
Obligations: review every disagreement, non-accept, unsure/unsupported/
  ambiguous row, and alleged card-specific dependency; keep genuine ambiguity
  `ambiguous` and vocabulary gaps `unsupported` (gap:<class>); preserve pass
  ids and record each rationale; make no classifier proposal during
  adjudication; implement no P-LEG proposal.
Exceptions (write "none" if none): <…>

Signed (adjudicator): <id>   Research lead: <id>   Program owner: <id>
```

### 3.3 Recorded pass-1 annotator attestation (Fable)

```text
THE DARK ANNOTATOR INDEPENDENCE ATTESTATION
Pass:                      1
Annotator identity:        claude-fable-5-pass1-2026-08-27
Date:                      2026-08-27
Protocol:                  structural-investigation-protocol.md v1.0, sha256 1bc05d357b24006a2eecc692f9bed5b86d1d828f116c2d741fb75662df4913bf
Annotation guide binding:  frozen Legends v1.0 / sha256 d31dee0a3b06494bd7ba0238be65b330e2366edb1b8bcf4e5e6a6f865de5d84b
Preregistration:           drk-structural-audit-preregistration.md, pre-population control-text sha256 b1c31c3b4dc1a0b4774bb5ed64c0d8549970d7f9768d1813794b48ad3338c2ba
Development export:         docs/audits/drk/units-export.tsv, sha256 4460c2de445161e8e67ac3bc88c668e23ca6f2645ebaf0a483ddd455de4e0a16

I attest that, as of the date above:
1. I have not queried, read, quoted, segmented, or annotated any Dark (drk)
   card's Oracle text in a heuristic-design, proposal, or review context before
   the baseline freeze. During the freeze I saw only aggregate counts, the
   export column header, and file hashes — no Dark card row.
2. I have not read, and will not read before both passes are sealed, the other
   annotator's annotations, notes, row-level hypotheses, or candidate list.
3. I will annotate only the frozen held-out-safe development export identified
   by the sha256 above, will not run unfiltered card searches over the set, and
   will report any held-out identity that appears in my view immediately.
4. I will use unsure / unsupported / ambiguous / adjudicate as defined in the
   preregistration rather than guess, and will not discuss or implement
   classifier changes (including P-LEG-1..3) during the pass.
5. My sealed pass will be delivered with a content hash and timestamp.
Exceptions or prior exposure to declare: none

Signed: claude-fable-5-pass1-2026-08-27
Received by research lead: <Avidiyah, pending countersignature>
```

### 3.4 Governance deviation and owner-directed reconciliation

Recorded 2026-08-27 at the direction of program owner Avidiyah:

1. Preregistration §12 required both annotators and the adjudicator to be
   assigned before the audit opened. Pass 1 was subsequently authorized,
   opened, completed, and sealed while pass 2 and the adjudicator remained
   pending. This is a governance deviation from the preregistered assignment
   timing; it is not rewritten as prior compliance.
2. The sealed pass 1 is preserved and must not be restarted or modified.
   Its supplied SHA-256
   `06bc1c29ad33f10e9ea989d27eb0a69248190482b163ae005dabf1707cbcadb9`
   was verified by pass 2 with an aggregate-only hash check; pass 2 did not
   open, display, search, diff, or otherwise inspect its contents.
3. Preservation is authorized because the export and guide were already
   frozen; pass 1 remained independent; no held-out exposure or structural
   drift is known; pass 2 had not opened Dark rows or inspected pass-1
   judgments; and the deviation concerns assignment timing rather than the
   annotation instructions or row evidence.
4. The remaining roles are assigned as follows: pass 2
   `gpt-5.3-codex-pass2-2026-08-27`; adjudicator
   `fresh-dark-adjudicator-2026-08-27`. Both are distinct from Fable and from
   each other.
5. The adjudicator may not inspect an eligible Dark row, pass 1, pass 2, or an
   alignment/agreement report until pass 2 is sealed. The adjudicator must not
   serve as either annotator and must make no classifier proposal during
   adjudication.
6. Avidiyah authorizes this narrow reconciliation and authorizes pass 2 to open
   only after this record and the pass-2 attestation below are complete. This
   authorization preserves all frozen inputs and does not populate empirical
   findings or pass-1 results.

### 3.5 Recorded pass-2 annotator attestation

```text
THE DARK ANNOTATOR INDEPENDENCE ATTESTATION
Pass:                      2
Annotator identity:        gpt-5.3-codex-pass2-2026-08-27
Date:                      2026-08-27
Protocol:                  structural-investigation-protocol.md v1.0, sha256 1bc05d357b24006a2eecc692f9bed5b86d1d828f116c2d741fb75662df4913bf
Annotation guide binding:  frozen Legends v1.0 / sha256 d31dee0a3b06494bd7ba0238be65b330e2366edb1b8bcf4e5e6a6f865de5d84b
Preregistration:           drk-structural-audit-preregistration.md, sha256 eaa6bda819b4623aa45b8acdc3d1b3f23c3bdf6a856dae87058ac58bbd8d06a2
Development export:         docs/audits/drk/units-export.tsv, sha256 4460c2de445161e8e67ac3bc88c668e23ca6f2645ebaf0a483ddd455de4e0a16

I attest that, as of the date above:
1. I did not inspect eligible Dark card text before the frozen baseline in a
   heuristic-design, proposal, or review context.
2. I have not read pass 1 and will not read its annotations, notes, row-level
   hypotheses, candidate list, or comparisons before my pass is sealed.
3. I will annotate only the frozen held-out-safe pass-2 file identified above,
   will not run unfiltered card searches over the set, and will report any
   held-out identity that appears in my view immediately rather than continue.
4. I will not discuss row judgments with Fable.
5. I will use unsure / unsupported / ambiguous / adjudicate rather than guess,
   and I will not propose, discuss, or implement classifier changes, including
   P-LEG-1, P-LEG-2, or P-LEG-3, during this pass.
6. My sealed pass will be delivered with a content hash and timestamp.
Exceptions or prior exposure to declare: none

Signed: gpt-5.3-codex-pass2-2026-08-27
Received by research lead / program owner: Avidiyah, authorization supplied 2026-08-27
```

### 3.6 Adjudicator assignment and pre-access attestation

```text
THE DARK ADJUDICATOR ASSIGNMENT
Adjudicator identity:      fresh-dark-adjudicator-2026-08-27
Assigned by:               research lead Avidiyah, 2026-08-27; approved by program owner Avidiyah, 2026-08-27
Independence:              neither pass-1 nor pass-2 annotator; reserved as a
                           third identity and barred from eligible Dark text,
                           rows, both passes, and comparison output until pass 2
                           is sealed
Inputs the adjudicator may open, only after both passes are sealed:
  - sealed pass 1, supplied sha256 06bc1c29ad33f10e9ea989d27eb0a69248190482b163ae005dabf1707cbcadb9
  - sealed pass 2 `docs/audits/drk/units-annotated-pass2.tsv`, sha256
    `5b555148ce2072cf4a460e33c403a7819e8738096806f6dee679b0ed28643621`,
    sealed 2026-08-27T23:45:35-05:00
  - the alignment/agreement report (preregistration §7.3)
Authority order: Comprehensive Rules and Oracle text control; rulings clarify.
Obligations: review every disagreement, non-accept, unsure/unsupported/
  ambiguous row, and alleged card-specific dependency; keep genuine ambiguity
  `ambiguous` and vocabulary gaps `unsupported` (gap:<class>); preserve pass
  ids and record each rationale; make no classifier proposal during
  adjudication; implement no P-LEG proposal.
Exceptions: none declared at assignment

Assignment recorded by: gpt-5.3-codex-pass2-2026-08-27
Research lead / program owner authorization: Avidiyah, 2026-08-27
```

## 4. Program-owner authorization (to open)

### 4.1 Reproduction (governance checks for this record)

```powershell
git rev-parse HEAD ; git status --short
cargo build --release
cargo test
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
python -m unittest discover scripts/python -p "test_*.py"
python scripts/python/verify_export_safety.py drk --mtg .\target\release\mtg-discover.exe --runs 2
```

### 4.2 Original pre-opening requirements (historical)

The following list records what the entry record said remained before the
original opening. It is retained to preserve the assignment-timing history;
§3.4 and §4.4 record the subsequent reconciliation.

- Phase 3 input freeze: retained held-out-safe export, two blank pass copies,
  Dark experiment manifest, populated preregistration §3 and pre-audit baseline.
- Two annotator independence attestations (§3.1) and one adjudicator note
  (§3.2), bound to the retained export hash.
- Program-owner authorization (§4.3).

### 4.3 Program-owner "authorized to open" sign-off

Recorded 2026-08-27: program owner Avidiyah authorized opening in-session
("authorize the dark"). The authorization opens **pass 1** now; pass 2 and the
adjudicator are not yet assigned and must be assigned and attested before pass 2
opens and before adjudication respectively (§1 items 7–8). The audit cannot be
called adjudicated/closed until that roster is complete.

```text
THE DARK STRUCTURAL AUDIT — AUTHORIZATION TO OPEN
Program owner:             Avidiyah
Date:                      2026-08-27
Measurement-freeze commit: 70fa956515123b80d33ab08a13e938d54c6b66f8
Entry record reviewed:     docs/gates/dark-entry-record.md §1: items 1-6 and 9
                           satisfied; item 8 partial (pass-1 Fable attested);
                           items 7 and 10 addressed by this authorization for
                           pass 1, with pass 2 / adjudicator assignment deferred
                           to before their respective stages
Preregistration §3 block:  populated verbatim at the freeze commit (yes)
Build/tests at freeze:     cargo test 89 passed, 0 failed; fmt and clippy clean;
                           Python unittest 21 passed, 0 failed
Development export:        sha256 4460c2de445161e8e67ac3bc88c668e23ca6f2645ebaf0a483ddd455de4e0a16;
                           verified held-out-safe by aggregate counts only
                           (119 -> 113 cards, 6 held-out excluded; 110 with text;
                           163 records; held-out export records = 0)
Roles:                     pass 1 claude-fable-5-pass1-2026-08-27 (Fable),
                           attested §3.1/§3.3; pass 2 PENDING assignment;
                           adjudicator PENDING assignment
Held-out registry bound:   protocol §6.3 pool + Combust, Malignus, Lava Burst,
                           Wild Slash (and any later logged incident)

I authorize The Dark (drk) structural audit to open under protocol v1.0 and the
preregistration named above, exhaustively over the eligible non-held-out
development partition only. Pass 1 (Fable) may open the frozen export now; the
independent pass-2 annotator and the separate adjudicator must be assigned and
attested before pass 2 opens and before adjudication, and the audit is not
adjudicated/closed until then. This authorization does not accept any proposal
(including P-LEG-1..3), does not open Gate 1 review, and does not authorize
parser, IR, engine, or simulator work. It lapses if any preregistration §11.1
stop condition is later found to have been unmet at the freeze, in which case
the audit closes and a governance issue is recorded before any further row is
read.

Signed: Avidiyah (program owner), recorded in-session by Fable on 2026-08-27
```

### 4.4 Program-owner governance reconciliation and pass-2 authorization

Recorded 2026-08-27 from the program-owner direction supplied by Avidiyah:

```text
THE DARK STRUCTURAL AUDIT — GOVERNANCE RECONCILIATION / PASS-2 AUTHORIZATION
Program owner:             Avidiyah
Date:                      2026-08-27
Measurement-freeze commit: 70fa956515123b80d33ab08a13e938d54c6b66f8
Deviation:                 pass 1 opened before pass 2 and the adjudicator were
                           assigned, contrary to preregistration §12
Preservation decision:     preserve sealed pass 1 without restart or modification
Preservation rationale:    frozen export and guide; pass-1 independence;
                           no known held-out exposure or structural drift;
                           pass 2 unopened and unexposed to pass-1 judgments;
                           assignment-timing deviation only
Pass 2:                    gpt-5.3-codex-pass2-2026-08-27, attested §3.5
Adjudicator:               fresh-dark-adjudicator-2026-08-27, assigned §3.6;
                           no Dark-row/pass/comparison access before pass 2 seals
Frozen pass-2 input:       docs/audits/drk/units-annotated-pass2.tsv;
                           blank sha256 de150fc1a9cc0bebcfa78bf95b7ccda5de4835f587f76109ba7f0635d0ce63b7;
                           163 rows / 163 unique stable keys / structural keys
                           aligned with the retained export

I authorize the narrow governance reconciliation recorded in §3.4 and
authorize pass 2 to open after §3.5 is recorded. This does not revise the
frozen annotation instructions, does not expose pass 1 to pass 2, does not
authorize the adjudicator to open any Dark input before pass 2 is sealed, and
does not populate empirical findings or pass-1 results. The audit remains open
and unadjudicated until the independent pass is sealed and the assigned third
identity completes adjudication.

Signed: Avidiyah (program owner), direction supplied 2026-08-27;
recorded by gpt-5.3-codex-pass2-2026-08-27
```

## 5. Pass-2 sealing record

Pass 2 was sealed without opening or comparing pass 1 and without producing an
agreement report:

```text
Path:                       docs/audits/drk/units-annotated-pass2.tsv
Annotator:                  gpt-5.3-codex-pass2-2026-08-27
Sealed:                     2026-08-27T23:45:35-05:00
SHA-256:                    5b555148ce2072cf4a460e33c403a7819e8738096806f6dee679b0ed28643621
Rows / unique stable keys:  163 / 163
Frozen structural drift:    0
CR ids verified:            29 / 29
Pass comparison performed:  no
```

The file is sealed and must not be edited. The assigned adjudicator may now be
given the sealed pass artifacts and a separately generated alignment/agreement
report under preregistration §7.3; pass 2 did not generate or inspect that
report.
