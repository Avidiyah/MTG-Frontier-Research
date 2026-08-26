# MTG Frontier Research: Gated Long-Term Roadmap

**Program start:** 2026-08-26  
**Planning horizon:** 2026-08-26 through 2031-12-31  
**Active phase:** Phase 0 — Baseline stabilization and research governance  
**Primary research frontier:** Oracle text + Comprehensive Rules → formal, executable card semantics  
**Status of dates:** Planning targets, subordinate to evidence gates

## 1. Program charter

### 1.1 Long-term objective

Build and validate a system that can translate previously unseen Magic: The
Gathering Oracle text into a deterministic, compositional, executable
representation without requiring card-specific implementation for the majority
of cards.

The intended long-term pipeline is:

```text
Oracle text + Comprehensive Rules + official rulings
    -> structural analysis
    -> semantic representation / IR
    -> validated executable effects
    -> authoritative rules engine
    -> game simulation
    -> telemetry
    -> search and learning agents
```

The research contribution is concentrated in the first three transitions.
Simulation, search, and learning are downstream consumers and must not drive
premature architecture.

### 1.2 Central research question

> Can arbitrary Oracle text be transformed into a sufficiently expressive
> formal representation that a general rules engine can execute previously
> unseen cards without card-specific code?

### 1.3 Program thesis

Magic is a formal rules system whose card language contains recurring
structures, operators, and typed arguments. The project will test how far those
regularities support deterministic or hybrid semantic translation. It will not
assume that all cards are reducible to templates, that one representation fits
all mechanics, or that language alone supplies every semantic distinction.

### 1.4 Definition of success

The final research target is not "an AI that plays Magic." Success means:

1. unseen Oracle text can be parsed into an explicit semantic representation;
2. the representation is validated before execution;
3. generic operations execute it against an authoritative game state;
4. behavior agrees with the Comprehensive Rules, Oracle text, and adjudicated
   test cases;
5. unsupported cards fail explicitly rather than silently producing plausible
   but incorrect behavior;
6. card-specific code is measured, isolated, and excluded from parser coverage;
7. downstream simulation and agents consume the same validated environment.

## 2. Scope control

### 2.1 In scope

- Current Oracle text, card characteristics, official rulings, and the
  Comprehensive Rules.
- Empirical discovery of structural and semantic regularities.
- Gold-standard annotations and stratified evaluation sets.
- Comparison and selection of a Magic-adequate semantic IR.
- Generic effect primitives and their execution semantics.
- Formal validation, unsupported-case reporting, and provenance.
- Integration with an existing engine or a bounded minimal engine substrate.
- Deterministic simulation once correctness gates are met.
- Telemetry, search, and learning only after the environment is validated.

### 2.2 Explicit non-goals until their gates open

- No production rules engine during Phases 0–3.
- No final IR commitment before the Phase 2 evidence package is accepted.
- No corpus-scale parser optimization before a frozen held-out set exists.
- No MCTS, self-play, deck construction, or gameplay optimization before the
  simulation gate.
- No new RL algorithm unless a later evaluation demonstrates that established
  methods are insufficient.
- No manual per-card scripting presented as parser progress.
- No claim of semantic equivalence based only on normalized strings,
  embeddings, frequency, or lexical similarity.
- No exhaustive modeling of every Comprehensive Rules subsystem before a
  minimal vertical slice demonstrates the representation.
- No user interface, hosted service, or multiplayer product work in this
  research program.

### 2.3 Scope-change rule

Any proposal that adds a deliverable, changes the active research question, or
opens work from a later phase requires a written change record containing:

1. the evidence motivating the change;
2. the current deliverable it replaces or delays;
3. schedule and evaluation impact;
4. alternatives considered;
5. an explicit accept/reject decision.

Unrecorded work does not become program scope. Interesting ideas are added to a
deferred-work register and remain inactive until their phase opens.

## 3. Gate and schedule policy

### 3.1 Dates do not open phases by themselves

Every phase has:

- an **earliest transition date**, before which the next phase cannot become
  active;
- a **planned gate date**, when evidence is reviewed;
- objective **exit criteria**;
- a bounded **extension window**.

A phase advances only when every mandatory exit criterion is met. Reaching a
date without passing the gate triggers one of three written decisions:

1. extend the phase within its stated extension window;
2. reduce or revise the hypothesis while preserving the program objective;
3. stop the affected branch because the evidence does not justify proceeding.

The project must not waive failed criteria merely to preserve the calendar.

### 3.2 Gate evidence

Every gate package must contain:

- reproducible commands and input snapshot identifiers;
- quantitative results with denominators;
- supporting examples and counterexamples;
- known failure classes;
- tests or adjudicated annotations;
- a decision record;
- updated current-state documentation.

### 3.3 Coverage accounting

Coverage must always be reported separately as:

- structurally segmented;
- semantically parsed;
- statically validated;
- executable;
- behaviorally validated;
- unsupported;
- manually implemented;
- ambiguous or awaiting adjudication.

Manual implementations and silent fallbacks never count as automatic coverage.

## 4. Master schedule

| Phase | Planned dates | Earliest transition | Planned gate | Maximum extension |
|---|---|---|---|---|
| 0. Baseline stabilization | 2026-08-26–2026-09-15 | 2026-09-16 | 2026-09-15 | 2 weeks |
| 1. Structural discovery | 2026-09-16–2027-01-31 | 2027-02-01 | 2027-01-31 | 6 weeks |
| 2. Semantic inventory and gold data | 2027-02-01–2027-06-30 | 2027-07-01 | 2027-06-30 | 8 weeks |
| 3. IR and execution-backend decision | 2027-07-01–2027-10-31 | 2027-11-01 | 2027-10-31 | 6 weeks |
| 4. Minimal formal vertical slice | 2027-11-01–2028-04-30 | 2028-05-01 | 2028-04-30 | 8 weeks |
| 5. Parser/executor Levels 1–3 | 2028-05-01–2028-12-31 | 2029-01-01 | 2028-12-31 | 12 weeks |
| 6. Structural generalization | 2029-01-01–2029-12-31 | 2030-01-01 | 2029-12-31 | 12 weeks |
| 7. Engine integration and simulation | 2030-01-01–2030-09-30 | 2030-10-01 | 2030-09-30 | 12 weeks |
| 8. Telemetry and established search | 2030-10-01–2031-03-31 | 2031-04-01 | 2031-03-31 | 8 weeks |
| 9. Learning and final evaluation | 2031-04-01–2031-12-31 | — | 2031-12-31 | No automatic extension |

These dates assume sustained research capacity. A gate review may shift all
later dates, but it may not overlap phases whose prerequisites remain unmet.

## 5. Phase 0 — Baseline stabilization and research governance

**Planned dates:** 2026-08-26–2026-09-15  
**Earliest Phase 1 start:** 2026-09-16  
**Gate 0 review:** 2026-09-15

### Objective

Establish one reproducible baseline and one authoritative description of the
current system before further corpus experiments change measurements.

### Work authorized

- Reconcile current segmenter behavior with repository documentation.
- Record the exact Scryfall and Comprehensive Rules snapshots.
- Stabilize CLI output contracts needed by experiments.
- Preserve the Alpha audit as the first completed investigation.
- Define experiment-report, decision-record, and gate-report templates.
- Establish an explicit deferred-work register.

### Required deliverables

- Updated current-state document matching live behavior.
- Reproducible corpus and Alpha measurements.
- Passing automated tests for current segmentation behavior.
- Written baseline schema for experiment results.
- List of known structural failure classes.

### Gate 0 exit criteria

- [x] A clean checkout can reproduce corpus metadata and template totals.
- [x] Documentation and CLI output describe the same segmentation model.
- [x] Every accepted Alpha-derived segmenter change has a regression test.
- [x] Generated data, source, findings, and scratch analyses are clearly
      separated.
- [x] No IR, executor, simulation, or AI implementation has entered scope.

Evidence, caveats, and the decision record: `docs/gates/gate-0-evidence.md`
(2026-08-26, pass with two recorded reproducibility caveats). The frozen
Phase 1 procedure is `docs/protocol/structural-investigation-protocol.md`.

### If the gate fails

Use at most two additional weeks for reconciliation. Do not begin the
set/era study against an unstable baseline.

## 6. Phase 1 — Structural discovery

**Planned dates:** 2026-09-16–2027-01-31  
**Earliest Phase 2 start:** 2027-02-01  
**Gate 1 review:** 2027-01-31

### Objective

Determine the reliable structural units of Oracle text and quantify where
surface structure succeeds or fails.

### Research questions

- What lies between token-level text and a complete card ability?
- Which structures nest or span lines?
- Which structures depend on card faces, type lines, keyword definitions, or
  rules-supplied abilities?
- How does structural novelty change across Magic's release history?
- Which normalizations preserve distinctions and which erase semantics?

### Work authorized

- Continue the first-printing-era walk after Alpha.
- Use exhaustive audits for early small sets and stratified samples for later
  eras; do not promise an exhaustive manual audit of every set.
- Test segmentation for keyword lists, modal abilities, ability words,
  quoted/granted abilities, linked abilities, delayed triggers, sagas,
  classes, rooms, adventures, split cards, transform cards, and other
  multi-part layouts as they enter the historical corpus.
- Run reversible normalization ablations one at a time.
- Link examples to relevant rules and rulings.

### Required deliverables

- Versioned structural taxonomy.
- Era-stratified failure matrix.
- Corpus novelty measurements with a documented denominator.
- Candidate segmentation specification.
- Frozen structural annotation guide.
- A reviewed structural gold set with development and held-out partitions.

### Gate 1 exit criteria

- [ ] The gold set spans release eras, card layouts, ability categories,
      frequent templates, rare templates, and known adversarial cases.
- [ ] Two independent passes or an equivalent adjudication process establish
      a documented reference segmentation.
- [ ] Structural metrics report boundary precision, recall, and exact-card
      agreement rather than template frequency alone.
- [ ] Remaining failures are categorized rather than silently absorbed into
      `spell_or_static_text`.
- [ ] The team can state which structures require rules/type-line context.
- [ ] The held-out structural set has been frozen before parser tuning.

### Prohibited during Phase 1

- Designing the final semantic operator inventory.
- Selecting GDL-II or any other IR as final.
- Building an effect executor.
- Treating line coverage as semantic coverage.

## 7. Phase 2 — Semantic inventory and gold evaluation data

**Planned dates:** 2027-02-01–2027-06-30  
**Earliest Phase 3 start:** 2027-07-01  
**Gate 2 review:** 2027-06-30

### Objective

Discover the minimum semantic distinctions required by representative cards
before choosing a representation.

### Research questions

- Which operators recur across effects?
- What typed arguments do quantities, mana, objects, players, zones,
  durations, events, conditions, choices, and references occupy?
- Which semantics come from Oracle text, card characteristics, keyword rules,
  general rules, or runtime state?
- Which superficially similar texts differ semantically?
- Which effects require recursive abilities, event replacement, continuous
  evaluation, or rule modification?

### Work authorized

- Annotate operator, argument, reference, and context requirements.
- Compare Oracle text with rules and rulings.
- Construct minimal pairs and counterexample suites.
- Survey semantic parsing, controlled language, text-to-program, program
  synthesis, event calculus, and related formalisms.
- Define uncertainty and unsupported-case labels.

### Required deliverables

- Candidate semantic operator inventory with evidence for each operator.
- Typed-argument and reference-resolution inventory.
- Rules-context provenance model.
- Semantically annotated development set.
- Frozen held-out semantic evaluation set.
- Inter-annotator or adjudication report.
- Requirements matrix for candidate IRs.

### Gate 2 exit criteria

- [ ] Every proposed operator is grounded in multiple examples or explicitly
      marked as a singleton/exception.
- [ ] The inventory covers simple effects, compositions, choices, targets,
      triggers, replacement effects, continuous effects, and at least one
      rule-modifying class.
- [ ] Annotation disagreements and genuine rules ambiguities are preserved.
- [ ] Evaluation separates operator identification, argument extraction,
      reference resolution, context retrieval, and whole-effect correctness.
- [ ] The requirements matrix is sufficient to reject inadequate IRs.
- [ ] No candidate IR has been selected merely because it is familiar.

### If the gate fails

Extend by at most eight weeks to repair the annotation scheme. Do not select an
IR against unstable semantic labels.

## 8. Phase 3 — IR and execution-backend decision

**Planned dates:** 2027-07-01–2027-10-31  
**Earliest Phase 4 start:** 2027-11-01  
**Gate 3 review:** 2027-10-31

### Objective

Select, adapt, or reject candidate representations and decide whether execution
should use an existing Magic engine, a minimal new substrate, or a hybrid.

### Candidate foundations

- GDL/GDL-II concepts: fact-based state, `legal`, `next`, `random`, `sees`.
- Arithmetic-capable logic or planning extensions.
- Typed AST or bytecode-style effect IR.
- Event calculus or rule/event systems.
- Declarative/imperative hybrid representations.
- Existing open-source Magic engine effect models.

### Required comparisons

Each candidate must be evaluated against:

- arithmetic and symbolic quantities;
- hidden information and randomness;
- zones, objects, ownership, control, and identity;
- costs, targets, choices, and legality;
- triggers and delayed triggers;
- replacement/prevention effects;
- continuous effects, layers, dependencies, and durations;
- nested and granted abilities;
- rule-changing permissions and restrictions;
- validation and explicit unsupported states;
- compilation/execution cost;
- provenance back to text and rules.

### Work authorized

- Implement disposable representation spikes, not production engines.
- Survey Forge, XMage, and other relevant open-source engines.
- Measure custom-card code and generic primitives where source access permits.
- Prototype the same adjudicated card subset in each serious candidate.

### Required deliverables

- Candidate comparison matrix.
- Engine/backend survey.
- At least two bounded representation spikes.
- Architecture decision record.
- Rejected-alternatives record.
- Versioned IR specification for the Phase 4 subset.

### Gate 3 exit criteria

- [ ] The selected IR represents the agreed subset without embedding card names
      or card-specific branches.
- [ ] Arithmetic, hidden information, and randomness have explicit designs.
- [ ] Unsupported constructs are representable as typed failures.
- [ ] Parser, validator, executor, and engine boundaries are explicit.
- [ ] The backend decision is supported by a prototype and maintenance analysis.
- [ ] At least one serious alternative has been tested and rejected with
      evidence.

### Prohibited during Phase 3

- Corpus-wide parser training or tuning.
- Full rules-engine construction.
- Search-agent implementation.
- Declaring GDL-II sufficient without testing Magic-specific requirements.

## 9. Phase 4 — Minimal formal vertical slice

**Planned dates:** 2027-11-01–2028-04-30  
**Earliest Phase 5 start:** 2028-05-01  
**Gate 4 review:** 2028-04-30

### Objective

Prove end-to-end feasibility on a deliberately bounded rules and card subset.

### Required slice

The slice must include more than creatures with numeric attributes. At minimum:

- zones and object identity;
- turn/phase progression needed by the subset;
- mana payment and costs;
- targets and choices;
- stack placement and resolution;
- draw, damage, life, move, destroy, and token operations;
- one activated ability;
- one triggered ability;
- one delayed trigger;
- one replacement effect;
- one continuous modifier;
- hidden-card draw and deterministic seeded randomness.

### Required deliverables

- Typed IR parser/serializer and validator.
- Generic primitive executor.
- Minimal authoritative state model.
- Deterministic replay format.
- Hand-authored reference IR only for testing the parser boundary.
- End-to-end conformance suite.

### Gate 4 exit criteria

- [ ] The same IR executes deterministically from a fixed initial state and seed.
- [ ] Invalid IR is rejected before state mutation.
- [ ] State transitions preserve object identity and zone invariants.
- [ ] Reference IR for the subset contains no card-specific executor branches.
- [ ] Results agree with adjudicated expected traces.
- [ ] Failures expose the responsible text span, IR node, and rule provenance.

### Scope boundary

The vertical slice is evidence about architecture, not a license to implement
all Comprehensive Rules. Missing mechanics remain explicitly unsupported.

## 10. Phase 5 — Parser/executor Levels 1–3

**Planned dates:** 2028-05-01–2028-12-31  
**Earliest Phase 6 start:** 2029-01-01  
**Gate 5 review:** 2028-12-31

### Objective

Demonstrate template recognition, compositional parsing, and parameter
generalization on frozen evaluation data.

### Success levels in scope

- **Level 1 — Template recognition:** known forms map to known primitives.
- **Level 2 — Compositional parsing:** multiple known structures form linked
  effects.
- **Level 3 — Parameter generalization:** unseen combinations of known
  operators and typed arguments compile correctly.

### Work authorized

- Deterministic grammars and typed parsers.
- Retrieval of rules and keyword definitions.
- Constrained statistical or language-model components where deterministic
  methods are insufficient.
- Static validation and confidence/unsupported reporting.
- Differential tests against reference IR and execution traces.

### Required deliverables

- Versioned parser and validator.
- Development, validation, and untouched test partitions.
- Per-category and whole-card metrics.
- Error taxonomy and unsupported-card report.
- Reproducible execution-based evaluation.

### Gate 5 exit criteria

- [ ] No evaluation examples leaked into grammar exceptions or prompts.
- [ ] Exact semantic correctness is reported alongside component metrics.
- [ ] Parser output that does not validate is never executed.
- [ ] Parameter-generalization tests use unseen combinations, not only unseen
      card names.
- [ ] Automatic and manually encoded coverage are reported separately.
- [ ] Performance on rare and adversarial categories is not hidden by aggregate
      frequency.

### Gate decision

The Gate 5 review sets quantitative continuation thresholds from the frozen
benchmark. Thresholds must be declared before the final test run; they must not
be retrofitted to observed results.

## 11. Phase 6 — Structural and rule-modifying generalization

**Planned dates:** 2029-01-01–2029-12-31  
**Earliest Phase 7 start:** 2030-01-01  
**Gate 6 review:** 2029-12-31

### Objective

Test Levels 4–6 on genuinely unseen structures and increasingly difficult
Magic semantics.

### Success levels in scope

- **Level 4 — Structural generalization:** unseen sentence structures compile
  from known semantic primitives.
- **Level 5 — New-card zero-shot execution:** newly released cards execute
  without card-specific implementation when they use supported mechanics.
- **Level 6 — Rule-modifying generalization:** permissions, prohibitions,
  replacements, continuous rules, and other rule changes are represented.

### Required capability tracks

- Continuous effects, layers, timestamps, and dependencies.
- Replacement and prevention ordering.
- Linked abilities and remembered choices.
- Copying, text changes, and ability granting.
- Variable quantities and runtime references.
- Alternate/additional costs and casting permissions.
- Multiplayer relations where required by selected tests.
- Explicit handling of finite-model limitations.

### Required deliverables

- Quarterly newly released-card challenge sets.
- Mechanic-held-out and structure-held-out evaluations.
- Rule-modification conformance suite.
- Unsupported-boundary specification.
- Scalability and validation-cost report.

### Gate 6 exit criteria

- [ ] New-card tests are frozen before implementation sees their Oracle text.
- [ ] The system distinguishes unsupported mechanics from parser mistakes.
- [ ] Rule-changing behavior is tested through state/action traces, not prose
      explanations.
- [ ] No card-specific branches are introduced to pass challenge cards.
- [ ] The supported subset is broad enough to justify engine integration.
- [ ] Residual exceptions are quantified and classified.

### Stop condition

If Level 4 generalization repeatedly requires card-specific implementation,
pause engine integration and publish the demonstrated boundary instead of
masking the failure with manual code.

## 12. Phase 7 — Engine integration and simulation

**Planned dates:** 2030-01-01–2030-09-30  
**Earliest Phase 8 start:** 2030-10-01  
**Gate 7 review:** 2030-09-30

### Objective

Integrate validated effects with an authoritative game environment and prove
that it can safely support hypothetical rollouts.

### Work authorized

- Complete or adapt required game-state and legality services.
- Integrate stack, priority, turn structure, state-based actions, triggers,
  replacement effects, and continuous effects for the supported subset.
- Add deterministic state cloning and replay.
- Add information-set observations and seeded random events.
- Profile correctness-critical execution paths.

### Required deliverables

- Engine adapter or bounded engine implementation.
- State cloning and deterministic replay.
- Legal-action API.
- Public/private observation API.
- End-to-end interaction and regression suite.
- Performance baseline without premature optimization.

### Gate 7 exit criteria

- [ ] Replaying the same action sequence and seed yields the same state trace.
- [ ] Hidden information does not leak through observations or legal-action APIs.
- [ ] Legal actions and state transitions pass adjudicated scenario tests.
- [ ] Clone/rollout execution cannot mutate the source state.
- [ ] Unsupported effects stop simulation explicitly.
- [ ] Correctness tests pass before performance targets are considered.

## 13. Phase 8 — Telemetry and established search

**Planned dates:** 2030-10-01–2031-03-31  
**Earliest Phase 9 start:** 2031-04-01  
**Gate 8 review:** 2031-03-31

### Objective

Adopt established simulation and search infrastructure without reopening solved
research questions.

### Work authorized

- Structured trajectory telemetry.
- Random and rule-based baseline agents.
- MCTS/UCT using existing implementations where practical.
- Information-set methods or ensemble determinization.
- Rollout-policy, pruning, and parallelization experiments.

### Required telemetry

```text
state_t
observation_t
legal_actions_t
chosen_action_t
state_t+1
reward_or_outcome
public_information
private_information
random_seed_or_event
rules_and_card_model_versions
```

### Gate 8 exit criteria

- [ ] Telemetry can reproduce every recorded transition.
- [ ] Private information access is explicit and testable.
- [ ] Random and rule-based agents establish correctness baselines.
- [ ] Search improves decisions on controlled scenarios without changing engine
      semantics.
- [ ] Search failures are separated from parser and engine failures.
- [ ] No custom search algorithm is invented without a measured need.

## 14. Phase 9 — Learning and final evaluation

**Planned dates:** 2031-04-01–2031-12-31  
**Final program review:** 2031-12-31

### Objective

Evaluate established learning methods on the validated environment and report
the actual boundary of automatic card understanding.

### Work authorized

- Supervised and imitation baselines.
- Reinforcement learning and self-play.
- Search-guided policies.
- Opponent modeling if the environment exposes the required information.
- Representation-learning experiments using validated IR and telemetry.

### Required deliverables

- Baseline comparison using fixed environment and card-model versions.
- Generalization evaluation on held-out decks, cards, and mechanics.
- Full-system error attribution.
- Reproducibility package.
- Final supported/unsupported capability statement.

### Final success criteria

- [ ] Environment correctness remains invariant across agent experiments.
- [ ] Agent improvements are measured against established baselines.
- [ ] Parser, executor, engine, search, and policy errors are attributable.
- [ ] New-card generalization is tested without card-specific implementation.
- [ ] Claims are limited to the demonstrated supported subset.
- [ ] Negative results and hard limits are reported.

Learning is optional. If the environment is not reliable at Gate 8, the program
ends with the environment findings rather than proceeding to AI.

## 15. Cross-phase evaluation strategy

### 15.1 Data partitions

Maintain separate partitions for:

- development examples;
- validation examples;
- frozen held-out examples;
- adversarial counterexamples;
- mechanic-held-out examples;
- structure-held-out examples;
- newly released zero-shot cards;
- interaction scenarios derived from rules and rulings.

### 15.2 Metric ladder

Progression must be measured at increasing levels:

1. structural-boundary correctness;
2. ability-kind and role correctness;
3. operator identification;
4. typed-argument extraction;
5. reference resolution;
6. rules-context retrieval;
7. whole-IR exactness or adjudicated equivalence;
8. static-validation success;
9. execution-trace agreement;
10. full interaction correctness;
11. unseen-card generalization.

High scores at a lower level do not substitute for evaluation at a higher one.

### 15.3 Authority hierarchy

Use evidence in this order:

1. Comprehensive Rules;
2. current Oracle text;
3. official rulings;
4. adjudicated interaction tests;
5. corpus measurements;
6. literature and existing engine behavior;
7. researcher interpretation as a hypothesis.

Existing engines are useful comparison points but are not normative authorities.

## 16. Architecture constraints carried forward

The following constraints apply unless a gate decision explicitly changes them:

- Parser output is data, not executable host-language code.
- The parser does not mutate game state.
- The executor does not interpret English.
- The agent does not depend on parser internals.
- Randomness is explicit and seedable.
- Player observations are distinct from authoritative state.
- Validation precedes execution.
- Every IR node can retain provenance to text and rules.
- Unsupported behavior is a typed outcome, not a best-effort guess.
- Generic primitives are preferred over card-specific branches.
- Versioned card data, rules, IR, and engine semantics accompany every result.

## 17. Risk register and containment

| Risk | Early signal | Containment |
|---|---|---|
| Line/template frequency is mistaken for semantics | High coverage but poor adjudicated agreement | Freeze semantic gold data before IR selection |
| IR is chosen too early | Examples are forced into awkward exceptions | Require two representation spikes and rejection evidence |
| Existing engine dictates research conclusions | Parser mirrors backend quirks | Keep normative tests independent of engine behavior |
| Manual card code hides failure | Coverage rises with exception count | Report manual and automatic coverage separately |
| Rules scope becomes unbounded | Vertical slice expands mechanic by mechanic | Gate each capability and preserve unsupported outcomes |
| Evaluation leakage | Held-out cards influence grammar or prompts | Freeze hashes and maintain challenge sets |
| Silent semantic errors | Plausible execution despite ambiguity | Typed validation and explicit uncertainty |
| AI work causes scope drift | Agent code begins before replay correctness | Enforce Gate 7 and Gate 8 prerequisites |
| GDL-II is treated as a complete answer | Arithmetic/layers/rule changes remain awkward | Test against the Phase 2 requirements matrix |
| Full Magic exceeds finite-model assumptions | Unbounded loops or rule-changing behavior | State supported assumptions and test bounded semantics |
| Schedule pressure weakens gates | Criteria are waived near target dates | Shift dates or stop; never advance on date alone |

## 18. Literature-derived build/buy/reuse decisions

### 18.1 Reuse rather than research

The reviewed literature supports treating these as established infrastructure:

- MCTS/UCT;
- rollout-policy enhancements;
- determinization and information-set search;
- propnet-style compiled reasoning;
- search parallelization;
- standard telemetry and learning methods.

These become integration work in Phases 8–9, not active research in earlier
phases.

### 18.2 Candidate foundations, not predetermined choices

GDL/GDL-II contributes important concepts:

- state as a fact database;
- declarative legality and transition rules;
- `random` for chance;
- `sees` for asymmetric observations;
- generated simulation from formal descriptions.

However, base GDL lacks native arithmetic and does not directly solve Magic's
continuous effects, layers, open-ended card language, or automatic
axiomatization. It remains a candidate until Gate 3.

### 18.3 The unresolved boundary

Every reviewed source starts after a machine-readable model exists. None
automatically converts Oracle text into executable semantics. That boundary is
why Phases 0–6 receive the majority of original research effort.

## 19. Literature evidence summary

### Ward and Cowling (2009)

**Monte Carlo Search Applied to Card Selection in Magic: The Gathering**  
DOI: 10.1109/CIG.2009.5286501

Demonstrates Monte Carlo card selection in a simplified, hand-built Magic
simulator. Rules, legal moves, combat, cards, decks, and heuristics were manually
implemented. Reusable later for search methodology; irrelevant to automatic
card semantics.

### Cowling, Ward, and Powley (2012)

**Ensemble Determinization in Monte Carlo Tree Search for the Imperfect
Information Card Game Magic: The Gathering**  
DOI: 10.1109/TCIAIG.2012.2204883

Shows that ensemble determinization, MCTS, pruning, and suitable rollout
policies can produce strong decisions once a simulator exists. The work still
uses a fixed, manually encoded lands-and-creatures game.

### Genesereth, Love, and Pell (2005)

**General Game Playing: Overview of the AAAI Competition**  
DOI: 10.1609/aimag.v26i2.1813

Establishes declarative game descriptions and generic reasoners. GDL can derive
legal actions and state transitions from axioms, but humans provide those
axioms, and base GDL assumes deterministic complete-information games.

### GDL-II and Thielscher

GDL-II adds `random` and `sees`, providing formal primitives for chance and
hidden information. It is universal for finite extensive-form games. It is the
leading architectural reference for Phase 3, but it still assumes
human-authored axioms and requires evaluation against Magic's arithmetic and
rule-changing semantics.

### Świechowski et al. (2015)

**Recent Advances in General Game Playing: A Comprehensive Survey**  
DOI: 10.1155/2015/986262

Shows that MCTS enhancements, propnets, knowledge extraction, and parallel
search are mature downstream infrastructure. The survey reinforces the program
decision not to spend early phases reinventing gameplay search.

## 20. Decision checklist before opening any later phase

Before work from a later phase begins, answer all of the following:

- Has the preceding gate passed in writing?
- Are the input data and rules versions frozen?
- Are development and held-out evaluations separated?
- Is the proposed work required by the active phase objective?
- Does it replace an existing deliverable or add scope?
- Can the result be evaluated objectively?
- Will unsupported cases remain visible?
- Does the work introduce card-specific behavior?
- Is an established library or engine available?
- Is this original research, required engineering, or optional downstream work?

If any answer is unknown, the work remains deferred.

## 21. Immediate authorized work

Until Gate 0 passes, the only active tasks are:

1. reconcile the current Rust segmenter with the current-state documentation;
2. rerun and record current corpus and Alpha measurements;
3. ensure accepted Alpha findings have regression tests;
4. freeze the next structural investigation protocol;
5. prepare the Gate 0 evidence package.

The next permitted research after Gate 0 is structural discovery. IR design,
engine implementation, simulation, search, telemetry, and learning remain
closed.

## 22. Deferred-work register

Items here are recorded, not scheduled. Each names the phase that may open
it and the evidence that motivated it. Nothing on this list is in scope until
its phase is active and a change record (§2.3) accepts it.

| Id | Item | Motivating evidence | Earliest phase | Owner |
|---|---|---|---|---|
| D1 | Record Scryfall bulk-snapshot identity in `cards.sqlite` and `info` (T1) | Gate 0 caveat A | Phase 1 (tooling) | Codex |
| D2 | Native `export --set` with stable unit keys (T2) replacing `export_units.py` | Protocol S3 | Phase 1 (tooling) | Codex |
| D3 | Re-measure Alpha B1 (typed-slot ablation), B2 (corpus recurrence) and V3 (printing invariance) with preserved scripts | Gate 0 caveat B | Phase 1 | research lead, after D2 |
| D4 | Keep `{T}`/`{Q}` distinct from mana in normalization (T10) | Alpha collision C1 (52 units) | Phase 1, via protocol S10 | proposal: research lead; change: Codex |
| D5 | Generic delayed-trigger detection for inverted, scoped `When …`-in-effect, reflexive `When you do`, and `at end of combat` forms (F1–F4) | Implemented 2026-08-26 via P-ARN-1/2; Alpha and Arabian Nights exports/metrics regenerated with zero drift. Unmarked Animate Dead and independent sentence-initial `When` remain deferred separately. | Complete | Codex |
| D6 | Type-line context for kind on instants/sorceries (F5) | Implemented 2026-08-26 via P-ARN-3; Disintegrate, Camouflage, and Eye for an Eye corrected; multiface cards use per-face type lines when available from ` // `-joined card type lines. | Complete | Codex |
| D7 | Prevention (CR 615) as a distinct kind | Implemented 2026-08-26 via P-ARN-4 as `prevention_effect`; Rock Hydra, Camel, and Desert Nomads corrected; activated/triggered prevention effects retain their cost/trigger kinds. | Complete | Codex |
| D14 | Splitting sentence-initial `When` clauses that are independent triggered abilities in the same paragraph (vanishing-style) | `arn` audit V2 class (c) | Phase 1, needs its own S8 search | research lead |
| D8 | Role value for referenced/lost quoted abilities (F11) | Animate Dead | Phase 1 taxonomy decision | research lead |
| D9 | CR-slot decomposition fields for activated/triggered units (T8) | Alpha V5, hypothesis N3 | Phase 1 (tooling) | Codex |
| D10 | Second annotator pass / adjudication of Alpha | Protocol S5.9; Gate 1 criterion | Phase 1 | second agent or owner |
| D11 | Held-out sampling and annotation from the §6.3 pool | Gate 1 criterion | Gate 1 review | research lead |
| D12 | Semantic hypotheses raised during audits (e.g. whether `{1}: Regenerate ~.` and `{B}: Regenerate ~.` are one operator) | Alpha | Phase 2 | — |
| D13 | Per-face characteristics for multi-face cards in the database | `docs/current-state.md` limitations | Phase 1 (tooling), when the walk reaches split/transform layouts | Codex |
