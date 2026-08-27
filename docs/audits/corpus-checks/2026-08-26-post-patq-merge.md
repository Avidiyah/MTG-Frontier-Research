# Post-merge acceptance evidence for P-ATQ-1..4 (commit `8e83221`)

Date: 2026-08-26 · Technical validator: Fable (Claude Code) · Protocol v1.0, steps
S8, S10 items 4–6, S11, S14. This is the post-merge acceptance pass for the four
Antiquities proposals. It does not re-audit Antiquities and it introduces no
segmentation logic. Historical reports in this directory (`…-delayed-split-overseg.md`,
`…-kind-rules-check.md`, both at `af150b0`) are the *pre-change* evidence and are
unchanged; every number below was regenerated at `8e83221`.

## 1. Frozen inputs

| Item | Value |
|---|---|
| Repository commit | `8e832213a60c67e8c652d9feeb05d3ab264f68a4` (2026-08-26 18:37 −05:00, "Validate merged Antiquities proposals"; worktree branch fast-forwarded from `8c0f229`) |
| Pre-change comparison binary | built from `8c0f229` (`git archive` of `Cargo.toml`, `Cargo.lock`, `src/`), the parent of PRs #1–#4 |
| Build | `cargo build --release` → `mtg-discover 0.1.0` |
| Tests / lint | `cargo test`: **82 passed, 0 failed**; `cargo fmt -- --check`: clean; `cargo clippy --all-targets -- -D warnings`: clean |
| Scryfall bulk snapshot (2026-08-25 drop) | `oracle-cards.jsonl.gz` 2026-08-26 01:38:46, 24,532,087 B, sha256 `9611b5d9…0ab2`; `rulings.jsonl.gz` 01:38:48, 5,366,171 B, sha256 `30646898…fe6f`; `default-cards.jsonl.gz` 01:38:57, 77,608,798 B, sha256 `d65608b4…667e` |
| `cards.sqlite` | 65,781,760 B, sha256 `d1c88cb9ab96531c2f2ce8f3b048c727240811e1f16acb141adbdb60998195c4` (identical in the worktree and the main checkout, built 01:40:45) |
| `mtg-discover info` | 38,626 cards (37,916 with Oracle text, 710 without, 3,212 multi-face, 553 first-printing sets); 78,949 rulings; 3,455 numbered rules, 752 glossary entries; CR effective 2026-08-07 |
| `sets --until 1994-03-04` | lea 290 / 275 text / 0 fallback · leb 2 / 2 / 0 · pcel 1 / 1 / 1 · arn 77 / 77 / 0 · atq 85 / 85 / 0 |
| Held-out pool | protocol §6.3, unchanged; pool cards are counted below, never listed or inspected |
| Codex adjudication of the three P-ATQ-3 residuals | **not available at the time of this pass** — see §7 |

## 2. Commands (run from the repository root, in this order)

```powershell
cargo build --release ; cargo test ; cargo fmt -- --check ; cargo clippy --all-targets -- -D warnings
$mtg = ".\target\release\mtg-discover.exe"
& $mtg info ; & $mtg sets --until 1994-03-04 ; & $mtg templates --limit 5000
python scripts/python/corpus_checks/dump_corpus_units.py corpus-units.jsonl        # 553 sets, 72,533 units, ~60 s
python scripts/python/corpus_checks/check_delayed_split.py corpus-units.jsonl <out>
python scripts/python/corpus_checks/check_kind_rules.py corpus-units.jsonl <out>
python scripts/python/corpus_checks/check_kind_rules_part2.py corpus-units.jsonl <out>
python scripts/python/corpus_checks/check_patq_s8.py --self-test
python scripts/python/corpus_checks/check_patq_s8.py corpus-units.jsonl docs/audits/corpus-checks/2026-08-26-patq-s8-search.md --pre <pre-change dump> --mtg .\target\release\mtg-discover.exe
python scripts/python/export_units.py <lea|leb|arn|atq> > <fresh export>
python scripts/python/audit_metrics.py docs/audits/<code>/units-annotated.tsv --export <fresh export> [--earlier <earlier fresh exports>]
```

The pre-change dump was produced by the same `audit export` loop with the `8c0f229`
binary (`--db`/`--rules` pointed at this checkout). The new script
`check_patq_s8.py` is the only tooling added; it restates the production regexes
in Python for the search, verifies that restatement against the binary on every
unit it fires on (`--mtg`: 3,340 top-level printed units, **0 mismatches**), checks
the P-ATQ-4 predicate against the dumped role (**0 disagreements**), and carries a
`--self-test`. Its full output is `2026-08-26-patq-s8-search.md`.

## 3. Corpus-wide before / after (`templates --limit 5000`)

| Measure | `8c0f229` (pre) | `8e83221` (post) | Δ |
|---|---:|---:|---:|
| Printed units | 71,682 | 71,563 | −119 |
| Rules-supplied units | 970 | 970 | 0 |
| Distinct templates | 37,344 | 37,299 | −45 |
| Coverage top 10 / 25 / 50 / 100 | 14.15 / 20.00 / 23.37 / 26.85 % | 14.17 / 20.03 / 23.41 / 26.90 % | |
| Coverage top 250 / 500 / 1,000 / 2,500 / 5,000 | 32.46 / 37.23 / 42.15 / 48.95 / 54.88 % | 32.52 / 37.28 / 42.18 / 48.96 / 54.87 % | |
| Kinds: triggered | 17,503 | 19,214 | +1,711 |
| Kinds: static/spell | 21,521 | 19,519 | −2,002 |
| Kinds: keyword | 17,630 | 17,840 | +210 |
| Kinds: activated | 12,000 | 11,998 | −2 |
| Kinds: replacement | 2,208 | 2,174 | −34 |
| Kinds: prevention | 181 | 166 | −15 |
| Kinds: CDA | 245 | 255 | +10 |
| Kinds: additional cost / cast restriction / ante | 317 / 68 / 9 | 319 / 69 / 9 | +2 / +1 / 0 |
| Roles: ability / mode / granted / delayed trigger | 67,075 / 2,121 / 1,504 / 982 | 67,045 / 2,121 / 1,506 / 891 | −30 / 0 / +2 / −91 |
| Most frequent template | `Flying` 3,526 (4.92 %) | `Flying` 3,526 (4.93 %) | |

Unit delta: −121 comma/colon delayed-trigger children merged back into their line
(P-ATQ-1) and +2 granted children exposed by the merged text. Kind deltas are
accounted for unit-by-unit in §§5–6 (P-ATQ-3 moves 2,053 units, P-ATQ-2 moves 10,
P-ATQ-1 merges 121; P-ATQ-4 changes role only).

## 4. P-ATQ-1 — retract split rule (c)

Source: `check_delayed_split.py` on the post-merge dump, plus a unit-level diff
against the pre-change dump (matched on `oracle_id`, face, source line, text).

| Measure | pre (`af150b0`/`8c0f229`) | post (`8e83221`) |
|---|---:|---:|
| `delayed_trigger` units | 982 (all children) | 891 = 861 children + 30 top-level (P-ATQ-4) |
| Split class of children: sentence / comma / colon / other | 861 / 115 / 6 / 0 | 861 / 0 / 0 / 0 |
| Children with resolvable parent | 982 / 982 | 861 / 861 (the 30 top-level units have no parent by design) |
| Children whose parent ends with `,` or `:` | 121 | **0** |
| Children beginning lowercase or with and/or/then | 115 | **0** |
| Top-level printed units whose template ends with `,` | 124 | 13 |
| Top-level printed units whose template ends with `:` | 133 | 128 |
| `delayed_trigger_unattached_candidate` signals | 14 | 186 (triggered 135 · static 45 · activated 6) |
| Residual supported phrase outside a delayed child (§6 of the script) | 93 | 206 |
| Recurring `At end of combat, if …` top-level triggers split | 0 / 7 | 0 / 7 |
| Sentence-level sample (script §5, 30 across decades) | 30 / 30 delayed (603.7) or reflexive (603.12) | 30 / 30, same judgement on re-inspection |

Fate of the 121 pre-change comma/colon children: every one is merged back into a
single unit on its printed line — 111 `triggered_ability`, 7 `activated_ability`,
2 `spell_or_static_text`, 1 `replacement_effect`; 116 of the merged units carry
`delayed_trigger_unattached_candidate`; the 5 without it hold the phrase inside a
quoted granted ability (Sakashima the Impostor, Simic Basilisk, Daretti, Scrap
Savant, Liliana, Defiant Necromancer, Breathkeeper Seraph), where the masked
signal correctly stays silent on the outer unit. The residual count rises by
exactly 113 = the 121 reverted units minus the 8 whose phrase is a scoped
`When … this turn/this way` form outside the residual regex. The three in-quote
mis-splits recorded at `af150b0` (Faerie Aerie, Firion, Diabolical Salvation) are
gone: the only 3 post-merge units that contain a quote *and* have a delayed
child (Sorry, "Ach! Hans, Run!", Dáin Ironfoot) are sentence-level splits whose
quotes lie entirely inside one side. The 13 remaining comma-final top-level
units are all Un-set augment/host half-cards (`ust`/`und`) whose printed line
itself ends with a comma; they are identical pre and post and are not segmenter
output.

Expected vs measured: the proposal estimated 982 → ~869 children and the
disappearance of 121 fragments and 3 in-quote splits; measured 982 → 861 children
(+30 top-level from P-ATQ-4), 121 → 0 fragments, 3 → 0 in-quote splits, 0 bare
condition/cost templates left at top level.

## 5. P-ATQ-2 — `can't be prevented` is not a prevention effect

S8 search over all 72,533 units for `can't|can’t|cannot be prevented`:

| Measure | Value |
|---|---|
| Units matching the prohibition idiom | 35 (held-out 5); decades 1990s 2 · 2000s 11 · 2010s 9 · 2020s 13 |
| Apostrophe forms in the corpus | straight `'` 35 · curly `’` 0 · `cannot` 0 (the regex accepts all three) |
| Post kinds | static/spell ability 31 · triggered 2 · activated 1 · static mode 1; **`prevention_effect` 0** |
| Pre kinds of the same units | static 22 · **prevention 9 (ability) + 1 (mode)** · triggered 2 · activated 1 |
| Units containing the prohibition *and* a separate `prevent` token (genuine prevention wrongly excluded) | **0** |
| Nearest non-matching wordings: `can't be regenerated/countered/blocked` next to `prevent` | 4 (Whippoorwill, Sewers of Estark, Demonfire, Banefire): 3 static, 1 activated — none is a CR 615.1a static, none affected |
| `isn't/wasn't prevented`, `unpreventable` | 0 units |

`prevention_effect` accounting 181 → 166: −9 role=ability prohibitions, −1 mode
prohibition (A-Ready to Rumble), −5 prefixed units re-labelled `triggered_ability`
by P-ATQ-3 (Favored Hoplite, Harvestguard Alseids, Loyal Unicorn, Crystal
Fragments `I, II`, Old Fat Spider Can't See Me `II`). Post-merge `check_kind_rules.py`
§A: 166 units = 147 role=ability (all judged CR 615.1a statics by the script's
classes, 0 `can't be prevented`, 0 trigger-word-initial), 13 modes, 6 granted;
`prevention_effect` on instant/sorcery faces: 0. Residual statics containing
`prevent` rose 9 → 17: the 9 granting frames already known plus the 8 non-pool
prohibition statics now correctly residual.

## 6. P-ATQ-3 — strip `<prefix> — ` before classification

Source: `2026-08-26-patq-s8-search.md` §§P3.1–P3.6 and X.

**Firing inventory.** 3,572 units (held-out 224; all `printed`; roles ability
3,325 · mode 241 · granted 6; decades 1990s 24 · 2000s 251 · 2010s 606 · 2020s
2,691). Categories: Saga chapter symbols on a Saga face 624 (17 distinct; **all
624 `triggered_ability`**); CR 207.2c ability words 1,406 (61 distinct — the rule
text was parsed from `Magic-Comprehensive_Rules.md` 207.2c; `Landfall` 198,
`Threshold` 103, `Delirium` 74 …); named modes (`role = mode`) 239 (214 distinct);
Roman numerals on a non-Saga face 2 (Phone a Friend `C —`/`D —`, correctly *not*
chapters); other 1,301 (598 distinct prefixes: CR 207.2d flavor words, Un-set/table
labels, and the false positives listed below). The complete distinct-prefix list
with a non-pool example per prefix is in the S8 artifact (every prefix occurring
≤ 2 times is listed, satisfying S11.2).

**Before/after kind transitions on the 3,572 fired units** (3,570 matched; the 2
unmatched are Ochre Jelly / A-Ochre Jelly, whose text changed under P-ATQ-1):
unchanged 1,517; `spell_or_static_text → triggered_ability` 1,786 (the intended
hidden-trigger-word recovery: body starts `whenever` 748 · `when` 249 · `at` 193;
the remaining 596 are almost all Saga chapters, whose kind CR 714.2b fixes
regardless of the body's leading verb);
`replacement_effect → triggered_ability` 36 (Landfall/Saga units whose effect
contains `instead`/`enters with` — correct); `prevention_effect → triggered_ability`
5 (the audit's fix rows); `spell_or_static_text → characteristic_defining_ability`
10 (Chroma/Domain P/T definitions — correct); `→ replacement_effect` 2,
`→ additional_cost` 2, `→ cast_restriction` 1, `activated → triggered` 1 (all
correct on inspection); **`spell_or_static_text → keyword_ability` 210** (see the
side effect below). Held-out units among all changes: 143 (counted only).

**False positives (prefix extracted where the text before ` — ` is not an ability
word, chapter symbol, named mode, or flavor word):**

| Class | Units | Kind effect | Sets |
|---|---:|---|---|
| Keyword syntax with a spaced em dash: `Prototype {cost} — P/T` (CR 702.160) | 21 | static → keyword (correct label, wrong reason) | bro 18, mh3 1, ybro 2 |
| Un-set keyword variants `Suspend 17 — {0}`, `Commander Suspend 4 — …`, `Fixed commander ninjutsu — …`, `Ransom — …` | 4 | static → keyword | unk, cmb1 |
| Spree cost labels `+ {M} — effect` (CR 702.169) | 51 | none (spell text either way; note these lines are not `role = mode`, a pre-existing gap) | otj |
| Die-roll / bounty / sticker table row labels `N —`, `N or N —`, `{TK}… —` | 264 | 41 rows become `keyword_ability` (Goblin Tutor, bounty tokens…), 96 P/T stickers become `keyword_ability`, rest unchanged | ugl, unf, unk, totc, sunf, afr (Treasure Chest `1 \| Trapped!`) |
| Inline mode header `Choose one — <modes on one line>` | 1 | none | tfth |
| Short villainous-choice clause `Each opponent faces a villainous choice —` | 1 | none | who |

**False negatives (prefix present but the rule refuses):** 16 units carry a spaced
em dash without firing. 12 have a period/colon before the dash and 4 are longer
than 45 characters; 14 of the 16 are Doctor Who / Monopoly / Captain America
constructions where the refusal is correct (`… faces a villainous choice — …`,
loyalty `−N: Heist! — …`, mode text ending in a sentence). **2 are genuine
flavor words containing sentence punctuation whose hidden trigger word stays
hidden**: The Eleventh Doctor `I. AM. TALKING! — Whenever …` and Captain America,
First Avenger `... Catch — At the beginning of combat …` (both classified
`spell_or_static_text`; should be `triggered_ability`). Unspaced em dashes
(`Suspend N—{M}`, `Ward—Pay N life.`, `Choose one —` headers with no body:
1,166 units) never fire, as designed.

**Side effect for the lead's attention — `is_keyword_line` on stripped bodies.**
210 units move `spell_or_static_text → keyword_ability` because the body after
the prefix is short and punctuation-free. Breakdown: sticker-sheet P/T bodies 96
and sticker keyword bodies 29 (`sunf`, set type *funny*); die-roll/bounty table
rows 41 wrong + 5 keyword bodies (`totc` tokens 36, `ugl`/`unf` 10); Prototype
P/T 21 (label correct: Prototype is a keyword ability); flavor word + genuine
keyword body 14 (`Diana — Equip {2}`, `Void Shields — Protection from …`, `Stage
2 — Evolve` — correct); 4 Un-set keyword variants (correct). Net: **141 newly
wrong keyword labels, all on *funny*/*token* set types (excluded from the walk by
S4 but present in corpus totals); 0 in expansion/core/commander sets**, versus 44
newly correct keyword labels. This is the known "short punctuation-free text is
labelled keyword" limitation becoming reachable through prefixed lines, not a
defect in the prefix rule; it is recorded, not fixed.

**Residual candidates (audit §7 A2: 8 → 3).** The 5 fixed rows are listed above.
The 3 that remain `prevention_effect` after prefix stripping, with the evidence
Codex needs (Oracle text and rulings from `card --rulings`):

- **Urza's Science Fair Project** (`ugl` 1998, Artifact Creature — Construct,
  `d071d5ca-64a4-47bc-aeb1-dd2dbc46a6a3`) #0:2 `2 — Prevent all combat damage it
  would deal this turn.` — a die-roll result line of the `{2}: Roll a six-sided
  die …` activated ability; no rulings. The unit is one row of a result table,
  not an ability of its own (boundary question, not a kind question).
- **Khârn the Betrayer** (`40k` 2022, `3246eff5-8739-469f-a401-e5de927dd6bb`)
  #0:2 `The Betrayer — If damage would be dealt to Khârn the Betrayer, prevent
  that damage and an opponent of your choice gains control of it.` — flavor word
  (CR 207.2d); ruling 2022-10-07: "An opponent gaining control of it is part of
  the replacement effect and happens immediately"; CR 615.1a: effects that use
  the word "prevent" are prevention effects.
- **Diamond Weapon** (`fin` 2025, `3beba02f-4d50-4a93-a13b-78d4595bfbfc`) #0:2
  `Immune — Prevent all combat damage that would be dealt to Diamond Weapon.` —
  flavor word; ruling 2025-06-06 calls it "Diamond Weapon's prevention effect".

## 7. P-ATQ-4 — spell-created delayed triggers

Source: S8 artifact §§P4.1–P4.6; post-merge `check_kind_rules.py` §B/B2.

**Population.** 12,466 printed units on instant/sorcery faces (per-face type
line); 10,780 top-level; roles ability 10,750 · mode 1,289 · delayed trigger 225
(195 P-ARN-1/2 children + 30 top-level) · granted 202. Top-level
`triggered_ability` units: **115 = 30 `role = delayed_trigger` (held-out 2) + 85
`role = ability` (held-out 4)**. The pre-change count of 111 top-level
`triggered_ability` units maps to 79 unchanged + 30 re-roled + 4 units whose
kind P-ATQ-3 recovered (Malicious Affliction, Bygone Marvels, Lumaret's Favor,
Summitfest Closing Ceremony — cast/`Whenever you cast` triggers, correctly kept
`role = ability`) + 2 units whose text P-ATQ-1 merged (Mangara's Blessing, Pure
Intentions #0:1). Python predicate vs dumped role: 0 disagreements on all 115.

**Positives (30; 28 non-pool inspected, 2 pool counted).** Temporal form `this
turn` 19 · `next` 10 · `this combat` 1; decades 1990s 2 · 2000s 3 · 2010s 11 ·
2020s 14; 3 on multi-face cards (Beck // Call, Indulge // Excess, Pigment
Wrangler // Striking Palette face 1). Every non-pool positive is a CR 603.7d
delayed trigger created by the spell's resolution; the two that mention a zone
word (Pure Intentions #0:0 "return *those* cards from your graveyard", Spellchain
Scatter "into your hand") name a destination, not the ability's own zone. **0
false positives.**

**Negatives (85 top-level `triggered_ability` units keeping `role = ability`).**
79 lack a stated duration (non-pool 75: cycling 21, self-zone
graveyard/discard/exile/library triggers 26, cast or resolution triggers 17,
haunt 3, suspend 2, Un-set/Alchemy instant permanents and miscellany 5, Ertai's
Meddling 1; held-out 4), 3 are excluded by off-stack evidence despite a
temporal phrase (Mangara's Blessing, Sunfire Balm, Pure Intentions #0:1 — all
correct), 3 are excluded as cast triggers despite `this turn` (Malicious
Affliction, Show of Confidence, Lumaret's Favor — all correct). **One
counterexample class to the proposal's stated-duration requirement:** Ertai's
Meddling (`tmp` 1997) #0:2 `At the beginning of each of that player's upkeeps, if
that card is exiled, remove a delay counter …` is a recurring delayed trigger the
spell creates (CR 603.7, 603.7b makes the duration optional; the rulings call the
result "a delayed spell") but has no `this turn`/`this combat`/`next`, so it
stays `role = ability`. 1 unit corpus-wide; recorded as a bounded false negative
of the pattern as proposed, not a defect of the implementation.

**False-negative sweep over every other I/S-face unit** (any kind/role, not
already `delayed_trigger`, containing a trigger word and a temporal phrase
outside quotes, with no delayed child): 59 candidates (held-out 5):

| Class | Units | Judgement |
|---|---:|---|
| Whole-unit inverted form `Draw a card at the beginning of the next turn's upkeep.` (Ice Age / Homelands / Mirage cantrips) | 43 | CR 603.7d delayed triggers that do not *begin* with a trigger word, so `kind` is spell text and P-ATQ-4 (which only re-roles `triggered_ability` units) is not reached. Same class as the 68 whole-line residuals in §4; outside the proposal's pattern |
| Same form as a mode child (Library of Lat-Nam, Sapphire Charm) | 2 | as above; modes are `kind = n/a` anyway |
| Duration-first `Until end of turn, whenever …` (Gaze of Pain + 1 pool) | 2 | delayed trigger with a leading duration; outside the pattern |
| Trigger word after a mid-sentence comma (`If this spell was kicked, whenever …`, `Until your next turn, whenever …`; 7 units incl. Howl of the Horde #0:1, Warhost's Frenzy, Season of the Bold) | 7 | conditional or duration-led delayed triggers inside spell text; outside the pattern (and outside P-ARN-1/2's sentence-boundary rules) |
| Sentence-initial `At the beginning of combat this turn, …` / `At the beginning of that combat, …` after a preceding sentence (Impulsive Return, World at War) | 2 | delayed triggers P-ARN-1's split does not reach because the phrase lacks `next`/`at end of combat`; a D5-class residual, not P-ATQ-4's |
| Mode child already `triggered_ability` (Twinferno) | 1 | modes are not re-roled by design |
| Granted quoted abilities (Tower Above, Valiant Farewell) | 2 | correctly `role = granted` |

I/S-face units carrying `delayed_trigger_unattached_candidate`: 47. Alternative
duration wordings searched among the 79 no-duration negatives (`until`, `each`,
`this game/phase/step`, `during`, `as long as`, `for the rest`): only `this game`
(the five `… Storm` cast triggers, correct) and Ertai's Meddling's `each` appear.

## 8. Regression (S10 item 5): fresh exports vs committed annotations

`export_units.py` at `8e83221`: lea 415 units (was 417), leb 2, arn 110 (was 112),
atq 124 (was 125). `audit_metrics.py --export` against the *committed* annotations:

| Set | Boundary (annotated) | Kind | Role | Dispositions | Novelty (fresh exports) | Drift rows |
|---|---|---|---|---|---|---|
| lea | 398 / 402 (ok 398 · under 2 · over 2 · unsure 1) | 390 / 391 | 396 / 396 | accept 409 · defect 5 · adjudicate 2 · ambiguous 1 | — | **4**: Cockatrice #1 changed[text,normalized] + #2 missing; Thicket Basilisk #0 changed + #1 missing |
| leb | 1 / 1 | 1 / 1 | 1 / 1 | accept 2 | 1 / 1 · 1 / 1 | 0 |
| arn | 110 / 112 (over 2) | 108 / 108 | 110 / 110 | accept 110 · defect 2 | 80 / 112 (0.714) · 79 / 95 (0.832) | **4**: Rukh Egg #0 changed + #1 missing; Nafs Asp #0 changed + #1 missing |
| atq | 123 / 125 (under 1 · over 1) | 123 / 123 | 123 / 123 | accept 123 · defect 2 | 96 / 125 (0.768) · 95 / 114 (0.833) | **2**: Battering Ram #1 changed + #2 missing |

Every drifted row is one of the five rule-(c) fragment pairs the proposal was
written to fix: the five parents are already `over` / `defect` in the committed
annotations, and their five `accept`ed children no longer exist as units because
the line is now one unit. **No unchanged row changes text, kind, role, or source,
so no previously accepted row becomes a non-accept result.** The five merged rows
need the re-annotation the proposal predicted (`under`, `missed = 1`, an honest
miss recorded by the unattached signal) — that is the research lead's step and
was not performed here; the committed exports, annotations and `metrics.json`
are therefore left at their pre-merge state so they stay a consistent pair.
Arabian Nights unit novelty against the fresh Alpha + Beta exports is 80 / 112
(committed value 77 / 109 was computed on the pre-merge exports; the difference
is the two merged Alpha rows and the two merged Arabian Nights rows).

## 9. Held-out exclusions (counts only)

Prefix rule firings 224; prohibition-idiom units 5; P-ATQ-4 positives 2,
negatives 4, sweep candidates 5; kind changes under P-ATQ-3 143; keyword-label
side effect 15. None was listed or inspected.

## 10. Unresolved or unexpected

1. **P-ATQ-3 residuals** (§6) await Codex's adjudication; this pass supplies the
   Oracle text, rulings and CR citations only.
2. **P-ATQ-3 side effect**: 141 newly wrong `keyword_ability` labels on
   funny/token set types through `is_keyword_line`; 2 flavor words with sentence
   punctuation keep a trigger word hidden. Both are bounded, listed, and outside
   the walk's set types or ≤ 2 units; neither is card-specific.
3. **P-ATQ-4 pattern boundary**: Ertai's Meddling (no stated duration) and the
   45 whole-unit inverted cantrip lines are CR 603.7d delayed triggers the
   proposal's pattern does not claim; the 7 comma-led and 2 sentence-initial
   `At the beginning of combat this turn` forms belong to the D5/D19 delayed-
   trigger residue, not to P-ATQ-4.
4. **Documentation**: the two historical check scripts still print `commit
   af150b0` in their banner (a literal string); their post-merge outputs were
   used for the numbers above and the banner is not evidence of the commit.
