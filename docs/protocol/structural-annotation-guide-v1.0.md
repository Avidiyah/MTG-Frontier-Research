# Structural annotation guide — candidate v1.0 (not frozen)

- Status: **candidate v1.0 — draft for review and calibration; not frozen,
  not hashed, not bound to any preregistration**
- Drafted: 2026-08-26, from repository state at `b649eba` (documentation
  only; no source, TSV, manifest, gate, finding, or protocol file was changed)
- Governs (once frozen): the two independent annotation passes and the
  adjudication of the Legends (`leg`) development export, under
  `docs/protocol/structural-investigation-protocol.md` v1.0 and
  `docs/findings/leg-structural-audit-preregistration.md`
- Non-observation statement: this draft was prepared from the protocol, the
  preregistration, the gate records, the committed `lea`/`leb`/`arn`/`atq`
  audit files, the live kind/role/source vocabulary in `src/segment.rs`, the
  export/metrics script contracts, and `Magic-Comprehensive_Rules.md`. No
  Legends card or export row and no held-out card was queried, opened,
  quoted, or annotated. Every real example below is a committed, non-held-out
  `lea`/`leb`/`arn`/`atq` row cited by audit path and `(name, index)`.

## 1. Status, purpose, and authority

**Purpose.** Turn protocol S5 / S7 / S13 and schema 4.2 into a row-by-row
codebook that two annotators can apply to the same frozen export and reach
the same answer for the same reason. The guide changes no methodology, no
hypothesis, no denominator, no stop condition, and no topology; it supplies
operational conventions where the protocol is terse.

**Authority order inside the annotation** (protocol S7, `docs/current-state.md`):
Comprehensive Rules > current Oracle text > official rulings > corpus
measurements > literature > agent interpretation. A ruling supports a
disposition but cannot override the CR; an interpretation without CR or
ruling support is `adjudicate`.

**Precedence among documents.** The protocol is methodologically
authoritative. The preregistration fixes hypotheses, measurements, topology,
and stop conditions. This guide is subordinate to both. **If this guide and
the protocol or preregistration appear to conflict: stop on that row, apply
the protocol wording, record the conflict in `note` as `GUIDE-CONFLICT:
<section> — <one line>`, disposition the row `adjudicate`, and report the
conflict after the pass is sealed. Do not silently follow the guide, and do
not edit the guide mid-pass** (preregistration §11.2).

**What this guide is not.** It defines no semantic operator, effect
representation, IR, parser, or behavioral-correctness criterion. `kind`,
`role`, and `source` are structural labels on surface form; agreeing that a
unit is a `triggered_ability` says nothing about what it does.

**Labelling of conventions.** Paragraphs marked **[Convention — new in this
guide]** make explicit a rule that the protocol leaves open; each is listed
again in §16 for calibration. Paragraphs marked **[Retrieved]** restate a
protocol, preregistration, CR, or committed-audit fact. Items the guide
deliberately cannot settle are in §17.

## 2. Annotation invariants

1. **Annotate structure, not meaning.** The question is always "which CR
   category does this span's *surface form* belong to, and is the span
   exactly one reference unit?" — never "what does it do?" or "how would a
   parser want this?"
2. **Read the whole card first.** Before dispositioning any row of a card,
   read every row of that card (all faces), the `type_line` column, and the
   full Oracle text as reconstructed from the rows. Read rulings when the
   disposition is not obvious from the CR (S5.1).
3. **Cite in authority order.** CR first; ruling only to clarify; never a
   corpus count as authority for a row.
4. **Preserve uncertainty and gaps.** `unsure`, `adjudicate`, `ambiguous`,
   `unsupported`, and `gap:<class>` are correct answers. A guess is not.
5. **No card-specific rules.** A row may depend on a card-specific ruling
   (`context = card_specific`), but no annotation may propose or presuppose
   a rule that names a card, set, or Oracle ID.
6. **No implementation talk.** During an independent pass, `note` records
   evidence and reasoning only — never "the segmenter should…", never a
   proposed regex, never a proposal id for a fix (preregistration §7.2).
7. **Never edit structural columns.** The export columns (`set` … 
   `normalized`) are frozen; the comparison tool refuses agreement on any
   structural drift. Add only the 13 annotation columns.
8. **Report exposure immediately.** If a held-out identity (protocol §6.3,
   plus the incident registry) appears in your view, stop and report; do not
   continue the pass (preregistration §11.2).

## 3. Mandatory row-decision sequence

Apply in this order to every row. Do not skip a step because an earlier one
"already decided" the row.

| # | Step | Output |
|---|---|---|
| 1 | Establish context: read all rows of the card, its type line(s), and reminder-stripped text; note face boundaries and quoted strings. | — |
| 2 | Identify the **reference units** of the card (§5.1) and map each emitted row onto them. | mental map |
| 3 | Judge **boundary** and parentage for this row (§5.2–5.4). | `boundary` |
| 4 | Count **missed** (§5.3). | `missed` |
| 5 | Decide whether kind/role are **eligible** for the denominators: they are iff `boundary = ok`; record them regardless (§4, `kind_ok`). | — |
| 6 | Judge `kind_expected`, `kind_ok`, `role_ok`, `source_ok` (§4, §6, §7). | four fields |
| 7 | Choose the **minimum sufficient context** (§8). | `context` |
| 8 | Record CR authority (§9). | `cr_ref` |
| 9 | Apply **structure tags** and **normalization flags** (§11). | `structure_tags`, `norm_issue` |
| 10 | Select the **disposition** by the decision tree (§10). | `disposition` |
| 11 | Write `note` if required (§4 `note`), using the note prefixes of §4. | `note` |
| 12 | Run the row-completeness check (§15, per-row items). | — |

## 4. Field-by-field codebook

**[Retrieved]** The annotated TSV = the 15 export columns
(`set, oracle_id, name, type_line, index, parent_index, depth, face, line,
kind, role, source, rule, text, normalized`) + the 13 annotation columns
below, one row per unit, stable key `(oracle_id, face, index)`. The eight
**judgement fields** compared for preregistered agreement (H8) are
`boundary, missed, kind_expected, kind_ok, role_ok, source_ok, context,
disposition`; `cr_ref`, `structure_tags`, `note` are reported separately;
`norm_issue` and `annotator` are not compared.

Values are exact, lowercase, no surrounding spaces. Multi-valued fields use
`;` with no spaces.

### 4.1 `boundary`

| Value | Definition |
|---|---|
| `ok` | The row's span (its `text`, minus any child spans that were emitted as their own rows) equals exactly one reference unit. |
| `under` | The span contains ≥ 2 reference units (or one reference unit plus a created delayed trigger that should be its own child). |
| `over` | The span is a fragment: less than one reference unit (a bare trigger condition, bare cost, half an effect, a quoted-text fragment). |
| `misattached` | The span is right, but `parent_index` is wrong: wrong parent, a child that should be top-level, or a top-level unit that should be a child of an emitted sibling. |
| `unsure` | The annotator cannot decide among the above after CR and rulings. |

Prerequisites: judged on every row, printed and rules-supplied. Interactions:
`kind_ok`/`role_ok` enter accuracy denominators only when `ok`; `missed` is
non-zero only when `under`.
Positive example: `lea` Wall of Air #1 `flying` → `ok` (one keyword ability, CR 702.9).
Common misuse: marking a parent `over` because a *correct* child was split
out of it (a parent that still equals one ability minus its child is `ok`);
marking a unit `under` because it has several sentences (an activated
ability's instructions are inside it, CR 602.1b).

### 4.2 `missed`

Non-negative integer. `0` unless `boundary = under`. Definition (§5.3): the
number of reference units inside this span that received no emitted row of
their own. Positive example: `atq` Tawnos's Coffin #1 → `under`, `missed = 1`
(one created delayed trigger inside the activated ability). Common misuse:
counting the row's own reference unit; counting missed *slots* (cost,
instruction) that are not reference units; putting a number on an `over`
row.

### 4.3 `kind_expected`

| Value | Use |
|---|---|
| one of the ten kinds in §6 | the kind the reference unit should carry |
| `n/a` | the row is a `mode` child (kind is not judged on modes, S5.5) |
| `gap:<class>` | the correct annotation is not expressible in the vocabulary (§10.3); `<class>` is `kind:<slug>`, `role:<slug>`, or `span:<slug>` |
| `unsure` | the annotator cannot decide |

Always filled, even when `boundary ≠ ok` (**[Convention — new in this guide]**
C1: for `under`, judge the reference unit that *starts* the span; for
`over`, the reference unit the fragment belongs to; for `misattached`, the
row's own unit). Positive: `lea` Animate Dead #3 → `keyword_ability`
(quoted Enchant ability, CR 702.5a). Misuse: writing the emitted `kind` back
when it is wrong; using `gap:` for a class the residual kind covers (§6.11).

### 4.4 `kind_ok`

`yes` (emitted `kind` = `kind_expected`), `no`, `n/a` (mode child), `unsure`
(`kind_expected = unsure`). For `gap:` rows, `kind_ok = no`. Enters the
accuracy denominator only when `boundary = ok` and value ∈ {`yes`,`no`}.
Misuse: `n/a` on a `granted` or `delayed_trigger` child (their kind is judged
normally, S5.5); `yes` on a mode-child row.

### 4.5 `role_ok`

`yes`, `no`, `unsure`. No `n/a` exists. Judged against §7. Enters the
denominator only when `boundary = ok`. Positive: `arn` Sandals of Abdallah #1
(`delayed_trigger` child) → `yes`. Misuse: `no` on a `misattached` row's
role when the role label itself is right (record `misattached` in
`boundary`, keep `role_ok` as the label judgement).

### 4.6 `source_ok`

`yes`, `no` — no `unsure`. `printed` is right for any span that contains
non-reminder Oracle text; `rules_supplied` is right only for a
reminder-only line (parenthetical kept in `text`) that describes an ability
the CR supplies (e.g. CR 305.6 basic-land mana ability). If genuinely
undecidable, set `source_ok = yes` only if the emitted value is defensible
and put the doubt in `note` with `disposition = adjudicate`.

### 4.7 `context`

`none`, `cr`, `type_line`, `game_state`, `card_specific` — exactly one value,
chosen by §8. Misuse: recording what was *consulted* rather than what was
*required*; `cr` on every row because "everything is in the CR".

### 4.8 `cr_ref`

`;`-separated rule ids in CR form (`113.3c`, `603.7a`, `702.9`); no spaces,
no `?`, no prose, no glossary words. Policy in §9: never blank. Positive:
`lea` Rock Hydra #1 → `615.1a;604.1`. Misuse: `113.6e?` (doubt belongs in
`note` + `adjudicate`); citing a section number alone (`603`) when a subrule
decides the row.

### 4.9 `structure_tags`

`;`-separated tags from the frozen v1.0 vocabulary only (§11.1). Mandatory
tags are listed in §11.2; others are optional observations. Misuse: any tag
outside the list (historical audits used many ad-hoc tags — they are not
authorized for Legends); omitting a mandatory tag; using a tag to carry a
disposition.

### 4.10 `norm_issue`

Blank, or `;`-separated values of the form `collision:<tag>` /
`fragmentation:<tag>` from the recognized classes in §11.4. Suspicion only;
never affects `disposition`.

### 4.11 `disposition`

`accept`, `defect`, `unsupported`, `ambiguous`, `adjudicate` — chosen by the
decision tree in §10; never by a tag or a normalization flag. Every row gets
exactly one.

### 4.12 `annotator`

One identifier per pass, identical on every row of the pass, matching the
identity on the independence attestation
(`docs/gates/legends-entry-record.md` §5.1). Never `;`-concatenated in a
pass file (concatenation is reserved for the adjudicated file).

### 4.13 `note`

Free text; **required** for every non-`accept` row and for every row that
§8, §9, §11, or §12 says needs one; optional otherwise. Start each distinct
statement with a prefix so notes are scannable and countable
(**[Convention — new in this guide]** C13):

| Prefix | Use |
|---|---|
| `REF:` | why the span is/isn't a reference unit (boundary reasoning) |
| `KIND:` / `ROLE:` / `SRC:` | reasoning for that field when not surface-obvious |
| `CTX:<value>` | additional contexts that were also required (§8) |
| `RULING:<YYYY-MM-DD>` | the official ruling relied on (date, no quotation needed) |
| `AMBIG:` | competing readings, each with its CR basis (required for `ambiguous`) |
| `GAP:` | the smallest missing structural distinction (required for `unsupported`) |
| `D19:<class>` / `D14:<class>` | attachment evidence class from §12 |
| `SPAN:` | representation limits (e.g. non-contiguous parent) |
| `PROPOSED_TAG:<slug> — <definition>` | tag proposal (§11.3) |
| `GUIDE-CONFLICT:` | §1 conflict report |
| `UNSURE:` | what would resolve an `unsure` field |

Never in `note`: implementation proposals, regexes, references to the other
annotator, held-out identities.

## 5. Boundary and attachment decision rules

### 5.1 Reference units **[Retrieved, protocol §1]**

A reference unit is one of:

- one ability under CR 113.3a–d (spell ability, activated, triggered,
  static), including everything the CR places *inside* it: activation
  instructions (602.1b), intervening-if clauses (603.4), instructions after a
  trigger's effect (603.1), reminder text (removed), and — under the accepted
  P-ATQ-1 rule — a delayed trigger created inside a *single sentence*;
- one non-ability element that Oracle prints on its own: a mode (700.2), a
  mode header, an additional-cost sentence (118.8), a cast restriction
  (604.6/113.6e), an ante instruction (407.3), a rules-supplied ability;
- one delayed triggered ability created by a preceding effect and printed as
  its own sentence(s) (603.7) — a child;
- one quoted ability that another unit grants, gains, loses, or refers to —
  a child.

A keyword list (`Flying, trample`) is *n* reference units (one per keyword,
CR 702.1); each split item is its own row.

### 5.2 Decision table

| Observation about the row's span | `boundary` | `missed` |
|---|---|---|
| Span = exactly one reference unit, correct parent | `ok` | 0 |
| Span = one reference unit *minus* correctly emitted children (mode, delayed trigger, quoted ability) | `ok` | 0 |
| Span holds 2+ abilities (e.g. two independent triggers in one paragraph, D14) | `under` | n−1 |
| Span holds an ability *and* a created delayed trigger printed as its own sentence(s) that was not emitted as a child (D19 class, `at the beginning of each…`, unscoped `When…`) | `under` | number of such unemitted triggers |
| Span holds an ability whose *single sentence* embeds a created delayed trigger (P-ATQ-1 kept it whole) | `under` | 1 |
| Span is a bare trigger condition, bare cost, partial effect, or quoted-text fragment | `over` | 0 |
| Span is right but `parent_index` names the wrong unit, or the unit should be top-level / should be a child | `misattached` | 0 |
| Cannot decide | `unsure` | 0 |

**[Convention — new in this guide]** C2 — *one defect, several rows.* Each
emitted row is judged on its own span. When a defect produces more than one
row (e.g. an `over` parent plus a correctly bounded child; two fragments of
one ability), every fragment row is `over`/`defect`, a correctly bounded
sibling stays `ok`, and the **first** affected row's `note` lists the sibling
indices (`REF: fragment; sibling #2 is the correct child`). Precedent:
`docs/audits/atq/units-annotated-pass2.tsv` Battering Ram #1 (`over`) / #2
(`ok`).

**[Convention — new in this guide]** C3 — `missed` counts reference units,
not slots. An activation cost, an activation instruction, an intervening-if,
or an in-sentence delayed-trigger *phrase* that the CR places inside the
ability is not a missed unit; the whole-sentence created delayed trigger
under P-ATQ-1 is (protocol's D15 slot, recorded as `under`, `missed 1`).

### 5.3 Effect of a boundary failure on other fields

- `kind_expected`, `kind_ok`, `role_ok`, `source_ok` are still filled (C1).
- They are excluded from kind/role accuracy denominators automatically
  (`audit_metrics.py` gates on `boundary = ok`); `source_ok` counts on all
  rows.
- `disposition` is `defect` when the correct boundary is expressible and
  cited (§10); `unsupported` when the export cannot represent it (§10.3);
  `ambiguous`/`adjudicate` otherwise.
- Structural exact-card correctness fails for the whole card on any
  `boundary ≠ ok` or `missed > 0`.

### 5.4 Top-level versus child; parent integrity

- Top-level: `parent_index` empty. Child: `parent_index` = the index of an
  earlier unit **on the same face**. A parent on another face, a parent
  index ≥ the child's index, or a missing parent is an export defect:
  report it (preregistration §7.3), disposition the row `adjudicate`, note
  `REF: export integrity`.
- Children may originate on a *later line* than their parent (a delayed
  trigger printed as its own line attaches to the preceding unit). That is
  not misattachment.
- A child's correct parent is the unit whose resolution *creates* it (delayed
  trigger), whose text *contains* it (quoted ability), or whose header it
  belongs to (mode). If the creating/containing unit is a different emitted
  row than `parent_index`, the row is `misattached`.

### 5.5 Modes and mode headers **[Retrieved + C4]**

- A modal ability is one reference unit *plus* one reference unit per mode.
  The header row (`Choose one —`, `{2}: Choose one —`, `When …, choose one —`)
  is the ability; `•` rows are `mode` children of it.
- **[Convention — new in this guide]** C4: the header row's `kind` is the
  kind of the whole modal ability (spell text → `spell_or_static_text`;
  `[Cost]:` header → `activated_ability`; trigger header →
  `triggered_ability`), judged normally, cite `700.2` (+ `602.1` / `603.1`).
  Precedents: `lea` Blue Elemental Blast #0, `arn` Pyramids #0.
- Mode rows: `kind_expected = n/a`, `kind_ok = n/a`, `role_ok` judged, cite
  `700.2`. The export serializes *some* kind on a mode row (e.g.
  `prevention_effect` on `lea` Healing Salve #2); ignore it.
- A mode that itself contains a delayed trigger, quoted ability, or
  `instead` clause stays one mode row; the inner structure is noted, not
  split (kind is `n/a` anyway).

### 5.6 Granted / quoted abilities

- Quoted text that is an ability (has a cost colon, a trigger word, or reads
  as a static statement) is a `granted` child; the parent's span is the rest
  of its text with `"[ability]"` in the template. Both parent and child are
  `ok` when that mapping holds.
- Quoted text that is *not* an ability (labels such as `"left"`/`"right"`,
  named counters, a quoted word) stays inside the parent; tag
  `short_quote_not_ability` on the parent (`lea` Raging River #0).
- A quoted ability emitted as a *top-level* row is `misattached`; a quoted
  ability not emitted at all leaves the parent `under`, `missed 1`.

### 5.7 Rules-supplied units

A reminder-only line is one reference unit when the CR supplies the ability
it describes (CR 305.6 basic-land mana abilities are the only cited form
today). `boundary` is judged like any other row; `source_ok` per §4.6; kind
judged normally (`activated_ability` for the mana form); `context =
type_line` (the type line, not the parenthetical, is what makes the ability
exist); cite the supplying rule (`305.6`). Precedent: `lea` Badlands #0.

### 5.8 Delayed-trigger parents and children

| Shape (accepted topology, preregistration §10) | Parent row | Child row |
|---|---|---|
| Nested: effect text, then a created delayed trigger as its own sentence(s) | `ok` if the remainder is one ability; tag `delayed_trigger_parent` | `ok`; `role = delayed_trigger`; `parent_index` = creator |
| Top-level spell-created (instant/sorcery whose unit *is* the created trigger, P-ATQ-4) | — | top-level, no parent, `role = delayed_trigger` |
| Quoted delayed trigger inside granted text | granting unit | `role = granted` (never promoted) |
| Single sentence embedding a created trigger (P-ATQ-1 kept whole) | `under`, `missed 1`, D15 slot | (none emitted) |
| Independent later trigger sharing a paragraph (D14) | `under` if merged; both `ok` if split as siblings | not a child |

A parent that becomes **non-contiguous** because the child sat between its
effect and its activation instruction is `ok` (the span minus the child is
one ability); write `SPAN: non-contiguous parent (T2/T8)` in `note`
(precedent: `atq` Rocket Launcher #0). See §17 U3.

## 6. Kind codebook

**[Retrieved]** The live vocabulary (`src/segment.rs`, `AbilityKind`
serialization) is exactly: `keyword_ability`, `activated_ability`,
`triggered_ability`, `replacement_effect`, `prevention_effect`,
`cast_restriction`, `additional_cost`, `characteristic_defining_ability`,
`ante_instruction`, `spell_or_static_text`. No other kind may be written
in `kind_expected`; a missing class is `gap:kind:<slug>` (§10.3).

| Kind | Structural meaning (surface form) | CR anchors | Required context | Nearest confusing class → how to tell |
|---|---|---|---|---|
| `keyword_ability` | A bare keyword (or keyword + parameter: `Enchant land`, `Protection from red`, `Banding`), reminder text removed; list items split one per row | 702.1, the keyword's own 702.x (702.5a Enchant, 702.9 Flying, 702.19 Trample, 702.22 Banding …) | `cr` (a numbered rule is what makes the word an ability) | Short static sentence without a period labelled keyword (known heuristic leak) → `spell_or_static_text`; ability word (207.2c) is not a keyword — kind is the body's kind; quoted keyword with trailing period (`lea` Animate Dead #3) is still `keyword_ability` |
| `activated_ability` | `[Cost]: [Effect.] [Instructions.]` — everything before the colon is cost (602.1a), instructions (`Activate only …`) are inside (602.1b); includes mana abilities (605.1a, tag `mana_ability`) | 113.3b, 602.1, 602.1a–b, 605.1a | `none` (colon form) | `{T}`/cost symbol inside a *trigger condition* is not a cost → `triggered_ability`; quoted `[Cost]:` text belongs to a `granted` child, not the granting static |
| `triggered_ability` | `When/Whenever/At [event], [effect]`, including inverted delayed forms (`Destroy it at the beginning of the next end step`), state triggers (603.8), intervening-if (603.4), triggered mana abilities (605.1b), Saga chapter symbols (714.2b) | 113.3c, 603.1, 603.4, 603.7, 603.8, 603.12, 714.2b | `none` (trigger word); `cr` when the trigger word is hidden by a prefix or inversion | A `when`/`if` clause *inside* an effect (603.1 instructions) is not a separate trigger; `If … would …, instead` is `replacement_effect` |
| `replacement_effect` | Static text using `instead`, `skip`, `enters with`, `As ~ enters`, `enters tapped`, `enter as a copy` | 614.1, 614.1a, 614.1c, 614.12 | `none` on a permanent; `type_line` when the same wording sits at top level on an instant/sorcery face (then it is spell text, 113.3a — P-ARN-3) | `instead` *inside* an activated/triggered effect keeps that kind (tag `instead_in_activated`); conditional mana (`add {C}{C} instead`) inside a mana ability is `activated_ability` (`atq` Urza's Mine #0) |
| `prevention_effect` | Static text that *performs or establishes* prevention with the word `prevent` | 615.1, 615.1a, 604.1 (+ 615.5 when it has an extra effect) | `cr` | `can't/cannot be prevented` is a prohibition (615.12; P-ATQ-2) → `spell_or_static_text`; `Prevent …` inside an activated/triggered/spell unit keeps that unit's kind; a `Prevent …` mode is `n/a` |
| `cast_restriction` | `Cast this spell only [timing/condition].` printed as its own unit | 604.6, 113.6e, 506.7 | `none` | `Activate only …` is inside an activated ability (602.1b); restrictions on *other* objects' play are residual statics |
| `additional_cost` | `As an additional cost to cast this spell, …` printed as its own unit | 118.8, 601.2b | `none` | Alternative costs (118.9), `costs {N} more/less` (113.6d) → `spell_or_static_text` + tag `cost_modification`; non-mana activation costs are inside `activated_ability` |
| `characteristic_defining_ability` | Unconditional statement defining P/T, color, subtype, etc. of the object itself (`~'s power and toughness are each equal to …`) | 604.3, 604.3a | `type_line` (the defined characteristic must be one the object has) | Conditional value-setting (`As long as …`, 604.3a(5)) is not an established CDA → `ambiguous` if CR/rulings do not settle it (`lea` Gaea's Liege #0); `gets +N/+N` continuous effects are residual statics; `As ~ enters, it becomes …` is `replacement_effect` |
| `ante_instruction` | The fixed sentence `Remove this card from your deck before playing if you're not playing for ante.` | 407.3 | `none` | Ante *effects* in spell text (`… puts a card from their deck into the ante`) are `spell_or_static_text` |
| `spell_or_static_text` | Residual: spell abilities of instant/sorcery faces (113.3a) and static abilities (113.3d, 604.1) not captured above — including the **residual-accepted classes** of §6.11 | 113.3a, 113.3d, 604.1 (+ the class rule) | `none`; `type_line` when the instant/sorcery gate decided it | Everything; decide by exclusion in the order of §6.12 |

### 6.11 Residual-accepted classes **[Convention — new in this guide]** C9

`spell_or_static_text` is the *correct* label — not a forced one — for these
classes, because the protocol's frozen tag vocabulary names them as
structures to tag (4.4) or an accepted proposal routes them there:

| Class | Tag / basis | Cite |
|---|---|---|
| Cost modification (`costs {N} more/less to cast`) | `cost_modification`; `lea` Fireball #0 | 113.6d |
| Payment restriction (`Spend only … mana on …`) | `payment_restriction` | 113.6e (if CR search finds nothing more specific, cite 113.6e and note `KIND: residual`) |
| Prevention prohibition (`… can't be prevented`) | P-ATQ-2 acceptance | 615.12 |
| Player-control, text-change, physical-action spell text | `player_control`, `text_change`, `physical_action` | 113.3a |
| Static that grants/loses/refers to a quoted ability | `granted_quoted_parent` | 604.1 (+ 113.3d) |
| Other static statements with no accepted kind (play restrictions on other objects, `can't be enchanted`, conditional `+N/+N`) | (none) | 604.1 |

Everything else that is a *CR-named ability or effect category with its own
numbered definition* and no accepted kind is `unsupported` (§10.3).

### 6.12 Order of exclusion for `kind_expected`

1. Mode child → `n/a`.
2. Ante sentence (407.3) → `ante_instruction`.
3. `As an additional cost to cast this spell` → `additional_cost`.
4. `Cast this spell only` → `cast_restriction`.
5. Saga chapter symbol on a Saga type line → `triggered_ability`.
6. Strip a structural prefix (`<label> —`, ≤ 45 chars, no `.`/`:`) mentally
   and classify the body (P-ATQ-3; 207.2c–d).
7. Bare keyword / keyword list item (702) → `keyword_ability`.
8. `[Cost]:` at the start → `activated_ability`.
9. `When/Whenever/At` at the start, or an inverted delayed form → `triggered_ability`.
10. Top-level on an instant/sorcery face → `spell_or_static_text` (113.3a),
    regardless of `instead`/`prevent`/CDA wording (P-ARN-3; H6).
11. Static with `prevent` (not `can't be prevented`) → `prevention_effect`.
12. Static with `instead`/`skip`/enters-with/as-enters → `replacement_effect`.
13. Unconditional characteristic definition → `characteristic_defining_ability`.
14. Otherwise → `spell_or_static_text`, or `gap:kind:<slug>` if §10.3 applies.

## 7. Role, source, and topology codebook

**[Retrieved]** Live `role` values: `ability`, `mode`, `delayed_trigger`,
`granted`. Live `source` values: `printed`, `rules_supplied`.

| Role | Definition | Parent | `role_ok = no` when |
|---|---|---|---|
| `ability` | An ordinary top-level ability, spell text, mode header, or non-ability printed element of the card; also an ordinary *independent* triggered ability (D14 sibling) | none | the unit is actually a mode, a created delayed trigger, or quoted text |
| `mode` | A `•` option of a modal ability | the header row | the bullet line is not a mode (e.g. a list of *effects* under a non-modal instruction) |
| `delayed_trigger` | A triggered ability created by the resolution of the parent's effect (603.7a–h) **or** by the resolution of the spell that is the unit itself (top-level, 603.7d) | creator, or none (top-level spell-created) | the trigger exists independently of any resolution (D14/N1), is a cast/resolve trigger of the spell, or functions off the stack (113.6b) |
| `granted` | A quoted ability that the parent grants, gains, **loses, or refers to** | containing unit | the quoted text is not an ability, or the child is a created delayed trigger rather than quoted text |

**[Convention — new in this guide]** C5 — `granted` covers *every* quoted
ability child regardless of whether the parent gains or loses it (the live
enum documents "grants, gains, or refers to"). Record the relation in `note`
(`ROLE: lost`, `ROLE: referenced`). This differs from the Alpha row Animate
Dead #2, which used `role_ok = unsure` / `adjudicate` for a lost ability;
see §17 U5.

**Topology (preserved from preregistration §10, H2)**:

| Case | `role` | `parent_index` | Distinguishing evidence |
|---|---|---|---|
| Top-level spell-created delayed trigger | `delayed_trigger` | empty | instant/sorcery face; the unit is the whole created trigger (`this turn` / `this combat` / `next …` scope); not about the spell's own casting/resolution; no off-stack evidence (113.6b) |
| Nested effect-created delayed trigger | `delayed_trigger` | valid, same face | preceding effect text creates it (603.7a/e/h); ruling may confirm |
| Quoted/granted delayed-trigger text | `granted` | valid | inside quotation marks |
| Ordinary independent triggered ability | `ability` | empty | complete 603.1 form; exists without any resolution (D14) |
| Off-stack ability printed on an instant/sorcery | `ability` | empty | cycling/suspend/haunt-class keyword, or self-reference in a graveyard/exile/discard zone (113.6b) |
| Cast/resolve trigger of the spell | `ability` | empty | `When you cast ~`, `~ resolves`, `~ is countered` |

**Source**: `printed` = the span contains non-reminder text. `rules_supplied`
= a reminder-only line describing a CR-supplied ability (`text` keeps the
parentheses; `rule` may carry the citation). `source_ok = no` when a reminder-
only line does *not* describe a rules-supplied ability, or when printed text
was flagged rules-supplied.

## 8. Context-selection convention

**[Retrieved]** Protocol S5.7 defines the five values; the schema stores one.
Committed practice is consistent on: keyword rows → `cr` (all 120 keyword
rows in `lea`/`arn`/`atq`); rules-supplied basic lands → `type_line`; CDAs →
`type_line`; type-line-gated instant/sorcery spell text → `type_line`;
conditional delayed-trigger creation → `game_state`; modes → `none`. It is
*inconsistent* on delayed-trigger children (`lea` Berserk #2 `none` vs `atq`
Rakalite #1 `cr`).

**Evaluation of the candidate rule** `card_specific > game_state > type_line
> cr > none`: adopted, with the criterion **"strongest additional context
actually *required* to reach the disposition"**, not merely consulted.
Reasons: (i) `card_specific` must never be masked — it is a preregistered
measurement and must be 0 in any accepted heuristic; (ii) `game_state` marks
units whose existence or shape text + rules cannot fix, which must not be
hidden behind a type-line dependency; (iii) `type_line` records a
characteristic dependency that a text-only classifier cannot see, whereas
`cr` records rules knowledge every classifier embeds; (iv) `none` is the
baseline. No repository evidence contradicts this ordering, and the four
committed audits are compatible with it.

**[Convention — new in this guide]** C6 — decision table (take the first row
that applies; list any *other* required context in `note` as `CTX:<value>`):

| Required to reach the disposition | `context` |
|---|---|
| A ruling about *this card* (not a generic ruling on the wording) | `card_specific` |
| The unit's existence, span, or attachment depends on game state (conditional creation of a delayed trigger, `if this ability has been activated N times`) | `game_state` |
| A characteristic from the type line or face: instant/sorcery vs permanent (kind gate), Saga, basic land type (rules-supplied), creature P/T for a CDA, Aura for Enchant | `type_line` |
| A numbered rule beyond the surface markers embedded in §6: a bare word is a keyword (702.x); a sentence is a *created* delayed trigger (603.7) rather than independent; a clause is inside the ability (602.1b, 603.1, 603.4); a prohibition is not prevention (615.12); a chapter symbol is a trigger (714.2); a quoted string without a trigger word or cost colon is (or is not) an ability | `cr` |
| The surface form alone (trigger word, cost colon, bullet, `instead`, `Prevent`, the ante sentence, `As an additional cost`, `Cast this spell only`) determines every judged field | `none` |

Consequences to apply uniformly: every `keyword_ability` row is `cr`; every
`delayed_trigger`-role row (nested or top-level) is at least `cr`; every
`rules_supplied` row is `type_line`; every `characteristic_defining_ability`
row is `type_line`; every mode row is `none` unless its header needed more.

## 9. CR-citation policy

**[Retrieved]** S5.2: an uncited `defect` is `adjudicate`. S7: "a boundary
or kind disposition must cite a CR rule." Committed practice left `cr_ref`
blank on 235/407 (`lea`), 60/108 (`arn`), 71/122 (`atq`) `accept` rows.

**[Convention — new in this guide]** C7 — **`cr_ref` is never blank.** This
satisfies S7 literally, removes the "was a citation needed?" judgement as a
disagreement source, and costs nothing because surface-obvious rows take a
fixed default:

| Row type | Minimum citation |
|---|---|
| Accepted `activated_ability` (printed, role `ability`) | `113.3b` (+ `602.1b` if it contains an activation instruction; `605.1a` if a mana ability) |
| Accepted `triggered_ability`, role `ability` | `113.3c` (+ `603.4` intervening if; `603.8` state trigger; `605.1b` triggered mana; `714.2b` chapter) |
| Accepted spell text on an instant/sorcery face | `113.3a` |
| Accepted static residual | `113.3d` or `604.1` (+ the class rule from §6.11) |
| `keyword_ability` | the keyword's 702.x subrule |
| `replacement_effect` / `prevention_effect` / CDA / cast restriction / additional cost / ante | the anchors in §6 |
| Mode header / mode | `700.2` (+ `602.1` / `603.1` for activated / triggered headers) |
| `delayed_trigger` role | `603.7` + the creating subrule (`603.7a`, `603.7d` spell-created, `603.7e` ability-created, `603.7h` conditional on the creating ability) |
| `granted` role | the child's own kind rule; parent additionally `604.1` |
| `rules_supplied` | the supplying rule (`305.6`) |
| Any `boundary ≠ ok` | the rule that defines the reference unit (`113.3b/c`, `602.1a`, `603.1`, `603.7a`–`603.7h`, `700.2`) |
| Any non-`accept` disposition | every rule that bears on the competing readings; `ambiguous` must cite the rule that fails to decide |
| Any `context ≠ none` | the rule that made the context necessary |

Rulings are cited in `note` (`RULING:<date>`), never in `cr_ref`. Cite the
most specific subrule that decides the row; add the parent rule only when it
adds information. Verify every id exists in `Magic-Comprehensive_Rules.md`
(`mtg-discover rules show <id>` or a text search); an unverifiable id is
recorded as `UNSURE: citation` in `note` with `adjudicate`.

## 10. Disposition and uncertainty decision tree

### 10.1 Tree (apply top-down; first match wins)

1. Any judged field is `unsure` **and** the annotator believes the CR or
   rulings *could* decide it with more work → **`adjudicate`**
   (`UNSURE:` note says what would resolve it).
2. The CR and rulings, fully consulted, *do not determine* one structural
   reading (two readings each with CR support) → **`ambiguous`**
   (`AMBIG:` note lists each reading with its rule; judged fields may be
   `unsure`; `boundary` may be `unsure`).
3. The correct boundary, kind, role, source, or attachment is **not
   expressible** in the frozen vocabulary or export shape → **`unsupported`**
   (`kind_expected = gap:<class>`, `kind_ok = no`, `GAP:` note; §10.3).
4. Any of `boundary ∈ {under, over, misattached}`, `kind_ok = no`,
   `role_ok = no`, `source_ok = no`, with the correct value expressible and
   `cr_ref` cited → **`defect`**.
5. Same as 4 but the annotator cannot cite a rule → **`adjudicate`** (S5.2).
6. `boundary = ok`, `kind_ok ∈ {yes, n/a}`, `role_ok = yes`,
   `source_ok = yes`, nothing `unsure` → **`accept`**.

`adjudicate` is always temporary: the adjudication may convert it to any
other value but the pass file keeps it.

### 10.2 Case table

| Situation | Disposition | Fields |
|---|---|---|
| Representable classifier error (wrong kind/role/source, fixable boundary) | `defect` | correct values; cite |
| Missing vocabulary distinction (a CR-named category with no kind; a role/attachment the schema cannot express) | `unsupported` | `gap:…`; `GAP:` note |
| True ambiguity after CR + rulings | `ambiguous` | competing readings; `unsure` where needed |
| Temporary annotator uncertainty | `adjudicate` | `unsure` field(s); `UNSURE:` note |
| Boundary error making kind ineligible | `defect` (or `unsupported`/`ambiguous` per 10.1) | kind fields filled, auto-excluded |
| Disposition rests on a card-specific ruling | whatever the ruling supports (`accept` is allowed) | `context = card_specific`; `RULING:` note |
| Existence/shape depends on game state | usually `accept` | `context = game_state`; cite `603.7h` or the applicable rule |
| Surface form fine, but a suspected normalization problem | unaffected (usually `accept`) | `norm_issue` only |

### 10.3 `gap:<class>` classes **[Convention — new in this guide]** C9

- `gap:kind:<slug>` — the reference unit's CR category has its own numbered
  definition and no accepted kind, and is not a residual-accepted class of
  §6.11 (e.g. an emblem-creating instruction's emblem text if it ever appears
  outside quotes; a category the annotator names by its CR section).
- `gap:role:<slug>` — the child's relation to its parent is none of
  mode / created delayed trigger / quoted ability.
- `gap:span:<slug>` — the correct unit cannot be represented as one
  contiguous emitted row with a single parent (D19 class A3), *other than*
  the non-contiguous-parent shape accepted under §5.8.

Never invent a kind name; the slug describes the class (`gap:kind:emblem`,
`gap:span:interleaved_child`). A `gap:` row still records
`role_ok`/`source_ok` normally.

## 11. Structure tags and normalization issues

### 11.1 Frozen v1.0 vocabulary **[Retrieved, protocol 4.4]** — the only permitted tags

`keyword`, `keyword_list_split`, `rules_supplied`, `mana_ability`, `mode`,
`mode_header`, `granted_quoted`, `granted_quoted_parent`,
`short_quote_not_ability`, `delayed_trigger_next`, `delayed_trigger_parent`,
`delayed_trigger_inverted`, `delayed_trigger_when`,
`delayed_trigger_recurring`, `delayed_trigger_end_of_combat`,
`conditional_creation`, `activation_instruction`, `intervening_if`,
`state_trigger`, `enters_replacement`, `instead_in_spell`,
`instead_in_activated`, `prevention_static`, `cda`, `conditional_cda`,
`cast_restriction`, `cost_modification`, `payment_restriction`, `ante`,
`multi_sentence`, `name_predicate`, `self_reference_name`,
`self_reference_this_ability`, `text_change`, `physical_action`,
`player_control`, `one_off_candidate`.

The ~55 additional ad-hoc tags in the committed `arn`/`atq` files
(`remembered_choice`, `dies_trigger`, `nonmana_cost`, …) were added under
S5.8 with findings-document definitions for those sets; they are **not**
authorized for a Legends pass. Use `note` prose instead.

### 11.2 Mandatory versus optional **[Convention — new in this guide]** C10

Mandatory whenever the structure is present (their absence is a completeness
failure):

| Tag | Trigger |
|---|---|
| `keyword` | every `keyword_ability` row |
| `keyword_list_split` | a keyword row that came from a comma list (lower-case initial in `text`) |
| `rules_supplied` | every `source = rules_supplied` row |
| `mana_ability` | activated/triggered mana ability (605.1a/b) |
| `mode`, `mode_header` | mode rows / header rows |
| `granted_quoted`, `granted_quoted_parent` | quoted-ability child / its parent |
| `short_quote_not_ability` | non-ability quoted text left in a unit |
| `delayed_trigger_parent` | parent of an emitted delayed-trigger child |
| one of `delayed_trigger_next` / `_inverted` / `_when` / `_recurring` / `_end_of_combat` | every delayed-trigger unit or in-unit delayed trigger (choose by wording: `next …` phrase; effect-first inversion; sentence-initial `When/Whenever`; stated recurring duration; `at end of combat`) |
| `conditional_creation` | creation of the delayed trigger is conditional on game state (`context = game_state`) |
| `activation_instruction` | `Activate only …` inside an activated ability |
| `intervening_if`, `state_trigger` | 603.4 / 603.8 forms |
| `enters_replacement`, `instead_in_spell`, `instead_in_activated` | the named forms |
| `prevention_static`, `cda`, `conditional_cda`, `cast_restriction`, `cost_modification`, `payment_restriction`, `ante` | the named classes (also on residual-accepted rows of §6.11) |
| `multi_sentence` | **automatic**: ≥ 2 sentence terminators (`.`, `!`, `?`) in `text` as exported (quoted text included) |
| `self_reference_name` | the card's own name appears in `text` |
| `name_predicate` | `named <name>` predicate over other objects |

Optional observations: `self_reference_this_ability`, `text_change`,
`physical_action`, `player_control`, `one_off_candidate` (only if the
annotator has *not* run a corpus search — record it as belief; the count
verifies it later).

### 11.3 Proposing a tag without introducing it

Write `PROPOSED_TAG:<slug> — <one-line definition>` in `note`; leave
`structure_tags` to the frozen list. The adjudication collects proposals
into the findings document; a tag becomes usable only in a later guide
version.

### 11.4 `norm_issue` **[Convention — new in this guide]** C11

Recognized classes (the only ones used in committed audits) and their
mechanical surface tests on `text`:

| Value | Test |
|---|---|
| `collision:tap_as_mana` | `{T}` or `{Q}` appears (the template collapses it to `{M}`) — apply even when the symbol sits in a trigger condition; it is a *suspected* collision |
| `fragmentation:land_type` | a basic land type word (Plains/Island/Swamp/Mountain/Forest, singular or plural) appears |
| `fragmentation:color_word` | white/blue/black/red/green appears as a word |
| `fragmentation:object_type` | a card type or subtype word varies an otherwise identical template (e.g. `Enchant land` vs `Enchant creature`) — annotator judgement; optional |

Multiple values are `;`-joined (committed precedent; the metrics script
splits on `;`). *Suspected* ≠ *verified*: a flag records that two units
might share or fail to share a template for normalization reasons; verified
collision requires two units with different CR semantics sharing a template,
and verified fragmentation requires an ablation. Neither the flag nor a
suspicion changes `disposition`.

## 12. D14 / D19 handling

**[Retrieved]** D19: a later *unscoped* sentence-initial `When/Whenever/At`
may be a delayed trigger created by the preceding effect → child. D14: it may
be an independent triggered ability sharing a paragraph → sibling/top-level.
Sentence adjacency decides neither (preregistration §10, H9;
`docs/findings/d19-attachment-research-design.md` §3–4). P-ATQ-1 (in-sentence
comma/colon) is a third, separate question.

Rules for a pass:

1. When a unit contains a later trigger-word sentence lacking the supported
   scoped markers (`this turn`, `this way`, `When you do`, `next …`), it is a
   **candidate**. Record `D19:<class>` or `D14:<class>` in `note` using the
   design's classes (P1–P5 positive; N1–N8 negative; A1–A5 ambiguous/
   unsupported), citing the CR question that decided it (603.7a/b/c, 603.1,
   113.6, 603.12, 607).
2. If permitted evidence (CR, Oracle text, official ruling) establishes
   creation (P1–P4): `boundary = under`, `missed = 1`, `defect`, tag
   `delayed_trigger_when`; if the ruling is card-specific, `context =
   card_specific` (precedents: `atq` Tawnos's Coffin #1, `lea` Animate Dead #1).
3. If evidence establishes independence (N1, N2): the merged unit is `under`,
   `missed = n−1`, `defect`, note `D14:N1`; if already split as siblings,
   both `ok`.
4. If evidence cannot decide (A1, A2, A4, A5): `boundary = unsure`,
   `disposition = ambiguous`, `AMBIG:` note with both readings. If the
   export could not represent the correct split (A3): `unsupported`,
   `gap:span:…`.
5. If the annotator simply ran out of evidence-gathering time:
   `adjudicate`, `UNSURE:`.
6. **No adjacency heuristic, no proposal.** Do not write "split after the
   first sentence" or any generic rule in `note`. Candidate evidence is
   descriptive; the D19 design's S8 search runs later.

## 13. Worked examples

Real rows cite `docs/audits/<set>/units-annotated.tsv` unless stated;
`(name #index)` is the stable identity within that set. Synthetic wording is
marked **[synthetic]** and belongs to no card. Only fields that matter are
shown; the H8 judgement fields are in the order
`boundary / missed / kind_expected / kind_ok / role_ok / source_ok / context / disposition`.

1. **Ordinary spell text** — `lea` Disintegrate #0 (`… exile it instead.`,
   Sorcery). `ok / 0 / spell_or_static_text / yes / yes / yes / type_line /
   accept`; `cr_ref 113.3a;614.1a`; tags `instead_in_spell;multi_sentence`.
   The type line is what stops `instead` from meaning replacement.
2. **Ordinary static** — `lea` Pirate Ship #0 (`This creature can't attack
   unless …`). `ok / 0 / spell_or_static_text / yes / yes / yes / none /
   accept`; under C7 cite `604.1`; `norm_issue fragmentation:land_type`.
3. **Keyword ability** — `atq` Clockwork Avian #0 `Flying`. `ok / 0 /
   keyword_ability / yes / yes / yes / cr / accept`; `702.9`; tag `keyword`.
4. **Keyword-list child** — `lea` Wall of Air #1 `flying` (from
   `Defender, flying`). As 3 plus tag `keyword_list_split`; `note KIND: split
   list item`.
5. **Activated ability with instruction** — `lea` Clockwork Beast #2
   (`{X}, {T}: … Activate only during your upkeep.`). `ok / 0 /
   activated_ability / yes / yes / yes / none / accept`; `113.3b;602.1b`;
   tags `activation_instruction;multi_sentence`; `norm_issue
   collision:tap_as_mana`. Three sentences, one reference unit.
6. **Triggered ability, state trigger** — `lea` Pirate Ship #2 (`When you
   control no Islands, sacrifice this creature.`). `ok / 0 /
   triggered_ability / yes / yes / yes / none / accept`; `113.3c;603.8`; tag
   `state_trigger`.
7. **Rules-supplied ability** — `lea` Badlands #0 `({T}: Add {B} or {R}.)`,
   `source rules_supplied`, `rule 305.6`. `ok / 0 / activated_ability / yes /
   yes / yes / type_line / accept`; `305.6;605.1a`; tags
   `mana_ability;rules_supplied`.
8. **Mode header and mode** — `lea` Blue Elemental Blast #0 `Choose one —`
   → `ok / 0 / spell_or_static_text / yes / yes / yes / none / accept`,
   `700.2;113.3a`, tag `mode_header`; #1 `Counter target red spell.` →
   `ok / 0 / n/a / n/a / yes / yes / none / accept`, `700.2`, tag `mode`.
   `arn` Pyramids #0 `{2}: Choose one —` is the activated-header form
   (`activated_ability`, `700.2;602.1`).
9. **Granted quoted ability** — `atq` Energy Flux #1 (`At the beginning of
   your upkeep, sacrifice this artifact unless you pay {2}.`, role
   `granted`, parent #0). `ok / 0 / triggered_ability / yes / yes / yes /
   none / accept`; `113.3c;603.1`; tag `granted_quoted`. Parent #0 carries
   `granted_quoted_parent` and cites `604.1`. The trigger word makes both
   the kind and the quoted-ability status surface-obvious.
10. **Under-segmentation, unscoped `When` (D19 P3)** — `atq` Tawnos's
    Coffin #1. `under / 1 / activated_ability / yes / yes / yes /
    card_specific / defect`; `602.1;603.7a;603.7e`; tags
    `delayed_trigger_when;multi_sentence`; note `D19:P3 RULING:<date>
    'its delayed triggered ability'`. (Committed row records `cr`; under C6
    a ruling *about this card* is `card_specific` — see §17 U8.)
11. **Under-segmentation, P-ATQ-1 in-sentence slot** — `atq` Battering Ram
    #1 (`Whenever this creature becomes blocked by a Wall, destroy that Wall
    at end of combat.`). `under / 1 / triggered_ability / yes / yes / yes /
    cr / defect`; `113.3c;603.7a;603.7e`; tag `delayed_trigger_end_of_combat`;
    note `REF: single sentence keeps a created delayed trigger (D15 slot)`.
12. **Over-segmentation (historical)** —
    `docs/audits/atq/units-annotated-pass2.tsv` Battering Ram #1 `Whenever
    this creature becomes blocked by a Wall,` → `over / 0 / triggered_ability
    / yes / yes / yes / cr / defect`, `113.3c;603.7a`, note `REF: bare trigger
    condition; sibling #2 is the correct child`; #2 `destroy that Wall at
    end of combat.` → `ok / 0 / triggered_ability / yes / yes / yes / cr /
    accept`. One defect, two rows (C2).
13. **Misattachment [synthetic]** — a two-line card: line 1 `{T}: Target
    creature gains flying until end of turn.`; line 2 `Choose one —` with two
    bullets; the export attaches the first `•` row to the line-1 activated
    ability instead of the header. Bullet row: `misattached / 0 / n/a / n/a
    / yes / yes / none / defect`; `700.2`; tag `mode`; note `REF: parent
    should be the header row #k`.
14. **Nested delayed trigger** — `lea` Berserk #2 (`At the beginning of the
    next end step, destroy that creature if it attacked this turn.`, role
    `delayed_trigger`, parent #1). `ok / 0 / triggered_ability / yes / yes /
    yes / cr / accept`; `603.7;603.7a;603.7d`; tag `delayed_trigger_next`.
    Parent #1: `ok`, `spell_or_static_text`, tag `delayed_trigger_parent`.
15. **Top-level spell-created delayed trigger [synthetic]** — an instant
    whose whole text is `Whenever a creature blocks this turn, it gets +0/+1
    until end of turn.` Export: top-level, `role delayed_trigger`, no
    parent. `ok / 0 / triggered_ability / yes / yes / yes / type_line /
    accept`; `603.7;603.7b;603.7d;113.3a`; tag `delayed_trigger_when`; note
    `ROLE: spell-created, top-level (P-ATQ-4 class); CTX:cr`. The same text
    on a creature is `role ability`.
16. **Prevention static versus prohibition** — `atq` Argothian Pixies #1
    (`Prevent all damage that would be dealt to this creature by artifact
    creatures.`) → `ok / 0 / prevention_effect / yes / yes / yes / cr /
    accept`, `615.1a;604.1`, tag `prevention_static`. **[synthetic]**
    `Damage that would be dealt by this creature can't be prevented.`
    (emitted `spell_or_static_text`) → `ok / 0 / spell_or_static_text / yes /
    yes / yes / cr / accept`, `615.12;604.1`, note `KIND: prohibition, not
    prevention (P-ATQ-2)`.
17. **Structural prefix [synthetic]** — `Moonrise — Whenever this creature
    attacks, it gets +1/+1 until end of turn.` (a fictional ability word,
    CR 207.2c; the segmenter records `prefix` and classifies the body). `ok / 0 /
    triggered_ability / yes / yes / yes / cr / accept`; `207.2c;113.3c`; note
    `KIND: ability-word prefix stripped before classification`. If the
    emitted kind were `spell_or_static_text`, the row would be `defect`,
    `kind_ok no`. (Note that the TSV export has no `prefix` column — §17 U2.)
18. **`unsupported` [synthetic]** — an activated ability whose effect reads
    `Exile target creature. At the beginning of the next end step, return
    it to the battlefield. Then draw a card.` The created delayed trigger
    (sentence 2) can be represented only as a child, but the parent's
    remaining text would then be two separated halves with the *child's*
    consequence (`Then draw a card`) ambiguous between parent and child; the
    contiguous single-parent export cannot express the correct spans (D19
    design class A3). Row: `unsure / 0 / gap:span:interleaved_child / no /
    yes / yes / cr / unsupported`; `602.1;603.7a;603.7e`; note `GAP: created
    trigger interleaved with parent effect; SPAN: cannot be emitted without
    discarding or reassigning text`. Contrast: a static that merely
    *describes* a structure the tags cannot name finely (e.g. a `costs {1}
    less` statement) is residual-accepted (§6.11) with `PROPOSED_TAG:` if
    wanted — `unsupported` requires a label or span the schema *cannot
    express*, not a structure it cannot describe finely.
19. **`ambiguous`** — `lea` Gaea's Liege #0 (two `As long as …` P/T
    statements). `unsure / 0 / unsure / unsure / yes / yes / type_line /
    ambiguous`; `604.3a`; tags `conditional_cda;multi_sentence;
    self_reference_name`; note `AMBIG: (a) one static with two conditional
    value-setting clauses, not a CDA per 604.3a(5); (b) two CDAs; rulings do
    not decide`.
20. **`adjudicate`** — `lea` Drain Life #0 (`Spend only black mana on X.`)
    as historically annotated: `ok / 0 / unsure / unsure / yes / yes / cr /
    adjudicate`, note `UNSURE: no numbered rule found for payment
    restriction`. Under this guide the calibrated answer is the residual-
    accepted class (§6.11: `spell_or_static_text`, `113.6e`, tag
    `payment_restriction`, `accept`); the historical row is the correct use
    of `adjudicate` when the annotator has not yet settled the citation.
21. **Field-level `unsure`** — `lea` Animate Dead #2 (`enchant creature card
    in a graveyard`, a *lost* quoted ability, role `granted`): historically
    `ok / 0 / keyword_ability / yes / unsure / yes / cr / adjudicate`. Under
    C5 the pass answer is `role_ok yes`, `accept`, note `ROLE: lost`; the
    row shows the correct mechanics of `unsure` → `adjudicate` when a role
    vocabulary question is open.
22. **Suspected collision and fragmentation** — `lea` Nightmare #1
    (`Nightmare's power and toughness are each equal to the number of Swamps
    you control.`): `norm_issue fragmentation:land_type`; `lea` Pirate Ship
    #1 (`{T}: This creature deals 1 damage to any target.`):
    `collision:tap_as_mana`. Dispositions unaffected (`accept`).
23. **D14/D19 candidate that remains observational [synthetic]** — one
    activated ability: `{2}, {T}: Tap target creature. When that creature
    becomes untapped, it deals 1 damage to you.` No ruling, no scoped
    marker. `that creature` is anaphoric to the effect (P1 evidence) but the
    later sentence is also a complete 603.1 ability (N1 shape). Row:
    `unsure / 0 / activated_ability / yes / yes / yes / cr / ambiguous`;
    `602.1;603.1;603.7a;603.7c`; note `D19:A2 AMBIG: (a) created delayed
    trigger via anaphora (603.7a/c); (b) independent trigger sharing the
    paragraph (603.1); no ruling`. No split rule is proposed.
24. **Independent trigger correctly left top-level (D14 negative)** — `atq`
    Clockwork Avian #2 (`At end of combat, if this creature attacked or
    blocked this combat, remove a +1/+0 counter from it.`). `ok / 0 /
    triggered_ability / yes / yes / yes / none / accept`; `113.3c;603.4`;
    tag `intervening_if`; note `D14:N2 recurring trigger of the object, not
    created by any effect`.

## 14. Independent-pass conduct

**[Retrieved, preregistration §7, entry record §5.1]**

1. Both annotators use the identical frozen guide version and the identical
   frozen export (hash recorded in preregistration §3). If the guide binding
   or export hash differs between passes, the passes are not comparable.
2. No communication about any row, card, hypothesis, or candidate list
   between annotators until both passes are sealed; no reading of the other
   pass, its notes, or its interim files.
3. No implementation proposals during annotation — not in `note`, not in
   side files, not in conversation.
4. Every row receives an explicit `disposition`; no row is left blank or
   "to be filled".
5. Each sealed pass is delivered with a content hash and timestamp; the file
   is not edited after sealing.
6. Guide inadequacy discovered mid-pass is **recorded, not repaired**:
   note `GUIDE-CONFLICT:` or `GUIDE-GAP:` on the affected rows, keep
   annotating under the guide as written, and report after sealing. The
   guide changes only in a new version, and a changed guide after one pass
   has seen the affected rows is a preregistration §11.2 pause condition.
7. Do not run unfiltered corpus searches; if a search is needed to decide a
   row (it should not be for a structural judgement), stop and use
   `unsure` / `adjudicate` instead.

## 15. Completion checklist (run before sealing)

Per row:

- [ ] `boundary` ∈ {ok, under, over, misattached, unsure}; `missed` is a
      non-negative integer, `0` unless `under`.
- [ ] `kind_expected` ∈ ten kinds ∪ {n/a, gap:…, unsure}; `n/a` iff
      `role = mode`; `gap:` iff `disposition = unsupported`.
- [ ] `kind_ok` ∈ {yes, no, n/a, unsure} and consistent with
      `kind_expected` vs emitted `kind`; `role_ok` ∈ {yes, no, unsure};
      `source_ok` ∈ {yes, no}.
- [ ] `context` ∈ {none, cr, type_line, game_state, card_specific};
      keyword → cr; rules_supplied → type_line; CDA → type_line;
      delayed_trigger role → cr or stronger.
- [ ] `cr_ref` non-empty, ids only, `;`-separated, every id verified.
- [ ] `structure_tags` ⊆ frozen list; mandatory tags present;
      `multi_sentence` matches the terminator count.
- [ ] `norm_issue` blank or recognized classes only.
- [ ] `disposition` ∈ {accept, defect, unsupported, ambiguous, adjudicate}
      and matches the §10.1 tree given the other fields.
- [ ] `note` present for every non-`accept` row, every `context ≠ none`
      requiring `CTX:`/`RULING:`, every `AMBIG:`/`GAP:`/`D19:`/`D14:` case.
- [ ] No implementation proposal, regex, held-out identity, or reference to
      the other annotator in `note`.

Per file:

- [ ] Row count equals the frozen export; every stable key
      `(oracle_id, face, index)` present exactly once; no added or dropped
      rows.
- [ ] The 15 structural columns are byte-identical to the export.
- [ ] `annotator` is one identical value on every row and matches the
      attestation.
- [ ] `python scripts/python/audit_metrics.py <pass.tsv> --export
      <frozen-export.tsv>` runs without error and reports drift 0.
- [ ] Content hash (sha256) computed and timestamp recorded; file sealed.

## 16. Conventions newly made explicit by this guide (for calibration)

| Id | Convention | Section |
|---|---|---|
| C1 | `kind_expected`/`kind_ok`/`role_ok` always filled; denominators gate on `boundary = ok` | §4.3 |
| C2 | One defect spanning several rows: judge each row's span; first affected row's note lists siblings | §5.2 |
| C3 | `missed` counts unemitted reference units, never slots | §5.2 |
| C4 | Mode-header row carries the whole modal ability's kind, judged normally | §5.5 |
| C5 | `granted` covers granted, gained, lost, and referenced quoted abilities | §7 |
| C6 | Context = strongest context *required*, precedence `card_specific > game_state > type_line > cr > none`; fixed consequences for keyword / rules-supplied / CDA / delayed-trigger rows | §8 |
| C7 | `cr_ref` never blank; default citations for surface-obvious rows; rulings go in `note` | §9 |
| C9 | Residual-accepted classes vs `gap:kind/role/span:<slug>` | §6.11, §10.3 |
| C10 | Mandatory vs optional tags; `multi_sentence` terminator definition | §11.2 |
| C11 | `norm_issue` recognized classes, mechanical tests, `;` joining | §11.4 |
| C13 | Note prefixes | §4.13 |

## 17. Unresolved issues that require a protocol or lead decision

| Id | Issue | Guide's interim position |
|---|---|---|
| U1 | Protocol S7 ("must cite") versus committed practice (blank `cr_ref` on most `accept` rows) | C7 never-blank with defaults; if the lead prefers the historical practice, replace §9's first sentence — no other section depends on it |
| U2 | The frozen export columns (protocol 4.1, `export_units.py`) carry no `prefix` column, yet H5's denominator is "units with a non-null `prefix`" | Annotators cannot see `prefix` in the TSV; either the freeze export adds the column (a T2 contract change) or H5 is computed from the JSON export at adjudication — not a guide decision |
| U3 | Non-contiguous parent spans (P-ARN-1 shape): accepted as `ok` + `SPAN:` note in `atq`, but preregistration §11.2 lists "schema cannot preserve the observed span" as a pause condition | Follow the ATQ precedent (`ok`); the lead should confirm before the freeze |
| U4 | Whether payment restrictions / cost modifications / prohibitions are residual-accepted (C9) or `unsupported` | Residual-accepted, on the evidence of the frozen tag names and the P-ATQ-2 acceptance |
| U5 | `granted` for lost/referenced quoted abilities (C5) vs the Alpha `adjudicate` precedent | C5, on the live enum's own definition |
| U6 | `multi_sentence`: protocol says "≥ 2 sentence terminators" without defining terminators | `.`, `!`, `?` in `text` as exported |
| U7 | `norm_issue` schema row lists single forms; practice uses `;` | `;` permitted |
| U8 | Whether a *generic* ruling on a wording class (e.g. the Gorgon Recluse ruling reused for other cards) is `cr` or `card_specific` | `card_specific` only when the ruling is about the card being annotated; a ruling on another card is corroboration recorded in `note` and the context is `cr` — this changes the value on rows like `atq` Tawnos's Coffin #1 (committed `cr`) and needs confirmation |
| U9 | Guide length versus the repository's 500-line file guideline | Left as one document so a pass uses a single frozen hash; can be split at freeze if required |

No item above is resolved here; each is a calibration question for the
research lead before the guide is frozen and hashed into preregistration §3.
