# The Dark (`drk`) structural-audit preregistration

- Date prepared: 2026-08-27
- Status: **preregistered research design; audit not started**
- Protocol: `docs/protocol/structural-investigation-protocol.md` v1.0
  (SHA-256 `1bc05d357b24006a2eecc692f9bed5b86d1d828f116c2d741fb75662df4913bf`)
- Annotation guide: frozen Legends v1.0
  `docs/protocol/structural-annotation-guide-v1.0.md`
  (SHA-256 `d31dee0a3b06494bd7ba0238be65b330e2366edb1b8bcf4e5e6a6f865de5d84b`),
  incorporated by reference and reused unchanged for The Dark
- Immediate methodological precedent: the closed Legends (`leg`) audit
  (`docs/findings/leg-structural-audit.md`). Its **procedure** governs this
  design; its **measurements and set-specific conclusions are not carried into
  The Dark** and appear here only where a prior observation is explicitly
  labelled as a comparison point, never as a Dark result.
- Permitted activation: only after this preregistration and the guide decision
  set are frozen, the input freeze (§13) completes, and every §12 entry
  condition reads satisfied in writing.

## 1. Purpose and non-observation statement

This document freezes the research design for The Dark structural audit before
its baseline is generated and before any auditor inspects an eligible Dark card.

Preparation of this preregistration used protocol text, the frozen annotation
guide, the D19 attachment research design, the committed `lea`/`leb`/`arn`/
`atq`/`leg` audit artifacts, source/test contracts, and aggregate-only set
metadata (`mtg-discover sets`) only. It did **not** query, print, quote,
inspect, segment, or annotate an individual Dark card, and it did **not**
inspect any held-out identity. It contains no Dark findings.

The audit remains closed until every entry condition in §12 is satisfied.

## 2. Objective

Determine how well the **frozen** accepted structural measurement baseline (the
segmenter and normalizer at the measurement-freeze commit, with no P-LEG
proposal implemented) identifies reference units, kinds, roles, sources, and
parent/child attachment in the next eligible first-printing development set, The
Dark, while measuring structural novelty against the finalized Alpha, Beta,
Arabian Nights, Antiquities, and Legends exports.

The investigation will:

- perform an exhaustive review of the eligible non-held-out Dark development
  partition under protocol S4;
- measure boundaries, kinds, roles, sources, context requirements,
  normalization risks, novelty, and independent annotation agreement;
- test the accepted delayed-trigger, prevention, prefix, type-line, keyword,
  modal, quoted/granted, and static-regeneration structural behaviours **as
  measurement hypotheses about the frozen classifier**, not as fixes;
- preserve D14 and D19 as separate attachment questions;
- record unsupported and ambiguous structures without forcing them into the
  residual kind;
- generate implementation proposals only after independent annotation and
  adjudication, and only under protocol S8–S12.

The objective is structural discovery. It selects no parser, semantic IR,
engine, execution model, or annotation platform.

**The frozen segmenter is measured first.** P-LEG-1 (quoted `bands with
other [quality]` kind), P-LEG-2 (static-regeneration replacement), and P-LEG-3
(missing quoted gained/lost children) are Legends research proposals only. None
is implemented before this audit's baseline freeze, and this audit does not
turn any of them into accepted behaviour. Where The Dark reproduces a
Legends-observed defect class, that reproduction is the measurement, recorded
against the frozen classifier.

## 3. Frozen inputs to record after the measurement freeze

The following block is populated in Phase 3 (§13) from live command output and
file hashes at the Dark measurement-freeze commit. No value may be copied from
this preregistration, from the Legends documents, or from an older findings
document merely because it is expected to remain unchanged. Expected Legends
values must not be pasted here.

Populated 2026-08-27 from live command output at the measurement-freeze commit.
The pre-population frozen control-text hash is recorded below; populating this
block changes the file hash, exactly as for Legends.

```text
Legends closure record:        docs/findings/leg-structural-audit.md; adjudicated
                              and closed 2026-08-27; final annotation drift 0;
                              426 keys; dispositions accept 409 / defect 16 /
                              unsupported 1; zero adjudicate/ambiguous
Repository / measurement-freeze commit:   70fa956515123b80d33ab08a13e938d54c6b66f8
Repository status:            clean at freeze selection; later governance commit
                              a384dc2 (this preregistration + outline + entry
                              record) and the Phase 3 frozen-input artifacts are
                              listed in docs/manifests/experiment-dark-freeze-2026-08-27.json
Protocol version and sha256:   structural-investigation-protocol.md v1.0;
                              1bc05d357b24006a2eecc692f9bed5b86d1d828f116c2d741fb75662df4913bf
Preregistration sha256:        pre-population frozen control text:
                              b1c31c3b4dc1a0b4774bb5ed64c0d8549970d7f9768d1813794b48ad3338c2ba
Annotation-guide version/hash: frozen Legends v1.0;
                              d31dee0a3b06494bd7ba0238be65b330e2366edb1b8bcf4e5e6a6f865de5d84b
mtg-discover version/build:    0.1.0; cargo build --release passed
cargo test:                    89 passed, 0 failed; fmt and clippy clean;
                              Python unittest (scripts/python test_*.py) 21 passed, 0 failed
Scryfall oracle snapshot:      oracle-cards.jsonl.gz; 2026-08-25; 24,532,087 bytes;
                              9611b5d93b20478a0ee46bae8b20a9eb39ee980f0ef4f5f6f6aaa8f7ab010ab2
Scryfall rulings snapshot:     rulings.jsonl.gz; 2026-08-25; 5,366,171 bytes;
                              3064689880a73f804f6e20411f6896d26aec06286eb4f2eb23d26e53779efe6f
Scryfall default snapshot:     default-cards.jsonl.gz; 2026-08-25; 77,608,798 bytes;
                              d65608b4993aeb2bd31ef8dfb41f6a9aa37396720d0a61d1354f528d8909667e
cards.sqlite identity:         65,781,760 bytes; mtime 2026-08-26T06:40:45.988Z;
                              d1c88cb9ab96531c2f2ce8f3b048c727240811e1f16acb141adbdb60998195c4
Comprehensive Rules:          effective 2026-08-07;
                              dc01ca5462085d6e3f7e85f548960a017522d1d851ac6a11d26ae14b6610c072
Corpus metadata:              38,626 cards; 37,916 with Oracle text; 710 without;
                              3,212 multi-face; 553 first-printing sets;
                              78,949 rulings; 3,455 numbered rules; 752 glossary entries
Set aggregate metadata:       113 eligible development cards; 110 with text;
                              163 printed units; 0 rules-supplied; 161 top-level /
                              2 child; face 0 = 163; prefix non-null/null = 0/163;
                              148 templates; 138 singleton; 25 multi-sentence
                              (corrected protocol instrument, emitted directly);
                              kinds activated 59, cast restriction 1, CDA 2,
                              keyword 24, prevention 1, replacement 5,
                              static/spell 37, triggered 34; roles ability 161,
                              delayed_trigger 2; source printed 163
Earlier audited sets:         lea export aabc1bd58ce38a0e73c9c9ce23344a124482dd02f36c470129aecd9bf609f3bf,
                              annotation cdd2438d5b44678200fe145dde49df7fc833974b8e19035d8391fa7002ecff91;
                              leb export 4cb90170876be1d945d79915780ab831b452f3878c79515f66d40e06ca2c6c05,
                              annotation 6a3a056385ff1618f9243dbdd9ed1cd673dedb8179711c45c65fe7117479494a;
                              arn export 4827f5be9305b4a6e84978395a2c80566565ebf1bfc0943ff705099ff68f327d,
                              annotation cfd31206e731061674ccce542c63198083c1ef0b80baebf38a8797e84e1656eb;
                              atq export 8ec1047b3443845ca61c63a0a5cc8c444ae7f6c693a9c357e88bf0d0b244143d,
                              annotation f01c9f84be527efbe0f9252190f7040219628deb78b277687262ef8f265eb8f4;
                              leg export c39a2d695b94ce33a2e16356dd93bc6dc614b7c83becfb2b2f72ad5cb298d2e3,
                              annotation 678fcb58ac0e6b50d213493ef2a477162c9c698bb6c4b942313c273c177cb6cc
Current corpus baseline:       71,563 printed units; 970 rules-supplied;
                              37,299 templates; coverage top 10/25/50/100/1,000/5,000 =
                              14.17/20.03/23.41/26.90/42.18/54.87%;
                              kinds activated 11,998, additional cost 319, ante 9,
                              cast restriction 69, CDA 255, keyword 17,840,
                              prevention 166, replacement 2,174, static/spell 19,519,
                              triggered 19,214; roles ability 67,045, delayed 891,
                              granted 1,506, mode 2,121
Held-out pool definition:      protocol §6.3; 2,096 identities;
                              377e12bdf80e0263c361f48ff2be241f600efd854b6eaa4f916d239a83067fc7;
                              snapshot-scryfall-2026-08-25.json
Held-out exclusion registry:  protocol §6.3 plus the cumulative incident
                              registry (Combust, Malignus, Lava Burst, Wild
                              Slash, and every other logged incident);
                              docs/gates/dark-entry-record.md §2
Development-export command:    python scripts/python/export_units.py drk
                              --mtg ./target/release/mtg-discover.exe
                              --exclude-heldout > docs/audits/drk/units-export.tsv
Development-export sha256:    4460c2de445161e8e67ac3bc88c668e23ca6f2645ebaf0a483ddd455de4e0a16
                              (equal to the aggregate-only verifier's expected TSV hash)
Blank pass copies (identical): docs/audits/drk/units-annotated-pass1.tsv and
                              units-annotated-pass2.tsv, both SHA-256
                              de150fc1a9cc0bebcfa78bf95b7ccda5de4835f587f76109ba7f0635d0ce63b7
Experiment manifest:          docs/manifests/experiment-dark-freeze-2026-08-27.json,
                              SHA-256 38d86700431ea64e9d3e96518124eef6151e82f25597134d24109e71ec2a20a9
Annotator pass 1:              claude-fable-5-pass1-2026-08-27 (candidate; Fable,
                              eligible — inspected no Dark row before the freeze;
                              attestation in docs/gates/dark-entry-record.md §3.1)
Annotator pass 2:              PENDING — a separate independent annotator that
                              cannot read pass 1, assigned by the research lead
Adjudicator:                   PENDING — a third identity that cannot read rows
                              before both passes seal, assigned by the research lead
```

The set-level pre-audit baseline (§13) must record, before any eligible card is
read: eligible development cards and cards with Oracle text; fallback and
held-out exclusions as aggregate counts only; printed and rules-supplied units;
top-level and child units; distinct and singleton templates; coverage
checkpoints where supported; kind, role, source, face, and parent/non-parent
histograms; the **corrected** protocol-defined multi-sentence count emitted
directly by the frozen binary (§4.4 of the protocol; the Legends 60/61
instrument discrepancy is already fixed in the binary and must not recur); and
the corpus-wide comparison totals required by S10/S11.

The baseline and export are immutable audit inputs. Any later segmenter change
is measured against them rather than silently replacing them.

## 4. Scope and partitions

### 4.1 Development partition

The development partition is every eligible Dark card with Oracle text that:

- belongs to the first-printing set `drk` under the repository's set
  definition;
- has `first_is_fallback = 0`;
- is not in the frozen held-out pool (protocol §6.3) or the incident registry;
- is not otherwise excluded by the protocol's set-type rules.

Aggregate set metadata reports 119 first-printing Dark cards and 116 with
Oracle text **before** held-out exclusion. The eligible non-held-out counts are
computed by the aggregate-only verifier at freeze (§13) and are **not** assumed
here. Review is exhaustive over every unit of this eligible non-held-out
partition; The Dark's 116-with-text size is well within the ≤ 400 exhaustive
threshold of protocol S4. "Exhaustive Dark audit" always means exhaustive over
this development partition, never over held-out records.

All faces and mechanically linked text objects sharing an Oracle identity stay
in one partition. No face of a held-out identity may enter development.

### 4.2 Regression / novelty partition

The regression and novelty corpus is the final committed state of all five
earlier audited sets:

- `docs/audits/lea/`;
- `docs/audits/leb/`;
- `docs/audits/arn/`;
- `docs/audits/atq/`;
- `docs/audits/leg/` (final adjudicated `units-annotated.tsv` and its frozen
  `units-export.tsv`).

Each set's export hash is recorded in §3 at freeze. No Dark-derived proposal
may be accepted without re-exporting and measuring all five regression sets
under protocol S10.

### 4.3 Held-out partition

The held-out partition remains protocol §6.3's frozen pool: Oracle text
present; `oracle_id` begins with hexadecimal `f`; `first_is_fallback = 0`; the
protocol's historical development-set exceptions (`lea`, `leb`, `arn`)
unchanged. The Dark is now a development set, so its own non-held-out cards are
development and its `oracle_id`-prefix-`f` non-fallback cards remain held-out
and are excluded from the development export.

Held-out cards may appear in aggregate corpus counts only. They may not appear
in an auditor-visible development export, candidate list, search result,
example, quotation, note, or annotation. The cumulative incident registry
(Combust, Malignus, Lava Burst, Wild Slash, and every other logged incident)
remains additively excluded from future held-out sampling. Any accidental
exposure is logged immediately and the exposed identity and all linked
faces/variants are excluded from later held-out sampling. No Gate 1 held-out
sample is selected in this preregistration.

## 5. Preregistered hypotheses

Every hypothesis is structural and tests the **frozen** classifier without
presuming its outcome. A falsified hypothesis becomes a reported finding; it
does not by itself authorize a classifier change. The list intentionally
mirrors the ten test areas required for The Dark; H1–H2 jointly cover the
boundary-fragmentation-and-topology area.

### H1 — Boundary fragmentation

**Claim:** The frozen delayed-trigger rules emit no parent that consists only
of a trigger condition, activation cost, or quoted-text fragment.

**Falsifier:** At least one emitted development unit is adjudicated `over`
because a delayed-trigger child left a condition-only, cost-only, or
quoted-text-fragment parent.

**Denominator:** All printed development units, with a separate count over all
nested delayed-trigger parent/child pairs.

### H2 — Delayed-trigger topology

**Claim:** Role and topology remain mutually consistent: every nested
`role = delayed_trigger` unit has a valid same-face parent; every top-level
spell-created `role = delayed_trigger` unit has no parent; quoted delayed text
remains `role = granted` rather than being promoted by the spell-created rule.

**Falsifier:** Any delayed-trigger unit has missing, cyclic, cross-face, or
wrong-class parentage, or any top-level / nested / granted class receives the
wrong role.

**Denominator:** All development units with `role = delayed_trigger` or
`role = granted`, reported separately for parented and parentless units.

### H3 — Keyword precision, including the quoted `bands with other` class

**Claim:** Every printed development unit the frozen classifier labels
`keyword_ability` is a keyword ability under the Comprehensive Rules, and
ordinary short static text is not mislabelled as a keyword.

**Falsifier:** At least one printed `keyword_ability` unit is adjudicated as
non-keyword (a precision failure).

**Denominator:** All printed development units labelled `keyword_ability`.

**Recall counterexample class (reported separately, descriptive):** complete
quoted `bands with other [quality]` abilities (CR 702.22b) that the frozen
classifier emits as `spell_or_static_text` rather than `keyword_ability`.
Legends adjudicated five such rows as kind defects **outside** the H3 precision
denominator; The Dark reports any Dark instances of this recall class without
presuming they exist and without treating them as H3 counterexamples.

### H4 — Prevention versus prohibition

**Claim:** Every development unit the frozen classifier labels
`prevention_effect` performs or establishes prevention under CR 615, and no
prohibition whose operative wording is `can't be prevented` / `cannot be
prevented` receives that kind.

**Falsifier:** Any `prevention_effect` row is a prohibition, a hidden trigger,
a spell instruction protected only by type-line context, or another
non-prevention structure; or any genuine positive prevention static is missed
solely because of the prohibition exclusion.

**Denominator:** All development `prevention_effect` units plus all non-quoted
development units containing a declared prevention/prohibition surface
candidate, with the candidate inventory produced only after the audit opens.

### H5 — Type-line-dependent spell/static classification

**Claim:** Top-level ordinary spell instructions on instant or sorcery faces do
not receive lexical `replacement_effect`, `prevention_effect`, or
`characteristic_defining_ability` kinds unless a CR 113.6 exception establishes
a genuine non-spell ability.

**Falsifier:** Any applicable development unit is misclassified because the
classifier ignored or misapplied its per-face type line.

**Denominator:** All top-level printed development units on instant or sorcery
faces, with multi-face records reported separately.

### H6 — Prefix classification

**Claim:** Every extracted `prefix` is a genuine structural label — an ability
word, flavor word, Saga chapter symbol, named mode, result label, or another
CR-supported label — and it does not hide the body's correct kind.

**Falsifier:** Any extracted prefix is ordinary classification-relevant prose;
any ability/flavor label still hides a trigger; or any Saga chapter is
classified from its effect verb rather than as a chapter trigger.

**Denominator:** All development units with a non-null `prefix`, reported by
prefix class, kind, role, and face type. If the denominator is zero it is
reported as such and the hypothesis is not evaluated.

### H7 — Quoted gained/lost ability boundaries

**Claim:** For every unit that grants, gains, loses, or refers to a complete
quoted ability, the frozen segmenter emits that quoted ability as a `granted`
child with a valid parent, and leaves no `under` parent that omits an emitted
reference unit and no `misattached` quoted child.

**Falsifier:** Any complete quoted gained/lost ability is left unemitted
(parent `under`, `missed ≥ 1`), emitted top-level (`misattached`), or given the
wrong role; or any non-ability quoted label is wrongly promoted to a `granted`
child.

**Denominator:** All development units containing quoted text that grants,
gains, loses, or refers to an ability, plus all emitted `role = granted`
children.

### H8 — Static-regeneration replacement classification

**Claim:** Every permanent static ability whose effect regenerates the
permanent each time it would be destroyed (CR 614.8 / 701.19b) is a
`replacement_effect` under the reference-unit definition, and the frozen
classifier's label for each such unit is recorded as correct or defect
accordingly.

**Falsifier (measurement):** The frozen classifier labels a static-regeneration
replacement unit `spell_or_static_text` (or any non-`replacement_effect` kind),
producing a kind defect; or a one-shot spell/activated regeneration instruction
is wrongly labelled `replacement_effect`.

**Denominator:** All development units whose surface form is a static
regeneration statement, plus all development `replacement_effect` units.

### H9 — Novelty relative to all five earlier audited sets

**Claim (falsifiable, non-numeric):** The Dark's unit novelty against the
pooled `lea`+`leb`+`arn`+`atq`+`leg` exports is strictly below 1.0 — at least
one Dark printed unit reuses a template that appears in an earlier audited
export.

**Falsifier:** Every Dark printed unit's template is absent from all five
earlier audited exports (unit novelty = 1.0).

**Denominator:** All printed development units. Template novelty is reported
separately over distinct printed development templates.

**Comparison point (bounded observation, not a preregistered pass/fail):** the
single-step change relative to the most recent audited set is reported
descriptively; no specific earlier set's novelty value is imported as a Dark
prediction, and no monotonic-novelty claim is made.

### H10 — Independent exact-row agreement

**Claim:** Before discussion, the two complete annotation passes agree on at
least 95% of aligned unit records across all eight preregistered judgement
fields.

**Falsifier:** Exact judgement-record agreement is below 0.95.

**Denominator:** Frozen export rows present in both passes; agreement requires
identical `boundary`, `missed`, `kind_expected`, `kind_ok`, `role_ok`,
`source_ok`, `context`, and `disposition` values. CR citations, structure tags,
and notes receive separate overlap/disagreement reporting rather than being
folded into this rate.

### H11 — D14 / D19 non-equivalence

**Claim:** Sentence adjacency and an unscoped later trigger word are not, by
themselves, sufficient to distinguish a D19 effect-created delayed trigger from
a D14 independent trigger sharing a paragraph.

**Falsifier:** After independent adjudication of every in-scope candidate, a
single generic adjacency-only surface rule separates all positive and negative
Dark cases with no ambiguity or counterexample.

**Denominator:** All adjudicated D14/D19 attachment candidates identified in the
Dark development audit plus the later non-held-out S8 comparison set.

## 6. Predeclared measurements and denominators

The findings report must publish every applicable protocol §4.5 measure with
numerator and denominator, plus the additions below. Values are computed by
`scripts/python/audit_metrics.py` against the frozen export and the five earlier
exports.

| Measurement | Numerator | Denominator |
|---|---|---|
| Boundary precision | Printed units with `boundary = ok` | Printed units judged, excluding `unsure` |
| Missed boundaries | Sum of `missed` | Count only |
| Boundary recall | Printed units with `boundary = ok` | `ok` units plus sum of `missed` |
| Kind accuracy | `kind_ok = yes` | Boundary-ok units with `kind_ok` in `{yes,no}` |
| Role accuracy | `role_ok = yes` | Boundary-ok units with `role_ok` in `{yes,no}` |
| Source accuracy | `source_ok = yes` | All units judged |
| Structural exact-card correctness | Eligible cards with no boundary error/unsure and all applicable kind/role/source judgements correct | Eligible development cards judged |
| Inter-annotator row agreement | Rows with identical preregistered judgement fields | Frozen export rows in both independent passes |
| Inter-annotator exact-card agreement | Cards whose rows agree on every preregistered judgement field | Eligible cards in both passes |
| Unsupported structures | Rows dispositioned `unsupported`, by `gap:<class>` | All units |
| Ambiguous structures | Rows dispositioned `ambiguous`, by competing reading | All units |
| Unresolved adjudication | Rows still `adjudicate` after adjudication | All units |
| Context distribution | Rows by `none`/`cr`/`type_line`/`game_state`/`card_specific` | All units |
| Normalization fragmentation | Rows tagged `fragmentation:*`, by tag | Printed units |
| Suspected collisions | Rows tagged `collision:*`, by tag | Printed units |
| Unit novelty | Printed units whose template occurs in no earlier audited export | Printed units |
| Template novelty | Distinct printed templates absent from all earlier audited exports | Distinct printed templates |
| Multi-sentence frequency | Printed rows tagged `multi_sentence` (corrected instrument) | Printed units |
| Kind/role/source histograms | Units in each value | All units, printed/rules-supplied split |
| Structure-tag prevalence | Rows carrying each frozen tag | All units and printed units |
| Delayed-trigger topology | Correct rows in each top-level/nested/granted class | All rows in that class |
| Quoted gained/lost outcomes (H7) | Correctly emitted `granted` children | All quoted-ability-bearing units |
| Static-regeneration outcomes (H8) | Correctly classified static-regeneration units | All static-regeneration units |
| D14/D19 attachment outcomes | Positive, negative, ambiguous, unsupported | All attachment candidates; descriptive only |
| Held-out exposure | Accidentally exposed identities | Expected zero; no accuracy rate |

Template coverage and singleton counts are descriptive measurements, not
correctness metrics. No semantic coverage percentage may be inferred from them.

## 7. Independent annotation and adjudication workflow

### 7.1 Preparation

1. Confirm Legends is closed and committed (done: `docs/findings/
   leg-structural-audit.md`).
2. Freeze the repository, data/rules snapshots, protocol, annotation guide,
   preregistration, five earlier exports, and held-out exclusion registry.
3. Generate a deterministic development export that excludes held-out
   identities before any row becomes visible to an auditor.
4. Verify the export schema and aggregate counts without printing or opening
   individual rows.
5. Hash the export and clone it into two identical blank annotation-pass files.

### 7.2 Independent passes

- Two annotators review the complete development export independently.
- Neither annotator reads the other's annotations, notes, row-level hypotheses,
  or candidate proposal list before both passes are sealed.
- Each annotator reads the full eligible card text and type line only after the
  baseline/export freeze, and consults CR/rulings as protocol S5 requires.
- Every row receives an explicit disposition. `unsure`, `unsupported`, and
  `ambiguous` are used rather than guesses.
- No implementation proposal is discussed during either pass; no P-LEG proposal
  is implemented.
- Each sealed pass receives a content hash and timestamp.

### 7.3 Comparison

After both passes are sealed: align rows only by the frozen stable unit identity
`(oracle_id, face, index)`; calculate the preregistered row-level and
exact-card agreement measures; publish confusion counts for boundary, kind,
role, source, context, and disposition; list every disagreement without
resolving it automatically; report missing or duplicate keys as export defects,
not annotation disagreement.

### 7.4 Adjudication

- The adjudicator reviews every disagreement, every non-`accept` row, every
  `unsure`/`unsupported`/`ambiguous` row, and every alleged card-specific
  dependency, and may open the two sealed passes and the agreement report only
  after both passes are sealed.
- CR and Oracle text control; official rulings clarify but do not override the
  CR.
- A genuine rules ambiguity remains `ambiguous` with both readings; a
  vocabulary gap remains `unsupported` with `kind_expected = gap:<class>`.
- The final adjudicated file preserves original pass IDs and the adjudication
  rationale.
- The audit is not called adjudicated until no row remains merely `adjudicate`
  without an explicit documented reason.

## 8. Unsupported and ambiguous dispositions

Use the frozen protocol/guide meanings without broadening them: `unsupported`
(the structural vocabulary cannot express the reference kind/role/source/span/
attachment — record `gap:<class>`, evidence, CR references, and the smallest
missing distinction; do not force into `spell_or_static_text`); `ambiguous`
(CR and authoritative rulings do not determine one reading — record every live
reading; exclude from unique-answer accuracy denominators); `adjudicate`
(temporary annotator uncertainty/disagreement, resolved by the second pass or
adjudicator); `unsure` (field-level, reported separately, never a guess).
Card-specific context may justify an annotation when an official ruling is
needed, but no accepted heuristic may contain a card name, set code, Oracle ID,
or per-card branch.

## 9. Observation / proposal / implementation separation

The audit proceeds in four sealed layers exactly as in protocol S8–S12 and
guide §9: observation (frozen segmenter output plus independent human
judgement, each claim with numerator, denominator, authority, and
counterexamples); adjudication (resolve disagreements or preserve
unsupported/ambiguous outcomes, no code changes); proposal (only after the
adjudicated report — a generic surface rule, CR class, defect rows, predicted
corpus effect, and falsifiers); acceptance (only after separate S8–S12
evidence, implementation, tests, corpus measurement, and regression review).

Rules: a frequency or template match is a candidate measurement, not ground
truth; a structural label is not a semantic operator; a proposal cannot
retroactively alter a preregistered hypothesis or denominator; observations that
do not support a generic rule remain findings; one-off or card-specific
behaviour is not repaired with a card branch; D14 and D19 candidates stay
observational until their separate research design is executed; **P-LEG-1,
P-LEG-2, and P-LEG-3 are not implemented or accepted by this audit**; the Dark
findings document may propose changes but may not implement them.

## 10. D14, D19, and delayed-trigger representation

D14 and D19 remain separate questions, handled exactly as in the frozen guide
§12 and `docs/findings/d19-attachment-research-design.md`
(SHA-256 `a78e201003a380c395d6ddb620a3d3cab8d1b4f2e25ec118eb8a6139634a3fc8`):

- **D19:** a later unscoped trigger-word sentence may be a delayed triggered
  ability created by resolution of preceding effect text, and therefore a child
  of that creating ability.
- **D14:** a later trigger-word sentence may instead be an independent printed
  triggered ability sharing an Oracle paragraph, and therefore a sibling or
  top-level reference unit.

Adjacency alone settles neither. During Dark annotation, candidate rows may be
tagged with the design's `P1–P5` / `N1–N8` / `A1–A5` classes for later
comparison, but **no D14/D19 classifier rule and no adjacency-only rule is
introduced**. P-ATQ-1 (in-sentence comma/colon fragments) remains a distinct,
already-decided question. The accepted topology distinction remains frozen:
top-level spell-created delayed trigger (`role = delayed_trigger`, no parent);
nested effect-created delayed trigger (`role = delayed_trigger`, valid parent);
quoted/granted triggered text (`role = granted`); ordinary independent
triggered ability (`role = ability`).

## 11. Stop conditions and escalation

### 11.1 Stop before opening the audit

Do not generate an auditor-visible Dark export or inspect any eligible row if:

- Legends is not closed and committed, or `docs/current-state.md` and the
  Legends closure disagree about the live baseline;
- the release build or tests fail at the intended measurement-freeze commit;
- source snapshots, CR identity, earlier export hashes, or the preregistration
  hash are missing;
- the development export cannot be proven to exclude every held-out identity;
- stable unit keys are duplicated, missing, nondeterministic, or inconsistent
  across repeated exports;
- an intended annotator has already inspected eligible Dark text in a
  heuristic-design context before the baseline freeze.

Escalate the smallest issue to the responsible technical owner, research lead,
or program owner. Do not work around a failed entry condition by reading cards
manually.

### 11.2 Pause during annotation

Pause and record a governance issue if: a held-out record appears in any
auditor-visible output; export drift occurs without an accepted change record;
the schema cannot preserve an observed parent/child span or attachment;
instructions would require changing the annotation guide after one pass has seen
the affected rows; a semantic or behavioural decision is being used to force a
structural label; or the two annotators discover they were not independent. An
exposed held-out identity is logged and excluded. A compromised pass is not
silently retained; the research lead decides whether to restart with a new
independent annotator or preserve the limitation explicitly.

### 11.3 Escalate after adjudication

A classifier proposal is eligible only when at least one adjudicated defect row
demonstrates a structural failure; the proposed rule is generic and
CR-grounded; positive, negative, and ambiguous classes can be stated without
card names; a future S8 search can inventory the rule and nearest non-matches
while excluding held-out records; the expected boundary, parentage, kind, role,
and corpus effect are declared; and failure can remain explicit if surface
evidence is insufficient. If D14 and D19 cannot be separated reliably from
surface form plus permitted context, no classifier proposal is made.

## 12. Entry checklist

The audit opens only when every item is checked in writing in
`docs/gates/dark-entry-record.md`:

- [ ] Legends is adjudicated, closed, and committed, and its final artifact
      hashes match its findings report.
- [ ] `docs/current-state.md` reflects the accepted live baseline.
- [ ] The measurement-freeze commit, data snapshots, CR, protocol, guide,
      preregistration, and five earlier export hashes are recorded.
- [ ] Build and tests pass at the measurement-freeze commit.
- [ ] A held-out-safe deterministic development export exists and has been
      verified by aggregate counts only.
- [ ] The cumulative held-out exclusion registry, including the four named
      incident exclusions, is bound to the audit.
- [ ] Both independent annotators and the adjudicator are assigned.
- [ ] Neither annotator has inspected eligible Dark text before the freeze.
- [ ] `docs/findings/drk-structural-audit.md` remains an empty outline until the
      baseline block is written verbatim.
- [ ] The program owner authorizes the audit to begin.

## 13. Input-freeze procedure (Phase 3, aggregate-only)

Execute after this preregistration and the guide decision set are frozen, with
no row-bearing output displayed:

1. `cargo build --release`; record the version.
2. Recompute corpus, rules, snapshot, tool, protocol, guide, five earlier-export,
   and held-out-registry identities (§3).
3. Run the aggregate-only held-out-safe verifier:
   `python scripts/python/verify_export_safety.py drk --mtg
   .\target\release\mtg-discover.exe --runs 2`. Require: held-out exclusion in
   SQLite before segmentation/serialization; byte-identical repeated JSON and
   TSV exports; unique `(oracle_id, face, unit_index)` keys; valid parent
   integrity; zero held-out export records; no row displayed.
4. Retain `docs/audits/drk/units-export.tsv` (its SHA-256 must equal the
   verifier's expected TSV hash), two identical blank annotation-pass copies,
   and a Dark experiment manifest under `docs/manifests/`.
5. Hash every retained input and populate §3 and the pre-audit baseline from
   actual command output — never from expected Legends values — including the
   corrected multi-sentence count emitted directly by the binary.

## 14. Empty findings outline

The empty, non-empirical outline is stored at
`docs/findings/drk-structural-audit.md`. No empirical result may be added to it
until the §12 entry checklist passes and opening is authorized.
