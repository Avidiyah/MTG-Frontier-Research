# MTG Frontier Research

An empirical research workbench for translating Magic: The Gathering Oracle
text into machine-usable effect semantics. The current work is measurement
and structural discovery, not building a parser, IR, rules engine, or
simulator — see `docs/current-state.md` for what phase is active now.

## Where to go next

- **Coding agents:** start at `docs/agent/agent-contract.md` — the canonical
  operating contract (state routing, evidence hierarchy, commands,
  architecture, handoff requirement). `CLAUDE.md`, `AGENTS.md`, and
  `.github/copilot-instructions.md` are thin, tool-specific pointers to it.
- **Current state and open questions:** `docs/current-state.md`.
- **Completed investigations:** `docs/findings/index.json` (a machine-readable
  catalog — read it before opening any file under `docs/findings/`,
  `docs/gates/`, or `docs/protocol/`).
- **Pipeline and CLI reference:** `docs/README.md`.
- **Literature review:** `docs/RESEARCH_NOTES.md`.

## Quick start

```powershell
cargo build --release
python scripts/python/mtg_card_pipeline.py all
.\target\release\mtg-discover.exe info
```

See `docs/agent/agent-contract.md` for the full command reference.
