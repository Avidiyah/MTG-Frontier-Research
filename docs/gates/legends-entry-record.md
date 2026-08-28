# Legends (`leg`) entry checklist completion record

- Date prepared: 2026-08-26
- Prepared by: Fable (Claude Code), technical validator
- Status: **entry record; audit not open — blockers remain (§3–§4)**
- Basis: repository state at `2355b6c` ("Merge superseded set audit tooling
  branch"), clean working tree; P-ATQ technical acceptance evidence merged at
  `bcf9eaa`
- Governs: `docs/findings/leg-structural-audit-preregistration.md` §12 (entry
  checklist) and `docs/gates/gate-1-readiness-matrix.md` §5 (pre-Legends entry
  dependencies)
- Non-observation statement: this record was prepared from control documents,
  committed acceptance artifacts, aggregate commands (`git`, `sha256sum`,
  `cargo test`) and the empty Legends outline only. No eligible Legends card
  was queried, inspected, quoted, or annotated. No held-out card was
  inspected. `docs/findings/leg-structural-audit.md` is unchanged except for a
  pointer to this file.

This is a governance artifact, not a findings document. It contains no
Legends measurement and authorizes nothing by itself: the audit opens only
when every §3 item reads **satisfied** and the program-owner sign-off in §5.3
is recorded here.

## 1. What "P-ATQ closed" means, and where it is recorded

The Legends preregistration permits activation "only after the P-ATQ
technical acceptance package closes without a material contradiction". That
condition is met by the following committed artifacts:

| Claim | Artifact |
|---|---|
| Research-lead acceptance of P-ATQ-1..4 (status: "research and technical S10 acceptance complete"; "Research prerequisite for Legends: PASSED") | `docs/findings/p-atq-research-acceptance-assessment.md` |
| Technical S10/S11 package at `8e83221`: frozen inputs, before/after corpus histograms, per-proposal evidence, regression on `lea`/`leb`/`arn`/`atq` | `docs/audits/corpus-checks/2026-08-26-post-patq-merge.md` |
| Protocol S8 searches for P-ATQ-3 (3,572 firings, binary cross-check 0 mismatches) and P-ATQ-4 (full 12,466-unit instant/sorcery-face sweep, 30 positives) | `docs/audits/corpus-checks/2026-08-26-patq-s8-search.md`, produced by `scripts/python/corpus_checks/check_patq_s8.py` |
| P-ATQ-1 isolated measurement (`8c0f229` → `bf9eb04`: 982 → 861 children, 0 added) | `docs/audits/corpus-checks/2026-08-26-delayed-split-overseg-post-p-atq-1.md`, `…-post-p-atq-1-4-HEAD.md` |
| S10 decision paragraphs for all four proposals and the P-ATQ-1 acceptance record | `docs/findings/atq-structural-audit.md` §8 |
| Re-annotation of the five rule-(c) rows (`under`, missed 1, D15 slot), fresh exports, regenerated metrics, drift 0 | commits `38e97f6`, `d00bdcd`, `9c01daa`; `docs/audits/{lea,leb,arn,atq}/` |
| Deferred-work register entries D15–D18 marked accepted/complete | `docs/roadmap.md` §22 |

**Material contradiction check.** One evidence-record correction exists and is
recorded explicitly rather than absorbed: the pre-change kind-rules report
described eight prefixed `prevention_effect` units as misfires; the
acceptance assessment corrects this to five kind corrections plus three
correct positives (Urza's Science Fair Project, Khârn the Betrayer, Diamond
Weapon), each adjudicated `ACCEPT` with CR 207.2d / 615.1a citations and
rulings. The P-ATQ-1 outcome (all 121 comma/colon children reverted, not the
estimated ~113) is likewise explained in the acceptance record (rule (c) was
deleted rather than guarded; §6 of the audit found none of the eight extra
parents to be a reference unit). Neither changes a disposition. No other
disagreement between the research assessment and the technical package is
recorded in any control document.

## 2. Post-merge reconciliation performed in this packet

Only wording that implied P-ATQ technical closure was still pending was
touched. Historical log entries were marked superseded, not rewritten, so the
dated record of what each session could and could not verify is preserved.

| File | Change |
|---|---|
| `docs/current-state.md` | Main-body P-ATQ-3 clause: "S10 acceptance pending Codex's adjudication of 3 residual prevention rows" → accepted within measured scope, residuals adjudicated correct positives (with artifact path). Four log entries (P-ATQ-2, P-ATQ-3, P-ATQ-4 "not yet accepted"; "all four implementations remain pending") given explicit supersession markers pointing at the acceptance entries. One new log entry for this reconciliation. |
| `docs/gates/gate-1-readiness-matrix.md` | Decision basis updated to `2355b6c`; §5 rows "P-ATQ technical acceptance package" and "Live baseline reconciled" → **satisfied**; "Frozen identities" deferred → **missing (now actionable)**; T7 status stated precisely; §6 conclusion no longer lists P-ATQ closure as open. G1.2 "assign annotators after P-ATQ closes" → now. |
| `docs/findings/atq-structural-audit.md` | Supersession markers on the four "Implemented, not yet accepted" dispositions and on the "left at pre-merge state pending re-annotation" sentence (since done at `38e97f6` / `d00bdcd` / `9c01daa`). |
| `docs/findings/leg-structural-audit.md` | One pointer line to this record. No section populated. |

Not changed, deliberately:

- `docs/findings/leg-structural-audit-preregistration.md` — frozen design;
  its "only after … closes" wording is a condition, not a stale status. Its
  sha256 at `2355b6c` is recorded in §3 item 4 and will change if it is
  edited.
- `docs/roadmap.md` — D15–D18 already read accepted/complete. The header's
  "Active phase: Phase 0" and §21 ("Until Gate 0 passes") predate the Gate 0
  pass and are not P-ATQ language; flagged to the program owner in §4, not
  edited here.

## 3. Entry checklist completion record (preregistration §12)

Statuses use the gate-matrix meanings: **satisfied** — committed evidence
meets the item as written; **partial** — evidence exists but the item is not
fully demonstrated; **missing** — the artifact or act does not yet exist.

| # | Checklist item (verbatim) | Evidence artifact(s) | Status | Remaining action · blocker owner |
|---|---|---|---|---|
| 1 | Claude's technical P-ATQ package passes and is incorporated into the acceptance record. | `docs/audits/corpus-checks/2026-08-26-post-patq-merge.md`; `…-patq-s8-search.md`; `docs/findings/atq-structural-audit.md` §8; `docs/findings/p-atq-research-acceptance-assessment.md` ("The subsequent technical package is recorded in …"); merge `bcf9eaa` | **satisfied** | — |
| 2 | Any contradiction is adjudicated without silently changing P-ATQ dispositions. | §1 above; `p-atq-research-acceptance-assessment.md` "Residual adjudications" and "Text for Claude's S10 decision record" item 4; `atq-structural-audit.md` P-ATQ-1 acceptance record ("Expected vs measured") | **satisfied** | — |
| 3 | `docs/current-state.md` reflects the accepted live baseline. | `docs/current-state.md` §"Current segmentation and normalization baseline" (71,563 / 970 / 37,299; roles 67,045 · 2,121 · 1,506 · 891; 861 + 30 delayed) agrees with `2026-08-26-post-patq-merge.md` §3 post column; stale clauses reconciled per §2 | **satisfied** (at the commit that lands this packet) | Re-check at the freeze commit · technical measurement owner |
| 4 | Frozen commit, data snapshots, CR, protocol, guide, and earlier export hashes are recorded. | Preregistration §3 is populated from live values. Clean measurement freeze `2e5173570077dab43cdfde2dc33d5a0e0831bd89`; governance-only role commit `b693a0c`. Snapshot manifest and final `docs/manifests/experiment-legends-freeze-2026-08-27.json` validate; final manifest sha256 `8c1d36b35f13ab8da8d45f1ee1c5fc1de009ff8b23ec59d0986722167174dc5c`. Protocol, guide, pre-population preregistration, CR, database/input, and final `lea`/`leb`/`arn`/`atq` export/annotation hashes are recorded in preregistration §3. | **satisfied** | — |
| 5 | Build and tests pass at the frozen commit. | Clean freeze candidate `2e5173570077dab43cdfde2dc33d5a0e0831bd89`: `cargo build --release` passed; `cargo test` **88 passed, 0 failed**; `cargo fmt -- --check` passed; `cargo clippy --all-targets -- -D warnings` passed; required Python suites **20 passed, 0 failed**; snapshot manifest validated. | **satisfied** | — |
| 6 | A held-out-safe deterministic development export exists and has been verified by aggregate counts only. | Aggregate-only verifier at freeze source bytes: 310 / 290 cards before exclusion, 17 held-out identities excluded, 293 / 273 after, 0 held-out export records, 426 / 426 unique keys, JSON/TSV key sequences identical, and two byte-identical runs per format. Expected TSV sha256 `c39a2d695b94ce33a2e16356dd93bc6dc614b7c83becfb2b2f72ad5cb298d2e3`. Retained `docs/audits/leg/units-export.tsv` and unopened pass copies match that hash; final manifest binds the file. | **satisfied** | — |
| 7 | The cumulative held-out exclusion registry, including the four named incident exclusions, is bound to the audit. | Protocol §6.3 plus Combust, Malignus, Lava Burst, and Wild Slash; preregistration §3 cites the registry. Snapshot and final experiment manifests bind the 2,096-card pool by non-disclosing digest `377e12bdf80e0263c361f48ff2be241f600efd854b6eaa4f916d239a83067fc7`; validation recomputed it from `cards.sqlite`. Research lead Avidiyah confirmed no additional incident for this freeze. | **satisfied** | — |
| 8 | Both independent annotators and the adjudicator are assigned. | §7 below: pass 1 `claude-fable-5-pass1-2026-08-27`; pass 2 `gpt-5.6-pass2-2026-08-27`; adjudicator `copilot-cli-adjudicator-2026-08-27`; assigned and approved by Avidiyah on 2026-08-27. | **satisfied** | — |
| 9 | Neither annotator has inspected eligible Legends text before the freeze. | Two §7 annotator attestations, personally confirmed to research lead Avidiyah before export retention, bind protocol, guide, preregistration, and expected TSV hash and declare no exceptions; adjudicator note declares the same non-observation condition. | **satisfied** | — |
| 10 | `docs/findings/leg-structural-audit.md` remains an empty outline until the baseline block is written verbatim. | File at `2355b6c` + this packet: header, placeholder comments, empty measurement table only (verified by reading the file) | **satisfied** | Keep unchanged until §3 item 4 is written verbatim · everyone |
| 11 | The program owner authorizes the audit to begin. | None | **missing** — **hard blocker** | Sign §5.3 only after items 1–10 all read satisfied · program owner |

**Readiness statement:** 10 satisfied (1–10), 1 missing (11). The Legends audit is
**not ready to open**. No eligible row may be inspected until this table shows
eleven **satisfied** entries and §5.3 is signed.

## 4. Open blockers, in dependency order

1. **Program-owner authorization** (item 11) — Avidiyah reviews items 1–10
   and signs §5.3. No technical, role, export, provenance, or registry blocker
   remains.

Observation outside this packet's scope, for the program owner:
`docs/roadmap.md` still reads "Active phase: Phase 0" and §21 "Until Gate 0
passes", while `docs/gates/gate-0-evidence.md` records Gate 0 passed and
countersigned on 2026-08-26 and three sets have since been audited under the
Phase 1 protocol. The roadmap's earliest Phase 1 transition date is
2026-09-16. This is a schedule/status inconsistency, not a P-ATQ contradiction;
it does not appear in the preregistration's stop conditions and is left for a
roadmap decision record (§2.3 / §3.1).

## 5. Role templates

Copy the relevant block into this file under a new "## 6. Signatures"
heading (or into the preregistration §3 identities) when executed. Fill every
`<…>`; do not delete a clause. Dates are absolute. "Identity" means the
agent/session or person label that will appear in the annotation file's
annotator column (compare `fork-pass2` and
`fable-reannotation-p-atq-1-2026-08-26` in `docs/audits/atq/`).

### 5.1 Annotator independence attestation (one per annotator)

```text
LEGENDS ANNOTATOR INDEPENDENCE ATTESTATION
Pass:                      <1 | 2>
Annotator identity:        <id as it will appear in units-annotated-passN.tsv>
Date:                      <YYYY-MM-DD>
Protocol:                  structural-investigation-protocol.md v1.0, sha256 <…>
Annotation guide binding:  <guide version / sha256 declared under item 4c>
Preregistration:           leg-structural-audit-preregistration.md, sha256 <…>

I attest that, as of the date above:
1. I have not queried, read, quoted, segmented, or annotated any Legends (leg)
   card's Oracle text in a heuristic-design, proposal, or review context before
   the baseline freeze (preregistration §11.1, last bullet).
2. I have not read, and will not read before both passes are sealed, the other
   annotator's annotations, notes, row-level hypotheses, or candidate proposal
   list (preregistration §7.2).
3. I will annotate only the frozen held-out-safe development export identified
   by sha256 <…>, will not run unfiltered card searches over the set, and will
   report any held-out identity that appears in my view immediately
   (preregistration §11.2) rather than continue.
4. I will use unsure / unsupported / ambiguous / adjudicate as defined in
   preregistration §8 rather than guess, and will not discuss implementation
   proposals during the pass (§7.2).
5. My sealed pass will be delivered with a content hash and timestamp.
Exceptions or prior exposure to declare (write "none" if none): <…>

Signed: <identity>        Received by research lead: <id, YYYY-MM-DD>
```

### 5.2 Adjudicator assignment note

```text
LEGENDS ADJUDICATOR ASSIGNMENT
Adjudicator identity:      <id>
Assigned by:               research lead <id>, <YYYY-MM-DD>; approved by program owner <id>, <YYYY-MM-DD>
Independence:              the adjudicator is neither pass-1 nor pass-2 annotator
                           and has not inspected eligible Legends text before the freeze:  <yes | exception stated below>
Inputs the adjudicator may open, and only after both passes are sealed:
  - sealed pass 1 <path, sha256, timestamp>
  - sealed pass 2 <path, sha256, timestamp>
  - the alignment/agreement report produced under preregistration §7.3
Authority order (preregistration §7.4): Comprehensive Rules and current Oracle
  text control; official rulings clarify but do not override the CR.
Obligations:
  - review every disagreement, every non-accept row, every unsure /
    unsupported / ambiguous row, and every alleged card-specific dependency;
  - keep a genuine rules ambiguity `ambiguous` with both readings, and a
    vocabulary gap `unsupported` with kind_expected = gap:<class>;
  - preserve original pass ids and record the rationale for each resolution;
  - do not call the audit adjudicated while any row is merely `adjudicate`
    without a documented reason;
  - make no classifier proposal during adjudication (preregistration §9).
Exceptions to declare (write "none" if none): <…>

Signed (adjudicator): <id>     Research lead: <id>     Program owner: <id>
```

### 5.3 Program-owner "authorized to open audit" sign-off

```text
LEGENDS STRUCTURAL AUDIT — AUTHORIZATION TO OPEN
Program owner:             <id>
Date:                      <YYYY-MM-DD>
Freeze commit:             <git rev-parse HEAD>
Entry record reviewed:     docs/gates/legends-entry-record.md, §3 shows eleven
                           "satisfied" entries as of <YYYY-MM-DD>
Preregistration §3 block:  populated verbatim at the freeze commit (yes)
Build/tests at freeze:     cargo test <N> passed, 0 failed; fmt and clippy clean
Development export:        sha256 <…>; verified held-out-safe by aggregate
                           counts only (rows before/after <n>/<m>; prefix-f
                           oracle_ids after filter = 0)
Roles:                     pass 1 <id>, pass 2 <id>, adjudicator <id>;
                           attestations 5.1 ×2 and note 5.2 on file
Held-out registry bound:   protocol §6.3 pool + Combust, Malignus, Lava Burst,
                           Wild Slash (and any later logged incident)

I authorize the Legends (leg) structural audit to open under protocol v1.0 and
the preregistration named above, exhaustively over the eligible non-held-out
development partition only. This authorization does not accept any proposal,
does not open Gate 1 review, and does not authorize parser, IR, engine, or
simulator work. It lapses if any preregistration §11.1 stop condition is later
found to have been unmet at the freeze, in which case the audit closes and a
governance issue is recorded before any further row is read.

Signed: <program owner id>
```

## 6. Reproduction (governance checks used for this record)

```powershell
git rev-parse HEAD ; git status --short
cargo build --release
cargo test                                        # 88 passed, 0 failed
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
python -m unittest scripts.python.test_audit_metrics scripts.python.test_export_units scripts.python.test_manifests
python scripts/python/verify_export_safety.py leg --mtg .\target\release\mtg-discover.exe --runs 2
python scripts/python/verify_manifests.py docs/manifests/snapshot-scryfall-2026-08-25.json docs/manifests/experiment-legends-freeze-2026-08-27.json
Get-Content docs/findings/leg-structural-audit.md                   # placeholders only
```

The later T7 verification captured filtered Legends exports internally and
discarded them; it printed only counts and hashes. No Legends row was opened,
printed, quoted, or inspected.

## 7. Legends role assignments and attestations

The research lead collected both annotator confirmations before any retained
Legends export was generated. The identity shown for each pass is the exact
value that must appear in that pass's `annotator` column.

```text
LEGENDS ANNOTATOR INDEPENDENCE ATTESTATION
Pass:                      1
Annotator identity:        claude-fable-5-pass1-2026-08-27
Date:                      2026-08-27
Protocol:                  structural-investigation-protocol.md v1.0, sha256 1bc05d357b24006a2eecc692f9bed5b86d1d828f116c2d741fb75662df4913bf
Annotation guide binding:  frozen Legends v1.0 / sha256 d31dee0a3b06494bd7ba0238be65b330e2366edb1b8bcf4e5e6a6f865de5d84b
Preregistration:           leg-structural-audit-preregistration.md, sha256 4c3e66afc0da339a67aefee14d026023d5a3ac6302c194cba6fa9025adf14ecf

I attest that, as of the date above:
1. I have not queried, read, quoted, segmented, or annotated any Legends (leg)
   card's Oracle text in a heuristic-design, proposal, or review context before
   the baseline freeze.
2. I have not read, and will not read before both passes are sealed, the other
   annotator's annotations, notes, row-level hypotheses, or candidate proposal
   list.
3. I will annotate only the frozen held-out-safe development export identified
   by sha256 c39a2d695b94ce33a2e16356dd93bc6dc614b7c83becfb2b2f72ad5cb298d2e3,
   will not run unfiltered card searches over the set, and will report any
   held-out identity that appears in my view immediately rather than continue.
4. I will use unsure / unsupported / ambiguous / adjudicate as defined in the
   preregistration rather than guess, and will not discuss implementation
   proposals during the pass.
5. My sealed pass will be delivered with a content hash and timestamp.
Exceptions or prior exposure to declare: none

Signed: claude-fable-5-pass1-2026-08-27
Received by research lead: Avidiyah, 2026-08-27
```

```text
LEGENDS ANNOTATOR INDEPENDENCE ATTESTATION
Pass:                      2
Annotator identity:        gpt-5.6-pass2-2026-08-27
Date:                      2026-08-27
Protocol:                  structural-investigation-protocol.md v1.0, sha256 1bc05d357b24006a2eecc692f9bed5b86d1d828f116c2d741fb75662df4913bf
Annotation guide binding:  frozen Legends v1.0 / sha256 d31dee0a3b06494bd7ba0238be65b330e2366edb1b8bcf4e5e6a6f865de5d84b
Preregistration:           leg-structural-audit-preregistration.md, sha256 4c3e66afc0da339a67aefee14d026023d5a3ac6302c194cba6fa9025adf14ecf

I attest that, as of the date above:
1. I have not queried, read, quoted, segmented, or annotated any Legends (leg)
   card's Oracle text in a heuristic-design, proposal, or review context before
   the baseline freeze.
2. I have not read, and will not read before both passes are sealed, the other
   annotator's annotations, notes, row-level hypotheses, or candidate proposal
   list.
3. I will annotate only the frozen held-out-safe development export identified
   by sha256 c39a2d695b94ce33a2e16356dd93bc6dc614b7c83becfb2b2f72ad5cb298d2e3,
   will not run unfiltered card searches over the set, and will report any
   held-out identity that appears in my view immediately rather than continue.
4. I will use unsure / unsupported / ambiguous / adjudicate as defined in the
   preregistration rather than guess, and will not discuss implementation
   proposals during the pass.
5. My sealed pass will be delivered with a content hash and timestamp.
Exceptions or prior exposure to declare: none

Signed: gpt-5.6-pass2-2026-08-27
Received by research lead: Avidiyah, 2026-08-27
```

```text
LEGENDS ADJUDICATOR ASSIGNMENT
Adjudicator identity:      copilot-cli-adjudicator-2026-08-27
Assigned by:               research lead Avidiyah, 2026-08-27; approved by program owner Avidiyah, 2026-08-27
Independence:              the adjudicator is neither pass-1 nor pass-2 annotator
                           and has not inspected eligible Legends text before the freeze: yes
Inputs the adjudicator may open, and only after both passes are sealed:
  - sealed pass 1 docs/audits/leg/units-annotated-pass1.tsv, sha256 and timestamp pending sealing
  - sealed pass 2 docs/audits/leg/units-annotated-pass2.tsv, sha256 and timestamp pending sealing
  - the alignment/agreement report produced under preregistration §7.3
Authority order: Comprehensive Rules and current Oracle text control; official
  rulings clarify but do not override the CR.
Obligations:
  - review every disagreement, every non-accept row, every unsure /
    unsupported / ambiguous row, and every alleged card-specific dependency;
  - keep a genuine rules ambiguity ambiguous with both readings, and a
    vocabulary gap unsupported with kind_expected = gap:<class>;
  - preserve original pass ids and record the rationale for each resolution;
  - do not call the audit adjudicated while any row is merely adjudicate
    without a documented reason;
  - make no classifier proposal during adjudication.
Exceptions to declare: none

Signed (adjudicator): copilot-cli-adjudicator-2026-08-27
Research lead: Avidiyah
Program owner: Avidiyah
```
