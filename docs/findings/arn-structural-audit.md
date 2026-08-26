# Arabian Nights (`arn`) structural audit, with Beta (`leb`) appendix

Date: 2026-08-26 · Commit: `b02239e` (no `src/` change since `02793f3`) ·
Build: `mtg-discover 0.1.0`, `cargo test` 22 passed ·
Snapshot: `oracle-cards.jsonl.gz` / `rulings.jsonl.gz` / `default-cards.jsonl.gz`
dated 2026-08-26 01:38 (Scryfall drop of 2026-08-25), `cards.sqlite` 01:40 ·
CR effective 2026-08-07 · Protocol: v1.0 · Annotator: Fable (pass 1) ·
Adjudicated: **no** (single annotator; two Alpha rows adjudicated, see §6)

## Scope

- `arn`: 77 cards, all with Oracle text, 0 fallback, released 1993-12-17.
  Exhaustive review (protocol S4: ≤ 400 cards). 109 printed units, 0
  rules-supplied.
- `leb` appendix: the 2 cards first printed in Beta (Circle of Protection:
  Black, Volcanic Island), 1 printed + 1 rules-supplied unit.
- `pcel` (memorabilia, fallback printings) excluded by S4.
- Held-out pool (protocol §6.3, `oracle_id` prefix `f`) excluded from every
  inspected example below; pool cards are counted only in aggregate hit
  totals.

## Hypotheses (pre-registered in `docs/gates/gate-0-evidence.md` §8)

- **N1 (novelty).** Unit novelty against earlier audited sets is informative
  from `arn` onward and will fall as sets accumulate. *Falsifier:* novelty
  ≥ Alpha's 1.0 or no decline by Legends. *Denominator:* printed units of the
  set; "earlier" = `lea` + `leb` exports.
- **N2 (keywords).** No non-keyword short sentence is labelled
  `keyword_ability`. *Falsifier:* one such unit. *Denominator:* the set's
  keyword units.
- **F1–F4 (delayed-trigger forms).** The inverted (`… at the beginning of the
  next …`), `When …`-in-effect, recurring, and `at end of combat` forms found
  in Alpha recur and are still not split. *Falsifier:* a set with these
  wordings that the segmenter splits, or corpus evidence that one form is
  not a delayed trigger. *Denominator:* corpus hits per pattern.
- **C1 (tap symbol).** `{T}`-bearing units share templates with mana-cost
  units. *Denominator:* printed units containing `{T}` or `{Q}`.

## Pre-audit baseline (S2, recorded before any card was read)

| Set | Cards | With text | Printed units | Rules-supplied | Templates | Singletons | Top-10 / 25 / 50 coverage | Kinds | Roles |
|---|---|---|---|---|---|---|---|---|---|
| `arn` | 77 | 77 | 109 | 0 | 92 | 83 | 24.77 / 38.53 / 61.47 % | activated 33 · triggered 31 · static 23 · keyword 16 · replacement 4 · additional cost 1 · ante 1 | ability 107 · mode 2 |
| `leb` | 2 | 2 | 1 | 1 | 1 | 1 | — | activated 1 | ability 1 |

Pre-implementation corpus-wide baseline from the then-current state:
70,799 printed + 970 rules-supplied units, 36,944 templates.

## Verified findings

### V1. Three more missed nested delayed triggers, all in forms already seen in Alpha

| Unit | Form | CR | Evidence |
|---|---|---|---|
| Rukh Egg #0 | inverted (`create … at the beginning of the next end step`) inside a dies trigger | 603.7a/e | same wording class as Stone Giant (Alpha), whose ruling names the delayed trigger |
| Nafs Asp #0 | inverted (`loses 1 life at the beginning of their next draw step unless …`) inside a damage trigger | 603.7a/e | ruling: the player may pay any time before that draw step |
| Sandals of Abdallah #0 | `When that creature dies this turn, destroy ~.` after an activated effect | 603.7a/b/e | identical modern template (Fatal Fissure, Make Your Mark, Together Forever — ruling on the last names "the delayed triggered ability") |

Boundary precision 106 / 109 printed units; recall 106 / 109; no `over`,
`misattached`, or `unsure` rows.

### V2. Corpus search of the delayed-trigger forms (S8)

Literal, case-insensitive `cards --field text` searches; hits include the
held-out pool in the totals only; 20 inspectable hits sampled evenly by name
per pattern, all decades represented.

| Pattern | Hits (pool) | Decades 1990s/2000s/2010s/2020s | Inspected | Result |
|---|---|---|---|---|
| `at the beginning of the next` | 595 (43) | 120 / 95 / 101 / 279 | 20 | all delayed triggers created by an effect |
| same, phrase *not* at sentence start (inverted) | 535 (40) | 111 / 83 / 93 / 248 | 20 | all delayed triggers; forms include `Sacrifice it …`, `Exile it …`, `Return … at the beginning of the next end step`, `Draw a card at the beginning of the next turn's upkeep`; two occur inside quoted granted abilities (Firion, The Caldaia) and one is compound (`… or if it would leave the battlefield`, Kathari Bomber) |
| `at end of combat` | 154 (8) | 41 / 25 / 24 / 64 | 20 | all delayed triggers; Gorgon Recluse ruling: "creates a delayed triggered ability that will destroy that creature at the end of combat step" |
| `. When ` (sentence-initial `When` after other text in a unit) | 798 (58) | 24 / 115 / 87 / 572 | 20 | **three classes:** (a) delayed triggers with stated duration (`When that creature dies this turn, …`); (b) reflexive triggers `When you do, …` (CR 603.12, follow delayed-trigger rules); (c) independent triggered abilities sharing a paragraph — `When the last is removed, sacrifice it.` (Aven Riftwatcher, Keldon Halberdier; cf. vanishing 702.63a), `When you spend this mana …` (Gilanra; CR 106.6 says this one *is* a delayed trigger) |

Conclusion: a sentence-level split at these phrases is supported as a
**boundary** rule; assigning the **role** (`delayed_trigger` vs. an
independent `ability`) is not determinable from the phrase alone for class
(c) and needs CR context. F4 is resolved: `at end of combat` is a delayed
trigger; Alpha's Cockatrice #1 and Thicket Basilisk #0 were re-dispositioned
to `under` (annotator `fable-pass1;adjudicated-2026-08-26`).

### V3. The `instead`-on-instant class recurs (F5)

Eye for an Eye #0 is a spell ability (CR 113.3a) labelled
`replacement_effect`. Corpus: 439 instants/sorceries contain `instead`
(29 in the pool); 20 inspected — every one is spell text whose effect
includes a conditional or replacement clause. No instant or sorcery in the
sample carried a replacement *ability* that functions off the stack; the
static abilities instants do carry (113.6d–g: cost modification, cast
restriction, can't-be-countered) do not use `instead`.

### V4. Prevention statics recur (F7)

Camel #1 and Desert Nomads #1 are static abilities generating prevention
effects (CR 615.1a), labelled residual. Corpus: 77 non-instant/sorcery cards
begin a sentence with `Prevent all damage that would be dealt to` (4 in the
pool); the 20 inspected split between activated/triggered abilities whose
effect prevents (correctly labelled by their cost/trigger) and statics
(Cho-Manno, Frodo, Glacial Chasm, Inviolability, Heart of Light, Sevinne,
Sanwell) that fall into the residual class. The taxonomy gap is not
Alpha-specific.

### V5. Activation instructions are common and never boundaries (F10)

`Any player may activate` — 42 hits (2 pool), 20 inspected, all
instructions naming who may activate (CR 602.1b), some compounded with
timing (`… but only as a sorcery`, `… but only during their turn`).
`Activate only` — 1,161 hits (83 pool), 20 inspected, all timing or
condition instructions. In `arn`: Desert #1, Library of Alexandria #1,
Ifh-Bíff Efreet #1, Aladdin's Lamp #0 (`X can't be 0.`, a cost aspect).

### V6. Novelty against Alpha + Beta (N1)

Unit novelty **77 / 109 (0.706)**; template novelty **76 / 92 (0.826)**;
earlier distinct templates 292. The 32 recurring units use 16 templates:
`Flying` ×7, `~ can't attack unless defending player controls an Island.` ×3,
`When you control no Islands, sacrifice ~.` ×3, `{M}: Add {M}.` ×3,
`~ doesn't untap during your untap step.` ×2, `Banding` ×2, `Enchant
creature` ×2, `Trample` ×2, and eight singletons (upkeep-pay-to-untap, ante
instruction, additional-cost sacrifice, `{M}, {M}: ~ deals N damage to any
target.`, `{M}: Add one mana of any color.`, `First strike`, `Enchanted
creature gets +N/+N.`, `At the beginning of your upkeep, sacrifice ~ unless
you pay {M}{M}.`). By kind, keyword units are 14 / 16 recurring; activated
5 / 33, triggered 5 / 31, static 6 / 23, replacement 0 / 4. Only one novel
template occurs twice inside `arn` (`At the beginning of your upkeep, ~
deals N damage to you.`). N1 is consistent with one data point; it is not
yet tested.

### V7. Keyword labelling (N2) holds

16 `arn` keyword units (Flying ×7, Banding ×2 incl. the `banding` split on
War Elephant, Trample ×2, Enchant creature ×2, Desertwalk, Protection from
red, First strike); no non-keyword unit is labelled keyword; the two
non-recurring keyword templates are `Desertwalk` (702.14) and `Protection
from red` (colour fragmentation of an Alpha template).

### V8. Tap-symbol collision (C1)

29 / 109 `arn` units contain `{T}` (Alpha: 52 / 398); all share cost
templates with mana-only costs after normalization.

## Bounded observations

- **New structures with no Alpha precedent** (tags, counts): coin flips
  (`coin_flip` ×3; CR 705; 79 corpus hits), a subgame (Shahrazad; the
  subgame rule id was not located — CR 726 is now "initiative"; needs a
  rules search), phasing with an `until ~ leaves the battlefield` duration
  (Oubliette; 3 corpus hits for `phases out until`), wishes (`outside the
  game`, Ring of Ma'rûf), set-name reference (City in a Bottle ×2),
  remembered choices (Jihad ×3), control change with conditional duration
  (Aladdin, Old Man of the Sea), opponent-chosen targets (Cuombajj
  Witches), a modal *activated* ability (Pyramids), mana with a spending
  restriction (Metamorphosis, CR 106.6), state triggers ×7 (three more
  `When you control no Islands` plus Jihad, Drop of Honey, Serendib Djinn,
  City in a Bottle). None of these is a boundary or label defect at this
  taxonomy's granularity; they are recorded for Phase 2.
- 22 / 109 printed units have ≥ 2 sentence terminators (automatic tag).
- Suspected normalization fragmentation (regex-flagged, unverified): land
  type 11, colour word 5, `Enchant <type>` 2.
- Oubliette's `Tap that creature as it phases in this way.` is an
  instruction inside the trigger's effect, not a separate replacement
  ability (ruling: phases in immediately after Oubliette leaves); accepted
  with tag `instruction_in_effect`.

## Unsupported and ambiguous cases

| Unit | Disposition | Reason |
|---|---|---|
| Camel #1, Desert Nomads #1 | resolved by P-ARN-4 | `prevention_effect` now represents CR 615 static prevention effects |
| — | no `ambiguous` or `adjudicate` rows in `arn` | |

## Accepted segmenter changes (implemented 2026-08-26)

**P-ARN-1 — sentence-level delayed-trigger split (generic).** Within a
printed unit, a sentence (outside quotes and reminder text) that contains
`at the beginning of the next`, `at the beginning of … next …`, or `at end
of combat`, and that is preceded by other effect text in the same unit,
becomes a `delayed_trigger` child of the unit (CR 603.7, 603.7a). Fix rows:
Alpha Dragon Whelp #1, Stone Giant #0, Nettling Imp #0, Cockatrice #1,
Thicket Basilisk #0; `arn` Rukh Egg #0, Nafs Asp #0. Counterexample search:
V2 (535 + 154 hits, 40 inspected, no counterexample; compound `… or if it
would leave the battlefield` stays in the child; matches inside quoted
abilities must remain inside the quoted child, as the existing rule already
requires). Sentence boundaries must not split on the period inside `X/X` or
mana symbols; the conditional lead-in (`If this ability has been activated
four or more times this turn, …`) stays with the child.

**P-ARN-2 — `When … this turn,` / `When you do,` sentences as delayed
triggers.** A sentence-initial `When`/`Whenever` clause that follows effect
text in the same unit *and* carries a stated duration (`this turn`, `this
way`) or is the reflexive `When you do` form becomes a `delayed_trigger`
child (CR 603.7b, 603.12). Fix rows: Alpha Animate Dead #1 (`When ~ leaves
the battlefield, …` — **note:** no stated duration; ruling names it a
delayed trigger; falls outside this rule and stays a defect), `arn` Sandals
of Abdallah #0. Sentence-initial `When` *without* those markers (class (c) in
V2) must **not** be split with role `delayed_trigger`; whether to split it as
an independent `ability` is a separate proposal needing its own
counterexample search (Aven Riftwatcher vs. Gilanra differ under the CR).

**P-ARN-3 — type-line context for lexical replacement.** On cards whose
type line is `Instant` or `Sorcery` (single-faced; multi-face handled per
face), a top-level unit that is not a cast restriction, additional cost,
cost modification, or `can't be countered` static is classified
`spell_or_static_text` even if it contains `instead`/`skip` (CR 113.3a).
Fix rows: Alpha Disintegrate #0, Camouflage #1; `arn` Eye for an Eye #0.
Counterexample search: V3 (439 hits, 20 inspected, none). Requires the
segmenter to receive the type line (it already receives the name).

**P-ARN-4 — prevention kind (taxonomy decision, D7).** Add
`prevention_effect` (CR 615.1a: statics whose text uses `prevent` and no
cost/trigger) or document that 615 is folded into `replacement_effect` with
a citation. Fix rows: Alpha Rock Hydra #1; `arn` Camel #1, Desert Nomads #1.
Counterexample search: V4 (77 hits, 20 inspected; the split between
statics and cost/trigger abilities is clean because kind order tests
activated/triggered first).

All four proposals were accepted and implemented without card-name branches.
The implementation is still a surface-form segmenter/classifier, not a
semantic IR.

Implementation behavior:

- **P-ARN-1.** A supported delayed-trigger sentence or clause becomes a
  `delayed_trigger` child of the unit that creates it. Supported forms are
  `at the beginning of the next ...`, `at the beginning of ... next ...`,
  and `at end of combat`. Matching text in reminder text is stripped before
  segmentation; matching text in a quoted ability is split within that quoted
  ability's subtree. Conditional lead-ins and compound conditions remain in
  the child. A trailing activation-instruction sentence remains on the
  activated parent.
- **P-ARN-2.** A following sentence-initial `When`/`Whenever` clause is split
  as a `delayed_trigger` child only when it contains `this turn` or `this way`,
  or when it is the reflexive `When you do` form. Unscoped sentence-initial
  `When`/`Whenever` remains deferred under D14. Animate Dead's unmarked
  delayed trigger remains a known defect.
- **P-ARN-3.** Segmentation/classification receives the current per-face type
  line. A top-level Instant or Sorcery unit is `spell_or_static_text` despite
  lexical `instead`, `skip`, or `prevent` wording, unless an earlier static
  exception applies (`cast_restriction`, `additional_cost`, or existing
  cost/counterability text that remains residual static). Multi-face cards use
  the face's type line when the stored card-wide type line is joined with
  ` // `.
- **P-ARN-4.** `prevention_effect` is a distinct kind for static abilities
  that generate CR 615 prevention effects. Activated and triggered abilities
  whose effects prevent damage keep their activated/triggered kind.

Validation:

- Regression tests were added for the Alpha and Arabian Nights positive
  examples; quoted and reminder-text delayed-trigger counterexamples;
  recurring upkeep and recurring end-of-combat triggers; independent unscoped
  sentence-initial `When`; instant/sorcery static exceptions; multiface
  type-line classification; and activated/triggered/spell prevention
  counterexamples.
- `cargo test`: 41 passed.
- `cargo build --release`: passed.
- Re-exported `lea`, `leb`, and `arn`; regenerated metrics with
  `scripts/python/audit_metrics.py`; all three exports report zero drift.

Corpus before/after:

| Scope | Before | After |
|---|---:|---:|
| Corpus printed / rules-supplied units | 70,799 / 970 | 71,682 / 970 |
| Corpus distinct templates | 36,944 | 37,344 |
| Corpus top-10 / 100 / 1,000 / 5,000 coverage | 14.32 / 27.14 / 42.37 / 54.88 % | 14.15 / 26.85 / 42.15 / 54.88 % |
| Corpus kinds | activated 11,999 · additional cost 317 · ante 9 · cast restriction 68 · CDA 247 · keyword 17,630 · replacement 2,628 · static/spell 21,281 · triggered 16,620 | activated 12,000 · additional cost 317 · ante 9 · cast restriction 68 · CDA 245 · keyword 17,630 · prevention 181 · replacement 2,208 · static/spell 21,521 · triggered 17,503 |
| Corpus roles | ability 67,078 · delayed trigger 94 · granted 1,506 · mode 2,121 | ability 67,075 · delayed trigger 982 · granted 1,504 · mode 2,121 |
| `lea` printed / templates | 398 / 291 | 403 / 294 |
| `arn` printed / templates | 109 / 92 | 112 / 95 |

The unit and template increases are expected: Alpha gains five delayed-trigger
children and Arabian Nights gains three delayed-trigger children. `lea`
template count also changes because the split delayed-trigger children become
standalone templates and Rock Hydra's prevention static changes kind; `arn`
adds three new child templates. Kind shifts are expected from moving
instant/sorcery spell text out of `replacement_effect` and introducing
`prevention_effect`.

Over-segmentation checks:

- `lea`, `leb`, and `arn` regenerated exports have zero drift after accepted
  annotation updates.
- The Clockwork Beast recurring `At end of combat` line is covered by a
  negative test and remains a top-level triggered ability, not a delayed child.
- Post-change audit signals: `lea` 39 (`activation_restriction_embedded_candidate`
  8, `conditional_cda_candidate` 1, `delayed_trigger_unattached_candidate` 1,
  `payment_restriction_embedded_candidate` 1, `quoted_text_not_extracted_candidate`
  1, `residual_multi_sentence_unit` 28); `leb` 0; `arn` 5
  (`activation_restriction_embedded_candidate` 2,
  `residual_multi_sentence_unit` 3). The remaining `lea`
  delayed-trigger signal is the retained Clockwork-style recurring trigger
  candidate; `arn` has no delayed-trigger signal.
- Aggregate corpus pattern counts, held-out cards not inspected: 595 cards
  contain `at the beginning of the next`, 788 contain both `at the beginning
  of` and `next`, 154 contain `at end of combat`, 798 contain `. When `,
  554 contain `prevent`, and 442 Instant/Sorcery cards contain `instead`.

Remaining known defects/deferred cases:

- Animate Dead's unmarked delayed trigger remains a defect outside P-ARN-2.
- D14 remains deferred: independent sentence-initial triggered abilities
  sharing a paragraph are not split.
- Alpha still has two missed boundaries outside these proposals and one
  known kind defect for Animate Dead's quoted trailing-period Enchant keyword.

Recommendation: Antiquities (`atq`) structural research is cleared to begin;
Codex did not begin that research.

### Research-lead review of this implementation (2026-08-26, after the Antiquities audit)

P-ARN-2, P-ARN-3 and P-ARN-4 are ratified; the sentence-level half of
P-ARN-1 is ratified. The un-proposed single-sentence rule (c) — split at the
last `: `/`, ` before the phrase — is **rejected**: the corpus S11 check
(`docs/audits/corpus-checks/2026-08-26-delayed-split-overseg.md`) finds 108
bare-condition and 5 bare-cost parents corpus-wide, 0/40 sampled fragments
that are reference units, and 3 splits inside quoted abilities. Rukh Egg #0
and Nafs Asp #0 are therefore re-dispositioned `over`/`defect` (their
children stay `ok`), and Arabian Nights stands at boundary 110/112, accept
110 · defect 2. The rows Codex marked `codex-accepted` were re-read by the
research lead and carry `fable-review-2026-08-26`; the replacement rule is
proposed as P-ATQ-1 in `docs/findings/atq-structural-audit.md`. The
"112 / 112" figures above are Codex's pre-review numbers and are superseded.

## Measurements (protocol section 4.5; `docs/audits/arn/metrics.json`)

| Field | `arn` | `leb` |
|---|---|---|
| Printed / rules-supplied units | 112 / 0 | 1 / 1 |
| Boundary precision | 112 / 112 (1.0) | 1 / 1 |
| Missed boundaries / recall | 0 / 112 / 112 | 0 / 1 / 1 |
| Kind accuracy | 110 / 110 (1.0); 2 n/a (modes) | 1 / 1 |
| Role accuracy / source accuracy | 112 / 112; 112 / 112 | 1 / 1; 2 / 2 |
| Dispositions | accept 112 | accept 2 |
| Context | none 87; CR 24; type line 1; game state 0; card-specific 0 | type line 1; none 1 |
| Suspected fragmentation | land type 11; colour 5; object type 2 | colour 1 |
| Tap-symbol collision | 29 | 1 |
| Unit / template novelty vs earlier audited sets | 80 / 112; 79 / 95 | 1 / 1; 1 / 1 (vs `lea`) |
| Multi-sentence units | 21 / 112 | 0 |
| Drift vs fresh export | 0 | 0 |

Alpha after accepted P-ARN updates: boundary precision 400 / 402, missed 2,
kind accuracy 392 / 393, dispositions accept 411; defect 3; adjudicate 2;
ambiguous 1 (`docs/audits/lea/metrics.json`).

## Reproduction

```powershell
git rev-parse HEAD ; cargo build --release ; cargo test
$mtg = ".\target\release\mtg-discover.exe"
& $mtg info ; & $mtg sets --until 1993-12-17
& $mtg templates --set arn --limit 5000 ; & $mtg templates --set leb --limit 5000
python scripts/python/export_units.py arn > docs/audits/arn/units-export.tsv
python scripts/python/export_units.py leb > docs/audits/leb/units-export.tsv
python scripts/python/audit_metrics.py docs/audits/arn/units-annotated.tsv --export docs/audits/arn/units-export.tsv --earlier docs/audits/lea/units-export.tsv --earlier docs/audits/leb/units-export.tsv
python scripts/python/audit_metrics.py docs/audits/lea/units-annotated.tsv --export docs/audits/lea/units-export.tsv
& $mtg cards "at the beginning of the next" --field text --limit 500      # page with --offset 500
& $mtg cards "at end of combat" --field text --limit 500
& $mtg cards ". When " --field text --limit 500
& $mtg cards "instead" --field text --limit 500                            # filter type_line Instant/Sorcery
& $mtg card "Gorgon Recluse" --rulings ; & $mtg card "Nafs Asp" --rulings ; & $mtg card "Oubliette" --rulings
& $mtg rules show 603.7 ; & $mtg rules show 603.12 ; & $mtg rules show 615.1 ; & $mtg rules show 602.1
```

The search sampling (every ⌊n/20⌋-th inspectable hit by name, pool
excluded) is deterministic for a fixed database.
