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

### Current normalization baseline

The Rust baseline splits Oracle text on lines, excludes `//` face separators,
classifies a small set of structural forms, removes reminder text, replaces
self-references, and collapses mana symbols and integers.

| Measurement | Value |
|---|---:|
| Structural text units | 67,738 |
| Distinct normalized templates | 37,912 |
| Top 10 template coverage | 12.07% |
| Top 100 template coverage | 23.81% |
| Top 1,000 template coverage | 38.77% |
| Top 5,000 template coverage | 51.41% |

The most frequent normalized unit is `Flying` with 2,812 occurrences (4.15%).

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

- Line boundaries are only a first approximation of ability boundaries. The
  Alpha audit (`docs/findings/lea-segmentation-audit.md`) found both
  directions of failure: keyword lists and embedded delayed triggers put
  several abilities on one line; Siren's Call and modal spells spread one
  ability over several lines.
- The classifier runs on raw text while the normalizer strips reminder
  text, so 34% of Alpha's keyword lines are labelled static text.
- Reminder-only lines (basic and dual lands) normalize to nothing; their
  ability is supplied by CR 305.6, not by text.
- Replacement effects, casting restrictions, additional costs,
  characteristic-defining abilities, ante text, and quoted (granted)
  abilities are all present in Alpha and all collapse into
  `spell_or_static_text` or are mislabelled.
- Modal headers, modes, ability words, nested instructions, and
  paragraph-spanning structures need stronger segmentation.
- Reminder-text removal is lexical and does not model its semantic value.

### Normalization baseline

- All mana symbols collapse to `{M}`, losing color, generic, variable,
  alternative, and special-mana distinctions.
- All integers collapse to `N`, losing semantic roles such as cost, damage,
  quantity, power/toughness modification, and counter count.
- Object descriptions, types, subtypes, zones, players, durations, and
  references are not typed.
- Self-reference has two surface forms (card name; `this <type>`) and only
  the name form is normalized. `named X` is a counterexample to blind name
  replacement (Plague Rats).
- Normalized-string equality is not semantic equivalence.

### Evaluation

- There is no gold-standard segmentation or semantic annotation set.
- There is no agreed semantic operator inventory.
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

1. **Segmentation audit:** per set, classify failures across card faces,
   paragraphs, modal spells, keyword lists, ability words, and nested text.
   Alpha is done; Arabian Nights (`arn`) is next. Decide on the segmenter
   changes proposed in the Alpha findings before or during that step.
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
