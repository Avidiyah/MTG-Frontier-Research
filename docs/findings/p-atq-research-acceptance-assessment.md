# P-ATQ research acceptance assessment

- Date: 2026-08-26
- Research decision basis: repository state at commit `8e83221`
- Decision scope: P-ATQ-1 through P-ATQ-4
- Status: research and technical S10 acceptance complete
- Prepared by: Codex, acting as research lead for evidence interpretation and proposal adjudication

## Executive decision

- **Another full Antiquities audit:** NO.
- **Bounded acceptance pass sufficient:** YES.
- **Research prerequisite for Legends:** PASSED.
- **Remaining gate condition:** none for ATQ; Legends must still follow its
  preregistered baseline-freeze and held-out-safe entry procedure.
- **Research-side blocking items:** none.

P-ATQ-1 through P-ATQ-4 are accepted on the research evidence. The technical
checks may reopen an individual proposal if they produce a concrete
counterexample, unexplained regression, or measurement conflict. They do not
justify another full Antiquities audit unless they reveal evidence substantially
broader than any currently recorded class.

The subsequent technical package is recorded in
`docs/audits/corpus-checks/2026-08-26-post-patq-merge.md` and
`docs/audits/corpus-checks/2026-08-26-patq-s8-search.md`. The final
reconciliation retains all four acceptances within their stated bounds.

## Scope and method

This is an acceptance assessment of the four implemented Antiquities proposals,
not a second Antiquities audit. It does not alter code or classifier rules and
does not begin the Legends audit.

The assessment used the evidence order required by
`docs/protocol/structural-investigation-protocol.md`:

1. the Comprehensive Rules effective 2026-08-07;
2. current Oracle text from the repository's 2026-08-25 Scryfall snapshot;
3. official rulings stored in `cards.sqlite`;
4. committed Antiquities annotations and corpus-check reports;
5. the post-merge aggregate measurements in commit `8e83221`;
6. research interpretation, stated explicitly as inference rather than fact.

The assessment distinguishes four evidence types:

- **Repository fact:** committed annotation, report, measurement, implementation
  description, or current CLI output from the bound local snapshot.
- **Authoritative rules fact:** a statement established by the Comprehensive
  Rules or an official ruling.
- **Inference:** the research judgement connecting the repository observation to
  the rules taxonomy.
- **Recommendation:** the acceptance decision or bounded follow-up.

The external stack report was used only for its methodological separation of
surface structure, semantic representation, and behavioral validation. No
parser-framework, formalism, or engine claim was used as evidence that a kind or
role classification is correct.

## Evidence baseline

### Antiquities annotations

The Antiquities development set contains 85 cards and had a pre-change export of
125 printed units. Two independent annotation passes agreed on all judgement
fields for all 125 units. The adjudicated pre-change result was:

- 123 accepted units;
- one over-segmentation defect: Battering Ram's condition-only parent;
- one under-segmentation defect: the unscoped delayed trigger inside Tawnos's
  Coffin;
- 123/123 applicable kind judgements correct;
- 123/123 applicable role judgements correct.

The Battering Ram defect motivates P-ATQ-1. Tawnos's Coffin belongs to D19 and is
not a P-ATQ-1 through P-ATQ-4 acceptance blocker.

### Combined post-merge measurement

Commit `8e83221` records the following combined output after P-ATQ-1 through
P-ATQ-4 were merged:

| Measurement | Current value |
|---|---:|
| Printed structural units | 71,563 |
| Rules-supplied units | 970 |
| Distinct normalized templates | 37,299 |
| Delayed-trigger roles | 891 |
| Nested delayed-trigger children | 861 |
| Top-level spell-created delayed-trigger roles | 30 |
| Comma/colon delayed-trigger children | 0 |
| `prevention_effect` units | 166 |
| `can't be prevented` prohibition misfires | 0 |
| Prefix-related prevention candidates remaining from the recorded eight | 3 |

These are measurements of the current heuristic baseline, not assertions of
complete semantic correctness.

## Proposal disposition summary

| Proposal | Disposition | Confidence | Core reason | Technical gate follow-up |
|---|---|---:|---|---|
| P-ATQ-1 | ACCEPT | High | The former comma/colon parents were fragments; valid sentence-level delayed-trigger children remain and quote mis-splits disappear. | Attach fresh exports and earlier-set regression metrics. |
| P-ATQ-2 | ACCEPT | High | `Can't be prevented` prohibits prevention; it does not perform or establish prevention. | Confirm wording variants and mixed-clause negative cases in S8. |
| P-ATQ-3 | ACCEPT | Medium | Prefix removal exposes the real body, with a necessary Saga trigger override; all three residuals are correct prevention positives. | Correct the eight-item historical description and attach the complete prefix rule-firing inventory. |
| P-ATQ-4 | ACCEPT | Medium | The bounded rule identifies spell-created delayed triggers without conflating nested, quoted, cast/resolve, or off-stack triggers. | Complete the full instant/sorcery near-match S8 search and regression package. |

## P-ATQ-1: retract single-sentence comma/colon delayed-trigger splitting

### Question

Does the evidence support removing the heuristic that split a delayed-trigger
phrase at a comma or colon within a single sentence, while retaining supported
sentence-boundary splits?

### Repository facts

The pre-change S11 report identified 121 comma/colon-level delayed-trigger
children:

| Former parent class | Count |
|---|---:|
| Bare trigger condition | 108 |
| Bare activation or loyalty cost | 5 |
| Trigger condition plus only part of an effect | 5 |
| Ability-word or quoted-text fragment | 3 |

In the 40-row judgement sample:

- 38 children expressed a genuine delayed trigger created by the surrounding
  ability's effect;
- two cases were mangled because the split point was inside quoted granted
  text;
- zero of the 40 parent fragments was a valid reference unit;
- three corpus cases split inside quoted abilities.

The same report sampled 30 sentence-level children across decades. All 30 were
valid delayed or reflexive triggered abilities under CR 603.7 or 603.12. None
was an independent trigger, keyword, or reminder-text artifact.

The implemented rule is slightly more conservative than the proposal's initial
wording. Rather than trying to decide whether the material before an internal
comma or colon is a complete effect clause, it removes the single-sentence
backward comma/colon search entirely. The post-merge corpus therefore contains
861 nested delayed-trigger children, all sentence-level, and no comma/colon
children.

### Authoritative rules facts

- CR 113.3c defines a triggered ability as a trigger condition plus an effect.
  A leading `When`/`Whenever`/`At ... ,` condition is not an ability by itself.
- CR 602.1a states that everything before the colon is an activation cost. A
  cost-only prefix is not a standalone ability.
- CR 603.7 and 603.12 support the retained delayed and reflexive trigger
  children when they begin at an actual sentence boundary after a creating
  effect.

### Research inference

The former heuristic often detected a real delayed-trigger clause but placed the
unit boundary incorrectly. Correct recognition of the inner semantic event does
not make a fragmentary parent a valid structural unit. Retaining the whole
single-sentence ability and recording an unattached delayed-trigger candidate is
more faithful than emitting a false boundary.

Quoted-text splitting is resolved by the retraction because no internal comma or
colon is searched as a child boundary. Valid quote structure remains represented
by the separate granted/quoted child mechanism.

### Disposition

**ACCEPT.** The observed single-sentence population supports complete retraction
of comma/colon splitting. No retained counterexample requires reopening the
rule. The 861 sentence-level children demonstrate that the supported positive
class remains active.

### Remaining uncertainty

A future or currently unmeasured single-sentence ability may contain a genuine
in-unit delayed trigger after a complete effect clause. Under the accepted rule,
that case remains whole and should be surfaced as an unattached candidate. It is
an explicit under-segmentation signal for a later bounded measurement, not a
reason to recreate fragmentary parents.

D19's unscoped `When ...` class is also an under-segmentation question, but it is
separate from whether internal comma/colon boundaries are valid.

### Evidence

- `docs/audits/corpus-checks/2026-08-26-delayed-split-overseg.md`, especially
  sections 8-9.
- `docs/findings/atq-structural-audit.md`, sections 3, 6, 8, and the local
  corpus-validation follow-up.
- `docs/audits/atq/units-annotated.tsv`, Battering Ram rows.
- CR 113.3c, 602.1a, 603.7, and 603.12 in
  `Magic-Comprehensive_Rules.md`.

## P-ATQ-2: exclude prevention prohibitions from `prevention_effect`

### Question

Is text stating that damage `can't be prevented` structurally and
rules-taxonomically different from an effect that performs or establishes
prevention?

### Repository facts

The pre-change kind-rules check found nine role=`ability`
`prevention_effect` units whose operative wording was `can't be prevented`.
The report classified them as rule-modifying statics rather than prevention
effects. The post-merge corpus measurement reports zero such units remaining in
`prevention_effect`.

A bounded, read-only non-pool query during this assessment found:

- 16 non-pool cards with a matching current Oracle unit;
- 16 matching units;
- zero matching units that also contained the positive command word `prevent`
  in the same unit.

The implementation also recognizes `cannot be prevented` and straight or curly
apostrophe forms, even though the bound current corpus search returned no
`cannot be prevented` card text.

### Authoritative rules facts

- CR 615.1 defines prevention effects as continuous effects that watch for a
  damage event and completely or partially prevent the damage.
- CR 615.1a states that effects using `prevent` to indicate damage that will not
  be dealt are prevention effects.
- CR 615.12 separately defines effects stating that damage `can't be
  prevented`. Applicable prevention effects still apply to that damage but do
  not prevent it.
- CR 101.2 supplies the general rule that a `can't` effect takes precedence
  over an effect that allows or directs the prohibited event.

### Research inference

A positive prevention instruction performs or establishes prevention. A
`can't be prevented` statement instead constrains whether other prevention
effects can succeed. The latter is therefore a prohibition concerning
prevention, not a prevention effect itself.

The current vocabulary has no dedicated permission/prohibition structural kind,
so the appropriate present behavior is to fall through to the residual
`spell_or_static_text` kind. That fallback does not claim semantic equivalence
between all residual statics; it merely avoids the incorrect prevention label.

### Disposition

**ACCEPT.** The exclusion is narrow, rules-supported, and confirmed against the
recorded defect class. It preserves actual prevention text, including prevention
effects with additional consequences.

### Remaining uncertainty

The principal negative case is a single structural unit that contains both a
genuine prevention instruction and the `can't be prevented` collocation. No
such non-pool unit was found in the bounded query. If one is later found, the
current whole-unit exclusion may be too coarse and the affected proposal should
be revised narrowly.

Future Oracle wording could also introduce a new prohibition idiom. Absence of
such wording from the current corpus is not a claim that the lexical class is
closed forever.

### Evidence

- `docs/audits/corpus-checks/2026-08-26-kind-rules-check.md`, section A2.
- `docs/findings/atq-structural-audit.md`, P-ATQ-2 and local validation
  follow-up.
- CR 101.2, 615.1, 615.1a, and 615.12 in
  `Magic-Comprehensive_Rules.md`.

## P-ATQ-3: classify after structural prefix extraction

### Question

Does the evidence support extracting a leading ability-word, flavor-word,
Saga-chapter, named-mode, or comparable result label before ordinary kind
classification?

### Repository facts

The original kind-rules report grouped eight role=`ability`
`prevention_effect` units under a common leading `<prefix> — body` form. The
group contained three distinct outcomes:

1. three ability-word prefixes hiding a trigger word in the body;
2. two Saga chapter symbols whose printed bodies began with `Prevent`;
3. three labels whose bodies were themselves genuine prevention effects.

The post-merge measurement reduces the eight recorded prefix-related prevention
candidates to the three cases in category 3. Exact current segmentation of
those cards confirms that each has a recorded `prefix` and a
`prevention_effect` body.

The initial report's description of all eight as kind misfires was therefore
overbroad. The correct acceptance record is:

- **five actual kind corrections:** Favored Hoplite, Harvestguard Alseids,
  Loyal Unicorn, Crystal Fragments // Summon: Alexander, and Old Fat Spider
  Can't See Me;
- **three correct post-prefix validation positives:** Urza's Science Fair
  Project, Khârn the Betrayer, and Diamond Weapon.

This correction changes the evidence interpretation, not the implementation.

### Authoritative rules facts

- CR 207.2c states that ability words appear at the beginning of abilities and
  have no special rules meaning.
- CR 207.2d similarly defines flavor words as italic leading descriptions with
  no special rules meaning. `The Betrayer` and `Immune` belong to this class;
  they are not ordinary ability words or modal choices.
- CR 714.2 states that a Saga chapter symbol is a keyword ability representing
  a triggered ability. CR 714.2b expands `{rN}—[Effect]` into a lore-counter
  trigger.
- CR 700.2 defines actual modal spells and abilities. A result label or flavor
  word should not be called a mode merely because it precedes an em dash.

### Research inference

Ability and flavor words should not obscure the ability body's structural
signals. Removing those labels before ordinary classification is correct.

A Saga chapter marker is not semantically inert, however. Naïvely deleting the
chapter symbol and classifying a body beginning `Prevent` would reproduce the
wrong `prevention_effect` result. The implemented type-line and Roman-numeral
override is therefore essential: the whole chapter ability is
`triggered_ability`, irrespective of the body verb.

Numeric outcome labels and flavor words may expose bodies whose correct kind is
still `prevention_effect`. Persistence in the prevention population after prefix
extraction is not itself evidence of failure.

### Residual adjudications

| Card | Machine disposition | Relevant Oracle unit | Removed prefix | Classification body | Correct kind | Adjudication |
|---|---|---|---|---|---|---|
| Urza's Science Fair Project | `ACCEPT` | `2 — Prevent all combat damage it would deal this turn.` | Source label `2`; normalized/stored prefix `N` | `Prevent all combat damage it would deal this turn.` | `prevention_effect` | Correct positive. The die-result number is a structural label and the body directly creates prevention under CR 615.1a. The earlier eight-item misfire grouping was wrong for this row. The coarseness of representing each die result as role=`ability` is separate from P-ATQ-3 kind classification. |
| Khârn the Betrayer | `ACCEPT` | `The Betrayer — If damage would be dealt to Khârn the Betrayer, prevent that damage and an opponent of your choice gains control of it.` | `The Betrayer` | `If damage would be dealt to ~, prevent that damage and an opponent of your choice gains control of it.` | `prevention_effect` | Correct positive. The prefix is a CR 207.2d flavor word. CR 615.1a and 615.5 cover prevention with an additional effect. The official ruling dated 2022-10-07 confirms that the opponent gaining control is part of the replacement/prevention process and happens immediately. |
| Diamond Weapon | `ACCEPT` | `Immune — Prevent all combat damage that would be dealt to Diamond Weapon.` | `Immune` | `Prevent all combat damage that would be dealt to ~.` | `prevention_effect` | Correct positive. `Immune` is a CR 207.2d flavor word and the body is unambiguous CR 615.1a prevention. The official ruling dated 2025-06-06 expressly calls it `Diamond Weapon's prevention effect`. |

Machine-readable residual result:

```json
{
  "Urza's Science Fair Project": "ACCEPT",
  "Khârn the Betrayer": "ACCEPT",
  "Diamond Weapon": "ACCEPT"
}
```

### Disposition

**ACCEPT.** The rule is structurally and rules-taxonomically supported when the
acceptance record explicitly preserves the Saga exception and corrects the
historical eight-item description to five corrections plus three correct
positives.

### Remaining uncertainty

The completed S8 inventory records 3,572 firings. It found bounded structural
false positives, two punctuated flavor-word false negatives, and 141 newly
incorrect `keyword_ability` labels on funny/token products through the
pre-existing `is_keyword_line` heuristic. No newly incorrect keyword labels
occur in expansion/core/commander products. These limitations remain visible;
a later refinement would be a bounded proposal, not a broad second ATQ audit.

### Evidence

- `docs/audits/corpus-checks/2026-08-26-kind-rules-check.md`, section A2.
- `docs/findings/atq-structural-audit.md`, P-ATQ-3 and the local validation
  follow-up.
- Current local `segment --card` output for the three residual cards.
- Current local `card --rulings` output for Khârn the Betrayer and Diamond
  Weapon.
- CR 207.2c, 207.2d, 615.1a, 615.5, 700.2, and 714.2 in
  `Magic-Comprehensive_Rules.md`.

## P-ATQ-4: top-level spell-created delayed-trigger role

### Question

Should a top-level instant or sorcery unit that is itself the delayed-trigger
text created by resolving the spell receive role=`delayed_trigger` under the
implemented bounded rule?

### Repository facts

The original corpus check found 111 top-level `triggered_ability` units on
instant or sorcery faces and divided them into:

| Structural class | Count |
|---|---:|
| Off-stack triggered ability of the card | 65 |
| Delayed trigger created by resolving the spell | 30 |
| Trigger concerning the spell's own casting or resolution | 16 |

The implementation applies only to a unit that is:

- top-level;
- printed rather than rules-supplied;
- already kind=`triggered_ability`;
- initially role=`ability`;
- on an instant or sorcery face;
- positively scoped by `this turn`, `this combat`, or `next`;
- not a cast/resolve trigger;
- not supported by evidence that the card's own ability functions from a
  graveyard, exile, discard, cycling, suspend, or haunt context.

It changes the role in place. It does not create, duplicate, or reparent a
unit, and it does not recurse into child units.

The committed non-pool desk cross-check applied the implementation to the 105
inspectable records from the historical 111-unit listing. Twenty-eight received
role=`delayed_trigger`; 77 remained role=`ability`. Every result agreed with
the recorded manual CR classification. The two meaningful disagreements with
the older measurement script both favored the implementation:

- a cast trigger that also mentioned `this turn` was correctly excluded;
- a genuine delayed trigger mentioning other cards in a graveyard was correctly
  retained because the zone word was not a self-zone condition.

The combined post-merge aggregate contains exactly 30 top-level
spell-created delayed-trigger roles and 861 nested delayed-trigger children.

### Authoritative rules facts

- CR 113.3a defines ordinary instant and sorcery instructions as spell
  abilities, subject to the activated, triggered, and static exceptions.
- CR 113.6 and 113.6b-k describe abilities that function from zones other than
  the stack or battlefield, supporting the off-stack negative classes.
- CR 603.1 defines triggered-ability surface form.
- CR 603.7a states that resolving spells and abilities may create delayed
  triggered abilities.
- CR 603.7b distinguishes triggers that may recur because they have a stated
  duration such as `this turn`.
- CR 603.7d states that when a spell creates a delayed triggered ability, that
  spell is its source and the spell's controller as it resolved controls the
  delayed trigger.

### Top-level versus nested structure

A P-ATQ-4 unit is the printed delayed-trigger clause that constitutes the
spell's entire relevant instruction. The current schema has no separate card-face
node to use as a parent. It therefore remains top-level with
`parent_index = null` and role=`delayed_trigger`.

This is distinguishable from:

- a sentence- or line-level delayed-trigger child created by an earlier effect,
  which has role=`delayed_trigger` and a non-null `parent_index`;
- a delayed trigger inside quoted granted text, which retains role=`granted`;
- a cast/resolve trigger of the spell, which remains role=`ability`;
- an off-stack ability printed on an instant or sorcery card, which likewise
  remains role=`ability`.

The role is a structural provenance distinction. It does not claim that the
system has represented the trigger's executable behavior.

### False-positive assessment

The principal false-positive classes are bounded by independent gates:

- ordinary non-trigger spell text fails the existing kind gate;
- identical wording on a permanent fails the instant/sorcery type-line gate;
- quoted and mode children are never traversed;
- cast and resolution triggers are hard negatives even if their effects mention
  `this turn`;
- cycling, suspend, haunt, and self-zone triggered abilities are excluded;
- a bare graveyard or exile word is not enough to exclude a valid delayed
  trigger, preventing an overbroad blacklist.

The regression tests and 105-row non-pool cross-check directly exercise these
classes.

### False-negative assessment

The rule does not claim to identify every possible temporal formulation. A
spell-created delayed trigger lacking `this turn`, `this combat`, or `next`
could remain role=`ability`. Other self-zone phrasings outside the implemented
zone vocabulary may also exist.

The completed S8 search inspected the full instant/sorcery-face population.
Recorded out-of-pattern classes include Ertai's Meddling, 45 whole-unit
inverted cantrip lines, duration-first forms, comma-led forms, and two
sentence-initial combat forms. They are explicit unsupported recall classes;
widening coverage requires a separate bounded proposal rather than reopening
the accepted P-ATQ-4 class.

### Disposition

**ACCEPT.** The implemented rule is an appropriately conservative structural
classifier for the evidenced class. Acceptance covers the stated bounded
pattern and its negative gates; it is not a claim of exhaustive delayed-trigger
recall.

### Evidence

- `docs/audits/corpus-checks/2026-08-26-kind-rules-check.md`, sections B and
  B2.
- `docs/findings/atq-structural-audit.md`, P-ATQ-4 and the local validation
  follow-up.
- `docs/current-state.md`, P-ATQ-4 implementation description and current
  baseline.
- CR 113.3a, 113.6, 603.1, and 603.7 in
  `Magic-Comprehensive_Rules.md`.

## Deferred but non-blocking questions

### D19: unscoped `When ...` after an effect

Tawnos's Coffin and Animate Dead demonstrate delayed-trigger text introduced by
an earlier effect without the temporal scope required by the current scoped
heuristics. This remains a distinct under-segmentation question. Its S8 search
must distinguish effect-created delayed triggers from independent printed
triggers.

D19 does not contradict P-ATQ-1. P-ATQ-1 answers whether an internal comma or
colon is a valid boundary; D19 asks when an unscoped sentence-initial `When`
clause is structurally a child of preceding effect text.

### D14: independent triggers sharing a paragraph

Some Oracle paragraphs contain multiple independent triggered abilities.
Splitting these requires evidence that the later `When` clause is independent,
not created by the preceding effect. This is the nearest negative class for D19
and must remain a separate bounded investigation.

### Semantic interpretation

The accepted kinds and roles do not determine:

- semantic operator identity;
- typed arguments or references;
- prevention/replacement ordering in execution;
- the behavior of additional effects;
- trigger ownership and control in arbitrary game states;
- executable equivalence between wording variants.

Those claims require later semantic annotations and behavioral fixtures. They
must not be inferred from structural acceptance.

### Frozen held-out evaluation

The frozen pool remains a later gate-evaluation asset rather than evidence for
these development-set changes. Corpus-wide aggregate counts may include pool
cards, but card text from the pool must not inform heuristic design or proposal
adjudication.

## Held-out protocol incident

During this assessment, the initial literal CLI query for
`can't be prevented` emitted four cards from the frozen `oracle_id`-`f...`
pool before a non-pool filter was applied:

- Combust;
- Malignus;
- Lava Burst;
- Wild Slash.

Their records were not used as supporting examples, counterexamples, or grounds
for any disposition in this report. All subsequent bounded measurements were
filtered to non-pool records before output.

Protocol section 6.3 requires every accidentally inspected card to be logged and
excluded from later held-out samples. This section is that log. The four named
cards must not be selected for a later held-out annotation or gate sample.

This incident is a held-out governance remediation. It does not alter the
research adjudication of P-ATQ-1 through P-ATQ-4, which rests on committed
non-pool evidence, aggregate measurements, the three explicitly assigned
non-pool residuals, and authoritative rules.

## External-report relevance

`docs/findings/external-stack-research-2026-08-26.md` supports the following
methodological constraints used here:

- surface structure is not semantic interpretation;
- semantic representation is not behavioral validation;
- parser acceptance does not prove denotation or execution correctness;
- unsupported and ambiguous outcomes must remain visible;
- authoritative rules and adjudicated evidence take precedence over engine or
  parser behavior;
- new infrastructure should respond to a demonstrated coordination,
  representation, or performance need.

No current P-ATQ evidence demonstrates a need for new parser infrastructure,
storage, annotation software, or an engine. The four decisions concern bounded
changes to the existing structural measurement baseline.

## Text for Claude's S10 decision record

> **Research-lead acceptance, P-ATQ-1 through P-ATQ-4:** ACCEPT all four,
> subject to completion of the technical S10 package. P-ATQ-1 is supported
> because the former 121 comma/colon parents were fragments, including three
> quoted-text mis-splits, while all 861 retained nested children are
> sentence-level and the sampled sentence-level population was 30/30 valid.
> P-ATQ-2 is supported by CR 615.1a, 615.12, and 101.2: `can't be prevented`
> prohibits prevention rather than establishing it. P-ATQ-3 is supported with
> one evidence-record correction: five of the eight historical prefixed
> candidates were kind defects; Urza's Science Fair Project, Khârn the
> Betrayer, and Diamond Weapon are correct `prevention_effect` positives after
> prefix removal. `The Betrayer` and `Immune` are flavor words under CR
> 207.2d, while Saga chapter symbols require the implemented CR 714.2 trigger
> override. P-ATQ-4 correctly assigns role=`delayed_trigger` to bounded,
> top-level spell-created triggers without conflating them with nested
> delayed-trigger children, granted text, cast/resolve triggers, or off-stack
> abilities. Temporal phrasings outside `this turn`/`this combat`/`next`
> remain explicitly unsupported rather than claimed as covered. D19, D14,
> semantic interpretation, and frozen held-out evaluation remain separate and
> non-blocking.

Claude's completed technical record:

1. attach the fresh `lea`/`leb`/`arn`/`atq` regression metrics and explain each
   legitimate export change;
2. preserve the full before/after unit, template, kind, and role histograms;
3. attribute the gross `prevention_effect` count change rather than treating the
   aggregate delta as self-explanatory;
4. record P-ATQ-3 as five corrected kind defects plus three correct residual
   positives;
5. include the prefix rule-firing inventory and its nearest non-label
   counterexamples;
6. include the full instant/sorcery temporal near-match search for P-ATQ-4;
7. identify any conflict with the research dispositions rather than silently
   absorbing it into a metric;
8. preserve the held-out incident exclusions recorded above.

## Legends transition recommendation

The ATQ acceptance gate is closed. Legends is the next authorized structural
audit after its preregistered baseline-freeze and held-out-safe entry checks.
No second ATQ audit is a research prerequisite.

The following constraints must carry forward:

- preserve P-ATQ-1's whole-unit fallback for unresolved single-sentence
  delayed triggers;
- keep D19 and D14 separate until a bounded counterexample search distinguishes
  their attachment classes;
- describe `The Betrayer` and `Immune` as flavor words, not ability words or
  modal choices;
- preserve the structural distinction between top-level spell-created delayed
  triggers and nested delayed-trigger children;
- treat kinds and roles as heuristic structural measurements, not semantic or
  behavioral ground truth;
- leave semantic operator and execution claims for later phases;
- do not inspect or use the frozen held-out pool during Legends development;
- exclude Combust, Malignus, Lava Burst, and Wild Slash from all later held-out
  samples because of the incident logged above.

If future evidence produces a contradiction, reopen only the affected proposal
with the smallest bounded measurement needed to decide it.

## Evidence index

- `docs/protocol/structural-investigation-protocol.md`
  - S7: evidence hierarchy;
  - S8: counterexample-search requirements;
  - S10: heuristic acceptance;
  - S11: corpus-wide rule-firing and over-segmentation checks;
  - S13: unsupported and ambiguous outcomes;
  - section 6.3: frozen held-out pool and accidental-inspection rule.
- `docs/findings/atq-structural-audit.md`
  - adjudicated Antiquities findings;
  - P-ATQ-1 through P-ATQ-4 implementation records;
  - combined corpus-validation follow-up.
- `docs/audits/atq/units-annotated.tsv`
  - adjudicated Battering Ram and Tawnos's Coffin defects.
- `docs/audits/atq/units-annotated-pass2.tsv`
  - independent second pass.
- `docs/audits/atq/metrics.json`
  - pre-change audit metrics and agreement.
- `docs/audits/corpus-checks/2026-08-26-delayed-split-overseg.md`
  - former delayed-trigger fragment population and sentence-level sample.
- `docs/audits/corpus-checks/2026-08-26-kind-rules-check.md`
  - prevention prohibitions, prefix cases, and instant/sorcery trigger classes.
- `docs/audits/corpus-checks/2026-08-26-post-patq-merge.md`
  - frozen post-merge inputs, before/after histograms, and regression evidence.
- `docs/audits/corpus-checks/2026-08-26-patq-s8-search.md`
  - complete P-ATQ-3 firing inventory and P-ATQ-4 near-match search.
- `docs/current-state.md`
  - current combined corpus baseline and implementation descriptions.
- `docs/roadmap.md`
  - D14, D15-D19, and Legends transition state.
- `docs/findings/external-stack-research-2026-08-26.md`
  - methodological separation of structural, semantic, and behavioral claims.
- `Magic-Comprehensive_Rules.md`
  - CR 101.2, 113.3, 113.6, 207.2c-d, 602.1a, 603.1, 603.7, 603.12,
    615.1, 615.5, 615.12, 700.2, and 714.2.
