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
earlier distinct templates 374. **N1 is falsified at this step**: novelty
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

### P-ATQ-1 implementation disposition (2026-08-26)

**Implemented, not yet accepted under protocol S10.** `delayed_trigger_split`
in `src/main.rs` was narrowed to the two proposed cases (sentence-level
generic/inverted `next`/`at end of combat`, and scoped `When`/`Whenever ...
this turn`/`this way`/`When you do`); the single-sentence backward
comma/colon search (rule (c)) was deleted rather than guarded, so it can no
longer fire on a leading trigger-condition comma, an activation-cost colon,
or (having searched unmasked text) inside a quoted ability. When no
sentence-boundary split point exists, the unit is left whole and the
existing `delayed_trigger_unattached_candidate` audit signal (the T8-style
unresolved-trigger slot already in `suspicious_signals`) is extended to
cover this case, so no new signalling mechanism was added. Regression tests
were added for: sentence-level splitting still producing a valid parent and
a `delayed_trigger` child; the Battering-Ram-class leading trigger-condition
comma (plain and inverted-`next`-phrase forms, matching Cockatrice/Thicket
Basilisk/Rukh Egg/Nafs Asp's wording class); the `{T}:` activation-cost
colon; a quoted granted ability's internal punctuation, for both the outer
unit and the granted child itself; and the unattached-trigger signal firing
in the conservative-fallback case. `cargo fmt -- --check`, `cargo test` (45
passed), `cargo clippy --all-targets -- -D warnings`, and `cargo build
--release` all pass.

What is **not** done: this was implemented in a sandboxed session whose
network egress policy blocks `api.scryfall.com` (confirmed 403 via the
agent-proxy status endpoint), so `cards.sqlite` and the Scryfall bulk files
do not exist and could not be regenerated. Section 6's S11 corpus check
(`scripts/python/corpus_checks/check_delayed_split.py`), the corpus-wide
`templates` before/after totals, and the `audit_metrics.py` regression rerun
against `lea`/`leb`/`arn`/`atq` exports (S10 items 4–5) were **not run**.
The ~982 → ~869 delayed-trigger count and the disappearance of the 121
comma/colon and 3 in-quote splits above remain the *expected* effect from
S10 item 4's original estimate, not a measured one. `docs/current-state.md`
records this same caveat. A later session with data access must run the
reproduction commands in section 10, confirm the corpus counts, rerun
`audit_metrics.py` on the four earlier exports to confirm no new
non-`accept` rows, and only then treat P-ATQ-1 as accepted.

**P-ATQ-2 — `can't be prevented` is not a prevention effect.** Exclude text
matching `can't be prevented` from `prevention_effect`; classify as residual
static. Fix rows: the 9 units listed in check B §A (CR 615.1a defines
prevention effects by *preventing* damage).

### P-ATQ-2 implementation disposition (2026-08-26)

**Implemented, not yet accepted under protocol S10.** `classify_kind` in
`src/main.rs` gained a `prevention_prohibition` regex
(`can(?:'|’)?t be prevented|cannot be prevented`, checked against the same
lowercased normalized text as the existing `prevention` regex; apostrophe
optional and either straight or curly, since normalization does not fold
apostrophes and neither form could be ruled out without corpus access) and
the `prevention_effect` branch condition became `prevention.is_match(&lower)
&& !prevention_prohibition.is_match(&lower)`. No new `AbilityKind`, no
reordering of the surrounding `replacement`/`cda`/residual chain: a
prohibited unit simply falls through to whichever of those already-existing
branches its wording matches, landing on the residual `spell_or_static_text`
kind for the audit's 9 fix rows exactly as the proposal specified. The
ability-word/Saga-chapter/named-mode prefix class (the other 8 misfires in
check §7 A) is untouched, as scoped — P-ATQ-3 remains unimplemented.

Regression tests added: `prevention_prohibition_is_not_classified_as_
prevention_effect` (two independently worded `can't be prevented`
statics, `cannot be prevented`, and a curly-apostrophe `can’t be prevented`
variant, all landing on `spell_or_static_text`) and
`prevention_prohibition_exclusion_does_not_regress_genuine_prevention` (a
unit that both commands genuine prevention and separately describes damage
as "is prevented" still classifies as `prevention_effect`, demonstrating the
new regex is the narrow collocation and not a `contains "prevented"`
blanket rule). The existing `static_prevention_effects_have_their_own_kind`
and `prevention_in_activated_triggered_or_spell_text_keeps_precedence`
tests were left unchanged and still pass unmodified — none of their
fixtures use the prohibition wording, so no prior expectation needed to
change. `cargo fmt -- --check`, `cargo test` (47 passed), `cargo clippy
--all-targets -- -D warnings`, and `cargo build --release` all pass.

What is **not** done, for the same reason as P-ATQ-1: this session's network
egress policy again returns 403 for `api.scryfall.com` (re-confirmed this
session), so `cards.sqlite` does not exist and could not be regenerated.
`scripts/python/corpus_checks/check_kind_rules.py` reads `cards.sqlite`
directly (for type-line lookups) and its required input comes from
`dump_corpus_units.py`, which also needs the database — neither could run.
Consequently:

- The protocol's S8 counterexample search (corpus hits for `prevent`,
  `prevented`, `can't be prevented`, `cannot be prevented`, inspected across
  decades) was **not performed**. The apostrophe-form decision above is
  based on the local `Magic-Comprehensive_Rules.md` text (which uses a
  curly `’`) and this repository's own `src/main.rs` conventions (the CDA
  regex and every existing test fixture use a straight `'`, consistent with
  Scryfall's Oracle-text convention), not on a corpus sample — hence
  supporting both forms rather than picking one.
- The before/after count of `can't be prevented` misfires (audit figure: 9
  → expected 0) is **not measured** in this session.
- Whether any additional prohibition wording variant exists in the corpus
  beyond the two forms tested is **unknown**; none is assumed found, and
  none should be treated as resolved.
- The separate P-ATQ-3 prefixed-trigger class (8 misfires) is expected to
  be unaffected by this change (the new regex only excludes text matching
  the prohibition collocation) but this has not been corpus-verified either.

`docs/current-state.md` records this same caveat. A later session with data
access must run `dump_corpus_units.py` and `check_kind_rules.py`, confirm
the 9-misfire class is gone with no new false positives (genuine prevention
wrongly excluded) or false negatives (a prohibition variant still
misclassified), and only then treat P-ATQ-2 as accepted.

**P-ATQ-3 — strip ability-word / chapter / named-mode prefixes before
classification.** A leading `<words> — ` (no period, no colon, ≤ 45 chars)
is an ability word, Saga chapter, or named mode (CR 207.2c, 714.2); classify
the text after the dash and record the prefix as a field. Fix rows: the 8
prefixed triggers mislabelled prevention, and (to be measured) every other
prefixed unit currently classified residual. Needs its own S8/S11 pass.

### P-ATQ-3 implementation disposition (2026-08-26)

**1. Original observation.** Check §7 A2 above: 8 role=ability
`prevention_effect` units share a leading `<prefix> — ` construction
(`Heroic — Whenever …`, `Constellation — Whenever …`, `Lieutenant — At the
beginning …`, `2 — Prevent …`, `The Betrayer — If … prevent …`, `I, II —
Prevent …`, `Immune — Prevent …`, `II — Prevent …`), described there as the
prefix hiding the trigger word from the classifier.

**2. Hypothesis.** As stated in the proposal above: an em-dash-delimited
leading prefix of bounded length with no period or colon is structural
material (an ability word, a Saga chapter symbol, or a named mode/label),
not part of the classification-relevant text, and stripping it before
`classify_kind` should recover the correct kind without a card- or
set-specific exception list.

**3. Implementation.** `build_unit` in `src/main.rs` gained a new
`extract_prefix(normalized) -> Option<(String, &str)>` step, run on the
already-`normalize_text`-processed unit text immediately before
`classify_kind`. The pattern is `^([^.:]{1,45}?) — (\S.*)$`: anchored to
the start of the unit (a leading prefix only), the delimiter is
specifically an em dash (not any other dash or hyphen), the candidate
prefix may contain no period or colon and is capped at 45 characters
(matching the proposal's approximate bound), and the text after the dash
must be non-empty (so a bare mode header like `Choose one —`, with nothing
following on the same line, never matches). The detected prefix is
recorded verbatim on a new `prefix: Option<String>` field added to
`Segment` — the only new field; `text` (original Oracle text) and
`normalized` (the existing corpus-wide template) are both left completely
unchanged, so this is a classification/metadata change, not a
normalization change, per the task's explicit separation of source text,
prefix metadata, classification input, and normalized template. No
separate `classification_text` field was added: it is deterministically
`normalized` with `"<prefix> — "` removed from the front whenever `prefix`
is `Some`, which the task's "smallest appropriate field" guidance argues
against duplicating.

Two prefix categories are distinguished, matching the task's requirement
not to treat every prefix as semantically identical:

- **Saga chapter symbol (CR 714.2).** `is_saga_chapter_prefix` recognizes
  one or more comma-separated pure Roman numerals (`I`, `II`, `I, II`, …;
  the character set is exactly `IVXLCDM`, so a normalized Arabic numeral
  like `N` — from `2 —` — never matches). This is gated by `is_saga`, which
  requires the unit's per-face type line to carry the Saga subtype (the
  same per-face `type_line` already threaded through `build_unit` for
  P-ARN-3's instant/sorcery override). When both hold, the unit's `kind` is
  set to `triggered_ability` directly — `classify_kind` is never run on the
  stripped body at all — because CR 714.2b defines a chapter symbol as
  "a keyword ability that represents a triggered ability" regardless of
  what the printed effect text says; running `classify_kind("Prevent …")`
  on the stripped body would reproduce this exact failure one level down
  (`prevention_effect` instead of `triggered_ability`), which is precisely
  the literal "strip and classify body" interpretation section 5 of the
  task warned against forcing.
- **Everything else (ability word, CR 207.2c; named mode/label; a
  non-Saga numeral label).** The stripped body is passed to the existing,
  unmodified `classify_kind`, with the same `type_line` and
  `allow_spell_text_override` the whole (unstripped) unit would have
  received. This recovers a hidden `When`/`Whenever`/`At` trigger word
  (the `starts_with` checks in `classify_kind` only succeed when the
  trigger word is literally first) without touching the unanchored
  `prevent`/`instead`/CDA regex branches, which already scan the full text
  regardless of a leading prefix and so were never the source of the
  hidden-trigger failure. A mode child (`role = mode`) goes through the
  same `build_unit` call as any other unit, so a named-mode prefix (e.g.
  `Run and Hide —` inside a modal spell's `•` list) is recorded and
  stripped identically; role and the mode/ability distinction are
  untouched by this change.

No card name, set code, `oracle_id`, or ability-word/label vocabulary list
appears in the implementation; the rule is purely structural (delimiter,
length, punctuation, and, for the chapter case only, the CR-defined
Roman-numeral-and-Saga-type gate). Normalization (`normalize_text`) and the
`normalized` field were not touched, so the corpus-wide template baseline
in `docs/current-state.md` is unaffected by this change on its own.

**4. Tests.** 16 regression tests were added to `src/main.rs` (`cargo test`:
61 passed, up from 47), all synthetic and none naming an Antiquities card
in production code:

- an ability-word prefix recovering a `Whenever` trigger, and a second over
  an `At the beginning of` trigger (guards against a fix that only handles
  one trigger word — task item B);
- a multi-chapter (`I, II —`) and a single-chapter (`II —`) Saga marker on
  an `Enchantment — Saga` type line, both asserting `triggered_ability`
  even though the body starts with `Prevent` (task items C, D);
- the same Roman-numeral prefix on a *non*-Saga type line, asserting it is
  **not** treated as a chapter symbol and falls through to ordinary body
  classification (a counterexample to the chapter rule, beyond what the
  task listed by name);
- a named-mode prefix (`Run and Hide —`) inside an actual modal spell's
  `•` child, asserting the mode `role` survives and the body's
  `prevention_effect` kind matches what the existing prevention machinery
  already assigns to that wording once the label is out of the way (task
  item E);
- an early-colon guard, an early-period guard, and an overlong-prefix
  guard, each asserting `prefix` stays `None` and classification is
  unchanged from before this change (task items F, G, H);
- a mode-header em dash with no following body (`Choose one —`), asserting
  no prefix is recorded over its own bullet children — this is the one
  "ordinary em-dash usage does not misfire" case (task item I) this
  session could verify against a real, extremely common corpus pattern
  (CR 700.2 modal spells) without database access; see uncertainty below
  for what a full S8 search would still need to cover;
- the P-ATQ-2 `can't be prevented` exclusion and a prefix-free genuine
  `prevention_effect` case, both reconfirmed unaffected (task items J, K);
- two direct unit tests of `extract_prefix` and `is_saga_chapter_prefix`.

`cargo fmt -- --check`, `cargo test` (61 passed), `cargo clippy
--all-targets -- -D warnings`, and `cargo build --release` all pass.

**5. Corpus measurement.** **Not performed**, same blocker as P-ATQ-1 and
P-ATQ-2: this session's network egress policy again returns 403 for
`api.scryfall.com` (re-confirmed this session), so `cards.sqlite` does not
exist and could not be regenerated, and neither `dump_corpus_units.py` nor
`check_kind_rules.py`/`check_kind_rules_part2.py` could run. Consequently:

- The protocol's S8 counterexample search (§18 of the task: searching
  specifically for short, punctuation-clean, em-dash-joined constructions
  that are *not* an ability word, chapter symbol, or named mode, where
  stripping would be semantically wrong) was not performed against the
  corpus. It is informed only by CR 207.2c/714.2, this audit's own §7 A2
  evidence, and this session's knowledge of Magic Oracle-text conventions
  (planeswalker loyalty abilities are colon-delimited, not em-dash, so
  they cannot collide with this rule under any type line); no non-label
  short em-dash construction under the 45-character/no-period/no-colon
  bound was identified by inspection, but this is not a corpus search.
- The S11 corpus-wide over-segmentation check (every distinct line the
  rule fires on; false-positive rate; before/after `templates` and
  `prevention_effect`/`triggered_ability` totals; role and card-type
  breakdown) was not run.
- Whether all 8 historical misfires in §7 A2 actually change kind under
  this implementation is **reasoned, not measured**: 3 are ability words
  whose hidden trigger word is recovered (`Heroic`, `Constellation`,
  `Lieutenant` rows — Favored Hoplite, Harvestguard Alseids, Loyal
  Unicorn), expected `prevention_effect` → `triggered_ability`; 2 are
  genuine Saga chapter markers (`I, II —`, `II —` rows) expected to move
  the same way via the chapter-symbol path; the remaining 3 (`2 —` on a
  non-Saga Un-set card, `Immune —`, `The Betrayer —`) have bodies that
  already begin with `Prevent` or `If … would … prevent`, wording
  `classify_kind` already assigns `prevention_effect` to with or without
  the prefix present, so this rule is not expected to change their kind —
  they remain evidence for the general structural phenomenon and now carry
  recorded prefix metadata, but this proposal does not claim to "fix" them
  in the sense of changing their label.
- Whether any additional prefix wording exists in the corpus beyond the
  patterns evidenced in §7 A2 (for example, whether the 45-character bound
  or the no-period/no-colon constraints are too strict or too loose
  corpus-wide) is unknown.

`docs/current-state.md` records this same caveat.

**6. Remaining uncertainty.** All of section 5's gaps, plus: whether the
`is_keyword_line` interaction (a stripped body that is itself keyword-shaped
would now classify `keyword_ability` where the unstripped text could not,
since `is_keyword_line` excludes any text containing an em dash) ever fires
in the real corpus — no such case was found in the audited sets, but it was
not searched for corpus-wide; and whether a `classification_text`-shaped
audit signal (flagging a unit whose prefix was extracted, for reviewer
triage) would be useful — deferred rather than added speculatively, per the
task's instruction not to introduce a large prefix ontology at this stage.

**7. Acceptance status.** **Implemented, not yet accepted under protocol
S10/S11.** A later session with data access must run
`dump_corpus_units.py` and `check_kind_rules.py`/`check_kind_rules_part2.py`,
confirm the actual before/after `prevention_effect`/`triggered_ability`
counts and role/kind histograms, execute the S8 counterexample search and
S11 over-segmentation check this session could not run, re-run
`audit_metrics.py` against `lea`/`leb`/`arn`/`atq` to confirm no new
non-`accept` rows, and only then treat P-ATQ-3 as accepted.

**P-ATQ-4 — spell-only delayed triggers.** On an instant/sorcery face, a
top-level unit beginning `When/Whenever/At` with a stated duration (`this
turn`, `this combat`, `next`) and no off-stack zone reference is spell text
that creates a delayed trigger (CR 603.7d): keep kind `triggered_ability`,
set role `delayed_trigger` with the face as parent. Fix rows: the 30 units
in check B §B. Needs an S8 search for off-stack exceptions before
acceptance.

### P-ATQ-4 implementation disposition (2026-08-26)

**1. Original observation.** Check §7 B/B2 above: of 111 top-level
I/S-face `triggered_ability` units (105 non-pool), the script's
measurement heuristic split them into three classes — 30 "delayed trigger
created by the spell (603.7d)", 65 "off-stack triggered ability of the
card (113.6b: cycle/discard/graveyard/exile/haunt/suspend)", 16
"cast/resolve trigger of the spell" — all three currently share `role =
ability`. Class A (the 30) is structurally different from the other two:
resolving the spell *creates* the trigger (CR 603.7d), rather than the
unit being an ordinary ability of the card.

**2. Hypothesis.** As stated in the proposal above, refined by this
session's investigation (task step 7): a top-level, printed, `kind =
triggered_ability` unit on an instant/sorcery face is Class A only when it
carries positive evidence of a CR 603.7b stated duration (`this turn`,
`this combat`, or a `next ...` phrase), *and* carries no evidence that it
instead functions off the stack (CR 113.6b) or is about the spell's own
casting or resolution (a different, non-603.7d kind of triggered ability
that also happens to appear on a spell). The task's own instruction not to
blindly copy `check_kind_rules_part2.py`'s exclusion-word blacklist (`\bcycle\b|graveyard|discard
this card|exiled|suspended|haunts`) was investigated directly: see finding
3 below for two real corpus rows (already committed, non-pool) where that
blacklist disagrees with the CR-grounded rule, in both cases incorrectly.

**3. Implementation.** A new `apply_spell_created_delayed_triggers` pass
runs once inside `segment_text`, after every line's unit has already been
built and attached (mode children, P-ARN-1/P-ATQ-1 delayed-trigger
children, granted quoted abilities) but before index assignment. It walks
only the top-level `segments` slice — never recursing into `.children` —
and for each unit with `role == Ability && source == Printed && kind ==
Triggered`, calls a new `is_spell_created_delayed_trigger(classification_text,
face_type_line)` predicate; on a match it sets `segment.role =
DelayedTrigger` **in place**, changing nothing else.

`is_spell_created_delayed_trigger` is `is_instant_or_sorcery(type_line) &&
has_delayed_trigger_temporal_scope(text) && !is_cast_or_resolve_trigger(text)
&& !has_off_stack_evidence(text)`:

- `has_delayed_trigger_temporal_scope` — `\bthis turn\b|\bthis combat\b|\bnext\b`
  (CR 603.7b's "stated duration"). This is deliberately the same surface
  evidence the research script already used; the difference from the
  script is entirely in what disqualifies a match, not in what triggers a
  candidate look.
- `is_cast_or_resolve_trigger` — `^when you cast ~|\bcast ~ from\b|~ is
  countered\b|^when ~ resolves\b`, matched against the *same* normalized,
  self-reference-collapsed text `classify_kind` already used (`normalize_text`
  replaces both the card's own name and `this spell`/`this card` with `~`,
  so "When you cast this spell, ..." is `when you cast ~, ...` by the time
  this predicate runs). This exclusion is checked as a hard negative,
  independent of whether a temporal phrase is also present — necessary
  because a genuine cast trigger can still contain "this turn" elsewhere in
  its effect text (finding 3 below).
- `has_off_stack_evidence` — a bare match on a CR-defined off-stack keyword
  (`cycl(?:e|ing)`, `suspended`, `haunts?`; these are always about the
  object bearing them) **or** a proximity match between the unit's own
  self-reference `~` and a `graveyard`/`exile(d)`/`discard(ed/s)` zone word
  within the same sentence (`~[^.]{0,30}\bzone-word\b` or the reverse
  order). The proximity form is deliberately *not* a bare word blacklist:
  the task's own example — distinguish "if this card is in your graveyard"
  from "Whenever a card is put into a graveyard this turn" — is exactly
  what `~` co-occurrence checks for and a substring match does not.

All three predicates and `apply_spell_created_delayed_triggers` itself
operate on `classification_text(segment)`, a new helper that calls
`extract_prefix` (P-ATQ-3) again on `segment.normalized` and returns the
stripped body when a prefix was found, otherwise the full normalized text
— the exact text `classify_kind` already judged the unit on. This reuses
P-ATQ-3's own extraction function rather than duplicating or bypassing it,
per the task's explicit instruction.

**Face-as-parent representation (task step 15).** The current schema has
no `Segment` for a whole card face to point a `parent_index` at, and this
session did not add one. A P-ATQ-4 unit instead stays exactly where
`segment_text` already put it: in the top-level `segments` list for its
face, with no parent. CR 603.7d already says "the source of that delayed
triggered ability is that spell" — for a single printed unit that *is* the
spell's entire effect, "created by resolving this spell" and "is this
face's top-level ability" are the same fact, so no additional structure
was needed to represent it. This also directly answers the audit-export
distinguishability requirement (task step 21) without a new field: a
P-ATQ-4 unit is `role = delayed_trigger` with `parent_index = None`; a
P-ARN-1/P-ATQ-1 split *child* is `role = delayed_trigger` with
`parent_index = Some(...)`; a granted delayed trigger inside a quoted
ability is `role = granted`, a different role entirely. All three remain
distinguishable in `audit export` output from the existing `role` and
`parent_index` columns alone.

**4. Tests.** 21 regression tests were added to `src/main.rs` (`cargo
test`: 82 passed, up from 61), all synthetic:

- the four positive classes — `this turn`, `you next cast ... this turn`,
  `at the beginning of combat this turn`, `this combat` (task items A–D);
- the single-line, no-preceding-sibling case (task item E), which is the
  concrete defect this proposal fixes: before this change such a unit fell
  back to `role = ability` because the pre-existing `delayed_trigger_start`
  mechanism (used for a *separate* trailing line like "At the beginning of
  the next end step, ..." that continues an *earlier* line's effect) only
  keeps `role = delayed_trigger` when it can attach the unit as a child of
  a preceding sibling, and a lone top-level unit has none;
- cycling, graveyard-self, discard-self, and suspend negative classes
  (task items F–I);
- a direct positive counterexample showing the graveyard/exile/discard
  check is proximity-based rather than a blacklist: a delayed trigger
  scoped `this turn` that returns *someone else's* cards from a graveyard
  is correctly **not** excluded (see the "graveyard" discrepancy in
  finding 3, which failed exactly this case in the research script);
- a cast trigger that also contains a `this turn` phrase, asserting the
  cast/resolve exclusion applies regardless (task item J, extended: see
  the "Show of Confidence" discrepancy in finding 3, which is exactly this
  case);
- a resolution trigger (task item K);
- identical wording on a non-instant/non-sorcery face, confirming type-line
  context still gates the whole mechanism (task item L);
- non-trigger spell text containing `this turn`, confirming the `kind`
  gate excludes it before role is even considered (task item M);
- the existing multi-line P-ARN-1/P-ATQ-1 delayed-trigger *child* split,
  reconfirmed to produce exactly the same parent/child shape as before
  (task item N);
- P-ATQ-2's `can't be prevented` exclusion, reconfirmed unaffected (task
  item O);
- P-ATQ-3's Saga-chapter-symbol and ability-word prefix handling, both
  reconfirmed unaffected — including a case combining a P-ATQ-3 ability-word
  prefix with a P-ATQ-4 temporal clause, confirming the prefix hides
  P-ATQ-4's evidence no more than it hides `classify_kind`'s (task item P,
  extended);
- three direct unit tests of the new helper predicates.

`cargo fmt -- --check`, `cargo test` (82 passed), `cargo clippy
--all-targets -- -D warnings`, and `cargo build --release` all pass.

**5. Corpus measurement.** A full S8/S11 corpus pass was **not performed**:
this session's network egress policy again returns 403 for
`api.scryfall.com` (re-confirmed), so `cards.sqlite` does not exist and
neither `dump_corpus_units.py` nor `check_kind_rules_part2.py` could run
against the live corpus — the same blocker as P-ATQ-1/2/3.

This session did run a **desk cross-check against already-committed,
non-pool evidence**, distinct from a live corpus query: the 105 non-pool
rows of §7 B/B2's own printed listing (verbatim `unit_text`, `type_line`,
and `card_name` for all 111 I/S-face top-level `triggered_ability` units,
minus the 6 pool rows the script itself excludes from printing) were fed
through the release binary's `segment --text --type-line` command. Result:
28 classified `delayed_trigger`, 77 stayed `ability`, 0 execution errors.
Every one of the 28 matches this session's own independent, by-hand CR
classification of the same 105 rows (recorded in this session's working
notes, not committed separately). Diffing this rule's output against
`check_kind_rules_part2.py`'s blacklist heuristic on the identical 105
rows found exactly two disagreements:

- **A "Show of Confidence"-class row** ("When you cast [this spell], copy
  it for each other instant and sorcery spell you've cast this turn."):
  the *old* heuristic's `if`/`elif` structure lets its temporal-scope
  branch match and return before its separate cast/resolve branch ever
  runs, so it misclassifies this as a spell-created delayed trigger. The
  new rule's cast/resolve exclusion is a hard negative applied regardless
  of the temporal-scope match, so it correctly excludes this row.
- **A "Pure Intentions"-class row** ("Whenever a spell or ability an
  opponent controls causes you to discard cards this turn, return those
  cards from your graveyard to your hand."): the *old* heuristic's bare
  `graveyard` substring exclusion misclassifies this as off-stack, even
  though "graveyard" here names the destination of *other, already-
  discarded* cards, not a zone check on the ability's own object. The new
  rule's `~`-proximity check finds no self-reference near "graveyard" in
  this sentence and correctly includes it. This is precisely the false-
  exclusion class the task instructions asked this session to check for.

Both disagreements resolve in the new rule's favor on CR grounds, and both
are drawn from real, already-audited, non-pool corpus text rather than
invented examples.

This cross-check is evidence toward, but does not itself satisfy, S8/S11:
it does not cover the full 12,468-unit I/S-face population (only the 111
already flagged `triggered_ability` by the earlier audit), the 6
held-out-pool rows within that 111 (correctly not inspected, per protocol
§6.3), or any temporal phrasing outside `this turn`/`this combat`/`next`
that a false negative could hide behind. Before/after role and kind
histograms, the corpus-wide `templates` totals, and a rerun of
`audit_metrics.py` against `lea`/`leb`/`arn`/`atq` were **not** produced
this session.

**6. Remaining uncertainty.** Everything in finding 5's scope note, plus:
whether a stated-duration phrasing this proposal does not recognize exists
elsewhere in the corpus (task step 19 asks specifically for this; none was
found in the 105-row sample, which is not the same as a corpus-wide
search); whether `~ is in your library`-style self-zone checks (seen in
this session's evidence on a unit with no temporal phrase, so harmless
here, but not covered by `has_off_stack_evidence`) could produce a false
positive on some other card that also happens to use a temporal duration
phrase — deferred rather than added speculatively, since no such row was
found; and the unscoped `When …` embedded-delayed-trigger class (Tawnos's
Coffin, Animate Dead) remains explicitly out of scope, unchanged by this
session, per task step 27.

**7. Acceptance status.** **Implemented, not yet accepted under protocol
S10/S11.** A later session with data access must run
`dump_corpus_units.py` and `check_kind_rules_part2.py`, confirm the actual
before/after role distribution and the historical ~30-unit comparison
point against the live corpus, execute a true S8 counterexample search
over the full I/S-face population (not only the 111 rows already flagged
`triggered_ability`), rerun `audit_metrics.py` against
`lea`/`leb`/`arn`/`atq` to confirm no new non-`accept` rows, and only then
treat P-ATQ-4 as accepted. This is the last of the four Antiquities
proposals (P-ATQ-1..4); the unscoped `When …` class remains a separate,
deferred research question (D19), not part of this disposition.

**Deferred, not proposed:** the unscoped `When …` after effect text
(Tawnos's Coffin, Animate Dead) — needs its own S8 search separating it
from D14's independent triggers (D19).

### Local corpus-validation follow-up (2026-08-26)

After PRs #1–#4 were merged, the full local corpus and S11 scripts were run
against the combined implementation. The current output has 71,563 printed
units and 37,299 templates. P-ATQ-1 leaves 861 delayed-trigger children, all
sentence-level, with no comma/colon children. P-ATQ-2 leaves no
`can't be prevented` prohibition classified as `prevention_effect`. P-ATQ-4
marks exactly 30 qualifying top-level units `role = delayed_trigger`, distinct
from the 861 nested children.

P-ATQ-3 reduced the 8 recorded prefix-related prevention candidates to 3
(Urza's Science Fair Project, Khârn the Betrayer, and Diamond Weapon). This
measurement does not adjudicate whether those three are remaining defects or
correct post-prefix classifications. P-ATQ-3 therefore remains pending lead
review, as do the protocol-required re-annotation and acceptance decisions for
all four implementations. *(Superseded by the post-merge acceptance pass
recorded in the next subsection.)*

### Post-merge acceptance pass (2026-08-26, commit `8e83221`) — S10 dispositions

Evidence: `docs/audits/corpus-checks/2026-08-26-post-patq-merge.md` (summary,
frozen inputs, before/after totals, regression) and
`docs/audits/corpus-checks/2026-08-26-patq-s8-search.md` (raw output of the new
`scripts/python/corpus_checks/check_patq_s8.py`, which restates the P-ATQ-3/4
patterns for a corpus search, cross-checks them against the binary on 3,340
units with 0 mismatches, and carries a `--self-test`). Build at `8e83221`:
`cargo test` 82 passed, `cargo fmt --check` and `cargo clippy -D warnings` clean;
`cards.sqlite` sha256 `d1c88cb9…95c4` from the 2026-08-25 Scryfall drop. The
before/after comparison used a second binary built from `8c0f229`. Corpus-wide:
71,682 → 71,563 printed units, 37,344 → 37,299 templates, top-100 coverage 26.85
→ 26.90 %, delayed-trigger roles 982 → 891 (861 children + 30 top-level),
`prevention_effect` 181 → 166, triggered 17,503 → 19,214. Regression: fresh
exports of `lea`/`leb`/`arn`/`atq` drift from the committed annotations in
exactly the five rule-(c) fragment pairs (Cockatrice, Thicket Basilisk, Rukh
Egg, Nafs Asp, Battering Ram — parents already `over`/`defect`); no unchanged
row changes, so no accepted row becomes a non-accept result. The committed
exports/annotations/`metrics.json` are left at their pre-merge state pending the
lead's re-annotation of those five rows (`under`, `missed = 1`).

**Decision: P-ATQ-1 accepted** (2026-08-26, technical validator; S10 items 1–7
present). *Evidence:* report §4 — 121 → 0 comma/colon children, 3 → 0 in-quote
splits, 0 bare condition/cost templates at top level, all 121 fragments merged
back (116 carry `delayed_trigger_unattached_candidate`; the 5 without it hold
the phrase inside a quoted granted ability), sentence-level sample 30 / 30.
*Options considered:* guard rule (c) instead of deleting it — rejected in §6
already. *What would reverse it:* a sentence-level split producing a
non-reference-unit fragment, or a comma/colon child reappearing corpus-wide.
*Affected documents:* `docs/current-state.md`, `docs/roadmap.md` D15, the five
annotation rows above (lead).

**Decision: P-ATQ-2 accepted** (2026-08-26). *Evidence:* report §5 — 35
prohibition units corpus-wide (straight apostrophe only; `cannot` 0), 0
labelled `prevention_effect`, 0 units where a genuine prevention is excluded
alongside the idiom, 4 nearest-wording units (`can't be regenerated/countered`
next to `prevent`) unaffected; 181 → 166 accounted for unit-by-unit.
*What would reverse it:* a CR 615.1a static containing the idiom that must
remain `prevention_effect`. *Affected:* `docs/current-state.md`, D16.

**Decision: P-ATQ-3 — technical acceptance evidence complete; S10 acceptance
withheld pending Codex's adjudication of the three residual rows.** *Evidence:*
report §6 — 3,572 firings (chapters 624 / 624 `triggered_ability`; ability words
1,406, parsed from CR 207.2c; named modes 239; flavor words and labels 1,301),
2,053 kind changes of which 1,786 recover a hidden trigger word and 5 are the
audit's fix rows; binary cross-check 0 mismatches. *Counterexamples recorded
(bounded, none card-specific):* structural false positives — Prototype's
keyword syntax (21), spree `+ {M} —` cost labels (51, kind unchanged),
die-roll/bounty/sticker table labels (264), one inline `Choose one —`, one short
`villainous choice —`; false negatives — 2 flavor words containing sentence
punctuation (The Eleventh Doctor, Captain America, First Avenger) keep a trigger
word hidden; side effect — 210 stripped bodies now reach `is_keyword_line`,
141 of them wrongly labelled `keyword_ability` (all funny/token set types) and
44 correctly. *Residuals for Codex:* Urza's Science Fair Project #0:2 (a die-roll
result row of an activated ability; boundary question), Khârn the Betrayer #0:2
and Diamond Weapon #0:2 (flavor-word prevention statics; rulings quoted in the
report). *Placeholder:* **[Codex adjudication of the three residual P-ATQ-3
candidates: not yet supplied — accept/defect disposition and rationale to be
inserted here without altering the measurements above.]** *What would reverse
it:* Codex finding a defect in the prefix rule itself (as opposed to the
pre-existing keyword heuristic) or a prefix class whose stripping changes a
correct kind to a wrong one in a walk-eligible set type. *Affected:* D17,
`docs/current-state.md`.

**Decision: P-ATQ-4 accepted** (2026-08-26). *Evidence:* report §7 — full
12,466-unit instant/sorcery-face population searched, not only the 111
previously flagged units; 30 positives (28 non-pool inspected, all CR 603.7d;
0 false positives), 85 negatives inspected by class, predicate vs role 0
disagreements. *Counterexample classes outside the proposal's stated pattern,
recorded not fixed:* Ertai's Meddling (recurring delayed trigger with no stated
duration; CR 603.7b makes the duration optional), 45 whole-unit inverted cantrip
lines (`Draw a card at the beginning of the next turn's upkeep.`), 2
duration-first and 7 comma-led delayed triggers inside spell text, 2
sentence-initial `At the beginning of combat this turn` forms (D5 residue).
*What would reverse it:* a positive that is an off-stack or cast/resolution
trigger, or a decision to widen the pattern to duration-less forms (a new
proposal, not this one). *Affected:* D18, `docs/current-state.md`.

Legends is **not started**; its prerequisite (an accepted Antiquities baseline)
is met for P-ATQ-1/2/4 and blocked on the P-ATQ-3 adjudication above. D19 (the
unscoped `When …` class) remains a separate deferred question.

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
| Multi-sentence units (automatic) | 29 / 125 |
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
