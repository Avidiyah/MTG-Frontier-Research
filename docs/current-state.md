# Current State: MTG Frontier Research

Last verified: 2026-08-28

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
| `src/main.rs`, `src/cli.rs` | Thin Rust `mtg-discover` entry point and CLI argument definitions |
| `src/cards.rs`, `src/rules.rs`, `src/segment.rs`, `src/audit.rs` | Card/set queries, rules access, segmentation/template analysis, and structural-audit commands |
| `src/database.rs`, `src/util.rs`, `src/tests/` | Shared read-only database policy, JSON measurement helpers, and subsystem-oriented unit tests |
| `Cargo.toml`, `Cargo.lock` | Rust package and reproducible dependency resolution |
| `scripts/python/mtg_card_pipeline.py` | Fetch Scryfall bulk data (including all printings), load SQLite with first-printing columns, and run the original template baseline |
| `docs/findings/` | One document per completed investigation; read the newest after this file |
| `docs/protocol/structural-investigation-protocol.md` | Frozen (v1.0) set-by-set structural audit procedure, annotation schema, measurement definitions, held-out pool, tooling requirements |
| `docs/gates/` | One evidence package and decision record per roadmap gate (`gate-0-evidence.md`) |
| `docs/audits/<set>/` | Per-set artifact package: frozen unit export, unit-level annotation, `metrics.json` (Alpha done) |
| `scripts/python/export_units.py`, `scripts/python/audit_metrics.py` | Interim standard-library tools for the protocol (unit export; measurements, novelty, drift) |
| `scripts/python/verify_export_safety.py`, `scripts/python/verify_manifests.py` | Aggregate-only T7/determinism verification and lightweight provenance validation |
| `docs/manifests/` | Committed snapshot and experiment identities; generated bulk/database/export files remain uncommitted |
| `scripts/python/mtg_search.py` | Human-oriented interactive card lookup |
| `docs/agent/agent-contract.md` | Canonical, tool-agnostic agent operating contract (routing, evidence hierarchy, commands, architecture, handoff) |
| `.github/copilot-instructions.md`, `CLAUDE.md`, `AGENTS.md` | Thin, tool-specific entry points that point to `docs/agent/agent-contract.md` |
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
  stays `triggered_ability` (P-ATQ-4, accepted 2026-08-26 after a full
  instant/sorcery-face S8 sweep; 30 such units corpus-wide). It stays top-level (no `parent_index`), distinguishing
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
  other classification evidence) that follows it (P-ATQ-3, accepted within
  its measured scope 2026-08-26 — 3,572 firings, 0 binary/search
  mismatches; the 3 residual prefix-related prevention rows were
  adjudicated correct positives by the research lead in
  `docs/findings/p-atq-research-acceptance-assessment.md`; known bounded
  counterexamples: Prototype/spree/table-label prefixes, 2 flavor words
  containing sentence punctuation, and 141 short stripped bodies on funny/token
  sets that `is_keyword_line` now labels keywords); a Saga chapter symbol (pure Roman numerals on a
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
  (`scripts/python/corpus_checks/`, reports in `docs/audits/corpus-checks/`).
  The post-merge acceptance pass at `8e83221`
  (`2026-08-26-post-patq-merge.md`, S8 search `2026-08-26-patq-s8-search.md`)
  accepted P-ATQ-1 (982 → 861 sentence-level children, 0 comma/colon or
  in-quote fragments), P-ATQ-2 (35 prohibition units, 0 labelled
  `prevention_effect`, 0 genuine prevention excluded) and P-ATQ-4 (30 positives
  over the full 12,466-unit instant/sorcery-face population, 0 false
  positives; Ertai's Meddling-class duration-less and inverted-cantrip forms
  recorded as out of pattern). P-ATQ-3 is accepted within its measured scope:
  the 3 residual prefix-related prevention rows are correct positives, while
  its bounded counterexample classes remain recorded (including 2 punctuated
  flavor-word misses and 141 newly incorrect `keyword_ability` labels on
  funny/token products).
- A held-out pool is frozen (protocol §6.3: `oracle_id` prefix `f`,
  non-fallback, excluding `lea`/`leb`/`arn`; 2,096 cards) and bound to the
  current snapshot by a non-disclosing sorted-identity digest. T7 exclusion is
  implemented at the database query boundary for card search, database-backed
  segmentation, and audit export; the protocol TSV exporter validates the
  native exclusion attestation and stable keys before writing rows. The pool
  has not yet been sampled or annotated.
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
   P-ATQ-1..4 are accepted under S10 (2026-08-26). P-ATQ-1 has an isolated
   measurement at `bf9eb04`, and the combined technical evidence is in
   `docs/audits/corpus-checks/2026-08-26-post-patq-merge.md`. The five
   rule-(c) rows are re-annotated `under` with fresh exports and drift-free
    metrics. Legends (`leg`) opened on 2026-08-27 over 293 held-out-safe
    development cards (273 with text, 426 frozen units). Its measurement freeze
    is `2e517357`; the retained TSV SHA-256 is
    `c39a2d695b94ce33a2e16356dd93bc6dc614b7c83becfb2b2f72ad5cb298d2e3`.
    Before either pass opened, a technical diff check displayed development
    rows to the assigned adjudicator; that identity was disqualified and
    replaced. Avidiyah reviewed the incident and reauthorized opening with the
    replacement on 2026-08-27. Both independent passes sealed with
    preregistered exact-row agreement 409/426 (0.9601), passing H8, and
    exact-card agreement 256/273 (0.9377). The replacement adjudicator reviewed
    the required 30-row union and closed Legends: 409 accepted rows, 16 defects,
    one unsupported span gap, no ambiguous or unresolved rows, and zero export
    drift. Boundary precision/recall are 415/425 (0.9765) and 415/426 (0.9742);
    structural exact-card correctness is 257/273 (0.9414). The Dark (`drk`)
    was frozen at `70fa956` over 113 held-out-safe development cards (110
    with text, 163 units, export `4460c2de…`), annotated by two independent
    passes, and adjudicated and closed on 2026-08-28
    (`docs/findings/drk-structural-audit.md`): 159 accept, 3 defect, 1
    ambiguous, 0 unsupported, drift 0. Boundary precision/recall 160/163
    (0.9816) and 160/161 (0.9938); kind, role, and source accuracy 1.0;
    exact-card correctness 107/110 (0.9727); unit novelty 127/163 (0.7791).
    Preregistered row agreement was 141/163 (0.8650), so **H10 is
    falsified**; 18 of 22 disagreements are one context convention (C6 on
    plain instant/sorcery spell text), and non-context agreement is 159/163.
    One proposal (P-DRK-1, over-inclusive sentence-initial `When` child span)
    joins P-LEG-1..3 in the unimplemented S8–S12 pipeline. The next eligible
    set is Fallen Empires (`fem`).
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
- keep the log to one entry per milestone (a sentence or two); put detail in
  a `docs/findings/`, `docs/gates/`, or `docs/protocol/` file instead, and
  add or update that file's entry in `docs/findings/index.json` (id, path,
  scope, status, one-line summary, `supersedes`/`superseded_by`);
- do not silently change the active goal or long-term destination.

## Decision and discovery log

Full detail for every dated entry below lives in `docs/findings/`,
`docs/gates/`, or `docs/protocol/`, indexed machine-readably at
`docs/findings/index.json` (id, path, scope, status, one-line summary,
supersession links). This log stays a one-line-per-milestone pointer; do not
re-expand a milestone's narrative here — add or edit the detail in its own
findings/gate file and the index entry instead.

### 2026-08-25

- Decided the near-term objective is empirical discovery/measurement before
  committing to an IR, that this document serves terminal agents, and that
  the long-term destination is a rules engine, simulation, telemetry, and AI
  stack. First Rust baseline: 67,738 units, 37,912 templates, top-5,000
  coverage 51.41%.

### 2026-08-26

- Stated the working parseability hypothesis and decided to walk the corpus
  set by set from Alpha, recording each step in `docs/findings/`. Added
  first-printing derivation and `sets`/`--set` to the CLI (baseline numbers
  unchanged). See `lea-segmentation-audit` in the index for the Alpha audit
  and the seven resulting segmenter changes.
- Gate 0 reviewed, passed (two caveats), protocol v1.0 and the held-out pool
  frozen; all 412 Alpha units annotated. See `gate-0-evidence`.
- Arabian Nights (+ Beta) audited under protocol v1.0; motivated P-ARN-1..4,
  later implemented and re-exported with zero drift across `lea`/`leb`/`arn`.
  See `arn-structural-audit`.
- Antiquities audited two-pass (125/125 agreement); motivated P-ATQ-1..4
  and registered D15-D20. All four proposals implemented, then accepted
  under protocol S10 in a post-merge acceptance pass at `8e83221` (P-ATQ-3
  accepted within bounded scope). Corpus after merge: 71,563 printed units,
  37,299 templates, 861 sentence-level delayed-trigger children, 30
  top-level spell-created delayed triggers, 166 `prevention_effect` units.
  See `atq-structural-audit` and `p-atq-research-acceptance-assessment`.
- Gate 1 readiness assessed at `2355b6c` (P-ATQ technical package merged at
  `bcf9eaa`); pre-Legends T7 export gate closed (held-out exclusion in
  SQLite before export, byte-identical repeated exports, 0 held-out
  records). See `gate-1-readiness-matrix` and `pre-legends-technical-entry-evidence`.

### 2026-08-27

- Legends entry conditions passed; an adjudicator-disqualification incident
  (accidental row exposure via `git diff --cached --check`) was caught before
  either pass opened, the adjudicator replaced, and opening reauthorized by
  Avidiyah. Both passes sealed with 409/426 exact-row and 256/273 exact-card
  agreement, drift 0; audit closed with 409 accept / 16 defect / 1
  unsupported, motivating non-blocking P-LEG-1..3. See `leg-structural-audit`
  and `legends-entry-record`.
- The Dark preregistered (hypotheses H1-H11) and inputs frozen at `70fa956`;
  held-out-safe freeze export validated (163 unique keys, 0 held-out
  records). Pass 1 sealed before pass 2/adjudicator were assigned, a
  preregistration §12 deviation; Avidiyah authorized a narrow reconciliation
  preserving pass 1 and assigning pass 2 (`gpt-5.3-codex-pass2-2026-08-27`)
  and a separate adjudicator. See `drk-structural-audit-preregistration` and
  `dark-entry-record`.

### 2026-08-28

- The Dark pass 2 sealed (141/163 rows, 89/110 cards agreement, drift 0);
  adjudicator closed the audit. H1-H5 and H9 pass; **H10 falsified**
  (0.8650 < 0.95, a context convention rather than structural disagreement);
  H6/H7/H8-static/H11 have zero denominators. Open item for the next guide
  version: state C6's answer for plain spell text and a boundary value for
  over-inclusive child spans. See `drk-structural-audit`.
- Compressed this log and added `docs/findings/index.json` as a
  machine-readable catalog of findings/gates/protocol documents, to stop the
  log re-narrating detail that already lives in those files.
- Consolidated agent onboarding into one canonical, tool-agnostic
  `docs/agent/agent-contract.md`; `CLAUDE.md`, `AGENTS.md`, and
  `.github/copilot-instructions.md` are now thin entry points that point to
  it instead of duplicating routing, evidence-hierarchy, command, and
  architecture prose (and, in `AGENTS.md`/copilot's case, a stale
  "read the newest file in `docs/findings/`" instruction predating the
  index). Added a minimal root `README.md`. No research, code, or findings
  content changed.
- Added `docs/agent/context-map.json`, a versioned machine-readable
  context-routing layer (task class -> required/optional context, likely
  code ownership, validation commands, do-not-modify-casually list), plus
  the standard-library-only `scripts/python/agent_context.py` helper and
  its tests. Solves "which repository context does this task need", which
  the findings index and this document do not. `docs/agent/agent-contract.md`
  now points to it. No research, code behavior, or findings content changed.
- Added `scripts/python/validate_agent_context.py`, a standard-library-only
  structural validator for `docs/findings/index.json`,
  `docs/agent/context-map.json`, and the agent entry-point files (dangling
  references, asymmetric supersession, unindexed findings/gates/protocol
  documents, generated-artifact/role mismatches, dropped entry-point
  pointers), plus its tests
  (`docs/agent/README-validate-agent-context.md` documents scope) and a CI
  workflow (`.github/workflows/validate-agent-context.yml`) that runs it on
  relevant pushes/PRs. Fixed a real one-sided supersession found while
  building it: `leg-structural-audit` was missing `legends-entry-record`
  and `legends-opening-work-plan` in its `supersedes` list, even though
  both of those gate documents already named it in `superseded_by`. No
  research, code behavior, or findings content changed otherwise.
