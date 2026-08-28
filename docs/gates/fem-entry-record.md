# Fallen Empires (`fem`) entry checklist completion record

- Date prepared: 2026-08-28
- Prepared by: Claude Code (Sonnet 5), technical validator
- Status: **audit not open; the original pass-1 annotator is disqualified by
  the row-exposure incident in §3.4 and a replacement remains unassigned.
  Pass 2 and the adjudicator are assigned in §3.2-§3.3, but pass 2's
  independence attestation remains missing after the pre-authorization
  file-state deviation recorded in §3.2. The §4.3 conditional authorization
  is suspended. The earlier held-out exposure incident (§2) remains logged.**
- Basis: repository state at measurement-freeze commit
  `2823b1226c7d10bdb1d47d41a17cfeda709d4ecb`, clean working tree
- Governs: `docs/findings/fem-structural-audit-preregistration.md` §12
- Non-observation statement: this record was prepared from control documents,
  committed Legends/Dark/earlier-audit artifacts, aggregate commands (`git`,
  `sha256sum`, `cargo`, `mtg-discover info`/`sets`/`audit summary`/`audit
  signals`), and the empty Fallen Empires outline. **Exception, disclosed in
  full in §2:** `mtg-discover audit novelty fem` was run without a
  held-out-exclusion option and its complete output, including card names and
  normalized templates, was displayed; seven of those displayed identities are
  held-out. No other eligible Fallen Empires card was queried, inspected,
  quoted, or annotated beyond that incident and the aggregate-only commands
  above.

This is a governance artifact, not a findings document. It contains no Fallen
Empires measurement and authorizes nothing by itself: the audit opens only
when every §1 item reads **satisfied** and the program-owner sign-off in §4 is
recorded here.

## 1. Entry checklist status (preregistration §12)

Statuses: **satisfied** — committed evidence meets the item as written;
**partial** — evidence exists but the item is not fully demonstrated;
**pending** — the artifact or act does not yet exist.

| # | Checklist item | Evidence | Status |
|---|---|---|---|
| 1 | The Dark is adjudicated, closed, committed; final artifact hashes match its findings report. | `docs/findings/drk-structural-audit.md`; final annotation `aed8ab6309…`, export `4460c2de44…` — both verified equal to the findings report; final annotation has 163 unique keys, drift 0, dispositions {accept 159, defect 3, ambiguous 1}, zero `adjudicate`/`unsupported`. | **satisfied** |
| 2 | `docs/current-state.md` reflects the accepted live baseline. | `docs/current-state.md` "Last verified 2026-08-28"; The Dark closure and "next eligible set is Fallen Empires (`fem`)" recorded. | **satisfied** |
| 3 | Measurement-freeze commit, data snapshots, CR, protocol, guide, preregistration, and six earlier export hashes recorded. | Freeze commit `2823b122`. Protocol `1bc05d357b…`, guide `d31dee0a3b…`. Preregistration §3 populated from live output; preregistration file sha256 `41e298939d…`. Snapshot hashes unchanged from the Dark freeze: oracle `9611b5d9…`, rulings `3064689880…`, default `d65608b4…`, cards.sqlite `d1c88cb9…`, CR `dc01ca54…`. Earlier export hashes: lea `aabc1bd5…`, leb `4cb90170…`, arn `4827f5be…`, atq `8ec1047b…`, leg `c39a2d69…`, drk `4460c2de44…`. | **satisfied** |
| 4 | Build and tests pass at the measurement-freeze commit. | At `2823b122`: `cargo test` 89 passed / 0 failed; `cargo fmt -- --check` clean; `cargo clippy --all-targets -- -D warnings` clean; `python -m unittest discover scripts/python -p "test_*.py"` 47 passed. | **satisfied** |
| 5 | A held-out-safe deterministic development export exists and is verified by aggregate counts only. | `verify_export_safety.py fem --runs 2`: 102 → 93 cards (9 held-out excluded), 92 with text, 176 records, byte-identical repeated JSON+TSV, 176 unique `(oracle_id,face,unit_index)` keys, JSON/TSV key sequences identical, **0** held-out export records. Retained `docs/audits/fem/units-export.tsv` SHA-256 `095a25a7a0…` equals the verifier's expected TSV hash. Aggregate integrity re-check: 170 top-level, 6 children, 0 parent-integrity violations. No development-partition row displayed (see §2 for the separate incident). | **satisfied** |
| 6 | The cumulative held-out exclusion registry, including prior named incident exclusions, is bound to the audit. | Protocol §6.3 pool (count 2,096, digest `377e12bd…`, snapshot manifest) plus Combust, Malignus, Lava Burst, Wild Slash, and — newly added by this preparation's own incident, §2 — Farrel's Mantle, Fungal Bloom, Orgg, Spore Flower, Svyelunite Priest, Thelon's Chant, Vodalian War Machine; bound in `docs/manifests/experiment-fem-freeze-2026-08-28.json` and §2 below. | **satisfied, with a new incident added during this preparation** |
| 7 | Both independent annotators and the adjudicator are assigned, none is the disqualified preparer. | Original pass 1 `codex-pass1-2026-08-28` is disqualified by §3.4; replacement pass 1 is unassigned. Pass 2 `copilot-pass2-2026-08-28` and adjudicator `fresh-fem-adjudicator-2026-08-28` remain assigned. | **partial** |
| 8 | Neither annotator has inspected eligible Fallen Empires text before the freeze. | Original pass 1's pre-freeze attestation remains historically true, but §3.4 exposed that identity to eligible text and pass-2 judgments before either pass sealed, destroying pass independence and disqualifying it. Pass 2 still lacks a signed independence attestation. | **partial** |
| 9 | `docs/findings/fem-structural-audit.md` remains an empty outline until the baseline block is written verbatim. | File created as an empty outline; verified placeholders only. | **satisfied** |
| 10 | The program owner authorizes the audit to begin. | Avidiyah's 2026-08-28 conditional direction is recorded in §4.3, then suspended by the later §3.4 pass-independence incident. Replacement-role approval and reauthorization are pending. | **pending reauthorization** |

**Readiness statement:** items 1-6 and 9 are satisfied; item 6 carries a
disclosed held-out incident rather than a clean pass. Items 7-8 are partial and
item 10 is pending reauthorization. The audit does **not** open. A replacement
pass-1 identity must be assigned and attest from a clean session; pass 2 must
supply its missing attestation and receive a preserve-or-restart decision; and
the program owner must reauthorize the resulting roster. No annotator may open
or resume a Fallen Empires row before all three actions are recorded.

## 2. Held-out exclusion registry bound to this audit, including a new incident

- Protocol §6.3 frozen pool: Oracle text present; `oracle_id` begins `f`;
  `first_is_fallback = 0`; `first_set` not `lea`/`leb`/`arn`. Identity count and
  non-disclosing digest bound via
  `docs/manifests/snapshot-scryfall-2026-08-25.json`
  (`377e12bdf80e0263c361f48ff2be241f600efd854b6eaa4f916d239a83067fc7`).
- Pre-existing cumulative incident registry (additive, inherited from The
  Dark): Combust, Malignus, Lava Burst, Wild Slash.
- Fallen Empires's own `oracle_id`-prefix-`f` non-fallback cards remain
  held-out and are excluded from the development export by
  `--exclude-heldout`; 9 cards were excluded (102 → 93), reported as an
  aggregate by the Phase 3 verifier.

### 2.1 New incident: `audit novelty` held-out exposure (2026-08-28)

During Phase 3 preparation, the technical validator ran:

```text
mtg-discover.exe audit novelty fem --earlier lea --earlier leb --earlier arn --earlier atq --earlier leg --earlier drk
```

`audit novelty`'s implementation (`src/audit.rs`, `command_audit_novelty`)
calls `load_audit_cards(&conn, &set, false)` with `exclude_heldout` hardcoded
to `false` — the command has no `--exclude-heldout` flag. The full JSON
result, including the `novel_template_records` array (normalized templates
plus up to five representative card names per template), was displayed in the
session transcript before this was noticed. That array is drawn from the
complete 102-card `fem` first-printing population, not the 93-card
held-out-safe development partition.

**Exposure scope, determined by cross-referencing the displayed names against
the already-generated held-out-safe 93-card export** (a comparison performed
on data already displayed, revealing no new information): 7 of the displayed
representative card names are absent from the held-out-safe export and are
therefore held-out identities. Their names and normalized templates were
exposed:

- Farrel's Mantle
- Fungal Bloom
- Orgg
- Spore Flower
- Svyelunite Priest
- Thelon's Chant
- Vodalian War Machine

The remaining 2 held-out cards among the 9 excluded either produced no novel
template record or were not among the displayed representative names for a
shared template, and were not confirmed exposed by name.

**Disposition, authorized by program owner Avidiyah in-session (2026-08-28,
"Log as held-out incident, continue prep"):**

1. The seven identities above are added to the cumulative held-out
   incident-exclusion registry, effective immediately, and must be excluded
   from all future held-out sampling for every set, the same way Combust,
   Malignus, Lava Burst, and Wild Slash already are.
2. The technical validator/preparer for this session (Claude Code, Sonnet 5)
   is disqualified from serving as pass-1 annotator, pass-2 annotator, or
   adjudicator for the Fallen Empires audit, having inspected eligible
   (including held-out) Fallen Empires text before the freeze in a
   preparation context.
3. No figure in the preregistration or this record is sourced from the
   `audit novelty` command's output. The held-out-safe novelty figures (§3 of
   the preregistration; total printed units 176, novel units 125 / 71.02%,
   distinct templates 122, novel templates 100 / 81.97%) were recomputed by a
   separate local template-overlap comparison over the retained held-out-safe
   `docs/audits/fem/units-export.tsv` against the six earlier held-out-safe
   exports, producing aggregate counts only.
4. `mtg-discover audit novelty` should not be run again for any set without
   first excluding held-out identities from its input at the call site (a
   tooling gap, not a data problem); this is recorded here as an open
   tooling note rather than fixed by this record, since implementing a fix
   is outside preregistration/entry-record scope.
5. No further eligible Fallen Empires row, held-out or development, was
   displayed after this incident was caught. Aggregate-only commands
   (`audit summary --exclude-heldout`, `audit signals --exclude-heldout`,
   `verify_export_safety.py`, `export_units.py --exclude-heldout` redirected
   to file) were used for every subsequent figure.

This incident is bound to
`docs/manifests/experiment-fem-freeze-2026-08-28.json`
(`commands[].output_policy` for the `audit novelty` entry) and to this
section. Any later accidental exposure is logged here additively, exactly as
this one was.

## 3. Role assignments and attestations

The original three roles were assigned below. Original pass 1 is now
disqualified by §3.4 and must be replaced; it may not serve as pass 2 or
adjudicator either. Pass 2's assignment is bound to the annotator id already
present in its file, but its independence attestation remains missing. The
adjudicator assignment remains row-unexposed. The research lead and program
owner are Avidiyah.

### 3.1 Recorded pass-1 annotator independence attestation

```text
FALLEN EMPIRES ANNOTATOR INDEPENDENCE ATTESTATION
Pass:                      1
Annotator identity:        codex-pass1-2026-08-28
Date:                      2026-08-28
Assignment:                in-session user direction, 2026-08-28;
                           research-lead/program-owner countersignature pending
Protocol:                  structural-investigation-protocol.md v1.0, sha256 1bc05d357b24006a2eecc692f9bed5b86d1d828f116c2d741fb75662df4913bf
Annotation guide binding:  frozen v1.0, sha256 d31dee0a3b06494bd7ba0238be65b330e2366edb1b8bcf4e5e6a6f865de5d84b
Preregistration:           fem-structural-audit-preregistration.md, sha256 41e298939dcf6f8ce8b5cda778de8d1467cfa820d0c1bdc5655124cd9e69d982
Development export:         docs/audits/fem/units-export.tsv, sha256 095a25a7a0729bca12d515b2ce0a7395c0484d1fc335d11a913dec8c6c3b0d74
Frozen pass-1 input:        docs/audits/fem/units-annotated-pass1.tsv, blank
                           sha256 691f8b8c4f54fcbcafacc716ece331ff348e32d8ff0c5f40d5bfe79f1fe92c25

I attest that, as of the date above:
1. I have not queried, read, quoted, segmented, or annotated any eligible
   Fallen Empires (`fem`) card's Oracle text or development-export row before
   the measurement freeze or this attestation. I have seen only control
   documents, aggregate hashes and counts, the TSV header, and the seven
   held-out incident-registry names already disclosed in §2; I have not seen
   Oracle text or normalized templates for those identities.
2. I have not read, and will not read before both passes are sealed, the other
   annotator's annotations, notes, row-level hypotheses, or candidate list.
3. I will annotate only the frozen held-out-safe pass-1 file identified above,
   will not run unfiltered card or corpus searches over the set, and will stop
   and report immediately if any held-out identity appears in my view.
4. I will use `unsure` / `unsupported` / `ambiguous` / `adjudicate` as defined
   in the frozen protocol, guide, and preregistration rather than guess, and I
   will not propose, discuss, or implement classifier changes during the pass.
5. I will not open any eligible Fallen Empires row until every §1 checklist
   item is satisfied and the program-owner authorization in §4.3 is recorded.
6. My sealed pass will be delivered with a content hash and timestamp, and I
   will not modify it after sealing.
Exceptions or prior exposure to declare: none beyond the control-document and
aggregate-only access disclosed in item 1.

Signed: codex-pass1-2026-08-28
Received by research lead / program owner: Avidiyah, 2026-08-28; countersigned
under the owner direction recorded in §4.3
```

### 3.2 Pass-2 assignment and pre-authorization file-state deviation

```text
FALLEN EMPIRES PASS-2 ASSIGNMENT
Pass:                      2
Annotator identity:        copilot-pass2-2026-08-28
Assigned by:               research lead / program owner Avidiyah, 2026-08-28
Frozen pass-2 input:       docs/audits/fem/units-annotated-pass2.tsv, blank
                           sha256 691f8b8c4f54fcbcafacc716ece331ff348e32d8ff0c5f40d5bfe79f1fe92c25
Current file state:        sha256 91abeb36c9d08e8f5d41bcfa1e39d09253c56dbaf7c9b4f04a757430235cb74a
Control-check timestamp:   2026-08-28T14:52:08-05:00
Independence attestation:  MISSING; must be supplied by the assigned identity
Status:                    assigned, not authorized to resume or seal
```

Before this entry gate was opened, the pass-2 path ceased to match the frozen
blank hash. A governance-only check displayed no Oracle text, normalized text,
judgement values, citations, tags, or notes; it established only the row count,
the unique `annotator` metadata value `copilot-pass2-2026-08-28`, the absence
of a recorded attestation, and the content hash above. Pass 1 has not inspected
pass 2's judgements.

This timing is a governance deviation from preregistration §12 and is not
rewritten as prior compliance. The file is preserved provisionally and must
not be overwritten, resumed, treated as sealed, compared with pass 1, or
opened by the adjudicator. The assigned pass-2 identity must first provide a
signed independence attestation binding the frozen protocol, guide,
preregistration, export, and blank-input hash; it must also disclose when and
under what authorization the file changed. The research lead/program owner
must then decide in this record whether to preserve or restart that pass. Only
that reconciliation can satisfy checklist item 8 and activate §4.3 for row
access.

### 3.3 Adjudicator assignment

```text
FALLEN EMPIRES ADJUDICATOR ASSIGNMENT
Adjudicator identity:      fresh-fem-adjudicator-2026-08-28
Assigned by:               research lead / program owner Avidiyah, 2026-08-28
Independence:              distinct from both annotators and from the §2.1
                           disqualified preparer; personal attestation is due
                           before adjudication opens
Inputs permitted:          none until both passes are validly sealed and their
                           hashes/timestamps are recorded
Obligations:               follow preregistration §7.4 and the frozen protocol;
                           inspect no row or pass before both seals; make no
                           classifier proposal during adjudication
Exceptions:                none recorded

Assignment recorded by: codex-pass1-2026-08-28
Program owner: Avidiyah
```

### 3.4 Pass-1 row-exposure and independence incident

Recorded at `2026-08-28T15:09:40-05:00`: after staging the audit-preparation
package for the program owner's requested commit, original pass 1
`codex-pass1-2026-08-28` ran `git diff --cached --check`. Because the frozen
TSV rows intentionally end in empty tab-separated annotation columns, Git
reported them as trailing whitespace and printed row-bearing lines from both
annotation files. The output exposed eligible Fallen Empires Oracle text and
pass-2 judgement fields to the pass-1 identity before either pass was validly
sealed.

This is a protocol §7.2 / §11.2 independence failure, not a held-out-pool
exposure: the staged files are the held-out-safe development partition. It is
not rewritten as compliant. The original pass-1 identity stopped immediately,
did not open the TSVs directly, did not annotate any row, and did not modify
either annotation artifact. Aggregate hash verification after the incident
still showed pass 1 at its frozen blank SHA-256
`691f8b8c4f54fcbcafacc716ece331ff348e32d8ff0c5f40d5bfe79f1fe92c25`
and pass 2 at
`91abeb36c9d08e8f5d41bcfa1e39d09253c56dbaf7c9b4f04a757430235cb74a`.

Disposition pending program-owner confirmation:

1. `codex-pass1-2026-08-28` is disqualified from every Fallen Empires
   annotator and adjudicator role.
2. The blank pass-1 file is preserved unchanged for rebinding to a new,
   independently attested pass-1 identity only after owner assignment.
3. Pass 2 remains provisionally preserved under §3.2 and must not resume or
   seal before its own attestation and preserve-or-restart decision.
4. The adjudicator remains barred from all row/pass access.
5. The §4.3 conditional authorization is suspended. Avidiyah must approve the
   replacement pass-1 identity and reauthorize the completed roster.

## 4. Program-owner authorization (to open)

### 4.1 Reproduction (governance checks for this record)

```powershell
git rev-parse HEAD ; git status --short
cargo build --release
cargo test
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
python -m unittest discover scripts/python -p "test_*.py"
python scripts/python/verify_export_safety.py fem --mtg .\target\release\mtg-discover.exe --runs 2
python scripts/python/verify_manifests.py docs/manifests/experiment-fem-freeze-2026-08-28.json docs/manifests/snapshot-scryfall-2026-08-25.json
```

### 4.2 What remains before opening

- Avidiyah must assign a replacement pass-1 annotator, distinct from both
  existing annotator identities, the adjudicator, and the §2.1 preparer. The
  replacement must sign a fresh independence attestation before row access.
- The assigned pass-2 identity must provide the missing signed independence
  attestation and account for the pre-authorization file-state deviation in
  §3.2 without displaying row content to the replacement pass 1.
- The research lead/program owner must record a preserve-or-restart decision
  for pass 2 after receiving that attestation, then mark checklist item 8
  satisfied. If restart is chosen, preserve the current file as incident
  evidence and create a newly hashed blank input rather than overwriting it.
- Avidiyah must reauthorize the replacement roster after items 7-8 are
  satisfied. The superseded §4.3 authorization no longer permits row access.

### 4.3 Program-owner sign-off

```text
FALLEN EMPIRES CONDITIONAL OPENING AUTHORIZATION
Program owner:             Avidiyah
Date:                      2026-08-28
Direction received:        "Provide those assignment and authorize, create a
                           handoff for the next session"
Assigned roles:            pass 1 codex-pass1-2026-08-28;
                           pass 2 copilot-pass2-2026-08-28;
                           adjudicator fresh-fem-adjudicator-2026-08-28
Pass-1 countersignature:   accepted
§2.1 incident disposition: reviewed and preserved
Authorization:             approved, effective for row access only after every
                           other §1 checklist item is satisfied
Current blocking item:     item 8, missing pass-2 independence attestation and
                           preserve-or-restart reconciliation (§3.2)
```

This records the program owner's assignment and authorization without
backdating or concealing the pass-2 file-state deviation. It was subsequently
**suspended by the §3.4 pass-independence incident**. The audit remains closed,
and neither annotation pass nor the adjudicator may open or resume Fallen
Empires rows until a replacement roster is approved and reauthorized.
