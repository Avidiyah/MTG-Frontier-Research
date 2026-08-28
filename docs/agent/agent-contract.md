# Agent Operating Contract

This is the **canonical, tool-agnostic operating contract** for any coding
agent working in this repository — Claude Code, Codex, GitHub Copilot, or
otherwise. `CLAUDE.md`, `AGENTS.md`, and `.github/copilot-instructions.md` are
thin, tool-specific entry points that point here; do not re-duplicate this
document's content into them. If you are looking for what to do first, this
file is it.

## What this repository is

An empirical research workbench for translating Magic: The Gathering Oracle
text into machine-usable effect semantics. The long-term destination is:

```text
Oracle text + Comprehensive Rules
    -> formal card semantics
    -> general MTG rules engine
    -> game simulation and telemetry
    -> search, machine learning, and AI agents
```

Only some prefix of that pipeline is the active research frontier at any
given time; the rest is future work that must remain possible. **This
document does not say which stage is current** — that is state, not
principle, and it changes as the project progresses. Always get the current
phase, active goal, and non-goals from `docs/current-state.md` rather than
assuming yesterday's frontier is still today's.

## Where authoritative state lives (read in this order)

1. **`docs/current-state.md`** — the primary handoff document: current
   verified state, active research goal, current non-goals, evidence
   hierarchy, known limitations, and open questions. Read this first, every
   session, even if you read it recently — it changes as work lands.
2. **`docs/findings/index.json`** — a machine-readable catalog of every file
   in `docs/findings/`, `docs/gates/`, and `docs/protocol/` (id, path, scope,
   status, one-line summary, `supersedes`/`superseded_by`). Read this before
   opening any individual findings file, to decide which document is current
   for your task's scope.
3. Open **only** the specific findings/gates/protocol document(s) the index
   says are relevant. Do not read the `docs/findings/` directory blind or
   recursively — the index exists precisely so you don't have to.

`docs/RESEARCH_NOTES.md` holds the literature review; `docs/README.md`
documents the data pipeline and `mtg-discover` CLI in full detail; both are
reference material, not state.

## Evidence hierarchy

Keep facts, measurements, hypotheses, and design proposals separate. Use this
authority order for any claim about card behavior:

1. **Comprehensive Rules** for game definitions and normative mechanics.
2. **Current Oracle text** for what a card says.
3. **Official rulings** for clarifications and documented interactions.
4. **Corpus measurements** for frequency and distribution claims.
5. **Literature and existing engines** for prior approaches and architecture.
6. **Agent interpretation** only as a hypothesis to test.

Rulings are useful evidence but are not exhaustive semantic annotations.
Frequency of a normalized template is evidence of reuse, not proof of
semantic identity. A rule-search hit is not proof that the rule completely
determines a card's behavior.

## Research method

State hypotheses as falsifiable, search for counterexamples, and label
conclusions as verified findings, bounded observations, or hypotheses — never
present the segmenter/normalizer's output as semantic parsing; it is a
measurement instrument.

## Generated-artifact policy

`cards.sqlite`, `oracle-cards.jsonl.gz`, `rulings.jsonl.gz`,
`default-cards.jsonl.gz`, and `target/` are regenerable local artifacts,
gitignored, and must never be committed. `Magic-Comprehensive_Rules.md` is
tracked source data.

## Build, test, and run

Run from the repository root.

```powershell
# Rust CLI (the main research tool)
cargo build --release
$mtg = ".\target\release\mtg-discover.exe"
cargo test
cargo test normalization_handles          # single test by name substring
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings

# Data pipeline (standard-library-only Python — no pip install step, keep it that way)
python scripts/python/mtg_card_pipeline.py all      # fetch, load, analyze
python scripts/python/mtg_card_pipeline.py fetch    # requires network access to data.scryfall.io
python scripts/python/mtg_card_pipeline.py load     # rebuild cards.sqlite from local bulk files
python scripts/python/mtg_search.py <name>          # interactive card lookup
```

`mtg-discover` prints one JSON document to stdout on success; errors go to
stderr with a nonzero exit. Prefer these structured interfaces over scraping
human-oriented output. Paths default to `cards.sqlite` and
`Magic-Comprehensive_Rules.md` in the repo root (override with global `--db`
/ `--rules`).

```powershell
& $mtg info                                   # corpus + rules metadata (record before experiments)
& $mtg cards "draw a card" --field text       # search cards (literal, case-insensitive; fields: name/text/type/all)
& $mtg card "Lightning Bolt" --rulings        # exact name or oracle_id, optionally with rulings
& $mtg rules search "trigger condition"       # numbered rules + glossary
& $mtg rules show 603.1                       # rule and all subrules
& $mtg segment --card "Cryptic Command"       # structural segmentation + normalized template
& $mtg templates --limit 100 --min-count 2    # corpus-wide template frequency + coverage curve
& $mtg sets --until 2000-01-01                # first-printing sets in release order
& $mtg audit export lea                       # flattened structural units for one set (protocol tooling)
```

If you change segmentation or normalization, rerun `cargo test` and
`mtg-discover templates`, and refresh the affected measurements in
`docs/current-state.md` — do not leave baseline numbers stale.

## Architecture (durable shape)

- `src/main.rs` / `src/cli.rs` — thin `mtg-discover` dispatch and CLI
  definitions. Functional modules under `src/` own card/set queries, rules
  access, segmentation/templates, audits, shared read-only database policy,
  and subsystem-organized tests (`src/tests/`).
- `src/segment.rs` — `segment_text` / `segment_line` / `build_unit` (line-level
  segmentation into a tree of typed `Segment`s), `classify_kind` (surface-form
  classification), and `normalize_text` (card-name/`this <type>` self-reference
  folding, mana/integer collapsing, reminder-text stripping). Both `segment`
  and `templates` share this machinery, so changing it changes the
  corpus-wide baseline numbers recorded in `docs/current-state.md`.
- `scripts/python/mtg_card_pipeline.py` — builds the data: fetches Scryfall
  bulk data, loads `cards.sqlite` (`cards` keyed by `oracle_id`; `rulings`
  indexed on `oracle_id`; JSON-text columns for keywords/colors/legalities;
  `first_*` columns hold the derived first printing). Double-faced cards get
  face texts joined with `//` and `is_dfc = 1`.
- The normalizer/segmenter is deliberately crude — a measurement instrument,
  not a parser. See `docs/README.md` for the full pipeline/CLI reference.

## MCP tooling (rust-analyzer)

This repo has a `rust-analyzer`-backed MCP server configured
(`rust-analyzer-mcp`; see `.vscode/mcp.json` and `.mcp.json`). When it is
connected and trusted in your session, prefer its tools — get symbols, go to
definition, find references, hover — over grep/text search when tracing
behavior across the functional modules under `src/`. Fall back to grep/glob
if MCP tools aren't available in a given session. Install or reinstall
locally with:

```powershell
rustup component add rust-analyzer
cargo install rust-analyzer-mcp
```

## Handoff requirement

Before ending any task, leave a handoff document:

- Update `docs/current-state.md` (including `Last verified`) when
  repository-wide state changes. Its dated log stays one line per milestone —
  detail belongs in a findings/gate/protocol file, not the log.
- Otherwise, add a concise dated note under `docs/findings/` for scoped work.
- Either way, add or update the corresponding entry in
  `docs/findings/index.json` (id, path, scope, status, one-line summary,
  `supersedes`/`superseded_by`) whenever a findings, gate, or protocol
  document is added or changed.
- Capture what changed, what was verified, and the next open question.

## Repository conventions

- Commit messages: short, imperative summaries with optional scope context
  (e.g. `Add README describing the pipeline and search tool`).
- Keep PRs focused; describe the research or behavior change; list
  verification commands; link any relevant finding in `docs/findings/`.
- Do not duplicate this contract's content into `CLAUDE.md`, `AGENTS.md`, or
  `.github/copilot-instructions.md` — extend those only with instructions
  specific to that tool, and extend this file when the addition applies
  regardless of which agent is running.
