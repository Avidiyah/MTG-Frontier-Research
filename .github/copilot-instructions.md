# Copilot instructions

## Project focus and source of truth

This repository is an empirical research workbench for translating Magic: The
Gathering Oracle text into machine-usable effect semantics. The active frontier
is measurement: discover the corpus's linguistic and semantic structure before
choosing an intermediate representation. Do not build a parser, IR, rules
engine, or simulator unless the task explicitly changes that scope.

Read `docs/current-state.md` before starting research work. It is the primary
handoff for verified state, non-goals, evidence hierarchy, open questions, and
the current investigation sequence. Then read the newest relevant document in
`docs/findings/`. Use `docs/RESEARCH_NOTES.md` for literature context and
`docs/mtg_ai_research_roadmap.md` for the long-term pipeline.

Before ending any task, create a handoff document: update `docs/current-state.md`
for repository-wide truth, or add a concise dated note under `docs/findings/`
for scoped work. Record what changed, what was verified, and the next open
question.

## Build, test, and lint

Run commands from the repository root in PowerShell.

```powershell
# Development build
cargo build

# Optimized CLI used for corpus-wide experiments
cargo build --release
$mtg = ".\target\release\mtg-discover.exe"

# Full Rust test suite
cargo test

# One exact unit test (tests are organized by subsystem under src/tests/)
cargo test tests::normalization::normalization_handles_nested_reminder_text_and_self_references -- --exact

# Substitute another test name when targeting a different behavior
cargo test tests::rules::rules_parser_separates_numbered_rules_and_glossary -- --exact

# Formatting and lint checks
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
```

The Python scripts use only the standard library; keep them dependency-free.
They currently have no separate automated test or lint command.

```powershell
# Fetch all bulk inputs, rebuild cards.sqlite, and run the Python baseline
python scripts/python/mtg_card_pipeline.py all

# Run stages independently when inputs already exist
python scripts/python/mtg_card_pipeline.py fetch
python scripts/python/mtg_card_pipeline.py load
python scripts/python/mtg_card_pipeline.py analyze
```

`fetch` requires network access to Scryfall. `load` rebuilds the generated
database from the local bulk files.

## Architecture and data flow

```text
Scryfall oracle_cards + rulings + default_cards
    -> scripts/python/mtg_card_pipeline.py
    -> cards.sqlite
    -> src/*.rs (mtg-discover)
       + Magic-Comprehensive_Rules.md parsed on demand
    -> JSON observations
    -> docs/findings/ and, for repository-wide changes, docs/current-state.md
```

- The Python pipeline owns ingestion and the SQLite schema. `oracle_cards`
  supplies one record per Oracle identity; `rulings` joins by `oracle_id`;
  `default_cards` is used only to derive the earliest qualifying printing.
- First printing means the earliest paper, non-promo printing outside
  promo/token/memorabilia/minigame/alchemy sets. If none qualifies, the
  earliest printing of any kind is stored and `first_is_fallback` is set.
  Era-by-era research therefore tracks when effects entered the game, while
  still analyzing current Oracle wording rather than historical wording.
- `src/main.rs` is the thin `mtg-discover` entry point. Functional modules own
  card/set queries, rules access, segmentation/templates, audits, shared
  database policy, and regression tests. The CLI opens `cards.sqlite`
  read-only, parses the Comprehensive Rules directly from Markdown, and
  serializes every successful command as one JSON document.
- Rules parsing is structural, not semantic: numbered rules are recognized by
  their identifiers and glossary entries by document layout. Search and rule
  retrieval make the rules addressable evidence; they do not produce
  executable game logic.
- `segment_text`, `build_unit`, `classify_kind`, and `normalize_text` are shared by the
  single-card `segment` command and corpus-wide `templates` analysis. A change
  to shared segmentation or normalization changes the measured baseline.

## Research CLI

Successful commands emit JSON to stdout; errors go to stderr with a nonzero
exit status. Prefer these interfaces over scraping the Python search output.
The database and rules paths default to repository-root files and can be
overridden with global `--db` and `--rules`.

```powershell
& $mtg info
& $mtg cards "draw a card" --field text
& $mtg cards "creature" --field type --set lea
& $mtg card "Lightning Bolt" --rulings
& $mtg rules search "trigger condition"
& $mtg rules show 603.1
& $mtg segment --card "Cryptic Command"
& $mtg templates --limit 100 --min-count 2
& $mtg templates --set lea
& $mtg sets --type expansion --until 2000-01-01
```

Card searches are literal and case-insensitive; `%` and `_` are escaped rather
than treated as SQL wildcards. `card` requires an exact name or `oracle_id`.
Set filters refer to the derived first printing, not an arbitrary printing in
the Oracle Cards file.

## Rust MCP tooling

This repo has a `rust-analyzer`-backed MCP server configured
(`rust-analyzer-mcp`, see `.vscode/mcp.json` and `.mcp.json`). When it is
connected and trusted, prefer its tools — get symbols, go to definition, find
references, hover — over grep/text search when tracing behavior across the
functional modules under `src/`.

If the MCP tools aren't available in a given session (server not installed or
not trusted), fall back to grep/glob as usual. To install or reinstall it
locally:

```powershell
rustup component add rust-analyzer
cargo install rust-analyzer-mcp
```

## Repository-specific conventions

- Treat the current segmenter and normalizer as measurement instruments, not a
  parser. Line boundaries are not proven ability boundaries, classifier labels
  are heuristic, and normalized-string equality is not semantic equivalence.
- Apply the evidence hierarchy from `docs/current-state.md`: Comprehensive
  Rules, current Oracle text, official rulings, corpus measurements,
  literature, then agent interpretation. Rulings are not exhaustive labels,
  and frequency is not proof of meaning.
- State research claims as falsifiable hypotheses, actively search for
  counterexamples, and label results as verified findings, bounded
  observations, or hypotheses. Put completed investigations in
  `docs/findings/`, including corpus/rules snapshot dates and reproduction
  commands.
- If segmentation or normalization changes, run the focused tests and
  `mtg-discover templates`; update affected measurements in
  `docs/current-state.md`, including `Last verified`. Do not leave baseline
  numbers stale.
- Update `docs/current-state.md` only for repository-wide truth. Keep its main
  body current and its dated decision/discovery log short; detailed experiment
  history belongs in `docs/findings/`.
- Preserve the distinction between current Oracle wording and printing
  chronology when analyzing sets. `first_set` is an era-selection field, not a
  source of historical card text.
- Keep `cards.sqlite`, `*-cards.jsonl.gz`, `rulings.jsonl.gz`, and `target/` as
  generated local artifacts. Never commit them. `Magic-Comprehensive_Rules.md`
  is source data used by the Rust CLI and is tracked.
