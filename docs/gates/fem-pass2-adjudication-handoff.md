# Fallen Empires (`fem`) pass-2/adjudication handoff — for Codex

- Date prepared: 2026-08-28
- Prepared by: Claude Code (replacement pass-1 annotator, `human-fem-pass`)
- Prepared for: the next Codex terminal session
- Status: **pass 1 sealed; pass 2 not yet annotated; adjudication not started**
- Basis: `docs/gates/fem-entry-record.md` §3.5-§3.7 (roster, restart, seal)

## Read first

1. `docs/agent/agent-contract.md`, `docs/current-state.md`,
   `docs/findings/index.json` — in that order.
2. `docs/gates/fem-entry-record.md` in full, especially §3 (role history and
   incidents) and §3.7 (pass-1 seal).
3. `docs/findings/fem-structural-audit-preregistration.md` §6-§10 (agreement
   measures, adjudication workflow, unsupported/ambiguous meanings, D14/D19
   handling).
4. `docs/protocol/structural-annotation-guide-v1.0.md` (frozen v1.0) if you
   have not already internalized it for this audit.

## What is already done (do not redo)

- The audit is **open**. Roster: pass 1 `human-fem-pass` (sealed), pass 2
  `copilot-pass2-2026-08-28` (restarted, not yet sealed), adjudicator
  `fresh-fem-adjudicator-2026-08-28`.
- Pass 1 is sealed: `docs/audits/fem/units-annotated-pass1.tsv`, sha256
  `bbc90f7be3d089afd8fd71cc1f7660472ee99bab6dd2c70bb5c49798a90b9a55`, 176 rows,
  every row disposed. **Do not open this file until pass 2 is independently
  sealed** — reading it now would break pass independence exactly as the
  original incident did.
- Pass 2's file was reset to the frozen blank hash
  `691f8b8c4f54fcbcafacc716ece331ff348e32d8ff0c5f40d5bfe79f1fe92c25` after the
  prior deviation was preserved as incident evidence at
  `docs/audits/fem/units-annotated-pass2-incident-preserved-2026-08-28.tsv`
  (sha256 `91abeb36c9d08e8f5d41bcfa1e39d09253c56dbaf7c9b4f04a757430235cb74a`,
  do not modify).

## What you are here to do

1. As `copilot-pass2-2026-08-28`, independently annotate all 176 rows of
   `docs/audits/fem/units-annotated-pass2.tsv` against the eligible Oracle
   text, using only the frozen protocol and guide. Do not open pass 1. Do not
   run unfiltered card/corpus searches. Use `unsure`/`unsupported`/`ambiguous`
   rather than guessing.
2. **Do not use `git diff`, `git diff --check`, or any other row-printing Git
   command on staged audit TSVs** — this is exactly how the original pass-1
   identity was disqualified (§3.4). Use hash-only verification
   (`Get-FileHash`) instead.
3. Seal pass 2 with a content hash and timestamp, recorded in
   `docs/gates/fem-entry-record.md` as a new §3.8, in the same style as §3.7.
4. Once both passes are sealed, perform the §7.3 comparison (align by
   `(oracle_id, face, index)`, compute preregistered agreement measures,
   publish confusion counts, list every disagreement).
5. As adjudicator (`fresh-fem-adjudicator-2026-08-28`), resolve disagreements
   and non-`accept` rows per §7.4/§9. CR and Oracle text control; a genuine
   ambiguity stays `ambiguous`; a vocabulary gap stays `unsupported` with
   `kind_expected = gap:<class>`.
6. Populate `docs/findings/fem-structural-audit.md` only after adjudication,
   under protocol S8-S12 — do not add measurements to it before that.
7. Update `docs/gates/fem-entry-record.md`, `docs/findings/index.json`, and
   `docs/current-state.md` when the audit closes, and run
   `python scripts/python/validate_agent_context.py`.

## Do not touch

- Ice Age (`ice`) preparation is being done in a separate session (Sonnet 5)
  concurrently. Do not open, freeze, or query Ice Age cards, and do not edit
  any `ice`-scoped file it creates (see
  `docs/gates/ice-age-preparation-handoff.md` if you want to know its scope).
  If you finish FEM before that work lands, stop and hand off rather than
  starting Ice Age yourself, to avoid two freezes racing on the same
  commit.
