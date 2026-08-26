# Gate 0 evidence package — Baseline stabilization and research governance

Gate: Phase 0 → Phase 1 (`docs/roadmap.md` §5)
Prepared: 2026-08-26 by the research lead (Fable), single reviewer
Protocol: `docs/protocol/structural-investigation-protocol.md` v1.0

## 1. Decision

**Gate 0: PASS, with two recorded caveats.** Every exit criterion in
`docs/roadmap.md` §5 is met by evidence reproduced from a clean worktree at
commit `02793f3` (section 3). The caveats (section 6) are reproducibility
debts that do not touch the criteria as written, and each is paired with the
exact evidence that would revoke the pass. Phase 1 may open; its first
investigation is prepared in section 8 and has **not** been started.

This pass is not "the code builds and Alpha looks reasonable". It rests on:
a rebuilt binary and passing tests in a fresh worktree; corpus and Alpha
measurements re-run and matching the documented values digit for digit; a
unit-level annotation of all 412 Alpha units with CR citations
(`docs/audits/lea/units-annotated.tsv`); a per-change map from the seven
accepted Alpha segmenter changes to named regression tests; and a review that
narrowed three Alpha claims (section 4).

## 2. Frozen inputs

| Item | Value | Command |
|---|---|---|
| Commit | `02793f3` (Research Pass 2, Alpha) | `git rev-parse HEAD` |
| Build | `cargo build --release` succeeds; `cargo test`: 22 passed, 0 failed | — |
| Bulk snapshot | `oracle-cards.jsonl.gz`, `rulings.jsonl.gz`, `default-cards.jsonl.gz` dated 2026-08-26 01:38 local (Scryfall drop of 2026-08-25); `cards.sqlite` built 2026-08-26 01:40 | `ls -l *.jsonl.gz cards.sqlite` |
| Corpus | 38,626 cards; 37,916 with text; 710 without; 3,212 multi-face; 553 first-printing sets; 78,949 rulings | `mtg-discover info` |
| Rules | 3,455 numbered entries, 752 glossary entries, effective 2026-08-07 | `mtg-discover info` |
| Alpha | `lea`, 1993-08-05, 290 cards, 275 with text, 0 fallback | `mtg-discover sets --until 1993-08-05` |

## 3. Exit criteria

| # | Criterion (roadmap §5) | Evidence | Status |
|---|---|---|---|
| 0.1 | A clean checkout can reproduce corpus metadata and template totals | Fresh worktree at `02793f3`, `cargo build --release`; `info` and `templates` (corpus and `--set lea`) reproduce every value in `docs/current-state.md`: 70,799 printed + 970 rules-supplied units, 36,944 templates, top-10/100/1,000/5,000 coverage 14.32 / 27.14 / 42.37 / 54.88 %; Alpha 398 + 14 units, 291 templates, 265 singletons, kinds and roles identical | **Met** for the recorded snapshot (caveat A) |
| 0.2 | Documentation and CLI output describe the same segmentation model | `templates.normalization` block and `segment` output compared clause-by-clause with `docs/current-state.md`, `docs/README.md`, `.github/copilot-instructions.md`. One mismatch found and corrected in the docs: the CLI collapses `{T}` and `{Q}` to `{M}` too, which the docs described as "mana symbols". Two naming nits recorded (T9; README script paths fixed) | **Met** |
| 0.3 | Every accepted Alpha-derived segmenter change has a regression test | Section 5 maps the seven changes to the 22 tests by name | **Met** |
| 0.4 | Generated data, source, findings, scratch analyses clearly separated | `.gitignore` excludes `cards.sqlite*`, bulk files, `target/`; source in `src/` and `scripts/python/`; findings in `docs/findings/`; audits in `docs/audits/`; protocol and gates in `docs/protocol/`, `docs/gates/`. Three Alpha measurements (B1, B2, V3) came from scratch scripts that were *not* preserved — recorded as caveat B and downgraded in the Alpha document | **Met** for layout; caveat B |
| 0.5 | No IR, executor, simulation, or AI implementation has entered scope | `src/main.rs` contains search, rules retrieval, segmentation, template counting only; no state model, executor, or agent code; the protocol and this package add none | **Met** |

### Required Phase 0 deliverables

| Deliverable | Where | Status |
|---|---|---|
| Updated current-state document matching live behavior | `docs/current-state.md` (refreshed 2026-08-26) | Done |
| Reproducible corpus and Alpha measurements | commands in §9; `docs/audits/lea/units-export.tsv` + `metrics.json` | Done for CLI-derived measurements; B1/B2/V3 not reproducible (caveat B) |
| Passing automated tests for current segmentation behavior | `cargo test` 22/22 | Done |
| Written baseline schema for experiment results | protocol §4 (export, annotation, measurement definitions) | Done |
| List of known structural failure classes | §7 below | Done |
| Experiment-report, decision-record, gate-report templates | protocol §7 | Done |
| Deferred-work register | `docs/roadmap.md` §22 | Done |

## 4. Alpha findings: disposition after review

Legend: **accepted** = follows from recorded, reproducible evidence;
**bounded** = true for the stated denominator only; **unresolved** = evidence
recorded but not settled; **unsupported** = claim not backed by preserved
evidence and downgraded.

| Claim | Disposition | Notes / narrowed wording |
|---|---|---|
| V1 — 14 Alpha cards have only rules-supplied abilities (basic lands, duals; CR 305.6) | **Accepted** | Reproduced: 14 `rules_supplied` units, all cited `305.6` |
| V2 — reminder text broke keyword classification on 29/86 keyword-bearing lines | **Accepted** (historical segmenter); closed by change 1 | 90 keyword units now; annotation finds no keyword mislabelled |
| V3 — Oracle text identical across all printings (H4 refuted) | **Bounded → unsupported as reproducible**; retained as a bounded observation | Measured over 116,843 printings by a scratch script that was not preserved; cannot be re-run from the repository. Re-measure under T1/T2 before citing corpus-wide |
| V4 — "a line is not an ability" | **Accepted with narrower wording** | Keyword lists, modal spells, Siren's Call, Berserk, Animate Dead stand. Two items are **withdrawn from V4**: (a) *`Activate only …` restrictions are not separate abilities* — CR 602.1b makes activation instructions part of the activated ability; they are an unrepresented *slot*, not a missed boundary; (b) Animate Dead's last sentence is a *delayed triggered ability* created on resolution (ruling; CR 603.7a/e), not a "granted leaves-the-battlefield trigger" |
| V5 — CR defines a sub-line grammar (602.1, 603.1) | **Accepted** | T8 asks for the slot decomposition as extra fields |
| V6 — structural classes the classifier could not express | **Accepted**, one row narrowed | Gaea's Liege is **not** an established CDA: CR 604.3a(5) excludes abilities that set values only under a condition; its status is `ambiguous`. CDA count in Alpha is "3 detected + 1 ambiguous", not "3 of 4" |
| V7 — self-reference forms; `named X` counterexample | **Accepted** | Plague Rats reproduces |
| B1 — typed-slot ablation moves Alpha singleton share 68 % → 56 % | **Bounded, not reproducible** | Measured on the historical segmenter (388 units) by an unpreserved script; numbers are stale against the current 398-unit baseline. Keep as a hypothesis-generating observation; re-run under T2 |
| B2 — 47 % of Alpha templates never recur corpus-wide; 65 % of Alpha units recur | **Bounded, not reproducible** | Same; the CLI caps `templates` at 5,000 rows so corpus-wide recurrence cannot be rebuilt today (T3/T4) |
| N1–N3 hypotheses | **Unresolved** (correctly labelled) | N3's ≥ 95 % claim needs T8 |
| "Alpha's singleton share barely moves (66.6 %)" | **Accepted** | 265/291 templates are singletons at 398 units (reproduced) |
| "Multi-sentence printed units fall from 59 to 56" | **Bounded** | The annotation's automatic tag counts 59 at the *current* baseline (≥ 2 sentence terminators over 398 printed units); the audit's 56 used an unstated method. Use 59/398 with the stated method |

Alpha is an exhaustive **development and regression** set. None of the above
is evidence about corpus-wide behaviour; the corpus numbers in
`docs/current-state.md` measure the procedure, not correctness.

### Structural coverage is not semantic parsing

The segmenter now labels every Alpha unit; 390/395 judged printed units have a
correct boundary and 379/383 a correct kind label. That is structural
coverage of *surface form*. It says nothing about what any unit means: the
label `spell_or_static_text` covers Balance, Word of Command and Fog alike;
`Flying` and `Trample` are equal only in being keyword names; and 52 units
share cost templates that conflate the tap symbol with mana (§7, C1).

## 5. Accepted changes → regression tests

| Change (Alpha proposal) | Tests in `src/main.rs` |
|---|---|
| 1. Classify on reminder-stripped text | `reminder_bearing_keyword_is_classified_on_stripped_text` |
| 2. Split keyword lists | `keyword_lists_split_on_comma_and_semicolon`, `ordinary_comma_text_is_not_split` |
| 3. Rules-supplied units | `reminder_only_lands_are_rules_supplied_units` |
| 4. New kinds + quoted abilities before the colon test | `structural_classes_are_detected_from_normalized_text`, `quoted_granted_ability_is_a_child_and_does_not_make_parent_activated`, `short_quoted_words_are_not_abilities` |
| 5. `this <type>` → `~`, `named X` preserved | `this_object_self_references_normalize_to_tilde`, `named_predicate_survives_self_reference_normalization` |
| 6. Modal header + modes as one unit | `modal_spell_is_one_ability_with_mode_children`, `triggered_modal_header_keeps_its_kind` |
| 7. Delayed trigger attaches to the creating text | `delayed_trigger_line_attaches_to_originating_spell_text`, `inline_delayed_trigger_is_split_from_spell_effect`, `recurring_upkeep_trigger_is_not_a_delayed_trigger`, `delayed_trigger_phrase_inside_quotes_is_not_split` |
| Shared: indexing, faces, normalization, rules parser | `indices_are_preorder_across_nested_units`, `segmentation_excludes_face_separator_and_tracks_faces`, `normalization_handles_nested_reminder_text_and_self_references`, `rules_parser_separates_numbered_rules_and_glossary`, `rule_hierarchy_supports_numeric_and_lettered_subrules`, `set_predicate_always_references_its_parameter`, `like_metacharacters_are_escaped` |

The tests are synthetic-text tests and do not read `cards.sqlite`; they pass
in a worktree without the database. The defects found by the unit-level
audit (§7) are *new* findings, not accepted changes, and have no tests yet.

## 6. Caveats and the evidence that would revoke the pass

**Caveat A — snapshot identity is prose.** `cards.sqlite` records nothing
about which Scryfall drop it was built from; the identifier is the bulk
files' dates. A fresh `fetch` on another day produces different totals.
*Revokes 0.1 if:* the bulk files are lost or regenerated before T1 lands, so
that the documented totals cannot be reproduced at all.

**Caveat B — three Alpha measurements are not reproducible.** V3, B1 and B2
were produced by session scratch scripts that were not committed. They are
downgraded above and may not be cited as verified. *Revokes 0.4 if:* any
future finding relies on them without re-measurement, or if scratch analyses
are again left unpreserved (the protocol now requires them under
`scripts/python/` or `docs/audits/`).

Neither caveat changes a criterion's status as written in the roadmap.

## 7. Known structural failure classes (from the Alpha unit audit)

Single-annotator, unadjudicated (`annotator = fable-pass1`), 412 units,
`docs/audits/lea/metrics.json`:

| Measurement | Value |
|---|---|
| Boundary precision | 390 / 395 judged printed units (0.987); 3 `unsure` |
| Missed boundaries | 5 (all nested delayed triggers) → recall 390 / 395 (0.987) |
| Kind accuracy | 379 / 383 (0.990) on correctly bounded, applicable units; 7 n/a or unsure |
| Role accuracy | 388 / 388; 2 `unsure` (Animate Dead's lost/gained quoted abilities) |
| Source accuracy | 412 / 412 |
| Dispositions | accept 398 · defect 8 · adjudicate 4 · ambiguous 1 · unsupported 1 |
| Context needed | none 293 · CR 99 (91 keywords, delayed triggers, prevention) · type line 19 (14 rules-supplied, 2 CDA, 2 instant/sorcery `instead`, Gaea's Liege) · game state 1 · card-specific 0 |

Failure classes (each is a generic surface pattern, none card-specific):

| Id | Class | Alpha rows | CR | Status |
|---|---|---|---|---|
| F1 | Delayed trigger with effect *before* the `at the beginning of the next …` phrase | Dragon Whelp #1, Stone Giant #0, Nettling Imp #0 | 603.7a | defect (missed boundary) |
| F2 | Delayed trigger created by a `When …` sentence inside a triggered ability | Animate Dead #1 | 603.7a/e | defect |
| F3 | Recurring delayed trigger with a stated duration (`each of your upkeeps for the rest of the game`) | Cyclopean Tomb #1 | 603.7b | defect |
| F4 | `at end of combat` inside a trigger's effect | Cockatrice #1, Thicket Basilisk #0 | 603.7 | adjudicate |
| F5 | Lexical `instead` on an instant/sorcery → `replacement_effect` | Disintegrate #0, Camouflage #1 | 113.3a, 614.1a | defect; needs type-line context |
| F6 | Quoted keyword with trailing period → residual | Animate Dead #3 | 702.5a | defect |
| F7 | Prevention static (615) has no kind | Rock Hydra #1 | 615.1a | unsupported (taxonomy gap) |
| F8 | Conditional P/T-defining statements | Gaea's Liege #0 | 604.3a(5) | ambiguous |
| F9 | Payment restriction `Spend only … on X` | Drain Life #0 | citation unresolved | adjudicate |
| F10 | Activation instructions are unrepresented slots (not boundaries) | 9 units | 602.1b | accepted structure; T8 |
| F11 | Role vocabulary lacks "referenced (lost) ability" | Animate Dead #2 | — | adjudicate |

Normalization (flags are regex-suspected unless marked verified):

| Id | Class | Alpha units | Status |
|---|---|---|---|
| C1 | `{T}`/`{Q}` collapse to `{M}`: `{T}: Add {G}.` ≡ `{G}: Add {C}.` | 52 | **verified** by `segment --text` probes; proposal T10 |
| C2 | Magnitude loss `+N/+N` (Holy vs. Unholy Strength) | — | expected by design |
| P1 | Colour-word fragmentation (laces, Wards, Circles, charms, Blasts) | 37 | suspected (B1 showed the merge on the historical baseline) |
| P2 | Basic-land-type fragmentation (`Destroy all Plains/Islands`, landwalks) | 27 | suspected |
| P3 | `Enchant <type>` fragmentation | 41 | suspected |

## 8. Next investigation: Arabian Nights (prepared, not started)

Baseline recorded without reading any card (`templates --set arn --limit 1`):

| Set | Cards | With text | Printed units | Rules-supplied | Templates | Kinds | Roles |
|---|---|---|---|---|---|---|---|
| `arn` (1993-12-17) | 77 | 77 | 109 | 0 | 92 | activated 33 · triggered 31 · static 23 · keyword 16 · replacement 4 · additional cost 1 · ante 1 | ability 107 · mode 2 |
| `leb` (1993-10-04) | 2 | 2 | 1 | 1 | 1 | activated 1 | ability 1 |

- **Development corpus:** the 77 `arn` cards, all units (exhaustive per
  protocol S4), plus the 2 `leb` cards as an appendix. `pcel` (memorabilia,
  8 fallback cards) is excluded by S4.
- **Regression corpus:** `docs/audits/lea/units-annotated.tsv` (412 units);
  any accepted change is re-exported and checked with
  `audit_metrics.py --export`.
- **Held-out:** protocol §6.3 pool (2,096 cards, `oracle_id` prefix `f`,
  excluding `lea`/`leb`/`arn`), frozen 2026-08-26, not to be inspected.
- **Hypotheses to carry in:** N1 (novelty is informative from `arn` onward;
  compute unit and template novelty against `lea` + `leb` exports), N2
  (no non-keyword short sentence is mislabelled keyword; `arn` has 16
  keyword units to check), F1–F4 (delayed-trigger forms: search `arn` for
  `at the beginning of the next` and `at end of combat` before proposing a
  generic split rule), C1 (count `{T}` units).
- **Entry condition:** this gate's decision is countersigned by the program
  owner; Codex's T2 export, if available as a complete commit, is verified
  against `export_units.py` on `lea` (byte-identical unit texts) before use.

## 9. Reproduction

```powershell
git rev-parse HEAD                      # 02793f3 or a descendant that changes no src/ file
cargo build --release ; cargo test      # 22 passed
$mtg = ".\target\release\mtg-discover.exe"
& $mtg info
& $mtg sets --until 1994-01-01
& $mtg templates --limit 5000
& $mtg templates --set lea --limit 5000
& $mtg templates --set arn --limit 1
& $mtg segment --text "{T}: Add {G}." --name Probe ; & $mtg segment --text "{G}: Add {C}." --name Probe
python scripts/python/export_units.py lea > docs/audits/lea/units-export.tsv
python scripts/python/audit_metrics.py docs/audits/lea/units-annotated.tsv --export docs/audits/lea/units-export.tsv
```

## 10. Decision record

**Decision:** Gate 0 passes (2026-08-26, research lead; awaiting program-owner
countersignature in this file).
**Evidence:** sections 2–5. **Options considered:** (a) pass; (b) pending
until T1 pins the snapshot and B1/B2/V3 are re-measured; (c) fail. (b) was
rejected because the roadmap's criteria do not require snapshot pinning or
those three measurements, and holding Phase 1 for them would block the
research the tooling exists to serve; the debts are registered
(`docs/roadmap.md` §22) with owners. (c) had no supporting evidence.
**Reversal:** either caveat's revoking condition in §6.
**Affected documents:** `docs/current-state.md`, `docs/roadmap.md`,
`docs/findings/lea-segmentation-audit.md`, this file, the protocol.
