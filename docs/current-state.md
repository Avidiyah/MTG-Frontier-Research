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
| Printed structural units | 71,682 |
| Rules-supplied units (reminder-only lines, counted separately) | 970 |
| Distinct normalized templates | 37,344 |
| Top 10 template coverage | 14.15% |
| Top 100 template coverage | 26.85% |
| Top 1,000 template coverage | 42.15% |
| Top 5,000 template coverage | 54.88% |
| Kinds | static/spell 21,521 · triggered 17,503 · keyword 17,630 · activated 12,000 · replacement 2,208 · additional cost 317 · CDA 245 · prevention 181 · cast restriction 68 · ante 9 |
| Roles | ability 67,075 · mode 2,121 · granted 1,504 · delayed trigger 982 |

The most frequent normalized unit is `Flying` with 3,526 occurrences (4.92%).

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
- `kind` labels are heuristic. Replacement detection is lexical
  (`instead`/`skip`/enters-with/as/tapped) except that top-level instant and
  sorcery spell text is classified with type-line context; static prevention
  text is a distinct `prevention_effect` kind; CDA detection covers the
  `~'s power and toughness are each equal to` form but not conditional
  forms (Gaea's Liege); payment restrictions (`Spend only black mana on X`)
  are still residual static text; short static sentences without a period
  are labelled keywords.
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
  independent passes, 125 / 125 agreement). After P-ARN-1..4 and the lead
  review: Alpha boundary 398 / 402 (under 2, over 2, unsure 1), kind
  392 / 393; Arabian Nights 110 / 112 (over 2); Antiquities 123 / 125
  (under 1, over 1), kind 123 / 123, unit novelty 96 / 125 and template
  novelty 95 / 114 against the three earlier sets. The `over` rows are
  condition-only parents produced by the single-sentence split rule (c),
  rejected in review (P-ATQ-1). These are development and regression sets,
  not gold sets and not evidence about the corpus.
- Corpus-wide S11 checks of the P-ARN rules are scripted
  (`scripts/python/corpus_checks/`, reports in `docs/audits/corpus-checks/`):
  delayed-trigger splits 982 (sentence-level 861 sound; 121 comma/colon
  fragments and 3 in-quote splits defective); `prevention_effect` 181
  (144 / 161 top-level correct; 9 `can't be prevented`, 8 ability-word /
  chapter-prefixed misfires); instant/sorcery faces carry 0 lexical
  replacement/prevention labels but 30 spell-only delayed triggers are
  labelled top-level triggers.
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
   is audited and adjudicated (`docs/findings/atq-structural-audit.md`)
   with proposals P-ATQ-1..4 awaiting Codex. Legends (`leg`, 310 cards,
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
