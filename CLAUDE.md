# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with
code in this repository.

## Start here

**Read `docs/agent/agent-contract.md` first.** It is the canonical,
tool-agnostic operating contract for this repository: what it is, the
authoritative-document routing (`docs/current-state.md` →
`docs/findings/index.json` → the specific finding), the evidence hierarchy,
build/test commands, architecture, generated-artifact policy, and the
handoff requirement. Everything below is Claude-Code-specific and additive to
that contract — it does not restate it.

## Claude Code specifics

- Claude Code can drive the `mtg-discover` CLI and `cargo`/`python` commands
  directly via the Bash tool; follow the commands in
  `docs/agent/agent-contract.md` rather than inventing new ones.
- When the `rust-analyzer-mcp` MCP server (see `docs/agent/agent-contract.md`
  for setup) is connected and trusted in this session, prefer its
  code-navigation tools over grep/glob for tracing behavior in `src/`.
- Keep files under 500 lines and prefer editing existing files over creating
  new ones, per this machine's global `CLAUDE.md` rules — this applies to
  documentation changes in this repo too.
