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

Corpus-wide baseline unchanged from `docs/current-state.md` (70,799 + 970
units, 36,944 templates).

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
| Camel #1, Desert Nomads #1 | `unsupported` (`gap:prevention`) | CR 615 class absent from the kind vocabulary |
| — | no `ambiguous` or `adjudicate` rows in `arn` | |

## Proposed segmenter changes (S10 items 1–3; not implemented; for Codex via decision)

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

Each proposal still needs S10 items 4–7 (corpus before/after totals,
over-segmentation check, regression re-export of `lea`/`leb`/`arn`, tests)
before acceptance; none names a card.

## Measurements (protocol §4.5; `docs/audits/arn/metrics.json`)

| Field | `arn` | `leb` |
|---|---|---|
| Printed / rules-supplied units | 109 / 0 | 1 / 1 |
| Boundary precision | 106 / 109 (0.9725) | 1 / 1 |
| Missed boundaries · recall | 3 · 106 / 109 | 0 · 1 / 1 |
| Kind accuracy | 101 / 104 (0.9712); 2 n/a (modes) | 1 / 1 |
| Role accuracy · source accuracy | 106 / 106 · 109 / 109 | 1 / 1 · 2 / 2 |
| Dispositions | accept 103 · defect 4 · unsupported 2 | accept 2 |
| Context | none 87 · CR 21 · type line 1 · game state 0 · card-specific 0 | type line 1 · none 1 |
| Suspected fragmentation | land type 11 · colour 5 · object type 2 | colour 1 |
| Tap-symbol collision | 29 | 1 |
| Unit / template novelty vs earlier audited sets | 77 / 109 · 76 / 92 | 1 / 1 · 1 / 1 (vs `lea`) |
| Multi-sentence units | 22 / 109 | 0 |
| Drift vs fresh export | 0 | 0 |

Alpha after the two adjudications: boundary precision 390 / 397, missed 7,
dispositions accept 398 · defect 10 · adjudicate 2 · ambiguous 1 ·
unsupported 1 (`docs/audits/lea/metrics.json`).

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
