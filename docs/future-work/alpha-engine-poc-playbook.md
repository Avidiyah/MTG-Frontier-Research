# Future Work Playbook: Alpha-Set Parser -> IR -> Effect Execution PoC

Status: Draft (not active by default)  
Owner: Repository maintainers and future implementation agents  
Last updated: 2026-08-28

## 1) Why this exists

This document is a future-work implementation playbook for a narrow proof of
concept that explores:

1. parsing Alpha-era card effects,
2. compiling them into a typed intermediate representation (IR), and
3. executing those effects in an interactable engine loop.

It is intentionally scoped to avoid prematurely locking the repository into an
engine architecture that cannot scale to the full Oracle corpus.

## 2) Scope contract (important)

This playbook does not replace the active research boundary in
[current-state.md](</C:/Users/mcclu/Desktop/Code Projects/MTG-Frontier-Research/docs/current-state.md>).
It is dormant future work until explicitly invoked in a task prompt.

When invoked:

- keep the measurement track operational and comparable;
- isolate engine PoC code and artifacts from core research outputs;
- treat unsupported semantics as explicit, typed gaps (never silent fallbacks).

## 3) Invocation protocol for agents

Use this exact style in future prompts:

> Execute `docs/future-work/alpha-engine-poc-playbook.md` through Phase X only.
> Respect all gate criteria and update agent instruction files per Section 10.

Execution rules:

- complete one phase at a time;
- do not start the next phase without passing the current gate;
- include verification output and unresolved gaps in handoff notes.

## 4) Two-lane repository strategy (anti-bottleneck)

Maintain two explicit tracks:

1. **Research lane (stable):** current corpus measurement stack and findings.
2. **Engine lane (isolated):** parser/IR/executor PoC for Alpha-focused trials.

The PoC is successful only if both lanes remain usable and independently
verifiable.

## 5) Proposed repository layout

```text
docs/
  current-state.md
  findings/
  future-work/
    alpha-engine-poc-playbook.md
    capability-matrix-template.md
    adr-template.md
  engine-poc/
    decisions/
    capability-matrix.md
    known-gaps.md
    conformance/

data/
  fixtures/
    alpha/
      cards/
      scenarios/
      expected/

crates/
  discover-cli/        # existing measurement entrypoint and commands
  text-segmentation/   # measurement-specific segmentation/normalization
  oracle-schema/       # typed card/effect metadata model
  parser-alpha/        # Alpha-limited parser to AST + IR builder
  ir-core/             # versioned IR definitions + validator
  engine-domain/       # game state, zones, stack, priority, events, SBA
  executor/            # IR opcode interpreter
  conformance-tests/   # scenario runner and golden assertions
```

Notes:

- keep generated artifacts out of git as currently required;
- prefer additive crate introduction over risky rewrites;
- if this layout is partially adopted, preserve naming consistency.

## 6) Architecture boundaries

Hard boundaries that prevent early lock-in:

1. Parser emits AST/IR only.
2. Executor consumes IR only.
3. Engine domain exposes stable APIs to executor, not parser internals.
4. IR is versioned from day one (`ir/v1`) and validated at compile/load time.
5. Every unsupported construct returns typed diagnostics with rule/card context.

Do not allow direct parser -> engine shortcuts.

## 7) Initial Alpha capability envelope

Start with explicitly bounded semantics:

- simple spells (`deal damage`, `draw`, `destroy`, `counter`, `pump`);
- activated mana abilities;
- basic ETB/leave triggers where applicable in selected cards;
- straightforward target selectors.

Exclude in initial pass unless explicitly added:

- replacement/prevention families at broad coverage;
- layer-heavy continuous interactions beyond baseline checks;
- hidden-zone complexity beyond minimal legality constraints.

Track support in
[capability-matrix.md](</C:/Users/mcclu/Desktop/Code Projects/MTG-Frontier-Research/docs/engine-poc/capability-matrix.md>)
using:

- `supported`,
- `partial`,
- `unsupported`,
- `blocked-by-model`.

## 8) Phase plan with gate criteria

### Phase 0 - Scaffold and contracts

Deliverables:

- workspace/crate scaffolding for parser, IR, engine domain, executor;
- `ir/v1` type definitions and validator shell;
- PoC capability matrix initialized.

Gate to pass:

- builds cleanly;
- no change in existing research command behavior;
- capability matrix committed with explicit out-of-scope list.

### Phase 1 - Alpha AST + IR lowering

Deliverables:

- parser for selected Alpha card text patterns;
- deterministic AST -> IR lowering;
- typed diagnostics for unsupported lines.

Gate to pass:

- representative card set parses reproducibly;
- unsupported features surface structured errors;
- no panic-path parsing for known invalid inputs.

### Phase 2 - Minimal execution loop

Deliverables:

- engine state primitives (zones, stack, turn/priority skeleton);
- executor opcodes for initial effect families;
- CLI/TUI interaction primitives (`cast`, `activate`, `pass`, `resolve`).

Gate to pass:

- scripted scenarios execute deterministically;
- legality checks reject invalid targets/cost states;
- state transitions logged as machine-readable artifacts.

### Phase 3 - Conformance harness

Deliverables:

- fixture-driven scenario tests with golden expected outputs;
- regression suite for selected Alpha cards and interactions;
- gap report generated from unsupported diagnostics.

Gate to pass:

- conformance suite passes for declared-supported features;
- every known unsupported item appears in gap report;
- no silent behavior drift in supported scenarios.

### Phase 4 - Expansion readiness review

Deliverables:

- architecture stress review against non-Alpha patterns;
- ADR updates for any contract changes;
- recommendation: proceed, refactor, or halt.

Gate to pass:

- clear evidence that architecture can absorb new feature classes without
  parser/executor entanglement;
- documented migration plan before adding new sets.

## 9) Conformance-first quality policy

For this PoC, scenario conformance is the primary quality signal.

Required:

- deterministic scenario fixtures;
- explicit preconditions and expected final state;
- replayable command/log trail for each scenario;
- structured error codes for unsupported semantics.

Optional later:

- richer UI layer;
- performance tuning;
- larger card coverage.

## 10) Multi-agent sync protocol (AGENTS/CLAUDE/Copilot)

To keep all coding agents aligned, every feature-class addition must update
all three instruction surfaces in one pull request:

1. [AGENTS.md](</C:/Users/mcclu/Desktop/Code Projects/MTG-Frontier-Research/AGENTS.md>)
2. [CLAUDE.md](</C:/Users/mcclu/Desktop/Code Projects/MTG-Frontier-Research/CLAUDE.md>)
3. [.github/copilot-instructions.md](</C:/Users/mcclu/Desktop/Code Projects/MTG-Frontier-Research/.github/copilot-instructions.md>)

### 10.1 What to update when a new feature lands

For each newly supported semantic/operator family:

- capability status (supported/partial/unsupported);
- crate/module ownership boundary;
- verification command(s);
- known failure modes or guardrails;
- whether it changes active research baseline assumptions.

### 10.2 Required sync checklist (copy into PR description)

- [ ] Updated `docs/engine-poc/capability-matrix.md`
- [ ] Updated PoC decisions or ADR note in `docs/engine-poc/decisions/`
- [ ] Updated `AGENTS.md` engine-PoC guidance block
- [ ] Updated `CLAUDE.md` engine-PoC guidance block
- [ ] Updated `.github/copilot-instructions.md` engine-PoC guidance block
- [ ] Verified all three agent files express the same capability state
- [ ] Added/updated conformance fixtures for the feature
- [ ] Recorded commands and results in handoff note

### 10.3 Canonical text block policy

To prevent drift, maintain one canonical snippet in this playbook under
"Agent Sync Snippet". Copy it verbatim into all three agent files when changed.

#### Agent Sync Snippet

```text
Engine PoC status (future-work track):
- Scope: Alpha-focused parser -> IR -> executor validation.
- Contract: parser emits IR only; executor consumes IR only.
- Safety: unsupported semantics must return typed diagnostics (no silent fallback).
- Source of truth: docs/engine-poc/capability-matrix.md
- Gate rule: do not expand to a new set until expansion readiness review passes.
```

## 11) ADR discipline for architecture changes

Any change to parser/IR/executor contracts requires a short ADR in
[docs/engine-poc/decisions/](</C:/Users/mcclu/Desktop/Code Projects/MTG-Frontier-Research/docs/engine-poc/decisions/>)
containing:

- context,
- decision,
- alternatives considered,
- consequences,
- rollback plan.

If no ADR is written, contract changes are considered incomplete.

## 12) Exit criteria for "PoC complete"

Declare Alpha PoC complete only when:

1. declared-supported Alpha scenarios pass conformance tests;
2. unsupported semantics are explicitly enumerated with diagnostics;
3. architecture boundaries remain intact (no parser-engine coupling shortcuts);
4. multi-agent instruction files are synchronized per Section 10;
5. next-step recommendation is documented (expand, refactor, or stop).

## 13) Immediate next action when this playbook is invoked

Start at Phase 0:

1. scaffold isolated crates and docs directories;
2. add initial capability matrix with supported/unsupported seed entries;
3. implement `ir/v1` skeleton and validation path;
4. verify existing research lane commands are unaffected.

