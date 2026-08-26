# Antiquities (`atq`) structural audit, with the post-P-ARN corpus checks

Date: 2026-08-26 · Commit: `af150b0` (mainline after Codex's P-ARN-1..4) ·
Build: `mtg-discover 0.1.0`, `cargo test` 41 passed ·
Snapshot: bulk files dated 2026-08-26 01:38 (Scryfall drop 2026-08-25), `cards.sqlite` 01:40 ·
CR effective 2026-08-07 · Protocol: v1.0 ·
Annotators: Fable (pass 1) and an independent forked agent (pass 2) ·
Adjudicated: **yes** — the two passes agree on every judgement field of all 125 units (protocol S5.9)

## Scope

- `atq`: 85 cards, all with Oracle text, 0 fallback, released 1994-03-04.
  Exhaustive review. 125 printed units (120 top-level, 5 children), 0
  rules-supplied.
- This audit ran on the *new* segmenter (P-ARN-1..4 implemented by Codex in
  `af150b0`), so it doubles as the first set audited against those rules.
  Section 5 records the research lead's review of Codex's implementation and
  of the annotation rows Codex re-dispositioned; sections 6–7 record the
  protocol S11 corpus-wide checks that review required.
- Held-out pool (protocol §6.3) excluded from every quoted example; pool cards
  are counted only in aggregate totals.

## 1. Hypotheses (stated before the audit)

- **N1 (novelty).** Unit novelty against `lea`+`leb`+`arn` stays below
  Arabian Nights' 77/109 as the earlier-set pool grows. *Falsifier:* novelty
  ≥ 0.706. *Denominator:* printed units.
- **N2 (keywords).** No non-keyword unit is labelled `keyword_ability`.
- **F-new (post-fix).** After P-ARN-1/2 no Antiquities unit contains an
  unsplit `at the beginning of … next` / `at end of combat` / scoped `When`
  phrase, and no split produces a fragment that is not a reference unit.
  *Falsifier:* any such row.
- **F7-new.** `prevention_effect` labels exactly the CR 615 statics.

## 2. Pre-audit baseline (S2, recorded before any card was read)

| Cards | With text | Printed | Rules-supplied | Templates | Singletons | Top-10 / 25 / 50 / 100 coverage | Kinds | Roles |
|---|---|---|---|---|---|---|---|---|
| 85 | 85 | 125 | 0 | 114 | 107 | 16.8 / 28.8 / 48.8 / 88.8 % | activated 47 · static/spell 25 · triggered 25 · keyword 13 · replacement 9 · prevention 3 · CDA 2 · ante 1 | ability 120 · delayed trigger 3 · granted 2 |

Corpus baseline at `af150b0`: 71,682 printed + 970 rules-supplied units,
37,344 templates, top-100 coverage 26.85 % (`docs/current-state.md`).

## 3. Verified findings

### V1. Export defect found and fixed: name collisions

`segment --card <name>` resolves names with `LIMIT 1`; Antiquities'
Shapeshifter shares its name with five token cards, so
`scripts/python/export_units.py` exported a Changeling token instead
(123 units vs. `templates`' 125). Fixed by segmenting each card's own
`oracle_text` with `--text/--name/--type-line`. After the fix the script and
the native `audit export atq` agree on every shared field for all 125 units,
and `lea`/`leb`/`arn` exports are byte-identical (modulo checkout CRLF) —
evidence toward the T2 acceptance condition.

### V2. One missed boundary: an unscoped `When …` delayed trigger (F2 class)

Tawnos's Coffin #1: `… Note the number and kind of counters … When this
artifact leaves the battlefield or becomes untapped, return that exiled card
… If you do, return the other exiled cards …` — the ruling names "its delayed
triggered ability" (CR 603.7a/e). Sentence-initial `When` without `this
turn`/`this way`, so outside P-ARN-2; same class as Animate Dead (Alpha).
Both passes: `under`, missed 1.

### V3. One over-segmentation: a condition-only parent (rule (c) class)

Battering Ram #1 `Whenever this creature becomes blocked by a Wall,` + child
#2 `destroy that Wall at end of combat.` The child is a correct delayed
trigger (Gorgon Recluse ruling class); the parent is a bare trigger
condition — a slot under CR 113.3c, not a reference unit. Both passes:
`over` (pass 2 called it a defect outright; pass 1 called it `adjudicate`
pending the corpus check in §6, which settled it as a defect).

### V4. Everything else segments and labels correctly (123 / 125)

Notable accepted rows: Rakalite and Rocket Launcher sentence-level delayed
triggers (rulings confirm); Rocket Launcher's parent is now
**non-contiguous** (the delayed sentence sat between the effect and the
activation instruction — the export format cannot express the gap; noted
for T2/T8); three `prevention_effect` statics (Argothian Pixies #1,
Argothian Treefolk, Artifact Ward #2; CR 615.1a); Shapeshifter #2 is a CDA
(ruling explicit; a *remembered-choice* CDA) and #1 a once-per-upkeep
trigger (ruling); Urza's Mine / Power Plant / Tower are activated mana
abilities with a conditional `instead` (correctly not replacement); Clockwork
Avian #2's recurring `At end of combat, if …` stays top-level; Energy Flux
and Tetravus quoted abilities are granted children (Tetravus grants text to
*tokens it creates*).

### V5. Keyword labelling (N2) holds

13 keyword units (Flying ×4, First strike ×2, Banding, Defender, Trample,
Vigilance, Enchant artifact ×2, Enchant creature); none mislabelled.
Wall of Spears and Yawgmoth Demon carry keywords on separate Oracle lines
(no list split).

### V6. Novelty against `lea`+`leb`+`arn` (N1)

Unit novelty **96 / 125 (0.768)**, template novelty **95 / 114 (0.833)**,
earlier distinct templates 295. **N1 is falsified at this step**: novelty
rose from 0.706 to 0.768 even though the earlier pool grew. Antiquities is
an artifact-themed expansion whose units are mostly activated abilities with
non-mana costs (sacrifice/discard/counter-removal), a cost form nearly absent
from Alpha and Arabian Nights. Novelty tracks *theme*, not only date; the
hypothesis needs a per-kind or per-cost-form breakdown before it is
informative (deferred D20).

## 4. Bounded observations

- New structures (tags): remembered-choice CDA (`Shapeshifter`),
  enters-as-choice-of-stats replacement (Primal Clay), `leaves or becomes
  untapped` delayed trigger with noted counters (Tawnos's Coffin),
  conditional-`instead` mana abilities (Urza's lands, CR 605.1a),
  mana-producing dies trigger (Su-Chi, 605.1b), tokens with granted text
  (Tetravus), effect-persistence clause (Titania's Song), mass animation and
  `loses all abilities` (Titania's Song, Xenic Poltergeist), ownership
  transfer (Bronze Tablet), non-mana activation costs as the majority cost
  form (`nonmana_cost` ×10), `named X` predicate on another object (Goblin
  Artisans, preserved by normalization), `{T}` appearing *inside a trigger
  condition* (Artifact Possession, Haunting Wind, Powerleech) where the
  automatic `collision:tap_as_mana` flag is a false positive.
- Context: none 101 · CR 22 · type line 2 (the CDAs) · game state 0 ·
  card-specific 0. Multi-sentence 27 / 125 (automatic).
- Suspected fragmentation flags: object type 2, colour 0, land type 1.

## 5. Research-lead review of Codex's P-ARN implementation (`af150b0`)

Scope of the review: the 544-line `src/main.rs` change is Codex's; this
section reviews its *effect on annotated units* and the annotation rows
Codex re-dispositioned (`annotator … codex-accepted`). Under protocol S5
those dispositions are the research lead's; every changed row was re-read
and now carries a `fable-review-2026-08-26` tag.

| Proposal | Implemented as | Lead verdict |
|---|---|---|
| P-ARN-1 sentence-level delayed split | as proposed for sentence-level cases (Dragon Whelp, Stone Giant, Nettling Imp, Rakalite, Rocket Launcher) — **ratified**; plus an un-proposed rule (c): for a *single* sentence, split at the last `: ` or `, ` before the phrase (Cockatrice, Thicket Basilisk, Rukh Egg, Nafs Asp, Battering Ram) | **rule (c) rejected** — see §6; those five parents are re-dispositioned `over`/`defect` |
| P-ARN-2 scoped `When` split | as proposed (`this turn`, `this way`, `When you do`) | **ratified** (Sandals of Abdallah); corpus check §7 C: 40/40 correct |
| P-ARN-3 type-line spell text | as proposed, per-face type lines | **ratified** (Disintegrate, Camouflage, Eye for an Eye); §7 B: 0 lexical misfires remain on 12,468 instant/sorcery-face units |
| P-ARN-4 `prevention_effect` kind | as proposed | **ratified** (Rock Hydra, Camel, Desert Nomads); §7 A: two misfire classes found |

Process note for the next cycle: implementation must not also close the
annotation rows; S10 item 5 asks for re-annotation *by the annotator*, and
S11 must be executed corpus-wide before acceptance (Codex's check covered
only the three audited sets plus `audit signals`). The corpus checks below
are what S11 requires; they are now scripted under
`scripts/python/corpus_checks/` with reports in `docs/audits/corpus-checks/`.

## 6. S11 corpus check A — delayed-trigger splits (`2026-08-26-delayed-split-overseg.md`)

Over 71,682 printed units, 982 `delayed_trigger` children (all with
resolvable parents):

| Measure | Value |
|---|---|
| Split class | sentence-level 861 / 982 · comma-level 115 · colon-level 6 · other 0 |
| Comma/colon parents (121) | bare trigger condition 108 · bare cost 5 · condition + partial effect 5 · ability-word/quote fragments 3; 70 distinct parent templates (`When ~ dies,` ×10, `When ~ attacks or blocks,` ×10, `When ~ enters,` ×7 …) |
| Judgement sample (40 comma/colon splits) | child is a delayed trigger created by the parent's effect: 38 / 40; parent fragment is a reference unit: **0 / 40** |
| Children starting lowercase | 115 / 982 (all comma/colon); starting `and`: 2 (a split inside a colour list) |
| Splits inside a quoted granted ability | 3 (`rfind` runs on unmasked text) — defect |
| Sentence-level sample | 30 / 30 delayed (603.7) or reflexive (603.12) triggers |
| Residual phrase outside a delayed child | 93 / 71,682: 14 top-level triggers that begin with the phrase (correct), 69 whole-line spell text (Ice Age cantrips — one spell ability, nothing to split), ~10 `[condition], at end of combat, …` multi-clause forms |
| Recurring `At end of combat, if …` triggers | 7, none split |

Conclusion: rules (a)/(b) are sound; rule (c) creates 113 fragments that are
not reference units and 3 in-quote mis-splits, and it promotes bare trigger
conditions to top-level templates (`When ~ dies,` is now in the corpus
top-5,000). Every condition-only parent in `lea`/`arn`/`atq` is
dispositioned `over`/`defect`.

## 7. S11 corpus check B — kind rules (`2026-08-26-kind-rules-check.md`)

- **A. `prevention_effect` (181 units, 10 in pool):** roles ability 161 /
  mode 14 / granted 6; 0 on instant/sorcery faces. Of the 161 top-level:
  **144 correct** (CR 615.1a statics); misfires: 9 `… can't be prevented`
  statics (rule-modifying prohibitions, not prevention effects) and 8
  ability-word / Saga-chapter / named-mode prefixed triggers (`Constellation
  — Whenever …`, `I, II — Prevent …`) whose prefix hides the trigger word.
  No misses (9 residual statics containing `prevent` are granting frames).
- **B. Type-line rule (12,468 units on instant/sorcery faces):**
  replacement / prevention / CDA labels: **0**. 111 top-level
  `triggered_ability` units: 65 off-stack abilities (cycling, flashback-style
  zones; CR 113.6b) correct; 16 cast/resolve triggers correct; **30 are
  spells whose entire text is a delayed trigger** (`Whenever a creature
  blocks this turn, …`; CR 603.7d) — a class P-ARN-1/2 cannot reach because
  there is no preceding effect text. Multi-face: 20 / 20 sampled faces use
  the right per-face type line.
- **C. Scoped `When` children (435 / 982: `When you do` 293, `this turn`
  75, `this way` 67):** 40 / 40 sampled are delayed or reflexive triggers;
  0 independent-trigger (D14) misfires.
- **D. Residual unsplit `. When you do` / `. When … this turn`:** 0.

## 8. Proposed changes (S10 items 1–3; for Codex via decision; none names a card)

**P-ATQ-1 — retract split rule (c).** Keep the sentence-level rules (a)/(b).
For a single sentence containing a delayed-trigger phrase, split only when
the text before the split point is a complete effect clause: never at the
comma closing a leading `When/Whenever/At …` trigger condition (CR 113.3c),
never at the activation-cost colon (602.1a), and never at a comma or colon
inside quotes (mask before searching). When no such point exists, keep the
unit whole and report the delayed-trigger creation as a signal / T8 slot.
Fix rows: Cockatrice #1, Thicket Basilisk #0, Rukh Egg #0, Nafs Asp #0,
Battering Ram #1 (these revert to `under` with a recorded in-unit delayed
trigger — an honest miss rather than a fragment). Expected corpus effect:
~113 / 121 comma-colon children revert (982 → ~869), 3 in-quote splits
vanish, no bare-condition templates remain top-level.

**P-ATQ-2 — `can't be prevented` is not a prevention effect.** Exclude text
matching `can't be prevented` from `prevention_effect`; classify as residual
static. Fix rows: the 9 units listed in check B §A (CR 615.1a defines
prevention effects by *preventing* damage).

**P-ATQ-3 — strip ability-word / chapter / named-mode prefixes before
classification.** A leading `<words> — ` (no period, no colon, ≤ 45 chars)
is an ability word, Saga chapter, or named mode (CR 207.2c, 714.2); classify
the text after the dash and record the prefix as a field. Fix rows: the 8
prefixed triggers mislabelled prevention, and (to be measured) every other
prefixed unit currently classified residual. Needs its own S8/S11 pass.

**P-ATQ-4 — spell-only delayed triggers.** On an instant/sorcery face, a
top-level unit beginning `When/Whenever/At` with a stated duration (`this
turn`, `this combat`, `next`) and no off-stack zone reference is spell text
that creates a delayed trigger (CR 603.7d): keep kind `triggered_ability`,
set role `delayed_trigger` with the face as parent. Fix rows: the 30 units
in check B §B. Needs an S8 search for off-stack exceptions before
acceptance.

**Deferred, not proposed:** the unscoped `When …` after effect text
(Tawnos's Coffin, Animate Dead) — needs its own S8 search separating it
from D14's independent triggers (D19).

## 9. Measurements (protocol §4.5; `docs/audits/atq/metrics.json`)

| Field | `atq` |
|---|---|
| Printed / rules-supplied units | 125 / 0 (120 top-level, 5 children) |
| Boundary precision | 123 / 125 (0.984): ok 123 · under 1 · over 1 · unsure 0 |
| Missed boundaries · recall | 1 · 123 / 124 (0.992) |
| Kind accuracy | 123 / 123 (1.0); 2 children n/a |
| Role · source accuracy | 123 / 123 · 125 / 125 |
| Dispositions | accept 123 · defect 2 |
| Context | none 101 · CR 22 · type line 2 · game state 0 · card-specific 0 |
| Unit / template novelty vs `lea`+`leb`+`arn` | 96 / 125 (0.768) · 95 / 114 (0.833) |
| Multi-sentence units (automatic) | 27 / 125 |
| Inter-annotator agreement (pass 1 vs pass 2, all judgement fields) | 125 / 125 |
| Drift vs fresh export | 0 |

After the lead review, Alpha stands at boundary 398 / 402 (under 2, over 2,
unsure 1; accept 409 · defect 5 · adjudicate 2 · ambiguous 1) and Arabian
Nights at 110 / 112 (over 2; accept 110 · defect 2).

## 10. Reproduction

```powershell
git rev-parse HEAD ; cargo build --release ; cargo test
$mtg = ".\target\release\mtg-discover.exe"
& $mtg info ; & $mtg sets --until 1994-03-04 ; & $mtg templates --set atq --limit 5000
python scripts/python/export_units.py atq > docs/audits/atq/units-export.tsv
& $mtg audit export atq                                  # must agree with the TSV on shared fields
python scripts/python/audit_metrics.py docs/audits/atq/units-annotated.tsv --export docs/audits/atq/units-export.tsv --earlier docs/audits/lea/units-export.tsv --earlier docs/audits/leb/units-export.tsv --earlier docs/audits/arn/units-export.tsv
python scripts/python/corpus_checks/dump_corpus_units.py corpus-units.jsonl          # ~1 min, gitignored output
python scripts/python/corpus_checks/check_delayed_split.py corpus-units.jsonl docs/audits/corpus-checks/2026-08-26-delayed-split-overseg.md
python scripts/python/corpus_checks/check_kind_rules.py corpus-units.jsonl docs/audits/corpus-checks/2026-08-26-kind-rules-check.md
python scripts/python/corpus_checks/check_kind_rules_part2.py corpus-units.jsonl docs/audits/corpus-checks/2026-08-26-kind-rules-check.md
& $mtg card "82a6d89d-9215-4540-b7d5-26cdd6afb05b" --rulings   # Shapeshifter (atq) by oracle_id; the name is ambiguous
& $mtg card "Tawnos's Coffin" --rulings ; & $mtg card "Gorgon Recluse" --rulings
```

The second annotation pass is `docs/audits/atq/units-annotated-pass2.tsv`
(annotator `fork-pass2`); the adjudicated file is `units-annotated.tsv`.
