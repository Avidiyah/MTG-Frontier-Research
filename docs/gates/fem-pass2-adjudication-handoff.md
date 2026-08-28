# Fallen Empires (`fem`) pass-2/adjudication handoff — for Codex

- Date prepared: 2026-08-28
- Prepared by: Claude Code (replacement pass-1 annotator, `human-fem-pass`)
- Prepared for: the next Codex terminal session
- Status: **PAUSED — the assigned pass-2 identity was exposed to pass-1
  judgements by the mandatory routing documents; pass 2 remains blank and
  adjudication has not started**
- Basis: `docs/gates/fem-entry-record.md` §3.8 (governance pause)

## Current stop condition

The previous version of this handoff instructed
`copilot-pass2-2026-08-28` to read `docs/findings/index.json` and
`docs/gates/fem-entry-record.md` in full. Both disclosed sealed pass-1
judgements; §3.7 of the entry record also disclosed field outcomes and named
row-level cases while simultaneously forbidding pass-2 access. The assigned
pass-2 identity stopped before opening either TSV or the frozen export.

Hash-only verification at `2026-08-28T16:19:31-05:00` confirmed:

```text
pass 1 sha256: bbc90f7be3d089afd8fd71cc1f7660472ee99bab6dd2c70bb5c49798a90b9a55
pass 2 sha256: 691f8b8c4f54fcbcafacc716ece331ff348e32d8ff0c5f40d5bfe79f1fe92c25
```

Pass 2 is still byte-identical to the frozen blank. Do not open either pass
TSV, the export, or any eligible FEM card in this session.

## Required next decision

Under preregistration §11.2, the research lead/program owner must:

1. retire `copilot-pass2-2026-08-28` from the pass-2 role;
2. assign a new independent pass-2 annotator with a fresh attestation;
3. provide a pass-2-safe routing packet that does not require reading
   `fem-entry-record.md` §3.7 or any other pass-1 judgement;
4. keep pass 2 and `fresh-fem-adjudicator-2026-08-28` as genuinely separate
   review identities and sessions; and
5. explicitly reauthorize annotation before the new pass opens the blank TSV.

Only after a valid pass 2 seals may comparison and adjudication resume under
preregistration §7.3-§7.4. The active session must not assume that decision.

## Do not touch

- Ice Age (`ice`) preparation is being done in a separate session (Sonnet 5)
  concurrently. Do not open, freeze, or query Ice Age cards, and do not edit
  any `ice`-scoped file it creates (see
  `docs/gates/ice-age-preparation-handoff.md` if you want to know its scope).
  If you finish FEM before that work lands, stop and hand off rather than
  starting Ice Age yourself, to avoid two freezes racing on the same
  commit.
