# Copilot instructions

## Start here

**Read `docs/agent/agent-contract.md` first.** It is the canonical,
tool-agnostic operating contract for this repository: what it is, the
authoritative-document routing (`docs/current-state.md` →
`docs/findings/index.json` → the specific finding), the evidence hierarchy,
build/test/lint commands, architecture and data flow, generated-artifact
policy, and the handoff requirement required before ending any task.
Everything below is Copilot-specific and additive to that contract.

## Copilot-specific notes

- Card searches performed via the CLI or ad hoc SQL against `cards.sqlite`
  are literal and case-insensitive; `%` and `_` are escaped rather than
  treated as SQL wildcards. `mtg-discover card` requires an exact name or
  `oracle_id`. Set filters (`--set`) refer to the derived first printing, not
  an arbitrary printing in the Oracle Cards file.
- Preserve the distinction between current Oracle wording and printing
  chronology when analyzing sets: `first_set` is an era-selection field, not
  a source of historical card text.
