# Legends (`leg`) structural-audit preregistration

- Date prepared: 2026-08-26
- Status: **preregistered research design; audit not started**
- Protocol: `docs/protocol/structural-investigation-protocol.md` v1.0
- Permitted activation: only after the P-ATQ technical acceptance package closes
  without a material contradiction

## 1. Purpose and non-observation statement

This document freezes the research design for the Legends structural audit
before its baseline is generated and before an auditor inspects any eligible
Legends card text.

Preparation of this preregistration used protocol text, aggregate measurements,
audit schemas, and prior decision records only. It did not query, quote, inspect,
or annotate an individual Legends card. It did not inspect a frozen held-out
card. It contains no Legends findings.

The audit remains closed until every entry condition in section 12 is satisfied.

## 2. Investigation objective

Determine how well the accepted structural measurement baseline identifies
reference units, kinds, roles, sources, and parent/child attachment in the next
eligible first-printing development set, while measuring structural novelty
against the finalized Alpha, Beta, Arabian Nights, and Antiquities exports.

The investigation will:

- perform an exhaustive review of the eligible non-held-out development
  partition under protocol S4;
- measure boundaries, kinds, roles, sources, context requirements,
  normalization risks, novelty, and annotation agreement;
- test accepted delayed-trigger, prevention, prefix, type-line, keyword,
  modal, and quoted/granted structural rules as measurement hypotheses;
- preserve D14 and D19 as separate attachment questions;
- record unsupported and ambiguous structures without forcing them into the
  residual kind;
- generate implementation proposals only after independent annotation and
  adjudication, and only under protocol S8-S12.

The objective is structural discovery. It does not select a parser, semantic
IR, engine, execution model, annotation platform, or other infrastructure.

## 3. Frozen inputs to record after ATQ closes

The following block must be populated from commands and file hashes after
Claude's P-ATQ technical package is accepted. No value may be copied from this
preregistration or an older findings document merely because it is expected to
remain unchanged.

```text
P-ATQ acceptance record:      <path, decision, commit>
Repository commit:            <git rev-parse HEAD>
Repository status:            <clean except explicitly listed governance artifacts>
Protocol version and sha256:   <structural-investigation-protocol.md>
Preregistration sha256:        <this file>
Annotation-guide version/hash: <frozen guide used by both passes>
mtg-discover version/build:    <release build and --version>
cargo test:                    <N passed, 0 failed>
Scryfall oracle snapshot:      <name, date/updated_at, size, sha256 if available>
Scryfall rulings snapshot:     <name, date/updated_at, size, sha256 if available>
Scryfall default snapshot:     <name, date/updated_at, size, sha256 if available>
cards.sqlite identity:         <size, mtime, content/source identity available>
Comprehensive Rules:           <effective date and file sha256>
Corpus metadata:               <verbatim mtg-discover info output>
Set aggregate metadata:        <verbatim eligible set summary; no row inspection>
Earlier audited sets:          <final lea, leb, arn, atq export paths and hashes>
Current corpus baseline:       <units, templates, coverage, kind histogram, role histogram>
Held-out pool definition:      <protocol 6.3 definition and manifest/hash>
Held-out exclusion registry:   <cumulative incident exclusions and linked identities>
Development-export command:    <held-out-safe deterministic command or procedure>
Development-export sha256:     <frozen non-held-out TSV>
Annotator pass 1:              <independent id>
Annotator pass 2:              <independent id>
Adjudicator:                   <id>
```

The set-level pre-audit baseline must then record, before any eligible card is
read:

- eligible development cards and cards with Oracle text;
- fallback and held-out exclusions as aggregate counts only;
- printed and rules-supplied units;
- top-level and child units;
- distinct templates and singleton templates;
- top-10, top-25, top-50, top-100, top-1,000, and top-5,000 coverage where the
  command supports them;
- kind, role, source, face, and parent/non-parent histograms;
- corpus-wide comparison totals required by S10/S11.

The baseline and export are immutable audit inputs. Any later segmenter change
is measured against them rather than silently replacing them.

## 4. Scope and partitions

### 4.1 Development partition

The development partition is every eligible Legends card with Oracle text that:

- belongs to the first-printing set under the repository's set definition;
- has `first_is_fallback = 0`;
- is not in the frozen held-out pool;
- is not otherwise excluded by the protocol's set-type rules.

Review is exhaustive over every unit of this eligible non-held-out partition.
The phrase “exhaustive Legends audit” always means exhaustive over this
development partition, never over held-out records.

All faces and mechanically linked text objects sharing an Oracle identity stay
in one partition. No face of a held-out identity may enter development.

### 4.2 Regression partition

The regression partition is the final post-P-ATQ annotated and exported state
of:

- `docs/audits/lea/`;
- `docs/audits/leb/`;
- `docs/audits/arn/`;
- `docs/audits/atq/`.

The final export hash for each set must be recorded after Claude closes ATQ. If
the accepted P-ATQ changes legitimately alter earlier rows, the re-annotation
and explanation are part of the frozen regression input.

No Legends-derived proposal may be accepted without re-exporting and measuring
all four regression sets under protocol S10.

### 4.3 Held-out partition

The held-out partition remains protocol section 6.3's frozen pool:

- Oracle text present;
- `oracle_id` begins with hexadecimal `f`;
- `first_is_fallback = 0`;
- the protocol's historical development-set exceptions unchanged.

Held-out cards may appear in aggregate corpus counts only. They may not appear
in an auditor-visible development export, candidate list, search result,
example, quotation, note, or annotation.

The following incident-exposed cards are permanently excluded from future
held-out samples:

- Combust;
- Malignus;
- Lava Burst;
- Wild Slash.

This list is additive, not exhaustive: every other exclusion already recorded
in the cumulative held-out incident registry remains excluded. Any future
accidental exposure must be logged immediately, and the exposed Oracle identity
and all linked faces/variants must be excluded from later held-out sampling.

No Gate 1 held-out sample is selected in this preregistration. Sampling and
annotation occur only at the declared Gate 1 review.

## 5. Preregistered hypotheses

Every hypothesis is structural. A falsified hypothesis becomes a reported
finding; it does not authorize a classifier change.

### H1 — Boundary fragmentation

**Claim:** The accepted delayed-trigger rules will emit no parent that consists
only of a trigger condition, activation cost, or quoted-text fragment.

**Falsifier:** At least one emitted development unit is adjudicated `over`
because a delayed-trigger child left a condition-only, cost-only, or
quoted-text-fragment parent.

**Denominator:** All printed development units, with a separate count over all
nested delayed-trigger parent/child pairs.

### H2 — Delayed-trigger topology

**Claim:** Role and topology remain mutually consistent: every nested
role=`delayed_trigger` unit has a valid parent; every top-level
spell-created role=`delayed_trigger` unit has no parent; quoted delayed text
remains role=`granted` rather than being promoted by the spell-created rule.

**Falsifier:** Any delayed-trigger unit has missing, cyclic, cross-face, or
wrong-class parentage, or any top-level/nested/granted class receives the wrong
role.

**Denominator:** All development units with role=`delayed_trigger` or
role=`granted`, reported separately for parented and parentless units.

### H3 — Keyword precision

**Claim:** Every printed development unit labeled `keyword_ability` is a
keyword ability under the Comprehensive Rules; ordinary short static text is
not mislabeled as a keyword.

**Falsifier:** At least one printed `keyword_ability` unit is adjudicated as
non-keyword.

**Denominator:** All printed development units labeled `keyword_ability`.

### H4 — Prevention taxonomy

**Claim:** Every development unit labeled `prevention_effect` performs or
establishes prevention under CR 615, and no prohibition whose operative wording
is `can't be prevented` or `cannot be prevented` receives that kind.

**Falsifier:** Any `prevention_effect` row is a prohibition, hidden trigger,
spell instruction protected by type-line context, or other non-prevention
structure; or any genuine positive prevention static is missed solely because
of the prohibition exclusion.

**Denominator:** All development `prevention_effect` units plus all
non-quoted development units containing a declared prevention/prohibition
surface candidate, with the candidate inventory produced only after the audit
opens.

### H5 — Prefix classification

**Claim:** Every extracted prefix is a genuine structural label—an ability word,
flavor word, Saga chapter symbol, named mode, result label, or another
CR-supported label—and it does not hide the body's correct kind.

**Falsifier:** Any extracted prefix is ordinary classification-relevant prose;
any ability/flavor label still hides a trigger; or any Saga chapter is
classified from its effect verb rather than as a chapter trigger.

**Denominator:** All development units with a non-null `prefix`, reported by
prefix class, kind, role, face type, and outcome.

### H6 — Type-line context

**Claim:** Top-level ordinary spell instructions on instant or sorcery faces do
not receive lexical `replacement_effect`, `prevention_effect`, or
`characteristic_defining_ability` kinds unless a CR 113.6 exception establishes
a genuine non-spell ability.

**Falsifier:** Any applicable development unit is misclassified because the
classifier ignored or misapplied its per-face type line.

**Denominator:** All top-level printed development units on instant or sorcery
faces, with multi-face records reported separately.

### H7 — Novelty persistence

**Claim:** Unit novelty against the finalized `lea`+`leb`+`arn`+`atq`
regression exports will be lower than Antiquities' recorded 96/125 (0.768).
This is a single-step prediction, not a restored claim that novelty decreases
monotonically with release date.

**Falsifier:** Development unit novelty is at least 0.768.

**Denominator:** All printed development units. Template novelty is reported
separately over distinct printed development templates.

### H8 — Independent agreement

**Claim:** Before discussion, the two complete annotation passes will agree on
at least 95% of aligned unit records across all judgement fields.

**Falsifier:** Exact judgement-record agreement is below 0.95.

**Denominator:** Frozen export rows present in both passes; agreement requires
identical `boundary`, `missed`, `kind_expected`, `kind_ok`, `role_ok`,
`source_ok`, `context`, and `disposition` values. CR citations, structure tags,
and notes receive separate overlap/disagreement reporting rather than being
folded into this rate.

### H9 — D14/D19 non-equivalence

**Claim:** Sentence adjacency and an unscoped later trigger word are not, by
themselves, sufficient to distinguish a D19 effect-created delayed trigger from
a D14 independent trigger sharing a paragraph.

**Falsifier:** After independent adjudication of every in-scope candidate and
the future S8 nearest-negative sample, a single generic adjacency-only surface
rule separates all positive and negative cases with no ambiguity or
counterexample.

**Denominator:** All adjudicated D14/D19 attachment candidates identified in the
development audit plus the later non-held-out S8 comparison set.

## 6. Predeclared measurements and denominators

The findings report must publish every applicable protocol section 4.5 measure
with numerator and denominator, plus the additions below.

| Measurement | Numerator | Denominator |
|---|---|---|
| Boundary precision | Printed units with `boundary = ok` | Printed units judged, excluding `unsure` |
| Missed boundaries | Sum of `missed` | Count only |
| Boundary recall | Printed units with `boundary = ok` | `ok` units plus sum of `missed` |
| Kind accuracy | `kind_ok = yes` | Boundary-ok units with `kind_ok` in `{yes,no}` |
| Role accuracy | `role_ok = yes` | Boundary-ok units with `role_ok` in `{yes,no}` |
| Source accuracy | `source_ok = yes` | All units judged |
| Structural exact-card correctness | Eligible cards with no boundary error/unsure and all applicable kind, role, and source judgements correct | Eligible development cards judged |
| Inter-annotator row agreement | Rows with identical preregistered judgement fields | Frozen export rows in both independent passes |
| Inter-annotator exact-card agreement | Cards whose rows agree on every preregistered judgement field | Eligible cards in both passes |
| Unsupported structures | Rows dispositioned `unsupported`, by `gap:<class>` | All units |
| Ambiguous structures | Rows dispositioned `ambiguous`, by competing reading | All units |
| Unresolved adjudication | Rows still `adjudicate` after the adjudication meeting | All units |
| Context distribution | Rows by `none`, `cr`, `type_line`, `game_state`, `card_specific` | All units |
| Normalization fragmentation | Rows tagged `fragmentation:*`, by tag | Printed units |
| Suspected collisions | Rows tagged `collision:*`, by tag | Printed units |
| Unit novelty | Printed units whose template occurs in no earlier audited export | Printed units |
| Template novelty | Distinct printed templates absent from all earlier audited exports | Distinct printed templates |
| Multi-sentence frequency | Printed rows tagged `multi_sentence` | Printed units |
| Kind/role/source histograms | Units in each value | All units, with printed/rules-supplied split |
| Structure-tag prevalence | Rows carrying each frozen tag | All units and printed units, both reported |
| Delayed-trigger topology | Correct rows in each top-level/nested/granted class | All rows in that class |
| D14/D19 attachment outcomes | Positive, negative, ambiguous, unsupported | All attachment candidates; descriptive only |
| Held-out exposure | Accidentally exposed identities | Expected zero; no accuracy rate |

Template coverage and singleton counts are descriptive measurements, not
correctness metrics. No semantic coverage percentage may be inferred from them.

## 7. Independent annotation and adjudication workflow

### 7.1 Preparation

1. Close and record P-ATQ acceptance.
2. Freeze the repository, data/rules snapshots, protocol, annotation guide,
   preregistration, earlier exports, and held-out exclusion registry.
3. Generate a deterministic development export that excludes held-out
   identities before any row becomes visible to an auditor.
4. Verify the export schema and aggregate counts without printing or opening
   individual rows.
5. Hash the export and clone it into two annotation-pass files.

### 7.2 Independent passes

- Two annotators review the complete development export independently.
- Neither annotator reads the other's annotations, notes, hypotheses about
  specific rows, or candidate proposal list before both passes are sealed.
- Each annotator reads the full eligible card text and type line only after the
  baseline/export freeze, and consults CR/rulings as protocol S5 requires.
- Every row receives an explicit disposition. `unsure`, `unsupported`, and
  `ambiguous` are used rather than guesses.
- No implementation proposal is discussed during either pass.
- Each sealed pass receives a content hash and timestamp.

### 7.3 Comparison

After both passes are sealed:

1. align rows only by the frozen stable unit identity;
2. calculate the preregistered row-level and exact-card agreement measures;
3. publish confusion counts for boundary, kind, role, source, context, and
   disposition;
4. list every disagreement without resolving it automatically;
5. report missing or duplicate keys as export defects, not annotation
   disagreement.

### 7.4 Adjudication

- The adjudicator reviews every disagreement, every non-`accept` row, every
  `unsure`, `unsupported`, and `ambiguous` row, and every alleged
  card-specific dependency.
- CR and Oracle text control; official rulings clarify but do not override the
  CR.
- A genuine rules ambiguity remains `ambiguous` with both readings.
- A vocabulary gap remains `unsupported` with `kind_expected = gap:<class>`.
- The final adjudicated file preserves original pass IDs and the adjudication
  rationale.
- The audit is not called adjudicated until no row remains merely
  `adjudicate` without an explicit documented reason.

## 8. Unsupported and ambiguous dispositions

Use the frozen protocol meanings without broadening them during annotation:

- **`unsupported`:** the current structural vocabulary cannot express the
  reference kind, role, source, span, or attachment. Record `gap:<class>`, the
  visible evidence, CR references, and the smallest missing structural
  distinction. Do not force the row into `spell_or_static_text`.
- **`ambiguous`:** CR and authoritative rulings do not determine one structural
  reading. Record every live reading and why the evidence does not choose.
  Exclude the row from accuracy denominators that require a unique answer.
- **`adjudicate`:** a temporary pre-adjudication state for annotator uncertainty
  or disagreement, not a final substitute for `unsupported` or `ambiguous`.
- **`unsure`:** a field-level value used when that field cannot be judged; it is
  reported separately and not converted into a guess.

Card-specific context may justify an annotation when an official ruling is
needed, but no accepted heuristic may contain a card name, set code, Oracle ID,
or per-card branch.

## 9. Separating observations from proposals

The audit proceeds in four sealed layers:

1. **Observation:** record the frozen segmenter output and independent human
   judgement. A claim includes numerator, denominator, authority, and
   counterexamples.
2. **Adjudication:** resolve disagreements or preserve unsupported/ambiguous
   outcomes. No code changes occur.
3. **Proposal:** only after the adjudicated report, state a generic surface
   rule, the CR class, defect rows, predicted corpus effect, and falsifiers.
4. **Acceptance:** only after separate S8-S12 evidence, implementation, tests,
   corpus measurement, and regression review.

Rules:

- a frequency or template match is a candidate measurement, not ground truth;
- a structural label is not a semantic operator or behavioral claim;
- a proposal cannot retroactively alter the preregistered hypothesis or
  denominator;
- observations that do not support a generic rule remain findings;
- one-off or card-specific behavior is not repaired with a card branch;
- D14 and D19 candidates stay observational until their separate research design
  is executed;
- the Legends findings document may propose changes but may not implement them.

## 10. D14, D19, and delayed-trigger representation

D14 and D19 are separate questions:

- **D19:** a later unscoped trigger-word sentence may be a delayed triggered
  ability created by resolution of preceding effect text and therefore a child
  of that creating ability.
- **D14:** a later trigger-word sentence may instead be an independent printed
  triggered ability sharing an Oracle paragraph and therefore a sibling or
  top-level reference unit.

Adjacency alone does not settle attachment. During Legends annotation, candidate
rows may be tagged for later comparison, but no D14/D19 classifier rule is
introduced.

P-ATQ-1 remains distinct: it rejects fragmentary comma/colon boundaries inside a
single sentence. D14/D19 concern sentence-level attachment and independence.

The accepted topology distinction also remains frozen:

- top-level spell-created delayed trigger: role=`delayed_trigger`, no parent;
- nested trigger created by preceding effect text: role=`delayed_trigger`, valid
  parent;
- quoted/granted triggered text: role=`granted`;
- ordinary independent triggered ability: role=`ability`.

## 11. Stop conditions and escalation criteria

### 11.1 Stop before opening the audit

Do not generate an auditor-visible Legends export or inspect any eligible row if:

- Claude's P-ATQ package is incomplete or materially contradicts a research
  disposition;
- `docs/current-state.md` and the accepted P-ATQ record disagree about the live
  baseline;
- the release build or tests fail at the intended frozen commit;
- source snapshots, CR identity, earlier export hashes, or preregistration hash
  are missing;
- the development export cannot be proven to exclude every held-out identity;
- stable unit keys are duplicated, missing, nondeterministic, or inconsistent
  across repeated exports;
- an intended annotator has already inspected eligible Legends text in a
  heuristic-design context before the baseline freeze.

Escalate the smallest issue to the responsible technical owner, research lead,
or program owner. Do not work around a failed entry condition by reading cards
manually.

### 11.2 Pause during annotation

Pause and record a governance issue if:

- a held-out record appears in any auditor-visible output;
- export drift occurs without an accepted change record;
- the schema cannot preserve the observed parent/child span or attachment;
- instructions would require changing the annotation guide after one pass has
  seen the affected rows;
- a semantic or behavioral decision is being used to force a structural label;
- the two annotators discover they were not independent.

An exposed held-out identity is logged and excluded. A compromised annotation
pass is not silently retained; the research lead decides whether to restart with
a new independent annotator or preserve the limitation explicitly.

### 11.3 Escalate after adjudication

A classifier proposal is eligible only when:

- at least one adjudicated defect row demonstrates a structural failure;
- the proposed rule is generic and CR-grounded;
- positive, negative, and ambiguous classes can be stated without card names;
- a future S8 search can inventory the rule and nearest non-matches while
  excluding held-out records;
- the expected boundary, parentage, kind, role, and corpus effect are declared;
- failure can remain explicit if surface evidence is insufficient.

If D14 and D19 cannot be separated reliably from surface form plus permitted
context, no classifier proposal is made. Retain the attachment signal and
unsupported/ambiguous outcome.

## 12. Entry checklist

The audit opens only when every item is checked in writing:

- [ ] Claude's technical P-ATQ package passes and is incorporated into the
      acceptance record.
- [ ] Any contradiction is adjudicated without silently changing P-ATQ
      dispositions.
- [ ] `docs/current-state.md` reflects the accepted live baseline.
- [ ] Frozen commit, data snapshots, CR, protocol, guide, and earlier export
      hashes are recorded.
- [ ] Build and tests pass at the frozen commit.
- [ ] A held-out-safe deterministic development export exists and has been
      verified by aggregate counts only.
- [ ] The cumulative held-out exclusion registry, including the four named
      incident exclusions, is bound to the audit.
- [ ] Both independent annotators and the adjudicator are assigned.
- [ ] Neither annotator has inspected eligible Legends text before the freeze.
- [ ] `docs/findings/leg-structural-audit.md` remains an empty outline until the
      baseline block is written verbatim.
- [ ] The program owner authorizes the audit to begin.

## 13. Empty findings-document outline

The empty, non-empirical outline is stored at
`docs/findings/leg-structural-audit.md`. It contains placeholders only for:

1. frozen inputs and audit status;
2. scope and exclusions;
3. preregistered hypotheses;
4. pre-audit baseline;
5. verified findings;
6. bounded observations;
7. unsupported and ambiguous cases;
8. D14 and D19 attachment observations kept separate;
9. proposed changes under S10 items 1-3 only;
10. measurements and agreement;
11. reproduction;
12. decision record.

No empirical result may be added to that file until the entry checklist passes.
