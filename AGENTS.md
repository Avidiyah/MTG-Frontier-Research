# Repository Guidelines

## Project Structure & Module Organization

This repository is an empirical MTG Oracle-text research workbench. The Rust CLI lives in `src/main.rs` as the `mtg-discover` binary. Python ingestion and lookup tools live in `scripts/python/`: `mtg_card_pipeline.py` fetches Scryfall data, builds `cards.sqlite`, and runs the original template baseline; `mtg_search.py` is a human-oriented card lookup tool. Research handoffs and findings live in `docs/`, especially `docs/current-state.md`, `docs/RESEARCH_NOTES.md`, and `docs/findings/`. `Magic-Comprehensive_Rules.md` is tracked source data. Generated local artifacts such as `cards.sqlite`, `oracle-cards.jsonl.gz`, `rulings.jsonl.gz`, `default-cards.jsonl.gz`, and `target/` must not be committed.

## Build, Test, and Development Commands

Run commands from the repository root, preferably in PowerShell.

```powershell
cargo build                  # development build
cargo build --release        # optimized research CLI
cargo test                   # Rust unit tests in src/main.rs
cargo fmt -- --check         # formatting check
cargo clippy --all-targets -- -D warnings
python scripts/python/mtg_card_pipeline.py all
python scripts/python/mtg_search.py "Lightning Bolt"
```

The pipeline `fetch` stage requires network access to Scryfall. Python scripts are standard-library only; do not add a `pip install` requirement without a strong reason.

## Coding Style & Naming Conventions

Use Rust 2024 idioms and `rustfmt` defaults. Keep CLI output machine-readable: successful `mtg-discover` commands should emit one JSON document to stdout, with errors on stderr and nonzero exit status. Prefer clear snake_case names for Rust functions, tests, and Python helpers. Keep comments short and focused on non-obvious research or parsing assumptions.

## Testing Guidelines

Add focused Rust unit tests in the existing `#[cfg(test)]` module when changing normalization, segmentation, rules parsing, or query behavior. Use descriptive test names such as `rules_parser_separates_numbered_rules_and_glossary`. If segmentation or normalization changes, rerun `cargo test` and recompute `mtg-discover templates`; update affected measurements in `docs/current-state.md`.

## Commit & Pull Request Guidelines

Recent commits use short, imperative summaries with optional scope context, for example `Add README describing the pipeline and search tool` or `Research Pass 2, Alpha`. Keep PRs focused, describe the research or behavior change, list verification commands, and link any relevant finding in `docs/findings/`.

## Agent-Specific Instructions

Read `docs/current-state.md` before research work and the newest relevant file in `docs/findings/`. Treat the current segmenter and normalizer as measurement tools, not a parser or semantic engine. Follow the evidence hierarchy: Comprehensive Rules, Oracle text, official rulings, corpus measurements, literature, then agent interpretation.
