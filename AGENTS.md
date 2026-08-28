# Repository Guidelines

## Start here

**Read `docs/agent/agent-contract.md` first.** It is the canonical,
tool-agnostic operating contract for this repository: what it is, the
authoritative-document routing (`docs/current-state.md` →
`docs/findings/index.json` → the specific finding), the evidence hierarchy,
build/test/lint commands, architecture, generated-artifact policy, and the
handoff requirement required before ending any task. This file only adds
conventions specific to working here as a general terminal coding agent; it
does not restate that contract.

## Project structure

The Rust `mtg-discover` CLI is split by function under `src/`; Python
ingestion and lookup tools live in `scripts/python/`; research handoffs and
findings live in `docs/` (see the contract for routing). Generated local
artifacts (`cards.sqlite`, `*.jsonl.gz`, `target/`) must not be committed.

## Coding style & naming conventions

Use Rust 2024 idioms and `rustfmt` defaults. Keep CLI output
machine-readable: successful `mtg-discover` commands emit one JSON document
to stdout, errors on stderr with a nonzero exit status. Prefer clear
snake_case names for Rust functions, tests, and Python helpers. Keep comments
short and focused on non-obvious research or parsing assumptions.

## Testing guidelines

Add focused Rust unit tests in the existing `#[cfg(test)]` module when
changing normalization, segmentation, rules parsing, or query behavior. Use
descriptive test names such as `rules_parser_separates_numbered_rules_and_glossary`.
