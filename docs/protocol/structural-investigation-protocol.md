# Structural-investigation protocol (v1.0, frozen 2026-08-26)

Status: **frozen** for the Arabian Nights investigation. Changes require a
protocol change record (section 9). Alpha (`docs/findings/lea-segmentation-audit.md`,
`docs/audits/lea/`) is the reference execution; where this document and the
Alpha documents disagree, this document wins for future sets and the Alpha
documents are corrected, not the protocol.

This protocol governs *structural* investigation only: where the boundaries of
Oracle-text units are, what CR category each unit's surface form belongs to,
where it sits in the card's structure, and whether the text or the rules
supply it. It does not define semantic operators, an IR, or executable
meaning. Nothing in it licenses describing segmenter output as parsing.

## 1. Vocabulary and the three-layer boundary

| Layer | What it is | Who owns it | May be claimed as |
|---|---|---|---|
| Observed text | Current Oracle text, type line, rulings, CR text | Scryfall / WotC | fact |
| Structural annotation | Unit boundaries, `kind`, `role`, `source`, parent links, CR citations | this protocol | verified finding or bounded observation, with denominators |
| Semantic hypothesis | What a unit *means*, equivalence between units, operator/argument structure | Phase 2 | hypothesis only |

Terms:

- **Unit** — one node emitted by `mtg-discover segment`: a span of one line of
  printed text (or a reminder-only line) with `kind`, `role`, `source`,
  optional `rule`, `text` (reminder text removed), `normalized` (template),
  `index` (pre-order within the card), `face`, `line`, and `children`.
- **Reference unit** — what the CR says the unit *should* be: one ability
  (CR 113.3a–d), or one non-ability structural element that Oracle text
  prints on its own (mode, mode header, additional cost, cast restriction,
  ante instruction, rules-supplied ability). Activation instructions
  (CR 602.1b) and delayed-trigger creation inside an effect are *inside*
  their ability unless the segmenter's declared nesting rules split them.
- **Boundary** — the span of a unit. Correct when the span equals exactly one
  reference unit.
- **Template** — the `normalized` string. Template equality is evidence of
  wording reuse, never of semantic identity.
- **Set** — a first-printing set (`first_set`), i.e. the set that introduced
  the card, analysed with *current* Oracle wording.

## 2. Frozen inputs (record before any measurement)

Every investigation begins by recording this block verbatim in its findings
document. Values come from commands, not from earlier documents.

```text
Repository commit:          git rev-parse HEAD
mtg-discover build:         cargo build --release (record `--version`)
cargo test:                 N passed, 0 failed
Scryfall bulk snapshot:     file dates of oracle-cards / rulings / default-cards .jsonl.gz
Corpus metadata:            mtg-discover info  (cards, with/without text, rulings, rule entries, CR effective date)
Set under investigation:    code, name, released_at, cards, cards_with_text, fallback_cards  (mtg-discover sets)
Earlier sets:               list of set codes already audited (regression corpus)
Held-out pool definition:   section 6.3, unchanged unless a change record says otherwise
```

Until the pipeline records snapshot identity in the database (tooling
requirement T1), the bulk-file modification dates *are* the snapshot
identifier, and they must be copied into the findings document.

## 3. Procedure

Execute the steps in order. Do not invent methodology mid-audit; if a step is
inadequate for the set, record the gap in the findings document and continue
with the step as written. Steps marked **[Codex]** identify tooling requirements;
use the documented interim path until the complete acceptance condition in
section 8 is met.

### S1 — Preconditions

- The previous gate has passed in writing (`docs/gates/`).
- `cargo build --release` and `cargo test` succeed at the recorded commit.
- The set's cards have not been read by the auditor in a heuristic-design
  context before this step (see section 6 for the leakage rules).

### S2 — Pre-audit baseline

Run and record, before reading any card:

```powershell
$mtg = ".\target\release\mtg-discover.exe"
& $mtg info
& $mtg sets --until <release date of the set>
& $mtg templates --set <code> --limit 5000
& $mtg templates --limit 5000          # corpus-wide, for the over-segmentation check in S11
```

Record: cards, cards with text, printed units, rules-supplied units, distinct
templates, singleton templates, coverage checkpoints, kind and role
histograms. These are the numbers a later segmenter change is compared to.

### S3 — Export the unit inventory

```powershell
python scripts/python/export_units.py <code> > docs/audits/<code>/units-export.tsv
```

The export is committed. It is the frozen object the annotation refers to;
`audit_metrics.py --export` detects drift if the segmenter changes later.
The native `audit export <code>` command provides the deterministic JSON
inventory used for exploration and agrees with Alpha's frozen TSV on all shared
fields. The script remains the annotation path until native TSV output satisfies
the complete T2 column contract in section 4.1.

### S4 — Review scope

- **Exhaustive** review (every unit of every card with text) is mandatory for
  sets with ≤ 400 cards with Oracle text. Alpha (275), Arabian Nights (77),
  Antiquities (85), Legends (290) and every set through 1995 qualify.
- Larger sets: exhaustive review of every unit whose template is *novel*
  relative to earlier audited sets (section 4.3), plus a seeded random sample
  of 100 non-novel units. The seed and the sample's `(oracle_id, index)` keys
  are committed with the audit.
- Sets with < 10 cards with text (e.g. `leb`, 2 cards) are audited as an
  appendix of the next set's investigation and recorded in their own
  `docs/audits/<code>/` directory.
- Excluded from the walk: cards with `first_is_fallback = 1`, and sets whose
  `type` is `memorabilia`, `promo`, `token`, `minigame`, or `alchemy`. They
  remain in corpus-wide measurements.

### S5 — Unit-level audit

Produce `docs/audits/<code>/units-annotated.tsv`: the export columns plus the
annotation columns of section 4.2, one row per unit, every row explicitly
dispositioned. Rules:

1. Read the card's full Oracle text and type line before dispositioning any
   of its units; read rulings when the disposition is not obvious from the CR.
2. Cite the CR rule(s) that determine the disposition in `cr_ref`. An
   uncited `defect` is not a defect; it is `adjudicate`.
3. Judge boundaries against the *reference unit* definition, not against what
   would be convenient for a parser.
4. Judge `kind` only on units whose boundary is `ok`; a mis-bounded unit's
   kind is not evaluated (record it, but it is excluded from the accuracy
   denominator).
5. `kind` for `mode` children is `n/a`. `kind` for `delayed_trigger` and
   `granted` children is judged normally.
6. Use `unsure` rather than guessing. `unsure` rows leave both numerator and
   denominator and are reported separately.
7. Record what context was needed to reach the disposition (`context`):
   `none` (surface form suffices), `cr` (a numbered rule was needed, e.g. to
   know a bare word is a keyword ability), `type_line` (card characteristics
   were needed, e.g. instant vs. permanent, CDA), `game_state` (the unit's
   existence or shape depends on the game state), `card_specific` (a ruling
   specific to this card was needed).
8. Tag structures with the controlled vocabulary in section 4.4; add a new
   tag only with a definition in the findings document.
9. One annotator pass is a **single-annotator, unadjudicated** audit and must
   be labelled so. A second independent pass, or an adjudication of every
   non-`accept` row and a 10% sample of `accept` rows, upgrades it to
   *adjudicated*. Gate 1 requires adjudicated sets.

### S6 — Hypotheses

Each investigation states its hypotheses before the audit in this form:

```text
H<n> (<one-word topic>). <Claim about structure, stated so that a count can refute it.>
Falsifier: <the concrete observation that would refute it>.
Denominator: <the population the count is taken over>.
```

Hypotheses about semantics are recorded as *out of scope* for this protocol
and carried to the deferred-work register.

### S7 — Evidence hierarchy and citation

Authority order (from `docs/current-state.md`): Comprehensive Rules >
current Oracle text > official rulings > corpus measurements > literature >
agent interpretation. In the annotation:

- a boundary or kind disposition must cite a CR rule;
- a ruling may support a disposition but cannot override the CR;
- a corpus count supports frequency claims only;
- an interpretation without CR or ruling support is `adjudicate`.

### S8 — Counterexample search

For every proposed structural rule (existing heuristic or new proposal):

1. State the rule as a surface pattern and the CR category it is meant to
   detect.
2. Search the corpus for the pattern (`mtg-discover cards <literal> --field
   text`) and for the *nearest non-matching wordings* (drop a word, invert
   order, put the pattern inside quotes or after a colon).
3. Inspect at least 20 hits, or all hits if fewer, spanning at least three
   decades of first printing (`sets`/`--set`).
4. Record the count of hits, the count inspected, and every counterexample
   found, with CR citation, in the findings document. A rule with an
   uninspected counterexample class is not accepted.
5. Cards in the held-out pool (section 6.3) may appear in counts but may not
   be quoted, inspected, or used as examples.

### S9 — Development, regression, held-out

Section 6.

### S10 — Heuristic acceptance

A segmenter or normalizer change proposed from an audit is accepted only
when all of the following are recorded in the change's findings section:

1. the CR rule(s) the heuristic implements, as a generic surface pattern (no
   card names, no `oracle_id`s, no per-card branches);
2. the defect rows it is meant to fix, by `(name, index)`;
3. the counterexample search of S8 for the new pattern;
4. corpus-wide before/after `templates` totals (units, rules-supplied,
   distinct templates, coverage checkpoints, kind and role histograms) and
   the over-segmentation check of S11;
5. `audit_metrics.py` re-run on every earlier annotated set with the new
   export, showing no new non-`accept` rows (regression), or an explicit
   re-annotation of the rows that legitimately changed;
6. regression tests per S12;
7. an updated `docs/current-state.md` baseline.

Proposals are written by the research lead; implementation and tests are
Codex's. A proposal that cannot be stated without naming a card is rejected.

### S11 — Corpus-wide over-segmentation check

Whenever a change adds splits (keyword lists, nested children, new kinds):

1. List every distinct *line* the new rule fires on corpus-wide
   (**[Codex]** T6; interim: `templates --limit 5000 --min-count 1` diffs
   plus targeted `cards` searches).
2. Inspect every produced unit that occurs ≤ 2 times corpus-wide, and a
   random 50 of the rest.
3. Report: lines matched, lines split, lines correctly refused, and every
   mis-split with its wording class. The Alpha keyword-list check (338
   candidate lines, 295 split, 43 refused, 58 rare items inspected) is the
   model.

### S12 — Required regression tests

For each accepted change, `src/main.rs` tests must include: one positive
case per defect row class it fixes (synthetic text is acceptable when the
wording is reproduced exactly), one negative case per counterexample class
found in S8, and one nesting/indexing case if the change touches children.
Test names must say what structure they protect. `cargo test` must pass
before the baseline numbers are refreshed.

### S13 — Unsupported and ambiguous cases

- `unsupported`: the current label vocabulary cannot express the correct
  annotation (e.g. a prevention static; `gap:<class>` in `kind_expected`).
  Recorded, counted, never forced into `spell_or_static_text` silently.
- `ambiguous`: the CR and rulings do not determine the correct annotation.
  Recorded with the competing readings; excluded from accuracy; listed in
  the findings document as an open question.
- `adjudicate`: the annotator could not decide; resolved by the second pass.
- None of these may be resolved by adding a card-specific rule.

### S14 — Final measurements and report

After the audit (and after any accepted change is merged and re-exported):

```powershell
python scripts/python/audit_metrics.py docs/audits/<code>/units-annotated.tsv `
    --export docs/audits/<code>/units-export.tsv `
    --earlier docs/audits/<earlier code>/units-export.tsv   # one per earlier audited set
```

The findings document (`docs/findings/<code>-structural-audit.md`) follows
the experiment-report template (section 7) and reports every field of
section 4.5 with numerator and denominator.

### S15 — Artifact package per set

```text
docs/audits/<code>/units-export.tsv        frozen unit inventory (S3)
docs/audits/<code>/units-annotated.tsv     dispositions (S5), one row per unit
docs/audits/<code>/metrics.json            audit_metrics.py output (S14)
docs/findings/<code>-structural-audit.md   hypotheses, baseline, findings, proposals, reproduction
src/main.rs tests                          only if a change was accepted (S12)
docs/current-state.md                      refreshed only if the corpus baseline changed
```

## 4. Schema and measurement definitions

### 4.1 Export columns (contract for T2)

`set, oracle_id, name, type_line, index, parent_index, depth, face, line,
kind, role, source, rule, text, normalized` — one row per unit, sorted by
name then index; `parent_index` empty for top-level units; `text` is the
printed text with reminder text removed (rules-supplied units keep the
parenthetical); UTF-8, tab-separated, no embedded newlines.

### 4.2 Annotation columns

| Column | Values | Meaning |
|---|---|---|
| `boundary` | `ok`, `under`, `over`, `misattached`, `unsure` | span = one reference unit; contains ≥ 2; is a fragment; right span wrong parent |
| `missed` | integer | reference units inside this unit that were not emitted (0 unless `under`) |
| `kind_expected` | a `kind` value, `n/a`, `gap:<class>`, `unsure` | what the label should be |
| `kind_ok` | `yes`, `no`, `n/a`, `unsure` | |
| `role_ok` | `yes`, `no`, `unsure` | |
| `source_ok` | `yes`, `no` | printed vs. rules-supplied |
| `context` | `none`, `cr`, `type_line`, `game_state`, `card_specific` | context needed for the disposition |
| `cr_ref` | `;`-separated rule ids | authority for the disposition |
| `structure_tags` | `;`-separated tags (4.4) | structures present |
| `norm_issue` | `collision:<tag>`, `fragmentation:<tag>` | suspected normalization issue (4.5) |
| `disposition` | `accept`, `defect`, `unsupported`, `ambiguous`, `adjudicate` | |
| `annotator` | free id | one value per pass |
| `note` | text | reasoning; required for every non-`accept` row |

### 4.3 Novelty

- **Template novelty** of a set = distinct printed templates in the set that
  occur in no earlier audited set's export ÷ distinct printed templates in
  the set.
- **Unit novelty** = printed units whose template occurs in no earlier
  audited set ÷ printed units in the set.
- "Earlier" means earlier *audited* sets' exports (not the whole corpus), so
  the denominator is documented and the metric is reproducible from committed
  files. Alpha's novelty is 1.0 by definition.

### 4.4 Structure-tag vocabulary (v1.0)

`keyword`, `keyword_list_split`, `rules_supplied`, `mana_ability`, `mode`,
`mode_header`, `granted_quoted`, `granted_quoted_parent`,
`short_quote_not_ability`, `delayed_trigger_next`, `delayed_trigger_parent`,
`delayed_trigger_inverted`, `delayed_trigger_when`,
`delayed_trigger_recurring`, `delayed_trigger_end_of_combat`,
`conditional_creation`, `activation_instruction`, `intervening_if`,
`state_trigger`, `enters_replacement`, `instead_in_spell`,
`instead_in_activated`, `prevention_static`, `cda`, `conditional_cda`,
`cast_restriction`, `cost_modification`, `payment_restriction`, `ante`,
`multi_sentence` (automatic: ≥ 2 sentence terminators), `name_predicate`,
`self_reference_name`, `self_reference_this_ability`, `text_change`,
`physical_action`, `player_control`, `one_off_candidate` (structure believed
unique in the corpus; verified only by a corpus count).

### 4.5 Measurements and denominators

| Field | Numerator | Denominator | Notes |
|---|---|---|---|
| Boundary precision | printed units with `boundary = ok` | printed units judged (excl. `unsure`) | emitted-unit correctness |
| Missed boundaries | Σ `missed` | — | count |
| Boundary recall | printed units with `boundary = ok` | ok + Σ `missed` | reference units recovered; `over` fragments inflate neither |
| Kind accuracy | `kind_ok = yes` | units with `boundary = ok` and `kind_ok ∈ {yes, no}` | modes and `unsure` excluded |
| Role accuracy | `role_ok = yes` | `boundary = ok` and `role_ok ∈ {yes, no}` | |
| Source accuracy | `source_ok = yes` | all units judged | |
| Normalization fragmentation | units tagged `fragmentation:*`, by tag | printed units | *suspected*, regex-flagged; verified only when an ablation shows the merge |
| Suspected collisions | units tagged `collision:*`, by tag | printed units | verified when two units with different CR semantics share a template |
| Unsupported structures | rows `unsupported`, by `kind_expected` | all units | |
| CR / type-line / game-state context | rows by `context` | all units | |
| Card-specific exceptions | rows with `context = card_specific` | all units | must be 0 in accepted heuristics |
| Unit / template novelty | 4.3 | 4.3 | |
| Multi-sentence units | tag `multi_sentence` | printed units | automatic |

Rates are reported as `numerator / denominator (value)`. No rate is reported
for a field that was not annotated. Single-pass results are labelled
*single-annotator*.

## 5. Interim tooling used by this protocol

| Script | Role | Replace with |
|---|---|---|
| `scripts/python/export_units.py` | flatten `segment` output for a set | T2 |
| `scripts/python/audit_metrics.py` | measurements of 4.5, novelty, drift | keep; consumes T2 output |

Both are standard-library Python and add no segmentation logic.

## 6. Development, regression, held-out

### 6.1 Development corpus

The set under investigation. Its cards may be read, quoted, and used to
design heuristics.

### 6.2 Regression corpus

Every earlier audited set's `units-annotated.tsv`. A change is regressed by
re-exporting each and running `audit_metrics.py --export`; any changed unit
text or new non-`accept` row must be explained.

### 6.3 Held-out pool (frozen 2026-08-26)

Definition: every card with Oracle text whose `oracle_id` begins with the
hex digit `f`, with `first_is_fallback = 0`, excluding sets `lea`, `leb`,
`arn`. Size at the 2026-08-25 snapshot: **2,096 cards** (1990s 295, 2000s
386, 2010s 600, 2020s 836; 67 multi-face). `lea` (14 such cards) is
excluded because Alpha was read exhaustively before the freeze; `leb` and
`arn` because they are the next development sets.

Rules:

- Pool cards are never quoted, inspected, or cited as examples or
  counterexamples while designing heuristics. They may be counted in
  corpus-wide aggregates.
- The pool is sampled and annotated only at a gate review; the sample's keys
  are committed with the gate package.
- Any accidental inspection is logged in the findings document with the
  card, and the card is excluded from later held-out samples.
- When the era walk reaches a set, that set's pool cards remain held-out; the
  set's development corpus is its non-pool cards. (Alpha and Arabian Nights
  predate the freeze and are exceptions, recorded here.)

**[Codex]** T7: an `--exclude-heldout` flag on `cards`, `segment`, and export.

## 7. Report templates

### 7.1 Experiment report (`docs/findings/<code>-structural-audit.md`)

```markdown
# <Set name> (<code>) structural audit
Date · Commit · Snapshot (bulk-file dates) · CR effective date · Annotator(s) · Adjudicated: yes/no
## Scope            (set definition, exclusions, review scope per S4)
## Hypotheses       (S6 format)
## Pre-audit baseline (S2 numbers, verbatim)
## Verified findings   (each: claim, count/denominator, CR citation, examples, counterexamples)
## Bounded observations
## Unsupported and ambiguous cases
## Proposed changes  (S10 items 1–3 for each; none implemented here)
## Measurements      (4.5 table; novelty)
## Reproduction      (every command)
```

### 7.2 Decision record

```markdown
## Decision: <title>  (date, decider)
Evidence · Options considered · Decision · What would reverse it · Affected documents
```

### 7.3 Gate report (`docs/gates/gate-<n>-evidence.md`)

Frozen inputs · criterion-by-criterion evidence with reproduction commands ·
required deliverables · known failure classes · what is missing · decision
record · exact evidence that would change the decision.

## 8. Tooling requirements for Codex (research observability, not parsing)

Ordered by how early the protocol needs them. Each is a requirement, not an
implementation; the research lead does not implement them.

The native `audit` command now partially satisfies T2 (deterministic JSON
export), T3 (summary denominators), T4 (novelty, including an explicit
`--earlier` audited-set comparison), and T5 (declared suspicious signals).
T2 remains open for contract-compatible TSV and the complete section 4.1
columns; the table records the full acceptance conditions rather than partial
implementation status.

| Id | Requirement | Serves | Acceptance |
|---|---|---|---|
| T1 | Record bulk-snapshot identity (file name, size, mtime or Scryfall `updated_at`, sha256) in `cards.sqlite` and print it from `info` | §2 | `info` output identifies the snapshot without prose |
| T2 | `export --set <code>` emitting every unit of the set with the 4.1 columns, deterministic order, JSON and TSV; stable unit key = `(oracle_id, face, index)` | S3, S5 | byte-identical output on repeated runs; matches `segment` per card |
| T3 | `templates --set` reporting singleton count and full template list beyond 5,000 (or `--all`) | S2 | denominators computable without scripts |
| T4 | Novelty against earlier sets: `audit novelty arn --earlier lea,leb` reporting unit and template novelty with the 4.3 denominators | S14 | agrees with `audit_metrics.py --earlier` |
| T5 | Residual and suspicious-case inventory: units matching a declared surface pattern but not split/classified (e.g. `at the beginning of the next` outside a split child; quoted strings not made children; `instead` in instants) as a listable report | S8, S11 | list is diffable across commits |
| T6 | Rule-firing inventory: for each heuristic, the distinct corpus lines it fires on, with counts | S11 | reproduces the Alpha keyword-list check |
| T7 | `--exclude-heldout` (pool definition of 6.3) on `cards`, `segment`, `export` | 6.3 | pool cards absent from output |
| T8 | `segment`/export field for the CR-slot decomposition of activated and triggered units (`cost`, `effect`, `instructions`; `trigger`, `effect`) as *additional* fields, without changing unit boundaries | Alpha V5, N3 | Alpha activated/triggered rows decompose; counterexamples listed |
| T9 | Consistent naming: `templates.coverage[].lines` → `units` (or documented alias) | criterion 0.2 | docs and JSON use one word |
| T10 | Normalization proposal (research lead's, needs S10 evidence before acceptance): keep `{T}`/`{Q}` distinct from mana symbols | Alpha collision `tap_as_mana` | corpus before/after totals |

Deterministic, diffable output is the overriding property: every report
must be stable across runs for a fixed database and commit.

## 9. Protocol change control

A change to this protocol is a decision record (7.2) appended to
`docs/gates/` for the gate in force, naming the section changed, the
evidence, and the sets audited under the previous version. Annotation files
carry the protocol version in their findings document; a schema change must
provide a migration for every committed `units-annotated.tsv`.
