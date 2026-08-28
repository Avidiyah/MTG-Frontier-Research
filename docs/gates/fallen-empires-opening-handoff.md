# Fallen Empires (`fem`) pass-1 opening handoff

- Date prepared: 2026-08-28
- Prepared for: next clean governance/replacement-pass-1 session
- Prepared by: `codex-pass1-2026-08-28` (now disqualified; entry record §3.4)
- Status: **audit closed; replacement pass 1, pass-2 reconciliation, and
  program-owner reauthorization required**
- Basis: `docs/gates/fem-entry-record.md` through §4.3 and the later §3.4
  independence incident, 2026-08-28
- Exposure disclosure: this session did not open either TSV directly or
  annotate a row, but `git diff --cached --check` printed eligible row text and
  pass-2 judgements; the identity is disqualified from all `fem` review roles

## Starting position

Fallen Empires preparation is complete at measurement-freeze commit
`2823b122`: the held-out-safe development export contains 176 rows and has
SHA-256 `095a25a7a0729bca12d515b2ce0a7395c0484d1fc335d11a913dec8c6c3b0d74`.
Pass 1 remains the frozen blank file with SHA-256
`691f8b8c4f54fcbcafacc716ece331ff348e32d8ff0c5f40d5bfe79f1fe92c25`.

Program owner Avidiyah assigned original pass 1
`codex-pass1-2026-08-28`, pass 2 `copilot-pass2-2026-08-28`, and adjudicator
`fresh-fem-adjudicator-2026-08-28`. The original pass-1 assignment and §4.3
conditional authorization are now suspended by entry-record §3.4: during the
requested commit workflow, `git diff --cached --check` printed row-bearing TSV
lines, exposing original pass 1 to eligible text and pass-2 judgements before
either pass sealed. Original pass 1 stopped without annotating and is
disqualified from every Fallen Empires annotator/adjudicator role.

The earlier pass-2 gate issue also remains incomplete. Before authorization was recorded, the
pass-2 path stopped matching its frozen blank hash and now hashes to
`91abeb36c9d08e8f5d41bcfa1e39d09253c56dbaf7c9b4f04a757430235cb74a`.
A governance-only check identified its `annotator` metadata as
`copilot-pass2-2026-08-28`; no row judgement, card text, citation, tag, or note
was displayed. That identity has not supplied the required independence
attestation or explained the pre-authorization modification. Entry checklist
item 8 therefore remains partial, and the audit is not open.

## First actions

1. Read `docs/agent/agent-contract.md`, `docs/current-state.md`, and
   `docs/findings/index.json` in that order; route as
   `structural-audit-research` and read the frozen protocol and guide fully.
2. Read `docs/gates/fem-entry-record.md`, especially §3.2, §3.4, §4.2, and
   §4.3.
   Do not open either annotation TSV or any eligible Fallen Empires card.
3. Have Avidiyah assign a new pass-1 identity distinct from original pass 1,
   pass 2, the adjudicator, and the disqualified preparer. The replacement
   must sign a fresh independence attestation before row access.
4. Obtain a signed attestation directly from the assigned pass-2 identity
   `copilot-pass2-2026-08-28`, binding the frozen protocol, guide,
   preregistration, export, and blank pass-2 hash. It must disclose when and
   under what authority the file changed and affirm independence from pass 1.
5. Have Avidiyah record a preserve-or-restart decision for pass 2.
   If preserving, record why the deviation does not compromise independence;
   if restarting, preserve the current file as incident evidence and create a
   new, hashed blank path—do not overwrite it.
6. Mark checklist items 7-8 satisfied only when the evidence supports them,
   then obtain explicit program-owner reauthorization. The superseded §4.3
   authorization does not reactivate automatically.
7. Only after reauthorization may the replacement pass-1 session open
   `docs/audits/fem/units-annotated-pass1.tsv`.
8. During pass 1, do not read pass 2, run unfiltered set/card searches, discuss
   classifier proposals, or inspect held-out identities. Annotate every row
   under the frozen guide, then seal pass 1 with SHA-256 and timestamp in the
   entry record. Leave findings and comparison work for after both passes seal.

## Opening boundary

The next session must stop before row access unless entry-record checklist
items 7-8 are **satisfied** and a new program-owner authorization is recorded
after both incidents. The adjudicator may inspect nothing until both
independent passes are validly sealed. Do not use `git diff`, `git diff
--check`, or another row-printing Git command on staged audit TSVs in any
annotator session; restrict checks to control documents or use hash-only
verification.

## Required handoff on completion

After resolving the gate or sealing pass 1, update this handoff, the entry
record, and their `docs/findings/index.json` summaries. Update
`docs/current-state.md` if the repository-wide audit state changes. Record the
new artifact hash, timestamp, what was verified, and the next open question.
Run:

```powershell
python scripts/python/validate_agent_context.py
```
