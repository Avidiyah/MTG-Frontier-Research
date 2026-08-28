# The Dark (`drk`) structural audit

- Status: **adjudicated and closed 2026-08-28 (adjudication opened after pass 2
  sealed at 2026-08-27T23:45:35-05:00); ready to freeze the next development
  set**
- Preregistration: `docs/findings/drk-structural-audit-preregistration.md`
- Entry checklist record: `docs/gates/dark-entry-record.md`
- Immediate precedent: `docs/findings/leg-structural-audit.md` (procedure only;
  no Legends measurement or set-specific conclusion is carried into The Dark)

The preregistration §12 entry checklist passed under the owner-authorized
assignment-timing reconciliation recorded in the entry record §3.4. Both
independent passes sealed before the adjudicator opened any Dark row, either
pass, or comparison output. The frozen segmenter was measured first; P-LEG-1..3
were not implemented for this audit.

## Frozen inputs and audit status

- Measurement freeze commit:
  `70fa956515123b80d33ab08a13e938d54c6b66f8`
- Governance commit after freeze (preregistration, outline, entry record):
  `a384dc2daecb27d8c22c945ee32d678c6ea9e500`
- Protocol v1.0 SHA-256:
  `1bc05d357b24006a2eecc692f9bed5b86d1d828f116c2d741fb75662df4913bf`
- Annotation guide, frozen Legends v1.0 (reused unchanged) SHA-256:
  `d31dee0a3b06494bd7ba0238be65b330e2366edb1b8bcf4e5e6a6f865de5d84b`
- Preregistration SHA-256 (populated): 
  `eaa6bda819b4623aa45b8acdc3d1b3f23c3bdf6a856dae87058ac58bbd8d06a2`
  (pre-population control text `b1c31c3b4dc1a0b4774bb5ed64c0d8549970d7f9768d1813794b48ad3338c2ba`)
- Retained export: `docs/audits/drk/units-export.tsv`, SHA-256
  `4460c2de445161e8e67ac3bc88c668e23ca6f2645ebaf0a483ddd455de4e0a16`
- Experiment manifest:
  `docs/manifests/experiment-dark-freeze-2026-08-27.json`, SHA-256
  `38d86700431ea64e9d3e96518124eef6151e82f25597134d24109e71ec2a20a9`
- Pass 1: `claude-fable-5-pass1-2026-08-27`, sealed in commit `a7d7567`
  (2026-08-27 22:29:38 -05:00)
  - SHA-256:
    `06bc1c29ad33f10e9ea989d27eb0a69248190482b163ae005dabf1707cbcadb9`
- Pass 2: `gpt-5.3-codex-pass2-2026-08-27`, sealed 2026-08-27T23:45:35-05:00
  (commit `61a7530`)
  - SHA-256:
    `5b555148ce2072cf4a460e33c403a7819e8738096806f6dee679b0ed28643621`
- Adjudicator: `fresh-dark-adjudicator-2026-08-27` (third identity; verified
  every hash above against the sealed files before opening any row)
- Final annotation: `docs/audits/drk/units-annotated.tsv`, SHA-256
  `aed8ab6309f7ad679c1a5e455c1d9a2d6567c7ad83121175c334480b80d25730`
  (163 rows, 163 unique keys, drift 0 against the retained export)
- Final metrics: `docs/audits/drk/metrics.json`, SHA-256
  `8c02d616c63f225f2695d29736a48ad1e85f59c31b7278d9c6f1dae604dade65`

## Scope and exclusions

The development partition is the 113 Dark first-printing cards remaining after
protocol §6.3 exclusion at the database boundary; 110 have Oracle text and
yield 163 frozen units. The aggregate-only verifier selected 119 cards before
exclusion, excluded 6 held-out identities, and found zero held-out records in
the export. Both passes covered all 163 units. No held-out identity appeared in
any auditor-visible output during annotation or adjudication. Held-out
evaluation, implementation, Gate 1 review, semantic IR, and engine work remain
out of scope.

## Preregistered hypotheses

The hypotheses remain exactly as frozen in
`docs/findings/drk-structural-audit-preregistration.md` §5 (H1–H11). This
document does not restate or modify them; outcomes are in "Verified findings"
and "Bounded observations".

## Pre-audit baseline

Recorded at the freeze, before either pass opened a row (preregistration §3):

- cards / cards with text: 113 / 110;
- printed / rules-supplied units: 163 / 0;
- top-level / child units: 161 / 2;
- face histogram: face 0 = 163;
- non-null / null `prefix`: 0 / 163;
- distinct / singleton templates: 148 / 138;
- multi-sentence units: 25 (corrected protocol instrument, emitted directly by
  the frozen binary);
- kinds: activated 59, cast restriction 1, CDA 2, keyword 24, prevention 1,
  replacement 5, static/spell 37, triggered 34;
- roles: ability 161, delayed trigger 2;
- source: printed 163.

Both sealed passes tagged the same 25 rows `multi_sentence`, equal to the
binary's count; the Legends instrument discrepancy did not recur.

## Adjudication record

- Both sealed files contain all 163 frozen keys with no duplicates, missing
  keys, or structural drift; the agreement report is valid.
- Preregistered exact-row agreement is **141 / 163 (0.8650)**, below H10's
  0.95 threshold. **H10 is falsified.**
- Exact-card agreement is **89 / 110 (0.8091)**.
- The adjudicator reviewed the union of all disagreements, all non-`accept`
  rows in either pass, and all alleged card-specific dependencies: **23
  distinct rows** (22 disagreements plus the consensus defect Venom #1). No
  row in either pass used `context = card_specific`.
- **18 rows differ only on `context`** (pass 1 `none`, pass 2 `type_line`),
  all top-level spell text on instant or sorcery faces without `instead`,
  `skip`, `enters`, `prevent`, or characteristic-defining wording. Adjudicated
  `none`: guide C6 records the strongest context *required*, and the §6.12
  step-10 type-line gate changes the kind only when the wording would
  otherwise match steps 11–13; plain instruction text reaches
  `spell_or_static_text` on any face, so the type line was consulted, not
  required (§4.7). Both passes independently recorded `type_line` for Blood
  of the Martyr's `instead`, which is the shape the gate actually decides.
- **Runesword #0 / #1** (pass 1 `accept` / `adjudicate`; pass 2 `unsupported`
  `gap:span:interleaved_child` on both): adjudicated **`defect`** on both
  rows. The paragraph's second sentence is a created delayed trigger (CR
  603.7a/603.7e, scoped `this turn`); its third and fourth sentences are a
  continuous effect (611.2a) and a replacement effect (614.1a) created by the
  activated ability's own resolution and are not conditioned on the trigger
  event, so they belong to the parent. The export gave them to the child. The
  correct shape — a one-sentence child under a non-contiguous parent — is the
  guide §5.8 / U3 accepted shape and is already emitted by the frozen export
  for `atq` Rocket Launcher #0/#1, so guide §10.1 step 3 (`unsupported`) does
  not apply; ownership of the trailing sentences is not ambiguous, which
  distinguishes the row from guide example 18. Parent #0 is `over` (a fragment
  of its ability); child #1 is `under`, `missed 0` (one reference unit plus
  parent text whose unit is emitted as #0). `under` is the nearest frozen
  boundary value for an over-inclusive child span and is recorded as a
  vocabulary observation, not a new value.
- **Angry Mob #1** (pass 1 `ambiguous`; pass 2 `defect`, CDA): adjudicated
  **`ambiguous`**, following frozen guide example 19 (`lea` Gaea's Liege #0),
  which calibrates the same two-clause mutually exhaustive P/T shape. Reading
  (b) — a CDA — has support the frozen guide does not treat as decisive (the
  printed `2+*` box and CR 208.2a; the two clauses jointly always set a value,
  so 604.3a(5) may not exclude it); reading (a) treats each clause as setting
  values only under its condition. No ruling exists. Neither reading is adopted
  and the row stays outside the kind-accuracy denominator.
- **Leviathan #1** (pass 1 `under`/1 `defect`; pass 2 `ok` `accept`):
  adjudicated **`accept`**. CR 113.2c delimits a card's abilities by paragraph
  break (keyword lines excepted); the single sentence is one static ability
  that generates both an enters-tapped replacement effect (614.1d) and a
  doesn't-untap continuous effect (604.1). No CR rule or ruling splits one
  sentence into two static abilities. The pass-1 reading is retained below as
  a bounded observation.
- **Venom #1**: consensus `defect` (in-sentence created end-of-combat delayed
  trigger; D15 / P-ATQ-1 slot; `missed 1`).
- No final row remains `adjudicate`. Both original pass IDs and the adjudicator
  ID are retained on all 163 rows; the 23 reviewed rows carry an `ADJ:`
  rationale. Supplemental fields (`cr_ref`, `structure_tags`, `norm_issue`,
  `note`) are pass-1 based, as in Legends, except on the four substantively
  adjudicated rows where citations and mandatory tags were corrected to match
  the adjudicated judgement (e.g. `delayed_trigger_when` on Runesword #1;
  `614.1d` and `113.2c` on Leviathan #1; `208.2a` on Angry Mob #1).

## Verified findings

### V1. The sentence-initial `When` delayed-child rule consumes trailing parent sentences

Runesword's created delayed trigger is emitted as a child, but the child span
also carries the two following sentences that are the activated ability's own
resolution effects. Parent #0 is therefore a fragment (`over`) and child #1 is
over-inclusive (`under`, `missed 0`). Two defect rows, one card. This is the
only boundary-precision failure in the partition (160 / 163, 0.9816).

The same rule produced a correct child on Whippoorwill, where the created
trigger is the paragraph's final sentence. The failure is specific to a
`When … this turn` child followed by further parent text; the correct output
shape (one-sentence child, non-contiguous parent) is already produced by the
frozen export for an inverted single-sentence child (`atq` Rocket Launcher).

### V2. One in-sentence created delayed trigger is missed

Venom #1 keeps `destroy the other creature at end of combat` inside the
triggered ability's sentence (D15 / P-ATQ-1 slot). One missed reference unit;
boundary recall 160 / 161 (0.9938). This reproduces the Legends-observed
Abomination class and is the measurement, not a new proposal.

### V3. Kind, role, and source labels are correct on every judged unit

Kind accuracy 159 / 159, role accuracy 160 / 160, source accuracy 163 / 163.
Within that:

- all 24 `keyword_ability` rows are genuine keywords (H3); the quoted `bands
  with other` recall class has zero Dark instances because the partition has
  no quoted text;
- the single `prevention_effect` row performs prevention, and all five
  declared prevention/prohibition surface candidates are correctly typed —
  four sit inside activated abilities, including Whippoorwill's `can't be
  prevented` prohibition (H4);
- all 20 top-level instant/sorcery-face units pass the type-line kind check
  (19 `spell_or_static_text`, 1 `cast_restriction`), including Blood of the
  Martyr's `instead` (H5);
- both `delayed_trigger` children have valid same-face parents and correct
  roles; no `granted` and no top-level spell-created delayed-trigger rows
  exist (H2);
- all five `replacement_effect` rows are correctly classified (H8's
  replacement denominator); no static-regeneration surface form occurs.

### V4. H10 is falsified by a single systematic convention divergence, not by structural disagreement

Independent exact-row agreement is 141 / 163 (0.8650). Eighteen of the 22
disagreements are `context` only (`none` vs `type_line`) on plain instant/
sorcery spell text, i.e. the two annotators applied guide C6 differently to
one recurring shape. Agreement over the seven non-context judgement fields is
159 / 163 (0.9755) — a bounded observation, not a substitute for the
preregistered measure. The four substantive disagreements are Runesword #0/#1,
Angry Mob #1, and Leviathan #1, each adjudicated above. This is a
guide-calibration finding for the next guide version (see "Proposed changes"),
not evidence about the frozen segmenter.

### V5. Novelty prediction passes

Unit novelty against the pooled `lea`+`leb`+`arn`+`atq`+`leg` exports (746
distinct earlier templates) is **127 / 163 (0.7791)**, strictly below 1.0, so
H9 passes. Template novelty is 126 / 148 (0.8514). Comparison point only: the
single-step change from Legends' unit novelty (0.7277) is upward; no
monotonic-novelty claim is made.

## Bounded observations

- H1 passes: neither nested delayed-trigger pair leaves a condition-only,
  cost-only, or quoted-fragment parent. Runesword shows the pass is narrow:
  its parent is a complete cost-plus-effect sentence but still a fragment of
  the ability because the child took the parent's later sentences.
- H6 (prefix) and H7 (quoted gained/lost abilities) have denominator zero in
  this partition and are not evaluated. H8's static-regeneration denominator is
  also zero; only its replacement-precision half is measured.
- H11 has zero in-scope candidates: both emitted delayed children carry the
  scoped marker `this turn`, and Venom's trigger is in-sentence (D19 class N7).
  The falsifier cannot be evaluated; no adjacency rule is supported or refuted.
- Structural exact-card correctness is 107 / 110 (0.9727); the three
  non-exact cards are Runesword, Venom, and Angry Mob (the ambiguous row).
- Multi-sentence frequency is 25 / 163 (0.1534), identical between the binary
  and both passes.
- Context distribution after adjudication: 130 `none`, 29 `cr`, 4
  `type_line`, 0 `game_state`, 0 `card_specific`.
- Leviathan #1 (`enters tapped and doesn't untap during your untap step`) is
  one static ability under CR 113.2c generating two continuous effects of
  different classes (614.1d replacement; 604.1). The pass-1 reading that treats
  the clauses as two abilities is recorded here; it has no CR delimiting rule
  behind it.
- Over-inclusive child spans have no exact value in the frozen boundary
  vocabulary; `under` with `missed 0` was used on Runesword #1 and is flagged
  for the next guide version.
- Supplemental-field agreement: `cr_ref` exact 119 / 163 (differences are
  mostly subrule granularity, e.g. `702.19` vs `702.19a`); `structure_tags`
  exact 161 / 163; `note` both blank 148, differing 3, pass-1-only 8,
  pass-2-only 4.
- Normalization flags are suspicions, not verified errors: tap-as-mana
  collision 41 / 163, land-type fragmentation 15 / 163, object-type
  fragmentation 9 / 163, color-word fragmentation 8 / 163.

## Unsupported and ambiguous cases

- **Unsupported: 0 / 163.** Pass 2's `gap:span:interleaved_child` on
  Runesword was not sustained because the correct spans are expressible in the
  frozen export shape (guide §5.8 / U3; `atq` Rocket Launcher precedent).
- **Ambiguous: 1 / 163.** Angry Mob #1: (a) a residual conditional
  characteristic-setting static (604.3a(5)); (b) a characteristic-defining
  ability (604.3a; 208.2a). Frozen guide example 19 calibrates this shape as
  ambiguous; no ruling decides it.
- **Unresolved adjudication: 0 / 163.**

## D14 independent-trigger observations

No D14 candidate exists in the development partition: no unit contains a later
unscoped sentence-initial trigger word. This zero does not support an adjacency
classifier and does not complete H11's required nearest-negative comparison.

## D19 effect-created delayed-trigger observations

- Runesword is **D19:P5** (non-contiguous created child): the created trigger
  is followed by instructions that belong to the parent, and both spans and
  the attachment are representable without discarding text. It is the one Dark
  row that separates "representable non-contiguous parent" from the Legends
  Giant Slug gap.
- Whippoorwill #1 is D19:P1 with the scoped marker `this turn` (class N5,
  already supported); its anaphor `the creature` refers to the parent's target.
- Venom #1 is class N7 (in-sentence, P-ATQ-1 regime), not a D19 split.
- H11 remains open; no D14/D19 classifier is accepted by this audit.

## Proposed changes

These are S10 items 1–3 only, written after the adjudication layer sealed. None
is accepted or implemented here, and none blocks freezing the next set.

1. **P-DRK-1 — bound the sentence-initial created-child span.** Generic
   candidate: when a `When/Whenever/At … this turn` (or otherwise scoped)
   sentence inside an activated or triggered ability is emitted as a
   `delayed_trigger` child, the child span ends at that sentence's terminator
   and any later sentences remain with the parent as a non-contiguous span
   (the accepted §5.8 / U3 shape). CR basis: 603.7a/603.7e for the child;
   611.2a and 614.1a for resolution-created continuous and replacement effects
   that stay with the parent. Positive: Runesword #0/#1. Nearest negatives
   that must stay whole: a created trigger whose *own* effect spans several
   sentences (CR 603.1 instructions such as a following `If you do, …`), and
   Whippoorwill-shaped children that already end the paragraph. Ambiguous
   class: guide example 18's shape, where a trailing sentence could be the
   trigger's consequence — the rule must abstain there. Expected effect:
   Runesword becomes `ok`/`ok`; the S8 corpus inventory of affected units and
   nearest non-matches (held-out excluded) is not yet run.
2. **Governance note, not a classifier proposal — C6 on plain spell text.**
   H10's failure is concentrated in one convention: whether `type_line` is
   "required" for plain instant/sorcery instruction text. The next guide
   version should state the adjudicated answer (`none` unless the §6.12
   step-10 gate changes the kind) as an explicit consequence in §8, and should
   name the boundary value for an over-inclusive child span. Neither change
   alters any judgement made here; a new version issues only between passes or
   before the next set (guide §14.6).

The D15 in-sentence class (Venom) stays under the decided P-ATQ-1 regime and
the D19 design; P-LEG-1 through P-LEG-3 remain unimplemented research
proposals.

## Measurements and agreement

| Field | Numerator | Denominator | Value |
|---|---:|---:|---:|
| Boundary precision | 160 | 163 | 0.9816 |
| Missed boundaries | 1 | n/a | count |
| Boundary recall | 160 | 161 | 0.9938 |
| Kind accuracy | 159 | 159 | 1.0000 |
| Role accuracy | 160 | 160 | 1.0000 |
| Source accuracy | 163 | 163 | 1.0000 |
| Structural exact-card correctness | 107 | 110 | 0.9727 |
| Independent row agreement | 141 | 163 | 0.8650 |
| Independent exact-card agreement | 89 | 110 | 0.8091 |
| Unsupported | 0 | 163 | 0.0000 |
| Ambiguous | 1 | 163 | 0.0061 |
| Unresolved adjudication | 0 | 163 | 0.0000 |
| Unit novelty | 127 | 163 | 0.7791 |
| Template novelty | 126 | 148 | 0.8514 |
| Multi-sentence frequency | 25 | 163 | 0.1534 |

Final dispositions: 159 accept, 3 defect, 1 ambiguous. Boundary values: 160
`ok`, 2 `under`, 1 `over`, 0 `unsure`. Final kind histogram is the frozen
baseline (activated 59, cast restriction 1, CDA 2, keyword 24, prevention 1,
replacement 5, static/spell 37, triggered 34); roles ability 161, delayed
trigger 2; all 163 sources printed. Pre-adjudication confusion (pass 1 →
pass 2): `context` none→type_line 18, none→cr 1; `boundary` ok→unsure 1,
under→ok 1; `disposition` accept→unsupported 1, adjudicate→unsupported 1,
ambiguous→defect 1, defect→accept 1; `role_ok` and `source_ok` had no
disagreement.

## Reproduction

```powershell
python scripts/python/audit_metrics.py `
  docs/audits/drk/units-annotated.tsv `
  --export docs/audits/drk/units-export.tsv `
  --earlier docs/audits/lea/units-export.tsv `
  --earlier docs/audits/leb/units-export.tsv `
  --earlier docs/audits/arn/units-export.tsv `
  --earlier docs/audits/atq/units-export.tsv `
  --earlier docs/audits/leg/units-export.tsv        # = docs/audits/drk/metrics.json

python scripts/python/audit_metrics.py `
  docs/audits/drk/units-annotated-pass1.tsv `
  --compare docs/audits/drk/units-annotated-pass2.tsv `
  --export docs/audits/drk/units-export.tsv         # row agreement 141/163

python -m unittest discover scripts/python -p "test_*.py"   # 21 passed
python scripts/python/verify_manifests.py docs/manifests/snapshot-scryfall-2026-08-25.json docs/manifests/experiment-dark-freeze-2026-08-27.json
.\target\release\mtg-discover.exe audit summary drk --exclude-heldout   # multi_sentence_unit_count 25
Get-FileHash -Algorithm SHA256 docs/audits/drk/units-annotated.tsv, docs/audits/drk/metrics.json
```

No source, protocol, guide, export, or sealed pass file was changed by the
adjudication; `cargo test` results at the freeze (89 passed) stand.

## Decision record

**Evidence.** Two complete sealed passes; 163/163 stable keys with zero drift
in both passes and in the final file; 141/163 preregistered agreement (H10
falsified) with 18/22 disagreements traced to one context convention; the
23-row adjudication union with CR/ruling review recorded on each row; the
Rocket Launcher export precedent for the non-contiguous-parent shape.

**Options.** (a) Leave The Dark open pending a guide revision and a re-pass to
recover H10; (b) close the empirical audit, report H10's failure and its cause,
and route the convention fix to the next guide version; (c) sustain the
`unsupported` gap on Runesword to avoid a third outcome neither pass chose.

**Decision.** Choose (b). The Dark is adjudicated and closed. H10's failure is
reported as a verified finding with its concentrated cause; it does not
invalidate the structural measurements, whose substantive disagreement rate is
4/163. Runesword is a representable boundary defect, not a schema gap, because
the frozen export already emits the correct shape elsewhere. P-DRK-1 enters the
S8–S12 pipeline alongside P-LEG-1..3; none is implemented or accepted here.

**Reversal evidence.** Reopen only for a demonstrated adjudication error,
structural drift in the frozen rows, an invalid CR/ruling basis, a ruling on
Angry Mob that decides its CDA status, or evidence that the export cannot in
fact emit a non-contiguous parent for a sentence-initial `When` child.

**Next set.** The next eligible first-printing development set is Fallen
Empires (`fem`, released 1994-11-01): 102 cards, 101 with Oracle text before
held-out exclusion (aggregate metadata only). Its preregistration and
held-out-safe freeze may begin immediately; the next guide version, if issued,
must be frozen before its passes open.
