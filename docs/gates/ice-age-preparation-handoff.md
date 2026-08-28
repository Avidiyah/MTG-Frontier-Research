# Ice Age (`ice`) preparation handoff — for Sonnet 5

- Date prepared: 2026-08-28
- Prepared by: Claude Code (`human-fem-pass`, replacement pass-1 annotator on
  the concurrently open Fallen Empires audit)
- Prepared for: the next Claude Sonnet 5 session
- Status: **no Ice Age preregistration exists yet; Fallen Empires is still
  open in a separate session. This handoff scopes preparation work that does
  not conflict with it.**

## Read first

1. `docs/agent/agent-contract.md`, `docs/current-state.md`,
   `docs/findings/index.json` — in that order.
2. `docs/gates/fem-entry-record.md` and
   `docs/gates/fem-pass2-adjudication-handoff.md` — to see exactly what the
   concurrent Codex session owns, so you know what to avoid.
3. The five most recent closed-audit preregistrations as procedural
   precedent, in particular `docs/findings/drk-structural-audit-preregistration.md`
   and `docs/findings/fem-structural-audit-preregistration.md` (do not treat
   FEM's measurements as closed — it is still open).
4. `docs/protocol/structural-investigation-protocol.md` v1.0 and
   `docs/protocol/structural-annotation-guide-v1.0.md` v1.0 (frozen; reuse
   unchanged unless a closed audit's adjudication motivates a proposal, which
   is not your call to make here).

## Why Ice Age, and why now

`docs/current-state.md` records The Dark closed and "the next eligible set is
Fallen Empires (`fem`)." Fallen Empires is now open (not yet closed — pass 2
and adjudication are in progress under Codex). Chronologically, the set after
Fallen Empires is **Ice Age** (`ice`, released 1995-06-03, 346 cards per
`mtg-discover sets`), skipping non-expansion products (HarperPrism Book Promos
is a promo set, not a development set under this protocol's scope; confirm
this classification still holds before excluding it). Ice Age is therefore the
set to prepare next — but it cannot be measurement-frozen or opened until
Fallen Empires closes, because:

- the preregistration's frozen-input block records the *prior* closed audit's
  hashes, and Fallen Empires must be that prior closure for Ice Age;
- `docs/current-state.md` must reflect Fallen Empires as closed before a new
  measurement freeze commit is meaningful;
- opening two structural-audit development exports at once has no precedent
  and is not covered by the protocol's stop conditions — do not attempt it.

## What you may do now, without conflicting with the Codex FEM session

These do not require FEM to be closed and do not touch any FEM-scoped file:

1. **Read-only reconnaissance of Ice Age at the aggregate level only** —
   exactly as the FEM preparer did before its own freeze: `mtg-discover sets`,
   `mtg-discover audit summary --exclude-heldout ice`,
   `mtg-discover audit signals --exclude-heldout ice`. **Do not** run
   `mtg-discover audit novelty` on `ice` — it has no `--exclude-heldout` flag
   and caused the FEM session's held-out exposure incident (see
   `docs/gates/fem-entry-record.md` §2.1). If you need novelty figures, wait
   until the freeze step's aggregate-only local template-overlap computation
   (see `docs/findings/fem-structural-audit-preregistration.md` §13 step 5 for
   the pattern to follow).
2. Draft (but do not finalize with live hashes) an Ice Age preregistration
   skeleton at `docs/findings/ice-structural-audit-preregistration.md`,
   mirroring the Fallen Empires preregistration's structure (purpose,
   objective, frozen-inputs placeholder, hypotheses H1-Hn, predeclared
   measurements, annotation workflow, stop conditions, entry checklist). Mark
   every frozen-input field as **pending**, not populated with guessed values.
   Do not include this file's `docs/findings/index.json` entry as `closed` or
   `preregistered`; use `status: draft_pending_fem_closure`.
3. Identify Ice Age's distinctive structural vocabulary the way the FEM
   preparation did for counters (H12) — Ice Age introduces cumulative upkeep,
   snow-covered lands/mana, and the "kicker"-adjacent design space is not yet
   present (that is Homelands/Visions-era; verify before asserting). Record
   candidate hypotheses only as a draft list, not as committed H-numbers,
   since those numbers depend on the final preregistration draft order.
4. Check whether the current `mtg-discover audit` commands and segmenter
   already have known gaps relevant to Ice Age's vocabulary (cumulative
   upkeep counters, "return ~ to its owner's hand", snow mana), using only
   already-committed prior audits and the segmenter source/tests — not by
   reading Ice Age card text directly.

## What you must not do

- Do not touch any `fem`-scoped file: `docs/audits/fem/**`,
  `docs/gates/fem-entry-record.md`, `docs/gates/fem-pass2-adjudication-handoff.md`,
  `docs/findings/fem-structural-audit*.md`. Those are owned by the concurrent
  Codex session until it hands off or the audit closes.
- Do not query, print, or annotate any individual Ice Age card's Oracle text
  outside the aggregate-only commands in step 1 above — that would put you in
  the same position as the FEM preparer's disclosed incident, except without
  even the excuse of an in-progress freeze.
- Do not populate `docs/current-state.md`'s "next eligible set" language as
  Ice Age being open — it is not, and won't be until FEM closes and this
  preparation's entry checklist passes.
- Do not run the measurement-freeze procedure (protocol §13-equivalent) to
  completion. Freezing requires FEM's closure to be the frozen predecessor
  audit; doing it now would need to be redone and would duplicate the kind of
  incident already logged twice in the FEM gate history.

## Handoff on completion

When you stop (whether the draft preregistration is finished or you pause for
any reason), write a completion note at the bottom of
`docs/findings/ice-structural-audit-preregistration.md` (if created) stating
exactly what remains, and update `docs/findings/index.json` with an entry for
it at `status: draft_pending_fem_closure`. Do not mark Ice Age as ready to
open. Run `python scripts/python/validate_agent_context.py` before ending the
session.
