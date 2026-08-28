# Fallen Empires (`fem`) structural-audit preregistration

- Date prepared: 2026-08-28
- Status: **preregistered research design; audit not started**
- Protocol: `docs/protocol/structural-investigation-protocol.md` v1.0
  (SHA-256 `1bc05d357b24006a2eecc692f9bed5b86d1d828f116c2d741fb75662df4913bf`)
- Annotation guide: frozen v1.0
  `docs/protocol/structural-annotation-guide-v1.0.md`
  (SHA-256 `d31dee0a3b06494bd7ba0238be65b330e2366edb1b8bcf4e5e6a6f865de5d84b`),
  incorporated by reference and reused unchanged for Fallen Empires
- Immediate methodological precedent: the closed The Dark (`drk`) audit
  (`docs/findings/drk-structural-audit.md`). Its **procedure** governs this
  design; its **measurements and set-specific conclusions are not carried into
  Fallen Empires** and appear here only where a prior observation is explicitly
  labelled as a comparison point, never as a Fallen Empires result.
- Permitted activation: only after this preregistration and the guide decision
  set are frozen, the input freeze (§13) completes, and every §12 entry
  condition reads satisfied in writing.

## 0. Governance incident disclosed before this preregistration was written

During aggregate baseline preparation on 2026-08-28, `mtg-discover audit
novelty fem --earlier lea --earlier leb --earlier arn --earlier atq --earlier
leg --earlier drk` was run and its full output (including
`novel_template_records`, which lists normalized templates and representative
card names) was displayed. That command does not support `--exclude-heldout`
and ran over the complete 102-card `fem` first-printing population, not the
93-card held-out-safe development partition. Cross-referencing the displayed
names against the held-out-safe export identified **seven held-out
identities** whose names and templates were exposed: Farrel's Mantle, Fungal
Bloom, Orgg, Spore Flower, Svyelunite Priest, Thelon's Chant, Vodalian War
Machine. This is logged as a held-out exposure incident in
`docs/gates/fem-entry-record.md` §2, the seven identities are added to the
cumulative held-out incident-exclusion registry, and the preparer
(Claude Code, this session) is disqualified from any Fallen Empires annotator
or adjudicator role. All aggregate figures in this preregistration were
recomputed afterward from the held-out-safe 93-card export only. No other
eligible development-partition text was read, quoted, or annotated beyond the
aggregate counts and the accidentally displayed novelty records above.

## 1. Purpose and non-observation statement

This document freezes the research design for the Fallen Empires structural
audit before an auditor inspects any eligible Fallen Empires card, subject to
the §0 disclosure above. Preparation of this preregistration used protocol
text, the frozen annotation guide, the committed `lea`/`leb`/`arn`/`atq`/
`leg`/`drk` audit artifacts, source/test contracts, and aggregate-only set
metadata (`mtg-discover sets`, `mtg-discover audit summary --exclude-heldout`,
`mtg-discover audit signals --exclude-heldout`, and a held-out-safe local
template-overlap computation) only. Beyond the §0 incident, it did not query,
print, quote, inspect, segment, or annotate an individual Fallen Empires
development card, and it did not intentionally inspect any held-out identity.
It contains no Fallen Empires findings.

The audit remains closed until every entry condition in §12 is satisfied.

## 2. Objective

Determine how well the **frozen** accepted structural measurement baseline
(the segmenter and normalizer at the measurement-freeze commit, unchanged
since The Dark's closure — no P-LEG or P-DRK proposal implemented) identifies
reference units, kinds, roles, sources, and parent/child attachment in the
next eligible first-printing development set, Fallen Empires, while measuring
structural novelty against the finalized Alpha, Beta, Arabian Nights,
Antiquities, Legends, and The Dark exports.

The investigation will:

- perform an exhaustive review of the eligible non-held-out Fallen Empires
  development partition under protocol S4;
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

**The frozen segmenter is measured first.** P-DRK-1 (over-inclusive
sentence-initial `When` child span) is a Dark research proposal only. It is
not implemented before this audit's baseline freeze, and this audit does not
turn it into accepted behaviour. Where Fallen Empires reproduces a
Dark-observed defect class, that reproduction is the measurement, recorded
against the frozen classifier. Fallen Empires also carries a large,
counter-heavy vocabulary (storage/spore/tide/net/credit/javelin/cube
counters) that the earlier five sets sampled only lightly; the audit records
whether the frozen classifier's counter-related surface forms (activated
sacrifice-a-counter abilities, upkeep triggers that place or remove a
counter, "as long as" counter-conditioned statics) are handled consistently,
without presuming a new proposal is needed.

## 3. Frozen inputs to record after the measurement freeze

The following block is populated in Phase 3 (§13) from live command output and
file hashes at the Fallen Empires measurement-freeze commit. No value may be
copied from this preregistration, from The Dark documents, or from an older
findings document merely because it is expected to remain unchanged.

Populated 2026-08-28 from live command output at the measurement-freeze
commit.

```text
The Dark closure record:      docs/findings/drk-structural-audit.md; adjudicated
                              and closed 2026-08-28; final annotation drift 0;
                              163 keys; dispositions accept 159 / defect 3 /
                              ambiguous 1; zero adjudicate/unsupported
Repository / measurement-freeze commit:   2823b1226c7d10bdb1d47d41a17cfeda709d4ecb
Repository status:            clean at the freeze commit; this preregistration,
                              the empty outline, the entry record, and the
                              Phase 3 frozen-input artifacts are pending on top
                              of it (docs/manifests/experiment-fem-freeze-2026-08-28.json)
Protocol version and sha256:   structural-investigation-protocol.md v1.0;
                              1bc05d357b24006a2eecc692f9bed5b86d1d828f116c2d741fb75662df4913bf
Preregistration sha256:        pre-population frozen control text: populated in the
                              same working pass; see the entry record for the
                              content hash of this file as committed
Annotation-guide version/hash: frozen v1.0;
                              d31dee0a3b06494bd7ba0238be65b330e2366edb1b8bcf4e5e6a6f865de5d84b
mtg-discover version/build:    0.1.0; cargo build --release passed (cached, no source change)
cargo test:                    89 passed, 0 failed; fmt and clippy clean;
                              Python unittest (scripts/python test_*.py) 47 passed, 0 failed
Scryfall oracle snapshot:      oracle-cards.jsonl.gz; 2026-08-25; unchanged;
                              9611b5d93b20478a0ee46bae8b20a9eb39ee980f0ef4f5f6f6aaa8f7ab010ab2
Scryfall rulings snapshot:     rulings.jsonl.gz; 2026-08-25; unchanged;
                              3064689880a73f804f6e20411f6896d26aec06286eb4f2eb23d26e53779efe6f
Scryfall default snapshot:     default-cards.jsonl.gz; 2026-08-25; unchanged;
                              d65608b4993aeb2bd31ef8dfb41f6a9aa37396720d0a61d1354f528d8909667e
cards.sqlite identity:         unchanged; d1c88cb9ab96531c2f2ce8f3b048c727240811e1f16acb141adbdb60998195c4
Comprehensive Rules:          effective 2026-08-07; unchanged;
                              dc01ca5462085d6e3f7e85f548960a017522d1d851ac6a11d26ae14b6610c072
Corpus metadata:              38,626 cards; 37,916 with Oracle text; 710 without;
                              3,212 multi-face; 553 first-printing sets;
                              78,949 rulings; 3,455 numbered rules; 752 glossary entries
Set aggregate metadata:       102 first-printing Fallen Empires cards, 101 with
                              text, 0 fallback (mtg-discover sets); 93 eligible
                              development cards / 92 with text after held-out
                              exclusion; 176 printed units; 0 rules-supplied;
                              170 top-level / 6 child; face 0 = 176;
                              prefix non-null/null = 0/176; 122 templates;
                              103 singleton; 19 multi-sentence (frozen binary
                              instrument, mtg-discover audit summary
                              --exclude-heldout); kinds activated_ability 83,
                              additional_cost 2, cast_restriction 1,
                              keyword_ability 12, prevention_effect 1,
                              replacement_effect 15, spell_or_static_text 28,
                              triggered_ability 34; roles ability 170,
                              delayed_trigger 4, mode 2; source printed 176;
                              signal histogram (mtg-discover audit signals
                              --exclude-heldout, aggregate only):
                              activation_restriction_embedded_candidate 3,
                              residual_multi_sentence_unit 2
Earlier audited sets:         lea export aabc1bd58ce38a0e73c9c9ce23344a124482dd02f36c470129aecd9bf609f3bf,
                              annotation cdd2438d5b44678200fe145dde49df7fc833974b8e19035d8391fa7002ecff91;
                              leb export 4cb90170876be1d945d79915780ab831b452f3878c79515f66d40e06ca2c6c05,
                              annotation 6a3a056385ff1618f9243dbdd9ed1cd673dedb8179711c45c65fe7117479494a;
                              arn export 4827f5be9305b4a6e84978395a2c80566565ebf1bfc0943ff705099ff68f327d,
                              annotation cfd31206e731061674ccce542c63198083c1ef0b80baebf38a8797e84e1656eb;
                              atq export 8ec1047b3443845ca61c63a0a5cc8c444ae7f6c693a9c357e88bf0d0b244143d,
                              annotation f01c9f84be527efbe0f9252190f7040219628deb78b277687262ef8f265eb8f4;
                              leg export c39a2d695b94ce33a2e16356dd93bc6dc614b7c83becfb2b2f72ad5cb298d2e3,
                              annotation 678fcb58ac0e6b50d213493ef2a477162c9c698bb6c4b942313c273c177cb6cc;
                              drk export 4460c2de445161e8e67ac3bc88c668e23ca6f2645ebaf0a483ddd455de4e0a16,
                              annotation aed8ab6309f7ad679c1a5e455c1d9a2d6567c7ad83121175c334480b80d25730
Current corpus baseline:       71,563 printed units; 970 rules-supplied;
                              37,299 templates; coverage top 10/25/50/100/1,000/5,000 =
                              14.17/20.03/23.41/26.90/42.18/54.87%;
                              kinds activated 11,998, additional cost 319, ante 9,
                              cast restriction 69, CDA 255, keyword 17,840,
                              prevention 166, replacement 2,174, static/spell 19,519,
                              triggered 19,214; roles ability 67,045, delayed 891,
                              granted 1,506, mode 2,121
Held-out-safe novelty vs six earlier sets: total printed units 176; units seen
                              earlier 51; novel units 125 (71.02%); distinct
                              templates 122; templates seen earlier 22; novel
                              templates 100 (81.97%). Computed by direct
                              template-overlap comparison over
                              docs/audits/fem/units-export.tsv against the six
                              earlier held-out-safe exports listed above
                              (aggregate counts only; the `mtg-discover audit
                              novelty` CLI command does not support
                              --exclude-heldout and is not used for this figure
                              — see §0)
Held-out pool definition:      protocol §6.3; 2,096 identities;
                              377e12bdf80e0263c361f48ff2be241f600efd854b6eaa4f916d239a83067fc7;
                              snapshot-scryfall-2026-08-25.json
Held-out exclusion registry:  protocol §6.3 plus the cumulative incident
                              registry (Combust, Malignus, Lava Burst, Wild
                              Slash, and — newly added by the §0 incident —
                              Farrel's Mantle, Fungal Bloom, Orgg, Spore Flower,
                              Svyelunite Priest, Thelon's Chant, Vodalian War
                              Machine); docs/gates/fem-entry-record.md §2
Development-export command:    python scripts/python/export_units.py fem
                              --mtg ./target/release/mtg-discover.exe
                              --exclude-heldout > docs/audits/fem/units-export.tsv
Development-export sha256:    095a25a7a0729bca12d515b2ce0a7395c0484d1fc335d11a913dec8c6c3b0d74
                              (equal to the aggregate-only verifier's expected TSV hash)
Blank pass copies (identical): docs/audits/fem/units-annotated-pass1.tsv and
                              units-annotated-pass2.tsv, both SHA-256
                              691f8b8c4f54fcbcafacc716ece331ff348e32d8ff0c5f40d5bfe79f1fe92c25
Experiment manifest:          docs/manifests/experiment-fem-freeze-2026-08-28.json
Annotator pass 1:              PENDING — assigned by the research lead; must not
                              be the preparer of this preregistration (disqualified
                              by the §0 incident)
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
histograms; the multi-sentence count emitted directly by the frozen binary
(§4.4 of the protocol); and the corpus-wide comparison totals required by
S10/S11.

The baseline and export are immutable audit inputs. Any later segmenter change
is measured against them rather than silently replacing them.

## 4. Scope and partitions

### 4.1 Development partition

The development partition is every eligible Fallen Empires card with Oracle
text that:

- belongs to the first-printing set `fem` under the repository's set
  definition;
- has `first_is_fallback = 0`;
- is not in the frozen held-out pool (protocol §6.3), the pre-existing
  incident registry, or the seven identities newly added by the §0 incident;
- is not otherwise excluded by the protocol's set-type rules.

Aggregate set metadata reports 102 first-printing Fallen Empires cards and 101
with Oracle text **before** held-out exclusion. The eligible non-held-out
counts (93 cards, 92 with text, 176 printed units) were computed by the
aggregate-only verifier at freeze (§13). Review is exhaustive over every unit
of this eligible non-held-out partition; Fallen Empires's 92-with-text size is
well within the ≤ 400 exhaustive threshold of protocol S4. "Exhaustive Fallen
Empires audit" always means exhaustive over this development partition, never
over held-out records.

All faces and mechanically linked text objects sharing an Oracle identity stay
in one partition. No face of a held-out identity may enter development.

### 4.2 Regression / novelty partition

The regression and novelty corpus is the final committed state of all six
earlier audited sets:

- `docs/audits/lea/`;
- `docs/audits/leb/`;
- `docs/audits/arn/`;
- `docs/audits/atq/`;
- `docs/audits/leg/`;
- `docs/audits/drk/` (final adjudicated `units-annotated.tsv` and its frozen
  `units-export.tsv`).

Each set's export hash is recorded in §3 at freeze. No Fallen Empires-derived
proposal may be accepted without re-exporting and measuring all six regression
sets under protocol S10.

### 4.3 Held-out partition

The held-out partition remains protocol §6.3's frozen pool: Oracle text
present; `oracle_id` begins with hexadecimal `f`; `first_is_fallback = 0`; the
protocol's historical development-set exceptions (`lea`, `leb`, `arn`)
unchanged. Fallen Empires is now a development set, so its own non-held-out
cards are development and its `oracle_id`-prefix-`f` non-fallback cards remain
held-out and are excluded from the development export.

Held-out cards may appear in aggregate corpus counts only. They may not appear
in an auditor-visible development export, candidate list, search result,
example, quotation, note, or annotation. The cumulative incident registry
(Combust, Malignus, Lava Burst, Wild Slash, Farrel's Mantle, Fungal Bloom,
Orgg, Spore Flower, Svyelunite Priest, Thelon's Chant, Vodalian War Machine,
and every other logged incident) remains additively excluded from future
held-out sampling. Any accidental exposure is logged immediately and the
exposed identity and all linked faces/variants are excluded from later
held-out sampling. No Gate 1 held-out sample is selected in this
preregistration.

## 5. Preregistered hypotheses

Every hypothesis is structural and tests the **frozen** classifier without
presuming its outcome. A falsified hypothesis becomes a reported finding; it
does not by itself authorize a classifier change. The list carries forward The
Dark's ten structural test areas unchanged (H1–H9, H11) and adds H12 for
Fallen Empires's counter-heavy vocabulary; H10 (independent exact-row
agreement) is retained as the inter-annotator reliability check.

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

### H3 — Keyword precision

**Claim:** Every printed development unit the frozen classifier labels
`keyword_ability` is a keyword ability under the Comprehensive Rules, and
ordinary short static text is not mislabelled as a keyword. Fallen Empires
introduces banding-adjacent surface forms ("gain banding," "gains banding
until end of turn," "creatures banded with it") that are reference points for
this hypothesis but not presumed counterexamples.

**Falsifier:** At least one printed `keyword_ability` unit is adjudicated as
non-keyword (a precision failure).

**Denominator:** All printed development units labelled `keyword_ability`.

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

**Claim:** Top-level ordinary spell instructions on instant or sorcery faces
do not receive lexical `replacement_effect`, `prevention_effect`, or
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
prefix class, kind, role, and face type. The aggregate freeze count is 0; if
the denominator remains zero after the audit opens it is reported as such and
the hypothesis is not evaluated.

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
accordingly. Fallen Empires is expected to exercise this hypothesis more than
earlier sets did, given its regenerating-creature theme; the claim makes no
prediction about the outcome.

**Falsifier (measurement):** The frozen classifier labels a static-regeneration
replacement unit `spell_or_static_text` (or any non-`replacement_effect` kind),
producing a kind defect; or a one-shot spell/activated regeneration instruction
is wrongly labelled `replacement_effect`.

**Denominator:** All development units whose surface form is a static
regeneration statement, plus all development `replacement_effect` units.

### H9 — Novelty relative to all six earlier audited sets

**Claim (falsifiable, non-numeric):** Fallen Empires's unit novelty against the
pooled `lea`+`leb`+`arn`+`atq`+`leg`+`drk` exports is strictly below 1.0 — at
least one Fallen Empires printed unit reuses a template that appears in an
earlier audited export.

**Falsifier:** Every Fallen Empires printed unit's template is absent from all
six earlier audited exports (unit novelty = 1.0).

**Denominator:** All printed development units. Template novelty is reported
separately over distinct printed development templates.

**Comparison point (bounded observation, not a preregistered pass/fail):** the
held-out-safe aggregate freeze figure (unit novelty 71.02%, template novelty
81.97%, §3) is reported descriptively; no monotonic-novelty claim is made
against The Dark's 0.7791 unit-novelty figure.

### H10 — Independent exact-row agreement

**Claim:** Before discussion, the two complete annotation passes agree on at
least 95% of aligned unit records across all eight preregistered judgement
fields.

**Falsifier:** Exact judgement-record agreement is below 0.95.

**Denominator:** Frozen export rows present in both passes; agreement requires
identical `boundary`, `missed`, `kind_expected`, `kind_ok`, `role_ok`,
`source_ok`, `context`, and `disposition` values. CR citations, structure tags,
and notes receive separate overlap/disagreement reporting rather than being
folded into this rate. The Dark's H10 was falsified (0.8650) on a single
recurring `context` convention (plain instant/sorcery spell text); the guide's
open C6 question (opening handoff line 30) remains unresolved going into this
audit, so a recurrence of the same disagreement class is not treated as a
guide-implementation deviation.

### H11 — D14 / D19 non-equivalence

**Claim:** Sentence adjacency and an unscoped later trigger word are not, by
themselves, sufficient to distinguish a D19 effect-created delayed trigger from
a D14 independent trigger sharing a paragraph.

**Falsifier:** After independent adjudication of every in-scope candidate, a
single generic adjacency-only surface rule separates all positive and negative
Fallen Empires cases with no ambiguity or counterexample.

**Denominator:** All adjudicated D14/D19 attachment candidates identified in
the Fallen Empires development audit plus the later non-held-out S8 comparison
set.

### H12 — Counter-vocabulary classification consistency

**Claim:** Fallen Empires's counter-management surface forms are classified
consistently with the frozen rules already exercised by earlier sets: an
upkeep trigger that places or removes a counter is `triggered_ability`; a cost
that sacrifices a permanent or removes counters to produce an effect is
`activated_ability`; a static "as long as ~ has/there are N counters" clause is
`spell_or_static_text` (CDA only if it defines power/toughness per the
existing `~'s power and toughness are each equal to` form); and an "enters
with N counters" clause attaches to the permanent's own printed unit rather
than being emitted as a spurious child.

**Falsifier:** Any counter-related unit receives a kind, role, or attachment
inconsistent with the claim above, or the frozen classifier is internally
inconsistent across two structurally identical counter-management surface
forms.

**Denominator:** All development units whose normalized text contains a
counter-type token (storage, spore, tide, net, credit, javelin, cube, or a
generic `+N/+N counter` / `-N/-N counter` reference), reported by counter type
and by kind/role/attachment outcome.

## 6. Predeclared measurements and denominators

The findings report must publish every applicable protocol §4.5 measure with
numerator and denominator, plus the additions below (including H12's counter
outcome table). Values are computed by `scripts/python/audit_metrics.py`
against the frozen export and the six earlier exports.

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
| Multi-sentence frequency | Printed rows tagged `multi_sentence` | Printed units |
| Kind/role/source histograms | Units in each value | All units, printed/rules-supplied split |
| Structure-tag prevalence | Rows carrying each frozen tag | All units and printed units |
| Delayed-trigger topology | Correct rows in each top-level/nested/granted class | All rows in that class |
| Quoted gained/lost outcomes (H7) | Correctly emitted `granted` children | All quoted-ability-bearing units |
| Static-regeneration outcomes (H8) | Correctly classified static-regeneration units | All static-regeneration units |
| Counter-vocabulary outcomes (H12) | Correctly classified counter-related units | All counter-related units, by counter type |
| D14/D19 attachment outcomes | Positive, negative, ambiguous, unsupported | All attachment candidates; descriptive only |
| Held-out exposure | Accidentally exposed identities | The §0 incident is the only known exposure; expected zero further exposure |

Template coverage and singleton counts are descriptive measurements, not
correctness metrics. No semantic coverage percentage may be inferred from them.

## 7. Independent annotation and adjudication workflow

### 7.1 Preparation

1. Confirm The Dark is closed and committed (done: `docs/findings/
   drk-structural-audit.md`).
2. Freeze the repository, data/rules snapshots, protocol, annotation guide,
   preregistration, six earlier exports, and held-out exclusion registry.
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
- No implementation proposal is discussed during either pass; no P-DRK proposal
  is implemented.
- Each sealed pass receives a content hash and timestamp.
- Neither annotator may be the preparer of this preregistration, per the §0
  disqualification.

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
retroactively alter a preregistered hypothesis or denominator; observations
that do not support a generic rule remain findings; one-off or card-specific
behaviour is not repaired with a card branch; D14 and D19 candidates stay
observational until their separate research design is executed; **P-DRK-1 is
not implemented or accepted by this audit**; the Fallen Empires findings
document may propose changes but may not implement them.

## 10. D14, D19, and delayed-trigger representation

D14 and D19 remain separate questions, handled exactly as in the frozen guide
§12 and `docs/findings/d19-attachment-research-design.md`:

- **D19:** a later unscoped trigger-word sentence may be a delayed triggered
  ability created by resolution of preceding effect text, and therefore a child
  of that creating ability.
- **D14:** a later trigger-word sentence may instead be an independent printed
  triggered ability sharing an Oracle paragraph, and therefore a sibling or
  top-level reference unit.

Adjacency alone settles neither. During Fallen Empires annotation, candidate
rows may be tagged with the design's `P1–P5` / `N1–N8` / `A1–A5` classes for
later comparison, but **no D14/D19 classifier rule and no adjacency-only rule
is introduced**. P-ATQ-1 (in-sentence comma/colon fragments) remains a
distinct, already-decided question. The accepted topology distinction remains
frozen: top-level spell-created delayed trigger (`role = delayed_trigger`, no
parent); nested effect-created delayed trigger (`role = delayed_trigger`,
valid parent); quoted/granted triggered text (`role = granted`); ordinary
independent triggered ability (`role = ability`).

## 11. Stop conditions and escalation

### 11.1 Stop before opening the audit

Do not generate an auditor-visible Fallen Empires export or inspect any
eligible row if:

- The Dark is not closed and committed, or `docs/current-state.md` and The
  Dark closure disagree about the live baseline;
- the release build or tests fail at the intended measurement-freeze commit;
- source snapshots, CR identity, earlier export hashes, or the preregistration
  hash are missing;
- the development export cannot be proven to exclude every held-out identity;
- stable unit keys are duplicated, missing, nondeterministic, or inconsistent
  across repeated exports;
- an intended annotator has already inspected eligible Fallen Empires text in a
  heuristic-design context before the baseline freeze (this disqualifies the
  §0 preparer from every annotator/adjudicator role).

Escalate the smallest issue to the responsible technical owner, research lead,
or program owner. Do not work around a failed entry condition by reading cards
manually.

### 11.2 Pause during annotation

Pause and record a governance issue if: a held-out record appears in any
auditor-visible output; export drift occurs without an accepted change record;
the schema cannot preserve an observed parent/child span or attachment;
instructions would require changing the annotation guide after one pass has
seen the affected rows; a semantic or behavioural decision is being used to
force a structural label; or the two annotators discover they were not
independent. An exposed held-out identity is logged and excluded (as done for
the §0 incident). A compromised pass is not silently retained; the research
lead decides whether to restart with a new independent annotator or preserve
the limitation explicitly.

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
`docs/gates/fem-entry-record.md`:

- [ ] The Dark is adjudicated, closed, and committed, and its final artifact
      hashes match its findings report.
- [ ] `docs/current-state.md` reflects the accepted live baseline.
- [ ] The measurement-freeze commit, data snapshots, CR, protocol, guide,
      preregistration, and six earlier export hashes are recorded.
- [ ] Build and tests pass at the measurement-freeze commit.
- [ ] A held-out-safe deterministic development export exists and has been
      verified by aggregate counts only.
- [ ] The cumulative held-out exclusion registry, including the §0 incident's
      seven newly excluded identities, is bound to the audit.
- [ ] Both independent annotators and the adjudicator are assigned, and none
      of the three is the §0-disqualified preparer.
- [ ] Neither annotator has inspected eligible Fallen Empires text before the
      freeze.
- [ ] `docs/findings/fem-structural-audit.md` remains an empty outline until
      the baseline block is written verbatim.
- [ ] The program owner authorizes the audit to begin.

## 13. Input-freeze procedure (Phase 3, aggregate-only)

Execute after this preregistration and the guide decision set are frozen, with
no row-bearing output displayed:

1. `cargo build --release`; record the version.
2. Recompute corpus, rules, snapshot, tool, protocol, guide, six earlier-export,
   and held-out-registry identities (§3).
3. Run the aggregate-only held-out-safe verifier:
   `python scripts/python/verify_export_safety.py fem --mtg
   .\target\release\mtg-discover.exe --runs 2`. Require: held-out exclusion in
   SQLite before segmentation/serialization; byte-identical repeated JSON and
   TSV exports; unique `(oracle_id, face, unit_index)` keys; valid parent
   integrity; zero held-out export records; no row displayed.
4. Retain `docs/audits/fem/units-export.tsv` (its SHA-256 must equal the
   verifier's expected TSV hash), two identical blank annotation-pass copies,
   and a Fallen Empires experiment manifest under `docs/manifests/`.
5. Hash every retained input and populate §3 and the pre-audit baseline from
   actual command output — never from expected earlier-set values — including
   the multi-sentence count emitted directly by the binary. **Do not run
   `mtg-discover audit novelty` without a held-out-safe alternative**; that
   command does not support `--exclude-heldout` (§0) — compute novelty by
   direct template-overlap comparison over the held-out-safe export and the
   six earlier held-out-safe exports instead.

## 14. Empty findings outline

The empty, non-empirical outline is stored at
`docs/findings/fem-structural-audit.md`. No empirical result may be added to it
until the §12 entry checklist passes and opening is authorized.
