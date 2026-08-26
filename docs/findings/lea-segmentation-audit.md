# Finding: Alpha (LEA) segmentation audit and first normalization ablation

Date: 2026-08-26
Corpus snapshot: Scryfall bulk 2026-08-25 (oracle-cards, rulings, default-cards, all same-day); CR effective 2026-08-07
Scope: every card whose first printing is Limited Edition Alpha (`first_set = 'lea'`), reviewed exhaustively (no sampling)

## Why Alpha, and what "Alpha" means here

The research plan walks the corpus set-by-set in release order so that
wording complexity grows over time. This is the first step.

Two caveats that hold for the entire era walk:

- **Oracle text is current wording, not period wording.** Alpha cards carry
  2026 Oracle templating. Ordering by first printing tracks *when a mechanic
  or effect entered the game*, not how it was phrased at the time. We are not
  measuring historical language drift.
- **"First printing"** is derived from Scryfall `default_cards`: the earliest
  paper, non-promo printing outside promo/token/memorabilia/minigame/alchemy
  sets (fallback: earliest printing of any kind, flagged). Scryfall's
  `oracle_cards` file holds one *arbitrary recent* printing per card (e.g.
  Lightning Bolt → Marvel Super Heroes Commander 2026), so it cannot answer
  this question by itself.

## Hypotheses tested

- **H1 (segmentation).** A line of Oracle text is a reliable proxy for one
  ability. *Falsifier:* lines that contain more than one ability, or
  abilities that span lines.
- **H2 (classification).** The five-way classifier (modal header / mode /
  triggered / activated / keyword / spell-or-static) labels Alpha lines
  correctly. *Falsifier:* lines whose label contradicts the CR's own
  definition of the ability kind.
- **H3 (typed-slot diversity).** Most of Alpha's template diversity is
  variation in typed arguments (color, land type, self-reference word, P/T
  modifier, object type) rather than distinct sentence structure.
  *Falsifier:* abstracting those slots leaves most templates as singletons.
- **H4 (printing noise).** Reminder text presence in `oracle_text` depends on
  which printing Scryfall chose, so it is noise. *Falsifier:* Oracle text is
  identical across printings.

## Baseline numbers (current segmenter, unchanged)

| Measurement | Value |
|---|---:|
| Alpha oracle_ids | 290 (295 printings; 5 duplicate basic-land arts collapse) |
| With Oracle text | 275 |
| Lines emitted by `segment` | 402 |
| Non-empty normalized units | 388 |
| Distinct normalized templates | 289 |
| Singleton templates | 264 (68.0% of units) |
| Top-10 / top-50 / top-100 coverage | 22.7% / 38.4% / 51.3% |
| Classifier labels | static/spell 189 · activated 88 · triggered 59 · keyword 57 · mode 6 · modal header 3 |

Reproduce: `mtg-discover templates --set lea` and `mtg-discover segment --card <name>`.

## Verified findings

### V1. 14 Alpha cards have no printed abilities at all — their only ability is supplied by the rules

Forest, Island, Mountain, Plains, Swamp and the nine dual lands have Oracle
text consisting solely of reminder text, e.g. `({T}: Add {G} or {U}.)`.
The mana ability is intrinsic to the basic land type (CR 305.6: "An object
with the land card type and a basic land type has the intrinsic ability
'{T}: Add [mana symbol]'"). The segmenter emits an `activated_ability` unit
whose normalization is empty; `templates` silently drops it (402 → 388).

This is the first concrete instance of the open question "which semantics
are supplied by the CR rather than the card": for these cards the answer is
*all of it*, and the trigger is a subtype on the type line, not text.

### V2. Reminder text breaks the keyword classifier on 34% of keyword-bearing lines

29 lines whose reminder-stripped text is a bare keyword (`Flying`,
`Defender`, `First strike`, `Protection from white`, `Banding`,
`Forestwalk`, `Reach`, `Enchant creature`, …) are labelled
`spell_or_static_text` because the parenthetical adds a period and pushes
the line over the 8-word limit. 57 keyword lines are labelled correctly, so
29 / 86 = 34% of keyword-bearing lines are misclassified. Cause: the
classifier runs on raw text while the normalizer strips reminder text.

### V3. Reminder text is canonical Oracle text, not printing noise (H4 refuted)

Scanning all 116,843 printings in `default_cards` (English, all 38,618
oracle_ids): zero cards have printings that differ in Oracle text once my
face-joining artefact for reversible printings is excluded (71 raw
differences, all of that kind). Whether a keyword carries reminder text is
therefore a stable property of the card's Oracle entry. It still has to be
stripped before classification (V2), but it cannot be blamed on which
printing was sampled.

### V4. A line is not an ability (H1 refuted in both directions)

One line, several abilities:

- Keyword lists: `Flying, trample` (Lord of the Pit), `Defender, flying`
  (Wall of Air), `Flying; banding` (Mesa Pegasus). Each keyword is a
  separate ability (CR 702.1).
- Animate Dead's second line contains an ETB triggered ability *and*, inside
  it, a granted leaves-the-battlefield trigger.
- Berserk: a spell effect followed by a delayed triggered ability
  (CR 603.7) in the same line.
- 8 activated abilities embed `Activate only …` restrictions
  (Cyclopean Tomb, Disrupting Scepter, Illusionary Mask, Jade Statue,
  Clockwork Beast, Rock Hydra, Nettling Imp, Instill Energy).

One ability, several lines:

- Siren's Call: line 2 is the spell effect, line 3 (`At the beginning of the
  next end step, destroy …`) is the delayed trigger *created by* that
  effect. The classifier labels line 3 `triggered_ability`, which is wrong
  under CR 603.7 — it is part of the spell ability.
- Modal spells (Blue/Red Elemental Blast, Healing Salve): header + modes are
  one ability (CR 700.2).

59 of 388 units (15%) contain two or more sentences; Word of Command has
five.

### V5. The CR itself defines a sub-line grammar the segmenter does not use

- CR 602.1 / 113.3b: activated abilities are written
  `[Cost]: [Effect.] [Activation instructions (if any).]`
- CR 603.1: triggered abilities are written
  `[When/Whenever/At] [trigger condition or event], [effect]. [Instructions (if any).]`
- CR 113.3d / 604.1: static abilities "are written as statements".

So for two of the four ability categories the rules give an explicit
three-slot structure. Every Alpha activated and triggered line fits it (the
only ambiguity is Zombie Master, V6).

### V6. Structural classes present in Alpha that the classifier cannot express

| Class | CR grounding | Alpha units | Examples |
|---|---|---:|---|
| Replacement effect (`instead`, `skip`, `As ~ enters`, `enters with`, `enter as a copy`) | 614.1a–c | ≥ 15 | Clone, Time Vault, Island Sanctuary, Black Vise, Clockwork Beast, Lich ×2, Library of Leng, Stasis |
| Casting-timing restriction (`Cast this spell only …`) | 604.6 (static ability functioning in hand), 506.7a–f (combat wording) | 5 | Berserk, Blaze of Glory, Camouflage, False Orders, Siren's Call |
| Additional cost | 118.8, 601.2b | 1 | Sacrifice |
| Payment restriction | 113.6e | 1 | Drain Life (`Spend only black mana on X`) |
| Characteristic-defining ability | 604.3 | 4 | Nightmare, Keldon Warlord, Plague Rats, Gaea's Liege |
| Ante instruction (not an ability) | 407 | 3 | Contract from Below, Darkpact, Demonic Attorney |
| Granted ability in quotes (an ability whose *argument* is an ability) | 201.5a–b (name references inside granted abilities) | 6 | Zombie Master, Farmstead, Vesuvan Doppelganger, Animate Dead, Earthbind, Raging River |

Zombie Master's `Other Zombies have "{B}: Regenerate this permanent."` is
labelled `activated_ability` because the quoted text contains a colon; it
is a static ability. Quoted text is the first evidence in the walk of
*recursive* structure — the eventual representation needs abilities as
first-class values.

### V7. Self-reference has two surface forms, and name replacement has a counterexample

Permanents refer to themselves as `this creature` / `this artifact` /
`this enchantment` / `this Aura` / `this land` / `this card`; instants and
sorceries and CDA permanents use the card name (Lightning Bolt, Fireball,
Disintegrate, Drain Life, Earthquake, Hurricane, Volcanic Eruption,
Simulacrum, Word of Command, Timetwister, Gaea's Liege, Keldon Warlord,
Nightmare). The normalizer only collapses the name form (CR 201.5: name
means "just that particular object").

Counterexample: Plague Rats — `the number of creatures named Plague Rats`
is a name *predicate* over other objects, not self-reference, yet the
normalizer rewrites it to `named ~`. Any self-reference normalization must
exclude the `named X` construction.

## Bounded observations

### B1. Typed-slot abstraction removes little of Alpha's diversity (H3 not supported at set level)

Ablations layered one at a time on the baseline normalization (measurement
script only; the repository normalizer is unchanged):

| Ablation (cumulative) | Templates | Singletons | Singleton share | Top-100 coverage |
|---|---:|---:|---:|---:|
| baseline | 289 | 264 | 68.0% | 51.3% |
| + color word → `{COLOR}` | 270 | 237 | 61.1% | 56.2% |
| + basic land type → `{LAND}` | 269 | 235 | 60.6% | 56.4% |
| + `this <type>` → `~` | 268 | 233 | 60.1% | 56.7% |
| + `±N/±N` → `{PT}` | 266 | 231 | 59.5% | 57.2% |
| + object type word → `{TYPE}` | 255 | 219 | 56.4% | 60.1% |

Solo effects: color 289→270, object type 289→277, everything else ≤ 2
templates. Merges created: `Enchant {TYPE}` (41 units, 5 forms), the
lucky-charm cycle, the Ward cycle, the lace cycle, the Circle of
Protection cycle, the `upkeep of enchanted {TYPE}'s controller` Aura cycle.

Interpretation: within Alpha, after slot abstraction, ~56% of units are
still unique sentences. Alpha diversity is predominantly *structural*, not
parametric — but this is expected for the set that *originates* the
language, and is the wrong test of the project hypothesis (see B2).

### B2. Alpha's templates recur, but 47% of them never recur anywhere

Against the full corpus (67,738 units, verified identical to the CLI's
count):

| Corpus-wide occurrences of an Alpha template | Templates | Share |
|---|---:|---:|
| 1 (unique in the entire corpus) | 135 | 46.7% |
| 2–4 | 73 | 25.3% |
| 5–19 | 39 | 13.5% |
| 20–99 | 28 | 9.7% |
| 100+ | 14 | 4.8% |

- 253 / 388 Alpha units (65.2%) have a template that recurs somewhere in
  the corpus.
- Alpha's 289 templates account for 9,674 / 67,738 corpus units (14.3%).
- Of the 135 corpus-unique templates, roughly half are members of
  parametric cycles whose siblings are also Alpha-only (the five laces,
  five Wards, four Circles of Protection, five lucky charms), i.e. they
  would merge under slot abstraction. The remainder — Chaos Orb, Word of
  Command, Camouflage, Raging River, Illusionary Mask, Animate Dead,
  Cyclopean Tomb, Balance, Lich, Time Vault, Library of Leng, the ante
  cards, Magical Hack/Sleight of Mind — are genuinely one-off structures:
  physical actions, player control, pile-making, text alteration, ante.
  These are exactly the mechanics the game abandoned.

## Hypotheses for the next steps (falsifiable)

- **N1.** The share of a set's units whose baseline template already
  appeared in an *earlier* set (novelty rate) rises over release date as
  templating matured. Alpha's is 0% by definition; the metric is only
  informative from Arabian Nights onward. *Falsifier:* novelty does not
  trend with date once set size is controlled for.
- **N2.** Classifying on reminder-stripped text and splitting keyword lines
  on `,`/`;` fixes all 29 + 3 Alpha misclassifications without introducing
  new ones. *Falsifier:* a short static sentence without a period that is
  not a keyword (none found in Alpha; search later sets).
- **N3.** The CR's `[Cost]: [Effect.] [Instructions.]` and
  `[Trigger], [effect]. [Instructions.]` patterns cover ≥ 95% of activated
  and triggered lines corpus-wide. *Falsifier:* lines where the first colon
  or comma is not the slot boundary (quoted abilities, costs containing
  commas like `{T}, Sacrifice ~:` already show the cost slot needs its own
  sub-grammar).

## Proposed segmenter changes (not made — awaiting decision, since they shift the corpus baseline)

1. Classify on reminder-stripped text (fixes V2).
2. Split keyword-only lines on `,` / `;` into one unit per keyword (V4).
3. Emit a `rules_supplied` unit for reminder-only lines instead of an
   empty normalization (V1).
4. Add classes: `replacement`, `cast_restriction`, `additional_cost`,
   `cda`, `ante`, and detect quoted abilities before the colon test (V6).
5. Normalize `this <type>` → `~`, excluding `named X` (V7).
6. Treat modal header + modes as one unit with sub-units (V4).
7. Attach a delayed-trigger line that follows spell text to that spell (V4;
   needs a rule: spell text on instants/sorceries never has independent
   triggered abilities except via 603.7).

Expected effect on the corpus baseline: (1)+(2) will *increase* unit count
and *decrease* distinct templates materially, because `Flying` alone is
already 4.15% of the corpus and keyword lists are common on modern cards.
All numbers in `docs/current-state.md` would need refreshing.

## Reproduction

```powershell
$mtg = ".\target\release\mtg-discover.exe"
& $mtg info
& $mtg sets --until 1995-12-31
& $mtg templates --set lea --limit 60
& $mtg segment --card "Siren's Call"
& $mtg rules show 602.1 ; & $mtg rules show 603.1 ; & $mtg rules show 614.1 ; & $mtg rules show 604.6
```

The ablation, corpus-recurrence, and printing-variance scripts were
session scratch scripts; their logic is described above and they reproduce
the CLI's 67,738 / 37,912 totals exactly, which is the check that they
implement the same segmentation and normalization as `src/main.rs`.
