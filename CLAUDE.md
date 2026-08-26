# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this repository is

An empirical research workbench for the problem of translating Magic: The Gathering Oracle text into machine-usable effect semantics. The active frontier is *measurement*, not implementation: discover the linguistic/semantic structure of the card corpus well enough that a later intermediate-representation (IR) choice is evidence-driven. Do **not** build a parser, rules engine, IR, or simulator unless explicitly asked — those are documented non-goals.

**Read `docs/current-state.md` first** — it is the primary handoff document: current verified state, research method, evidence hierarchy, non-goals, and open questions. Update it (including `Last verified`) only when repository-wide state changes. `docs/RESEARCH_NOTES.md` holds the literature review; `docs/README.md` documents the pipeline and tools in detail.

**Then Read `docs\findings` for the most recent research findings to review your current progress**

## Commands

```powershell
# Build the Rust CLI (the main research tool)
cargo build --release
$mtg = ".\target\release\mtg-discover.exe"

# Tests (all in src/main.rs #[cfg(test)] module)
cargo test
cargo test normalization_handles   # single test by name substring

# Regenerate data (network access to data.scryfall.io required for fetch)
python scripts/python/mtg_card_pipeline.py all      # or: fetch / load / analyze
python scripts/python/mtg_search.py <name>          # interactive card lookup
```

Python scripts are standard-library only — no pip install step, keep it that way.

### mtg-discover usage

Every successful command prints one JSON document to stdout; errors go to stderr with nonzero exit. Prefer these structured interfaces over scraping human output. Paths default to `cards.sqlite` and `Magic-Comprehensive_Rules.md` in the repo root (override with global `--db` / `--rules`).

```powershell
& $mtg info                                   # corpus + rules metadata (record before experiments)
& $mtg cards "draw a card" --field text       # search cards (literal, case-insensitive; fields: name/text/type/all)
& $mtg card "Lightning Bolt" --rulings        # exact name or oracle_id, optionally with rulings
& $mtg rules search "trigger condition"       # numbered rules + glossary
& $mtg rules show 603.1                       # rule and all subrules
& $mtg segment --card "Cryptic Command"       # structural segmentation + normalized template
& $mtg templates --limit 100 --min-count 2    # corpus-wide template frequency + coverage curve
```


## Architecture

- `src/main.rs` — the entire `mtg-discover` CLI (single file). Reads `cards.sqlite` read-only and parses `Magic-Comprehensive_Rules.md` on the fly(Still make sure to read `Magic-Comprehensive_Rules.md` to verify). Key shared machinery: `segment_text`/`segment_line`/`build_unit` (line-level segmentation into a tree of typed `Segment`s: `AbilityKind` × `StructuralRole` × `TextSource`, with mode, delayed-trigger and quoted-ability children), `classify_kind` (surface-form classification on normalized text), `normalize_text` (card name and `this <type>` → `~` except after `named`, mana → `{M}`, integers → `N`, reminder text stripped) — both `segment` and `templates` run through this same code, so changing normalization changes the corpus-wide baseline numbers in `docs/current-state.md`.
- `scripts/python/mtg_card_pipeline.py` — builds the data: fetches Scryfall bulk data (`oracle-cards.jsonl.gz`, `rulings.jsonl.gz`), loads into `cards.sqlite` (`cards` keyed by `oracle_id`; `rulings` indexed on `oracle_id`; JSON-text columns for keywords/colors/legalities). Double-faced cards get face texts joined with `//` and `is_dfc = 1`.
- The normalizer/segmenter is deliberately crude — a measurement instrument, not a parser. Do not present its output as semantic parsing.

## Rules for this repo

- `cards.sqlite`, `*.jsonl.gz`, and `target/` are regenerable local artifacts and gitignored — never commit them. `Magic-Comprehensive_Rules.md` is a tracked source file.
- Follow the evidence hierarchy in `docs/current-state.md`: Comprehensive Rules > Oracle text > official rulings > corpus measurements > literature > agent interpretation. Frequency of a normalized template is evidence of reuse, not proof of semantic identity.
- If you change segmentation or normalization, rerun `templates` and `cargo test`, and refresh the affected measurements in `docs/current-state.md` rather than leaving stale numbers.
- State hypotheses as falsifiable, search for counterexamples, and label conclusions as verified findings, bounded observations, or hypotheses.
