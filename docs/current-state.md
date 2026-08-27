# Current State: MTG Frontier Research

Last verified: 2026-08-26

## Purpose of this document

This is the primary handoff document for terminal research agents working in
this repository. It describes the latest verified state, the problem currently
being investigated, the evidence available, and the boundaries agents should
not cross without new evidence.

Keep the main body current rather than appending stale snapshots. Preserve only
important decisions and discoveries in the short dated log at the end. This is
not a detailed research journal, implementation specification, or substitute
for command output.

## Executive state

This repository is currently an empirical research workbench for the
approximately 30-year-old problem of translating Magic: The Gathering Oracle
text into machine-usable effect semantics.

The immediate objective is **not** to commit to an intermediate representation
(IR), build a complete parser, or implement a rules engine. The immediate
objective is to discover and measure the linguistic and semantic structure of
the full card corpus well enough that a later IR choice is evidence-driven.

The long-term destination is:

```text
Oracle text + Comprehensive Rules
    -> formal card semantics
    -> general MTG rules engine
    -> game simulation and telemetry
    -> search, machine learning, and AI agents
```

Only the first transition is the active research frontier. The repository does
not yet contain a formal semantic IR, executable card effects, a game-state
model, or a simulator.

## The overall problem

The core question is:

> Can arbitrary Oracle text be converted into correct, compositional,
> executable semantics without manually scripting every card?

This is harder than mapping sentences to labels. A successful account must
eventually represent at least:

- ability and card-face boundaries;
- keyword, activated, triggered, static, spell, modal, and replacement text;
- costs, events, conditions, choices, targets, quantities, durations, and
  zones;
- references between objects, players, abilities, and earlier choices;
- combinations and nesting of otherwise reusable effects;
- context supplied by the Comprehensive Rules rather than printed card text;
- continuous effects, layers, dependencies, replacement/prevention effects,
  linked abilities, state-based actions, and other resolution semantics;
- hidden information, randomness, multiplayer relationships, and arithmetic.

The present research must determine which distinctions are recoverable from
surface language, which require rules context, and which require explicit game
state or execution-time interpretation.

## Current research boundary

### Active goal

Discover and measure the corpus's linguistic and semantic structure before
choosing an IR.

This includes:

1. measuring reliable structural segmentation;
2. identifying repeated forms without assuming repeated meaning;
3. discovering candidate semantic operators and argument roles;
4. finding counterexamples to proposed normalizations or equivalence classes;
5. relating card wording to numbered rules, glossary definitions, and rulings;
6. defining evaluation data and measurable criteria for future parsers.

### Current non-goals

- Selecting GDL-II, an AST shape, a logic language, or another IR as final.
- Treating the existing normalizer as a parser.
- Building complete card execution or game simulation.
- Optimizing gameplay agents, MCTS, self-play, or deck construction.
- Claiming semantic equivalence from string similarity or frequency alone.
- Manually scripting cards as the primary solution.

GDL/GDL-II is a researched candidate and architectural reference, not a
decision. The literature reviewed so far begins after a machine-readable game
model already exists and therefore does not solve the active frontier.

## Repository map

| Path | Current role |
|---|---|
| `src/main.rs` | Rust `mtg-discover` CLI for structured corpus and rules exploration |
| `Cargo.toml`, `Cargo.lock` | Rust package and reproducible dependency resolution |
| `scripts/python/mtg_card_pipeline.py` | Fetch Scryfall bulk data (including all printings), load SQLite with first-printing columns, and run the original template baseline |
| `docs/findings/` | One document per completed investigation; read the newest after this file |
| `docs/protocol/structural-investigation-protocol.md` | Frozen (v1.0) set-by-set structural audit procedure, annotation schema, measurement definitions, held-out pool, tooling requirements |
| `docs/gates/` | One evidence package and decision record per roadmap gate (`gate-0-evidence.md`) |
| `docs/audits/<set>/` | Per-set artifact package: frozen unit export, unit-level annotation, `metrics.json` (Alpha done) |
| `scripts/python/export_units.py`, `scripts/python/audit_metrics.py` | Interim standard-library tools for the protocol (unit export; measurements, novelty, drift) |
| `scripts/python/mtg_search.py` | Human-oriented interactive card lookup |
| `.github/copilot-instructions.md`, `CLAUDE.md` | Agent onboarding, verified commands, architecture, and repository conventions |
| `docs/README.md` | Setup, command reference, and older pipeline documentation |
| `docs/RESEARCH_NOTES.md` | Literature findings and downstream architecture context |
| `Magic-Comprehensive_Rules.md` | Local Comprehensive Rules source used by discovery tools |
| `cards.sqlite` | Generated local card and ruling database |
| `oracle-cards.jsonl.gz` | Generated Scryfall Oracle Cards bulk input |
| `rulings.jsonl.gz` | Generated Scryfall rulings bulk input |
| `default-cards.jsonl.gz` | Generated Scryfall all-printings bulk input (first-printing derivation only) |
| `docs/mtg_ai_research_roadmap.md` | Long-term roadmap and literature map (Stages 1–7, success levels 0–6) |
| `target/` | Generated Rust build output |

The corpus files, SQLite database, and build output are regenerable local
artifacts and are not source code. Agents must not commit them.

## Verified data snapshot

The following values were produced by the release build of `mtg-discover` on
2026-08-26 from the Scryfall bulk snapshot of 2026-08-25 (oracle-cards,
rulings, and default-cards all from that day). Re-run the commands rather
than copying these values into later analysis if the corpus or rules file
may have changed.

### Corpus and authority data

| Measurement | Value |
|---|---:|
| Distinct Oracle-card records | 38,626 |
| Cards with Oracle text | 37,916 |
| Cards without Oracle text | 710 |
| Multi-face cards | 3,212 |
| Cards with a derived first printing | 38,626 (4,707 by fallback; 553 distinct sets) |
| Cards first printed in Alpha (`lea`) | 290 (275 with Oracle text) |
| Official ruling records | 78,949 |
| Parsed numbered rule entries | 3,455 |
| Parsed glossary entries | 752 |
| Comprehensive Rules effective date | 2026-08-07 |

Reproduce with:

```powershell
.\target\release\mtg-discover.exe info
```

### Current segmentation and normalization baseline

The Rust baseline splits Oracle text on lines, excludes `//` face separators,
removes reminder text, and builds a tree of typed units per card. Each unit
has a `kind` (heuristic CR category), a `role` (top-level ability, mode,
delayed trigger, granted/quoted ability), and a `source` (printed text or
rules-supplied semantics described only by reminder text). Keyword lists are
split into one unit per keyword; modes, delayed triggers created by an
effect, and quoted abilities nest under the unit they belong to.
Normalization replaces card-name and `this <type>` self-references with `~`
(preserving `named <name>` predicates), collapses mana symbols to `{M}` and
integers to `N`, strips `•`, and replaces a quoted ability with
`"[ability]"` in its parent.

| Measurement | Value |
|---|---:|
| Printed structural units | 71,563 |
| Rules-supplied units (reminder-only lines, counted separately) | 970 |
| Distinct normalized templates | 37,299 |
| Top 10 template coverage | 14.17% |
| Top 100 template coverage | 26.90% |
| Top 1,000 template coverage | 42.18% |
| Top 5,000 template coverage | 54.87% |
| Kinds | static/spell 19,519 · triggered 19,214 · keyword 17,840 · activated 11,998 · replacement 2,174 · additional cost 319 · CDA 255 · prevention 166 · cast restriction 69 · ante 9 |
| Roles | ability 67,045 · mode 2,121 · granted 1,506 · delayed trigger 891 |

The delayed-trigger role contains 861 nested children and 30 top-level
spell-created delayed triggers. The most frequent normalized unit is `Flying`
with 3,526 occurrences (4.93%).

Historical baseline (line = unit, raw-text classification, 2026-08-25 to
2026-08-26): 67,738 units, 37,912 templates, top-10/100/1,000/5,000 coverage
12.07% / 23.81% / 38.77% / 51.41%, `Flying` 2,812 (4.15%). The change is
analysed in `docs/findings/lea-segmentation-audit.md`.

Reproduce with:

```powershell
.\target\release\mtg-discover.exe templates --limit 100
```

These numbers measure the current segmentation and normalization procedure,
not the true number of effects. A distinct template may express the same
semantics as another template, and one template may express different
semantics in different contexts.

## Available research tools

Build once:

```powershell
cargo build --release
$mtg = ".\target\release\mtg-discover.exe"
```

| Command | Use |
|---|---|
| `& $mtg info` | Record corpus and rules metadata |
| `& $mtg cards <query>` | Search names, Oracle text, and type lines |
| `& $mtg cards <query> --field text` | Search only Oracle text |
| `& $mtg card <name-or-id> --rulings` | Inspect one exact card with rulings |
| `& $mtg rules search <query>` | Search numbered rules and glossary entries |
| `& $mtg rules show <id>` | Retrieve a rule and numeric or lettered descendants |
| `& $mtg segment --card <name>` | Inspect current segmentation and normalization |
| `& $mtg segment --text <text> --name <name>` | Probe synthetic or isolated text |
| `& $mtg templates` | Recompute template ranks and coverage |
| `& $mtg templates --set <code>` | Same, restricted to cards first printed in one set |
| `& $mtg cards <query> --set <code>` | Search restricted to one first-printing set |
| `& $mtg sets [--type <set_type>] [--until <date>]` | First-printing sets in release order with card counts |
| `python scripts/python/mtg_search.py <name>` | Human-oriented name lookup using the repository-root database |

Successful commands emit JSON to standard output. Errors use a nonzero exit
status. Prefer these structured interfaces over scraping human-oriented output.
The `rules` commands structurally extract numbered rules and glossary entries
from the Markdown document for search and retrieval. They do not translate the
Comprehensive Rules into semantic or executable game logic.

## Evidence hierarchy

Agents must keep facts, measurements, hypotheses, and design proposals
separate.

Use this authority order:

1. **Comprehensive Rules** for game definitions and normative mechanics.
2. **Current Oracle text** for what a card says.
3. **Official rulings** for clarifications and documented interactions.
4. **Corpus measurements** for frequency and distribution claims.
5. **Literature and existing engines** for prior approaches and architecture.
6. **Agent interpretation** only as a hypothesis to test.

Rulings are useful evidence but are not exhaustive semantic annotations.
Frequency is evidence of reuse, not proof of semantic identity. A rule search
hit is not proof that the rule completely determines a card's behavior.

## Known limitations and unresolved risks

### Data representation

- Multi-face cards store joined face text, but the database does not preserve
  complete per-face characteristics.
- First printing is derived (earliest paper, non-promo printing outside
  promo/token/memorabilia/minigame/alchemy sets). Oracle text is *current*
  wording, so ordering by first printing tracks when an effect entered the
  game, not how it was worded then.
- Oracle text is identical across all printings of a card (verified over
  116,843 printings), so reminder text is canonical, not printing noise.
- The corpus is a current Oracle snapshot, not a history of wording changes.
- Cards without Oracle text remain in the corpus and need explicit treatment.
- SQLite's default case-insensitive matching does not fully fold Unicode.

### Structural baseline

- Line boundaries are still the primary unit boundary. The segmenter now
  handles the Alpha and Arabian Nights failure modes (keyword lists, modes,
  supported `next` / `at end of combat` / scoped `When` delayed triggers,
  quoted abilities, reminder-only lines), but every rule is a surface-form
  heuristic: unmarked delayed triggers such as Animate Dead's final sentence,
  independent sentence-initial `When` abilities sharing a paragraph (D14),
  `Activate only ...` restrictions, ability words, and many multi-sentence
  units remain unsplit.
- A top-level `triggered_ability` unit on an instant/sorcery face whose
  trigger clause carries an explicit future/duration temporal scope (`this
  turn`, `this combat`, `next ...`; CR 603.7b) and no evidence it instead
  functions off the stack (a cycling/suspend/haunt keyword, or a self
  reference near a graveyard/exile/discard zone word; CR 113.6b) or is
  about the spell's own casting or resolution gets `role = delayed_trigger`
  in place, per CR 603.7d, rather than the default `role = ability`; `kind`
  stays `triggered_ability` (P-ATQ-4, implemented, not yet
  corpus-validated). It stays top-level (no `parent_index`), distinguishing
  it from a `delayed_trigger` *child* produced by the existing
  cross-line/sentence-boundary split (P-ARN-1/P-ATQ-1), which always has a
  parent.
- `kind` labels are heuristic. Replacement detection is lexical
  (`instead`/`skip`/enters-with/as/tapped) except that top-level instant and
  sorcery spell text is classified with type-line context; static prevention
  text is a distinct `prevention_effect` kind, excluding the `can't`/`cannot
  be prevented` prohibition idiom (P-ATQ-2), which falls through to the
  residual static kind instead; a leading `<prefix> — ` (ability word, Saga
  chapter symbol, or named mode/label; em dash, no period, no colon, ≤ 45
  characters) is detected and stripped before classification, and recorded
  on a new `prefix` field, so it can no longer hide the trigger word (or
  other classification evidence) that follows it (P-ATQ-3, implemented, not
  yet corpus-validated); a Saga chapter symbol (pure Roman numerals on a
  Saga type line) is classified `triggered_ability` per CR 714.2b regardless
  of its effect text's leading verb, rather than by classifying the
  stripped body; CDA detection covers the `~'s power and toughness are each
  equal to` form but not conditional forms (Gaea's Liege); payment
  restrictions (`Spend only black mana on X`) are still residual static
  text; short static sentences without a period are labelled keywords.
- Rules-supplied units carry a CR citation only for the basic-land mana
  ability form (305.6); the other 956 corpus reminder-only lines (Saga, Room,
  Class, Siege, scheme, conspiracy reminder text) are flagged but uncited.
- Quoted-ability detection (colon, trigger word, or ≥ 4 words) admits some
  non-ability quotations on Un-set and text-alteration cards.
- Reminder-text removal is lexical and does not model its semantic value.

### Normalization baseline

- All brace symbols collapse to `{M}` — mana symbols (losing color, generic,
  variable, alternative, and special-mana distinctions) **and also the tap
  and untap symbols `{T}` / `{Q}`**, so `{T}: Add {G}.` and `{G}: Add {C}.`
  share a template (verified 2026-08-26; 52 Alpha units carry `{T}`).
  Changing this is a normalization proposal (deferred item D4), not yet
  accepted.
- All integers collapse to `N`, losing semantic roles such as cost, damage,
  quantity, power/toughness modification, and counter count.
- Object descriptions, types, subtypes, zones, players, durations, and
  references are not typed.
- Self-reference normalization (`~`) covers the card name and a fixed list
  of `this <type>` words; `named <name>` is preserved. `this creature` inside
  a quoted granted ability also becomes `~` (it refers to the object that
  has the ability, not the granting card).
- Split keyword items are sentence-cased in the template (`Flying, trample`
  → `Flying`, `Trample`); `•` is stripped from modes, so a mode and a
  standalone sentence with the same wording share a template.
- Normalized-string equality is not semantic equivalence.

### Evaluation

- Structural annotations exist for Alpha (`docs/audits/lea/`, 417 units),
  Beta's two new cards (`docs/audits/leb/`) and Arabian Nights
  (`docs/audits/arn/`, 112 units) and Antiquities (`docs/audits/atq/`,
  125 units): CR citation per row. Alpha and Arabian Nights are
  single-annotator with lead review; **Antiquities is adjudicated** (two
  independent passes, 125 / 125 agreement). After P-ATQ-1's acceptance and
  re-annotation (2026-08-26): Alpha 415 units, boundary 396 / 400 (under 4,
  over 0, unsure 1), kind 388 / 389; Arabian Nights 110 units, 108 / 110
  (under 2); Antiquities 124 units, 122 / 124 (under 2, over 0), kind
  122 / 122, unit novelty 95 / 124 and template novelty 94 / 113 against the
  three earlier sets. The former `over` rows (condition-only parents of the
  rejected split rule (c)) are re-annotated as single `under` units with a
  recorded in-unit delayed trigger (D15 slot); defect totals are unchanged
  (Alpha 5, Arabian Nights 2, Antiquities 2). The committed exports are
  fresh and drift 0 from the annotations. These are development and
  regression sets, not gold sets and not evidence about the corpus.
- Corpus-wide S11 checks of the P-ARN and P-ATQ rules are scripted
  (`scripts/python/corpus_checks/`, reports in `docs/audits/corpus-checks/`):
  after P-ATQ-1, all 861 nested delayed-trigger children are sentence-level and
  no comma/colon fragments remain; after P-ATQ-2, no `can't be prevented`
  prohibition is labelled `prevention_effect`; P-ATQ-3 reduces the 8 recorded
  prefix-related prevention candidates to 3, which still require lead
  adjudication; P-ATQ-4 assigns `role = delayed_trigger` to exactly 30
  qualifying top-level instant/sorcery units.
- A held-out pool is frozen (protocol §6.3: `oracle_id` prefix `f`,
  non-fallback, excluding `lea`/`leb`/`arn`; 2,096 cards) but not yet sampled
  or annotated.
- There is no semantic annotation set and no agreed semantic operator
  inventory.
- There is no executable equivalence test or minimal rules model.
- Corpus-wide coverage currently measures normalized strings, not correctness.

## Research method for terminal agents

For each investigation:

1. Record `mtg-discover info` output or verify that the snapshot above is still
   current.
2. State one narrow, falsifiable hypothesis.
3. Define what observation would contradict it.
4. Search the corpus for supporting examples and likely counterexamples.
5. Inspect exact cards, relevant rulings, numbered rules, and glossary entries.
6. Probe the current segmenter to expose where structure is retained or lost.
7. Report counts and representative examples, including contradictions.
8. Label conclusions as verified findings, bounded observations, or hypotheses.
9. Update this document only when the repository-wide current state changes.
10. Add a short log entry only for a durable decision or material discovery.

Do not begin by inventing an ontology and fitting examples to it. Prefer
ablation studies and counterexample searches that reveal which distinctions the
data requires.

## Highest-value open questions

These are research questions, not an implementation backlog:

1. What is the smallest reliable structural unit above tokens and below a full
   card?
2. How much apparent template diversity comes from quantities, object
   descriptions, zones, durations, references, and other typed arguments?
3. Which surface forms are semantically equivalent, and which deceptively
   similar forms differ under the rules?
4. What candidate semantic operators recur across a large, stratified sample?
5. Which effects cannot be represented compositionally without rules or
   execution context?
6. How should a gold evaluation set be stratified by era, card type, wording
   frequency, mechanic, and rules complexity?
7. What minimum annotations would test segmentation, operator identification,
   argument extraction, reference resolution, and semantic equivalence
   separately?
8. How can numbered rules and glossary entries be linked to card text without
   treating lexical overlap as semantic grounding?

## Near-term investigation sequence

The corpus is walked **set by set in first-printing order** (Alpha →
present) so that wording complexity grows over time; each set gets a
findings document. Metric proposed for the walk: a set's *novelty rate*,
the share of its units whose template did not appear in any earlier set.

Unless new evidence changes priorities:

1. **Segmentation audit:** per set, following
   `docs/protocol/structural-investigation-protocol.md` (frozen v1.0).
   Alpha is done: its seven segmenter changes are implemented, tested and
   measured, and its unit-level annotation is committed. Gate 0 passed on
   2026-08-26 (`docs/gates/gate-0-evidence.md`). Arabian Nights (`arn`) and
   Beta (`leb`) are audited (`docs/findings/arn-structural-audit.md`);
   P-ARN-1..4 are implemented and reviewed (rule (c) rejected). Antiquities
   is audited and adjudicated (`docs/findings/atq-structural-audit.md`).
   P-ATQ-1 is accepted (technical S10 package 2026-08-26, isolated
   measurement at `bf9eb04`); P-ATQ-2..4 are implemented and
   research-accepted, pending their technical packages; P-ATQ-3 has 3
   residual prefix-related prevention candidates requiring adjudication. Legends (`leg`, 310 cards,
   290 with text — the last set below the 400-card exhaustive threshold
   before Ice Age) is next.
2. **Normalization ablations:** measure one reversible transformation at a time
   rather than applying increasingly lossy normalization as a bundle.
3. **Typed-slot discovery:** test candidate roles for numbers, mana, objects,
   zones, players, events, conditions, and durations.
4. **Semantic operator inventory:** derive operators from stratified examples
   and actively search for counterexamples.
5. **Evaluation-set design:** freeze a representative set before optimizing a
   parser against the complete corpus.
6. **IR comparison:** only after the preceding evidence exists, compare
   candidate representations against observed requirements.

## Maintenance contract

When updating this document:

- update `Last verified`;
- rerun affected commands and replace stale measurements;
- describe only behavior that exists in the repository;
- distinguish decisions from candidates and hypotheses;
- remove resolved questions or rewrite them to the new frontier;
- keep the repository map brief;
- keep the log short and link detailed findings elsewhere if they grow;
- do not silently change the active goal or long-term destination.

## Decision and discovery log

### 2026-08-25

- Decided that the near-term objective is empirical discovery and measurement
  before committing to an IR.
- Decided that this document primarily serves terminal agents such as Fable.
- Decided to maintain current truth plus a short dated decision/discovery log.
- Confirmed that the long-term destination is a general MTG rules engine,
  simulation, telemetry, and AI stack.
- Established the first Rust discovery baseline: 67,738 structural units and
  37,912 normalized templates; the top 5,000 cover 51.41% under the current
  lossy normalization.

### 2026-08-26

- Stated the working hypothesis under test: because Magic is a formal
  rules-based system, cards follow semantic patterns that an algorithm can
  parse without hand-coding the vast majority of effects. No coverage target;
  the goal is the upper bound of what is parseable.
- Decided to walk the corpus set by set from Alpha, recording each step in
  `docs/findings/`.
- Added first-printing derivation (`default_cards` bulk file, `first_*`
  columns) and `sets` / `--set` to the CLI. Baseline segmentation and
  normalization are unchanged, so corpus-wide numbers above still hold.
- Alpha audit: a line is not an ability (both directions); reminder text
  breaks keyword classification on 34% of keyword lines; 14 Alpha cards have
  rules-supplied abilities only; typed-slot ablations reduce Alpha's
  singleton share only from 68% to 56%; 47% of Alpha templates never recur
  in the corpus, half of those as parametric cycles. Verified that Oracle
  text does not vary across printings.
- Added repository-wide Copilot instructions aligned with this document and
  `CLAUDE.md`; restored `mtg_search.py` database discovery relative to the
  repository root so it works independently of the caller's working directory.
- Implemented the seven Alpha segmenter changes: classification on
  reminder-stripped text, keyword-list splitting, explicit rules-supplied
  units, new kinds (replacement, cast restriction, additional cost, CDA,
  ante), `this <type>` -> `~` with `named X` preserved, and nested modes,
  delayed triggers, and quoted abilities as typed children. This Alpha
  baseline was later superseded by the P-ARN updates recorded below.
- Gate 0 reviewed and passed with two recorded caveats (snapshot identity is
  prose-only; Alpha's B1/B2/V3 scratch measurements were not preserved and
  are downgraded to bounded observations). Froze the structural-investigation
  protocol v1.0 and the held-out pool. Annotated all 412 Alpha units; found
  5 missed nested delayed-trigger boundaries, 4 kind defects, and that `{T}`
  collapses to `{M}`. Narrowed three Alpha claims (activation instructions
  are not separate abilities per CR 602.1b; Animate Dead's last sentence is
  a delayed trigger; Gaea's Liege's CDA status is ambiguous under CR
  604.3a(5)). Arabian Nights plan prepared, not started.
- Gate 0 countersigned by the owner. Arabian Nights (77 cards, 109 units)
  and Beta (2 cards) audited under protocol v1.0: 3 more missed nested
  delayed triggers (Rukh Egg, Sandals of Abdallah, Nafs Asp), 1 more
  `instead`-on-instant kind defect (Eye for an Eye), 2 more prevention
  statics (Camel, Desert Nomads); unit novelty 77 / 109 vs Alpha + Beta.
  Corpus counterexample searches (535 inverted delayed-trigger hits, 154
  `at end of combat`, 798 sentence-initial `When`) support a generic
  sentence-level delayed-trigger split but show the delayed-vs-independent
  *role* needs CR context (reflexive triggers, CR 603.12; vanishing-style
  triggers). Alpha's Cockatrice / Thicket Basilisk adjudicated to missed
  delayed triggers via the Gorgon Recluse ruling.
- Implemented P-ARN-1 through P-ARN-4. The corpus baseline is now 71,682
  printed units + 970 rules-supplied, 37,344 templates, top-100 coverage
  26.85%, with 982 delayed-trigger children and 181 `prevention_effect`
  units. Re-exported and regenerated metrics for `lea`, `leb`, and `arn`;
  all three report zero drift. Arabian Nights now has 112 / 112 boundary,
  role, and source accuracy and 110 / 110 kind accuracy. Antiquities is
  cleared to begin, but Codex did not start that research.
- Lead review of `af150b0`: P-ARN-2/3/4 and sentence-level P-ARN-1 ratified;
  the un-proposed single-sentence split rule (c) rejected on corpus evidence
  (0/40 sampled comma/colon parents are reference units; 108 bare-condition
  parents; 3 in-quote splits). Condition-only parents in `lea`/`arn`/`atq`
  re-dispositioned `over`/`defect`. Fixed `export_units.py` name-collision
  defect (Shapeshifter vs. tokens); script and native `audit export` now
  agree field-for-field on `atq`. Antiquities audited with two passes
  (125/125 agreement): one unscoped-`When` miss (Tawnos's Coffin), one
  rule-(c) fragment (Battering Ram); unit novelty rose to 96/125, falsifying
  N1 as stated (novelty tracks theme, not only date). Corpus S11 checks
  preserved as scripts. Proposals P-ATQ-1..4 recorded; D15–D20 registered.
- Implemented P-ATQ-1: `delayed_trigger_split` in `src/main.rs` no longer
  searches backward for the nearest comma/colon before a delayed-trigger
  phrase in a single sentence (the rejected rule (c)); it only returns a
  split point at a complete sentence boundary (P-ARN-1 generic/inverted
  `next`/`at end of combat`, P-ARN-2 scoped `When`/`Whenever ... this
  turn`/`this way`/`When you do`). When a delayed-trigger phrase remains in
  an unresolved single sentence, the unit is kept whole and the existing
  `delayed_trigger_unattached_candidate` audit signal (T8-style slot) fires
  instead of a fabricated split; this required no new mechanism. Added
  `src/main.rs` regression tests: sentence-level splitting still creates a
  `delayed_trigger` child and a valid parent
  (`sentence_level_delayed_trigger_still_splits_as_a_child`); a leading
  `Whenever CONDITION,` trigger clause is no longer split off as its own
  parent, for both the plain and inverted-`next`-phrase forms
  (`end_of_combat_delayed_trigger_in_a_single_sentence_stays_whole`,
  `inverted_next_step_delayed_trigger_in_a_single_sentence_stays_whole`); an
  activation-cost colon (`{T}:`) is never emitted as its own unit
  (`activation_cost_colon_is_not_split_into_its_own_parent`); a quoted
  granted ability's internal comma/colon is never used as a split point,
  for either the outer or the inner unit
  (`delayed_trigger_and_punctuation_inside_quotes_are_not_split`,
  `delayed_trigger_inside_quoted_ability_stays_under_granted_child`); and
  the unattached-trigger signal fires for the conservative-fallback case
  (`suspicious_signals_flag_unresolved_single_sentence_delayed_trigger`).
  `cargo fmt -- --check`, `cargo test` (45 passed), `cargo clippy
  --all-targets -- -D warnings`, and `cargo build --release` all pass at
  this change. **Accepted 2026-08-26** (technical S10 package, branch
  `claude/p-atq-1-acceptance`): measured in isolation at `bf9eb04` against
  `8c0f229` on the same snapshot — 71,682 → 71,563 printed units, 37,344 →
  37,299 templates, 982 → 861 delayed-trigger children (all 121 comma/colon
  children removed, not the estimated ~113, because rule (c) was deleted
  rather than guarded; 0 added; 0 lowercase-initial; 0 in-quote), every
  merged unit carrying `delayed_trigger_unattached_candidate`. Regression:
  fresh exports drift in exactly the five fix rows, re-annotated `under`
  (missed 1, D15 slot), drift 0, no new non-`accept` row. Full record in
  `docs/findings/atq-structural-audit.md` ("P-ATQ-1 acceptance record").
- Implemented P-ATQ-2: `classify_kind` in `src/main.rs` no longer labels
  `can't be prevented` / `cannot be prevented` text as `prevention_effect`.
  A new `prevention_prohibition` regex (`can(?:'|’)?t be prevented|cannot be
  prevented`, matched against the same lowercased normalized text as the
  existing `prevention` regex, apostrophe optional/either form since
  normalization does not fold apostrophes) is checked alongside the
  existing prevention match; when both match, the unit falls through the
  existing `else if` chain (replacement, then CDA, then residual) exactly
  as it would if the prevention regex hadn't matched at all — no new kind,
  no reordering of the surrounding branches, no card- or set-specific
  logic. Added `src/main.rs` regression tests:
  `prevention_prohibition_is_not_classified_as_prevention_effect` (two
  distinct `can't be prevented` wordings plus `cannot be prevented` and a
  curly-apostrophe `can’t be prevented` variant all classify as
  `spell_or_static_text`, matching the existing residual-static fallback)
  and `prevention_prohibition_exclusion_does_not_regress_genuine_prevention`
  (a unit that both commands genuine prevention and separately describes
  damage as "is prevented" still classifies as `prevention_effect`, showing
  the exclusion is the narrow collocation and not a blanket `contains
  "prevented"` rule). All prior prevention/replacement/CDA tests, including
  `static_prevention_effects_have_their_own_kind` and
  `prevention_in_activated_triggered_or_spell_text_keeps_precedence`, were
  left unchanged and still pass — none of their fixtures used the
  prohibition wording. `cargo fmt -- --check`, `cargo test` (47 passed),
  `cargo clippy --all-targets -- -D warnings`, and `cargo build --release`
  all pass. **Not yet done, same blocker as P-ATQ-1:** this session's
  network egress policy again returns 403 for `api.scryfall.com`, so
  `cards.sqlite` could not be (re)generated and neither
  `scripts/python/corpus_checks/check_kind_rules.py` (which reads
  `cards.sqlite` directly for type-line lookups) nor
  `scripts/python/corpus_checks/dump_corpus_units.py` (its required input)
  could be run. The Antiquities audit's 9 `can't be prevented` misfires are
  therefore not re-measured, and the 8 ability-word/Saga-chapter/named-mode
  prefixed misfires from the same check (P-ATQ-3, explicitly out of scope
  here) are expected to remain untouched but likewise unverified. A search
  of the local `Magic-Comprehensive_Rules.md` text and the existing
  `src/main.rs` test fixtures (all of which already use a straight ASCII
  apostrophe for `can't`, consistent with Scryfall's Oracle-text
  convention) informed the regex, but this substitutes for the protocol's
  S8 corpus counterexample search rather than satisfying it. P-ATQ-2 is
  implemented and unit-tested but not yet accepted under protocol S10
  (items 4–5); a later session with data access must rerun
  `check_kind_rules.py`, confirm the 9-misfire class is gone with no new
  false positives or negatives, and refresh the baseline numbers above.
- Implemented P-ATQ-3: `build_unit` in `src/main.rs` now detects a leading
  `<prefix> — ` structural marker on the fully normalized unit text (em
  dash, prefix has no period or colon, prefix ≤ 45 characters, non-empty
  body after the dash) via a new `extract_prefix` function, before
  `classify_kind` runs. The detected prefix is recorded verbatim on a new
  `prefix: Option<String>` field on `Segment` (only field added; no new
  ontology of prefix categories); `text` and `normalized` are unchanged, so
  the original Oracle text and the existing corpus-wide template baseline
  are both preserved exactly. When no prefix is found, behavior is
  byte-for-byte identical to before this change. When a prefix is found,
  two cases: (1) a Saga chapter symbol — one or more comma-separated pure
  Roman numerals (`is_saga_chapter_prefix`) *and* the unit's per-face type
  line carries the Saga subtype (`is_saga`) — is classified
  `triggered_ability` directly, per CR 714.2b's "is a keyword ability that
  represents a triggered ability," without running the stripped body
  through `classify_kind` at all (stripping and classifying the body would
  reproduce the P-ATQ-3 failure as `prevention_effect` when the effect text
  starts with "Prevent", which is exactly the corpus-observed case); (2)
  every other prefix (ability word, CR 207.2c; named mode/label; a
  non-Saga numeral label) is stripped and the remaining body is classified
  by the existing, unmodified `classify_kind` with the same `type_line` and
  `allow_spell_text_override` the whole unit would have received, so a
  hidden `Whenever`/`At` trigger word is recovered and P-ARN-3's
  instant/sorcery spell-text override, the CDA check, and every other
  branch keep working unchanged on the shorter body. No card name, set
  code, or ability-word vocabulary list appears in the implementation; the
  rule is purely structural (delimiter, length, punctuation, and — for the
  chapter case only — the CR-defined Roman-numeral/Saga-type gate).
  Regression tests added in `src/main.rs` (16 new, all synthetic, none
  naming an Antiquities card): an ability-word prefix over a `Whenever`
  trigger and over an `At the beginning of` trigger (guards against a fix
  that only handles one trigger word); a multi-chapter (`I, II —`) and a
  single-chapter (`II —`) Saga marker, both asserting `triggered_ability`
  even though the body starts with `Prevent`; a Roman-numeral prefix on a
  *non*-Saga type line asserting it is **not** treated as a chapter symbol;
  a named-mode prefix (`Run and Hide —`) inside an actual modal spell's `•`
  child, asserting the mode `role` and the CR 615.1a `prevention_effect`
  body kind are both unchanged from what the existing prevention machinery
  already produces; an early-colon guard, an early-period guard, and an
  overlong-prefix guard, each asserting `prefix` stays `None` and the unit
  classifies exactly as it did before this change; a mode-header em dash
  with no following body (`Choose one —`) asserting no prefix is recorded
  over its own bullet children — the one "ordinary em-dash usage" case this
  session could verify against a real, extremely common corpus pattern
  without database access (see below); the P-ATQ-2 `can't be prevented`
  exclusion and a prefix-free genuine-prevention case, both reconfirmed
  unaffected; and two direct unit tests of `extract_prefix` and
  `is_saga_chapter_prefix`. `cargo fmt -- --check`, `cargo test` (61
  passed), `cargo clippy --all-targets -- -D warnings`, and `cargo build
  --release` all pass.

  **Not yet done:** this session's network egress policy again returns 403
  for `api.scryfall.com` (re-confirmed this session) and no `cards.sqlite`
  exists, the same blocker recorded against P-ATQ-1 and P-ATQ-2. The
  protocol's S8 corpus counterexample search for the prefix rule (§18 of
  the P-ATQ-3 task: searching specifically for short, punctuation-clean,
  em-dash-joined constructions that are *not* an ability word, chapter
  symbol, or named mode, where stripping would be semantically wrong) was
  **not performed** against the corpus; it is informed only by the
  Antiquities audit's own recorded evidence (the 8-unit prefix family in
  `docs/audits/corpus-checks/2026-08-26-kind-rules-check.md` §A2), CR
  207.2c/714.2, and this session's knowledge of Magic Oracle-text
  conventions (planeswalker loyalty abilities use a colon, not an em dash,
  so they cannot collide with this rule; no non-label short em-dash
  construction under the 45-character/no-period/no-colon bound was
  identified by inspection). The S11 corpus-wide over-segmentation check
  (candidate units matched, prefixes extracted by length/role/kind/card
  type/release year, false-positive rate) was **not run**, so the
  before/after `prevention_effect` count, the corpus-wide kind/role
  histograms, and whether all 8 historical Antiquities misfires actually
  change kind under this rule are **not measured** in this session.
  Reasoning through the 8 recorded misfires against this implementation
  (not corpus-verified): 3 are ability words whose hidden trigger word is
  now recovered (`Heroic`, `Constellation`, `Lieutenant` — Favored Hoplite,
  Harvestguard Alseids, Loyal Unicorn in the audit's own wording), kind
  changes from `prevention_effect` to `triggered_ability`; 2 are genuine
  Saga chapter markers that now classify `triggered_ability` via the
  chapter-symbol path rather than by the body's leading verb (`I, II —`,
  `II —`); the remaining 3 (`2 —` on a non-Saga Un-set card, `Immune —`,
  `The Betrayer —`) have bodies that already begin with `Prevent` or `If
  ... would ... prevent`, a wording `classify_kind` already assigns
  `prevention_effect` with or without the prefix present, so this rule is
  not expected to change their kind — they were still evidence for the
  general structural phenomenon, and now carry recorded prefix metadata,
  but are not "misfires" this change resolves. This reasoning is stated as
  a hypothesis from the audit's recorded wording, not a corpus
  measurement. `docs/findings/atq-structural-audit.md` records the same
  caveat. A later session with data access must run
  `dump_corpus_units.py` and `check_kind_rules.py`, confirm the actual
  before/after `prevention_effect` count and kind distribution, execute
  the S8 counterexample search and S11 over-segmentation check this
  session could not run, and only then treat P-ATQ-3 as accepted.
- Implemented P-ATQ-4: a new `apply_spell_created_delayed_triggers` pass in
  `src/main.rs` runs once per card face, after `segment_text` has already
  attached every line's unit (mode children, P-ARN-1/P-ATQ-1 delayed-trigger
  children, granted quoted abilities), and changes `role` from `ability` to
  `delayed_trigger` **in place** — never reparenting or duplicating a unit —
  on a top-level unit that is `source = printed`, `kind = triggered_ability`,
  on an instant/sorcery face, and satisfies a new
  `is_spell_created_delayed_trigger` predicate. That predicate requires a
  stated future/duration temporal scope in the CR 603.7b sense
  (`has_delayed_trigger_temporal_scope`: `this turn`, `this combat`, or
  `next ...`) **and** the absence of two kinds of negative evidence:
  `is_cast_or_resolve_trigger` (the condition is about the spell's own
  casting/resolution: `when you cast ~`, `~ resolves`, `~ is countered`) and
  `has_off_stack_evidence` (a CR-defined off-stack keyword — `cycle`,
  `suspended`, `haunts` — or the unit's self-reference `~` within the same
  sentence as a `graveyard`/`exile`/`discard` zone word, CR 113.6b). The
  zone check is a proximity match on `~`, not a bare word blacklist, so it
  does not misfire on a delayed trigger that merely mentions a graveyard as
  the destination of its effect rather than as the ability's own zone
  (e.g. "return **those cards** from your graveyard" vs. "if **this
  card/`~`** is in your graveyard"). All four helper functions operate on
  the same P-ATQ-3 classification text (`extract_prefix`'s stripped body
  when a prefix is present, else the full normalized text) that
  `classify_kind` already used to assign `triggered_ability`, so a prefix
  cannot hide P-ATQ-4's evidence either. `kind` is never changed. No `kind`
  variant, card name, or set code appears in the implementation.

  The distinct concrete failure this corrects: a single-line instant or
  sorcery whose *entire* printed text is a CR 603.7d delayed-trigger clause
  (e.g. "Whenever a creature blocks this turn, ...") previously kept
  `role = ability`, because the repository's existing cross-line delayed-
  trigger mechanism (the `delayed_trigger_start`-based check already in
  `segment_line`, used for e.g. a trailing "At the beginning of the next end
  step, ..." *line* that continues an earlier line's effect) only keeps
  `role = delayed_trigger` when it can attach the unit as a *child* of a
  preceding sibling; a lone top-level unit has none, so it fell back to
  `role = ability`. P-ATQ-4 is a separate mechanism for exactly that case:
  it changes an existing top-level unit's role in place and deliberately
  never attaches it as anyone's child, since resolving the spell and the
  delayed trigger it creates are the same printed unit (CR 603.7d: "the
  source of that delayed triggered ability is that spell"). `parent_index`
  stays absent either way, which is what distinguishes a P-ATQ-4 unit from
  a `delayed_trigger` *child* produced by the pre-existing mechanism (which
  always has a parent) and from a `granted` delayed trigger inside a quoted
  ability (a different `role` entirely) — no new field was needed to keep
  the three cases distinguishable in audit output.

  Regression tests added in `src/main.rs` (21 new; `cargo test`: 82 passed,
  up from 61), all synthetic: the four positive temporal-scope forms (`this
  turn`, `you next cast ... this turn`, `at the beginning of combat this
  turn`, `this combat`), including the single-line no-parent case that was
  the concrete pre-P-ATQ-4 defect; the `cycle`/graveyard-self/`discard
  this card`/`suspended` off-stack negative classes; a direct positive
  counterexample proving the zone check is proximity-based, not a
  blacklist (a delayed trigger scoped `this turn` that returns *someone
  else's* discarded cards from a graveyard is not excluded); a cast trigger
  that also contains a `this turn` phrase, proving the cast/resolve
  exclusion takes precedence rather than merely being redundant with it; a
  resolution trigger; identical wording on a non-instant/sorcery face
  (type-line context matters); non-trigger spell text containing `this
  turn` (kind gates the check before role does); the existing P-ARN-1/
  P-ATQ-1 multi-line delayed-trigger *child* split, reconfirmed unchanged
  in shape; P-ATQ-2's `can't be prevented` exclusion and P-ATQ-3's Saga-
  chapter/ability-word prefix handling, both reconfirmed unaffected; a
  case combining a P-ATQ-3 ability-word prefix with a P-ATQ-4 temporal
  clause, showing the prefix does not hide the evidence from either
  mechanism; and direct unit tests of the three new helper predicates.
  `cargo fmt -- --check`, `cargo test` (82 passed), `cargo clippy
  --all-targets -- -D warnings`, and `cargo build --release` all pass.

  **Partial corpus cross-check, not a full S8/S11 pass:** this session's
  network egress policy again returns 403 for `api.scryfall.com`
  (re-confirmed), so `cards.sqlite` does not exist and neither
  `dump_corpus_units.py` nor `check_kind_rules_part2.py` could run against
  the live corpus — the same blocker as P-ATQ-1/2/3. However, the 111
  I/S-face top-level `triggered_ability` units (105 non-pool) recorded
  verbatim in the already-committed
  `docs/audits/corpus-checks/2026-08-26-kind-rules-check.md` §B/§B2 are
  themselves reproducible evidence: this session ran the release binary's
  `segment` command against all 105 non-pool texts and their recorded type
  lines. 28 were classified `delayed_trigger` and 77 stayed `ability`, with
  no unexpected role or an execution error on any unit; every one of the 28
  matches this session's own by-hand CR-based classification of the same
  105 rows, and the two units whose text contains both a trigger-word start
  and a temporal-scope phrase yet were *not* flagged (Sunfire Balm: `cycle`;
  Show of Confidence: `cast this spell`) are exactly the off-stack and
  cast/resolve counterexamples this proposal is designed to exclude.
  Diffing this rule's output against `check_kind_rules_part2.py`'s own
  blacklist-based measurement heuristic on the same 105 rows found exactly
  two disagreements, both resolving in the new rule's favor on CR grounds:
  the old heuristic's `if`/`elif` ordering lets its temporal-scope branch
  win *before* its cast/resolve branch ever runs, so it misclassifies "When
  you cast [this spell], copy it for each ... spell you've cast this turn."
  as a delayed trigger; and the old heuristic's bare-word `graveyard`
  exclusion misclassifies a genuine delayed trigger ("... return **those**
  [other, discarded] cards from your graveyard to your hand") as off-stack,
  exactly the false-exclusion class the task description warned this
  session to check for. This is a strong desk cross-check against evidence
  already committed to the repository, not a freshly executed S8 corpus
  query or an S11 over-segmentation pass: it does not cover the full
  12,468-unit I/S-face population, the 6 held-out-pool rows in the 111 (not
  inspected, consistent with protocol §6.3), or corpus-wide false negatives
  outside the 111 units the earlier audit already flagged as
  `triggered_ability`. P-ATQ-4 is implemented and unit-tested, corroborated
  against the recorded historical evidence, but **not yet accepted under
  protocol S10/S11**. A later session with data access must run
  `dump_corpus_units.py` and `check_kind_rules_part2.py`, confirm the
  before/after role distribution and the ~30-unit historical comparison
  point on the live corpus, execute a true S8 counterexample search over
  the full I/S-face population (not just the 111 already flagged
  `triggered_ability` — a false negative could be hiding in a different
  temporal phrasing this proposal does not yet cover), rerun
  `audit_metrics.py` against `lea`/`leb`/`arn`/`atq` to confirm no new
  non-`accept` rows, and only then treat P-ATQ-4 as accepted. This session
  also could not run the still-outstanding P-ATQ-1/P-ATQ-2/P-ATQ-3 corpus
  validation for the same reason; that work remains open exactly as
  recorded in their own disposition entries above, unchanged by this
  session.
- Re-ran the full local corpus after merging P-ATQ-1..4: 71,563 printed units,
  37,299 templates, 861 sentence-level delayed-trigger children, 30 top-level
  spell-created delayed triggers, 0 comma/colon delayed splits, 166
  `prevention_effect` units, and 0 `can't be prevented` prohibition misfires.
  P-ATQ-3 reduced the 8 recorded prefix-related prevention candidates to 3;
  these remain for research-lead adjudication. All four implementations remain
  pending the protocol-required re-annotation and acceptance decision.
- Accepted P-ATQ-1 under protocol S10 (decider: research lead, per
  `docs/findings/p-atq-research-acceptance-assessment.md`; technical package
  on branch `claude/p-atq-1-acceptance`). The rule (c) retraction was
  measured in isolation (`8c0f229` → `bf9eb04`, same snapshot): all 121
  comma/colon delayed children revert (982 → 861; the proposal had estimated
  ~113), 0 children added, the surviving 861 are identical to HEAD's nested
  set, and every merged unit carries the T8-style
  `delayed_trigger_unattached_candidate` signal. Fresh `lea`/`leb`/`arn`/`atq`
  exports and metrics are committed; the five fix rows are re-annotated as
  `under` (missed 1) with defect totals unchanged. The corpus-check scripts
  gained a binary-path override and a commit-label argument. P-ATQ-2..4
  technical packages, D19 and D14 remain open.
