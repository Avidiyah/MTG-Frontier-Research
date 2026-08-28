# Legends (`leg`) structural audit

- Status: **adjudicated and closed 2026-08-27; ready to freeze the next
  development set**
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
  - SHA-256:
    `89e314dc1f97e1f959062f5974fe2a2d1f855610198d4917e74bb8dfadaeb088`
- Pass 2: `gpt-5.6-pass2-2026-08-27`
  - SHA-256:
    `0615077043274c1250e3f479ddf5da00d2c454acf9c2629f7df24b9da89f77f5`
- Adjudicator: `fresh-legends-adjudicator-2026-08-27` (replacement after the
  opening-control incident recorded in the entry record)
- Final annotation: `docs/audits/leg/units-annotated.tsv`, SHA-256
  `678fcb58ac0e6b50d213493ef2a477162c9c698bb6c4b942313c273c177cb6cc`
- Final metrics: `docs/audits/leg/metrics.json`, SHA-256
  `91c0fc03407d06474afd190a1b36063dbea9f901a4a96608e44b39b4a28165c3`

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
- multi-sentence units: 60 as reported by the frozen measurement binary;
- kinds: activated 97, ante 2, cast restriction 4, CDA 2, keyword 91,
  prevention 9, replacement 8, static/spell 133, triggered 80;
- roles: ability 399, delayed trigger 5, granted 12, mode 10;
- source: printed 426.

The protocol-defined count is **61 / 426**. Both sealed passes independently
tagged the same 61 rows. The frozen binary missed Life Matrix index 0 because
its first sentence terminator precedes a closing quote (`." `), while
`is_multi_sentence` counted only terminators followed immediately by a space.
The post-pass reconciliation changes only that descriptive counter to implement
protocol §4.4's already-frozen definition of two `.`, `!`, or `?` terminators;
it does not change the frozen export, either pass, any judgement, or any
correctness denominator. The frozen value remains above for provenance.

## Adjudication record

- Both sealed files contain all 426 frozen keys with no duplicates, missing
  keys, or structural drift.
- Preregistered exact-row agreement is **409 / 426 (0.9601)**, so H8 passes
  before adjudication.
- Exact-card agreement is **256 / 273 (0.9377)**.
- The adjudicator reviewed the union of all disagreements, non-`accept` rows,
  and alleged card-specific dependencies: **30 distinct rows**.
- Sixteen defect rows have the same eight judgement fields in both passes.
  Giant Slug index 0 is the only row where the passes disagree about a
  structural outcome (`unsupported` versus `defect`).
- The other 16 disagreement rows differ only on `context`.
- Giant Slug index 0 was adjudicated `unsupported`, not downgraded to a normal
  boundary defect. Its ruling establishes that both post-colon sentences form
  a created delayed ability, but extracting that child would leave a cost-only
  parent fragment that the frozen contiguous single-parent export cannot
  represent.
- Context adjudication applied the frozen precedence to context actually
  required: 127 `cr`, 280 `none`, 18 `type_line`, 0 `game_state`, and 1
  `card_specific` (Giant Slug).
- Life Matrix index 0 retains the required `SPAN: non-contiguous parent
  (T2/T8)` observation. Stangg index 0 is explicitly D19:P1 because both later
  inverted triggers depend on the token created by the first sentence.
- No final row remains `adjudicate` or `ambiguous`. Both original pass IDs and
  the replacement adjudicator ID are retained on all 426 rows; reviewed rows
  carry an `ADJ:` rationale.

## Verified findings

### V1. Eleven reference units are missed on ten emitted rows

Nine rows contain ten un-emitted created delayed triggers: Abomination; Glyph
of Doom; Hazezon Tamar; both Infernal Medusa abilities; Infinite Authority;
Stangg (two triggers); and Time Elemental. Two more rows miss quoted abilities:
Johan's gained `"Johan can't attack"` ability and Takklemaggot's lost
`"enchant creature"` ability. All ten emitted rows are `under`; Stangg has
`missed = 2`, producing **11 missed boundaries**.

This is a verified structural finding, not permission to split every temporal
phrase or quoted string. The delayed-trigger cases include in-sentence,
sentence-level scoped, and effect-first inverted forms whose safe generic
boundaries require separate positive and negative searches.

### V2. Five quoted “bands with other” children have the wrong kind

The granted children of Adventurers' Guildhouse, Cathedral of Serra, Master of
the Hunt, Mountain Stronghold, and Unholy Citadel are emitted as
`spell_or_static_text`. CR 702.22b defines each `bands with other [quality]`
form as a banding ability, so all five should be `keyword_ability`. Their
parentage and `granted` role are correct.

### V3. One static regeneration ability has the wrong kind

Clergy of the Holy Nimbus's `If this creature would be destroyed, regenerate
it.` is emitted as `spell_or_static_text`. CR 614.8 and 701.19b make its
repeated regeneration a destruction-replacement effect, so its correct kind is
`replacement_effect`.

### V4. Accepted topology, keyword precision, prevention, and type-line gates hold

- All five emitted `delayed_trigger` children and all twelve `granted`
  children have valid same-card parents and correct roles (H2).
- All 91 emitted `keyword_ability` rows are genuine keywords (H3). V2 is a
  recall failure outside that denominator, not a precision counterexample.
- All nine emitted `prevention_effect` rows perform prevention, and all 23
  declared prevention/prohibition surface candidates are correctly typed
  (H4).
- All 65 top-level instant/sorcery-face units pass the type-line kind check
  (H6).
- No non-null prefix exists, so H5 has denominator zero and is not evaluated.

### V5. Preregistered novelty and agreement predictions pass

Unit novelty is **310 / 426 (0.7277)**, below Antiquities' preregistered 0.768
threshold, so H7 passes. Exact independent judgement-record agreement is
**409 / 426 (0.9601)**, above H8's 0.95 threshold. These results do not imply
semantic coverage or monotonic novelty.

## Bounded observations

- H1 passes narrowly: no emitted delayed-trigger child leaves an actual
  condition-only, cost-only, or quoted-fragment parent. Giant Slug shows why
  this is not evidence that the representation is complete: the segmenter
  avoids such a fragment only by leaving the entire created delayed ability
  inside one unsupported row.
- The final audit has 409 accepted rows, 16 defect rows, and one unsupported
  row. Structural exact-card correctness is 257 / 273 (0.9414).
- The protocol-defined multi-sentence count is 61 / 426 (0.1432); the frozen
  measurement binary's 60 remains recorded in the baseline for provenance.
- Normalization flags are suspicions, not verified errors: color-word
  fragmentation 34 / 426, land-type fragmentation 12 / 426, object-type
  fragmentation 30 / 426, and tap-as-mana collision 69 / 426.

## Unsupported and ambiguous cases

- **Unsupported: 1 / 426.** Giant Slug index 0 is
  `gap:span:cost_only_parent`. The correct created child consumes the entire
  effect after `{5}:`; the frozen export cannot emit that child while retaining
  a complete activated-ability parent. The card-specific 2004-10-04 ruling is
  required to settle the scope of the created ability.
- **Ambiguous: 0 / 426.**
- **Unresolved adjudication: 0 / 426.**

## D14 independent-trigger observations

No new D14 independent-trigger candidate was adjudicated in the development
partition. This zero does not support an adjacency classifier and does not
complete H9's required future nearest-negative comparison.

## D19 effect-created delayed-trigger observations

- Glyph of Doom is D19:P1: its later trigger refers to the Wall targeted by
  the preceding instruction.
- Stangg is D19:P1: both inverted later triggers depend on the token created by
  the first sentence. They account for two missed units.
- Glyph of Life and Infinite Authority retain previously supported scoped-child
  observations; they are not evidence that unscoped adjacency is sufficient.
- H9 remains open because its falsifier requires the later S8 nearest-negative
  sample. No D14/D19 classifier is accepted by this audit.

## Proposed changes

These are S10 items 1-3 only. None is accepted or implemented here, and none
blocks freezing The Dark.

1. **P-LEG-1 — quoted `bands with other [quality]` kind.** Generic candidate:
   classify a complete granted quoted form covered by CR 702.22b as
   `keyword_ability`. Positives are the five V2 rows. Nearest negatives must
   include `"bands with other"` used only as an ability-class label (Shelkin
   Brownie and Tolaria) and ordinary short quoted prose.
2. **P-LEG-2 — static regeneration replacement.** Generic candidate:
   permanent static text of the form `If [this permanent] would be destroyed,
   regenerate it` is `replacement_effect` under CR 614.8/701.19b. The positive
   is Clergy of the Holy Nimbus. Nearest negatives must include one-shot spell
   and activated regeneration instructions.
3. **P-LEG-3 — missing quoted gained/lost children.** Generic candidate:
   complete ability text inside `gains` or `loses` quotes is a `granted` child,
   including short static and keyword forms. Positives are Johan and
   Takklemaggot. Nearest negatives include labels such as `"bands with other"`
   and quoted non-ability choices.

The delayed-trigger defects remain observations under the D19 design. Giant
Slug remains an explicit schema gap. Neither receives a classifier proposal
until the required positive, negative, and ambiguous inventories exist.

## Measurements and agreement

| Field | Numerator | Denominator | Value |
|---|---:|---:|---:|
| Boundary precision | 415 | 425 | 0.9765 |
| Missed boundaries | 11 | n/a | count |
| Boundary recall | 415 | 426 | 0.9742 |
| Kind accuracy | 399 | 405 | 0.9852 |
| Role accuracy | 415 | 415 | 1.0000 |
| Source accuracy | 426 | 426 | 1.0000 |
| Structural exact-card correctness | 257 | 273 | 0.9414 |
| Independent row agreement | 409 | 426 | 0.9601 |
| Independent exact-card agreement | 256 | 273 | 0.9377 |
| Unsupported | 1 | 426 | 0.0023 |
| Ambiguous | 0 | 426 | 0.0000 |
| Unresolved adjudication | 0 | 426 | 0.0000 |
| Unit novelty | 310 | 426 | 0.7277 |
| Template novelty | 281 | 315 | 0.8921 |
| Multi-sentence frequency | 61 | 426 | 0.1432 |

Final kind histogram: activated 97, ante 2, cast restriction 4, CDA 2, keyword
91, prevention 9, replacement 8, static/spell 133, triggered 80. Final role
histogram: ability 399, delayed trigger 5, granted 12, mode 10. All 426 sources
are printed.

## Reproduction

```powershell
python scripts/python/audit_metrics.py `
  docs/audits/leg/units-annotated.tsv `
  --export docs/audits/leg/units-export.tsv `
  --earlier docs/audits/lea/units-export.tsv `
  --earlier docs/audits/leb/units-export.tsv `
  --earlier docs/audits/arn/units-export.tsv `
  --earlier docs/audits/atq/units-export.tsv

python scripts/python/audit_metrics.py `
  docs/audits/leg/units-annotated-pass1.tsv `
  --compare docs/audits/leg/units-annotated-pass2.tsv `
  --export docs/audits/leg/units-export.tsv

cargo test
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
cargo build --release
.\target\release\mtg-discover.exe audit summary leg --exclude-heldout
```

The corrected summary reports `multi_sentence_unit_count: 61`; all other
Legends summary fields remain equal to the frozen pre-audit baseline.

## Decision record

**Evidence.** Two complete sealed passes, 409/426 preregistered agreement, the
30-row adjudication union, CR/ruling review recorded on each adjudicated row,
426/426 stable keys, and zero export drift.

**Options.** (a) leave Legends open pending implementation proposals; (b) close
the empirical audit while keeping proposals and D14/D19 work separate; (c)
force Giant Slug into an existing defect class.

**Decision.** Choose (b). Legends is adjudicated and closed. P-LEG-1 through
P-LEG-3 enter the later S8-S12 proposal pipeline; they do not retroactively
alter this audit and do not block the next set. Giant Slug remains unsupported
rather than being forced into a representable defect.

**Reversal evidence.** Reopen only for a demonstrated adjudication error,
structural drift in the frozen rows, an invalid CR/ruling basis, or evidence
that the frozen schema can represent Giant Slug's parent and child without
discarding, duplicating, or reassigning text.

**Next set.** The next eligible first-printing development set is The Dark
(`drk`, released 1994-08-01): 119 cards, 116 with Oracle text before held-out
exclusion. Its preregistration and held-out-safe freeze may begin immediately.
