# D19 unscoped-trigger attachment research design

- Date prepared: 2026-08-26
- Status: **design only; no corpus or card search performed**
- Scope: structural attachment and boundary evidence
- Related but separate: D14 and P-ATQ-1

## 1. Research question

When a printed structural unit contains effect text followed at a sentence
boundary by an unscoped sentence beginning with `When`, `Whenever`, or `At`, is
the later clause:

1. a delayed triggered ability created by resolution of the preceding effect
   and therefore a nested child (D19);
2. an independent printed triggered ability sharing a paragraph and therefore a
   sibling/top-level unit (D14);
3. quoted, granted, reminder, or otherwise outside the candidate class; or
4. genuinely ambiguous or unsupported by the current structural vocabulary?

“Unscoped” means the candidate lacks the already-supported positive markers
such as `this turn`, `this way`, `When you do`, or an explicit `next` phrase.
The question is attachment, not whether the sentence contains trigger syntax.

## 2. Target wording and attachment problem

The candidate surface form is:

```text
<creating-or-independent preceding sentence>. <When/Whenever/At clause>.
```

The hard question is whether the preceding sentence creates the later trigger.
Sentence adjacency is observable; creation dependency is a structural relation
that may require anaphora, rules context, or an official ruling.

A correct positive D19 analysis produces:

- one parent unit for the activated, triggered, spell, or static ability whose
  effect creates the delayed trigger;
- one child unit for the created delayed triggered ability;
- role=`delayed_trigger` on the child;
- a valid parent index within the same face;
- kind judged normally on both valid reference units.

A correct D14 analysis produces two independent abilities with no parent-child
attachment between them.

## 3. Predeclared classes

No individual card is assigned to a class in this design.

### 3.1 Positive D19 classes

**P1 — Referential dependency.** The later trigger refers to an object, choice,
quantity, status, or remembered value created or established only by the
preceding resolving effect.

**P2 — Created-zone dependency.** The preceding effect moves or marks a
particular object, and the later trigger monitors a future event involving that
same effect-defined object or relationship.

**P3 — Ruling-explicit creation.** An official ruling expressly calls the later
clause a delayed triggered ability or says it is created by resolution.

**P4 — CR-defined action creation.** A static or resolving ability permits or
instructs an action and the later trigger is created as a consequence under an
applicable CR 603.7 subrule.

**P5 — Non-contiguous created child.** The created trigger is followed by
instructions that belong to the parent or by a reflexive consequence that makes
the parent span non-contiguous. This remains positive only if both spans and
attachment can be represented without discarding text.

### 3.2 Negative classes

**N1 — D14 independent trigger.** The later sentence is a complete triggered
ability whose existence and trigger condition do not depend on resolving the
preceding ability.

**N2 — Same paragraph, independent source behavior.** Both sentences describe
abilities of the printed object that function independently in the relevant
zone; adjacency is formatting only.

**N3 — Quoted or granted trigger.** The later trigger occurs inside quotation
marks or granted text and belongs to the quoted/granted structure rather than a
D19 split.

**N4 — Reminder or rules-supplied text.** The trigger-like wording is reminder
text or a rules-supplied explanation and is governed by the `source` distinction.

**N5 — Already-supported scoped child.** The later sentence carries an existing
positive marker and is already covered by an accepted delayed-trigger rule. It
is a regression case, not a new D19 candidate.

**N6 — Top-level spell-created trigger.** The entire instant/sorcery unit is the
created delayed trigger and has no preceding creating sibling. This is the
P-ATQ-4 top-level class, not D19.

**N7 — Single-sentence internal trigger.** The candidate occurs after a comma or
colon rather than at a sentence boundary. P-ATQ-1 keeps that whole and may emit
an unattached signal; D19 does not restore the rejected split.

**N8 — Ordinary trigger instruction.** A later trigger-word clause is part of a
single triggered ability's effect or instruction structure under CR 603.1 and
does not represent a separately created trigger.

### 3.3 Ambiguous and unsupported classes

**A1 — Adjacency without dependency evidence.** The later trigger is compatible
with both an independent and a created reading, with no decisive CR or ruling.

**A2 — Unresolved anaphora.** A pronoun or description may refer to an
effect-created object or to a generally available object, and authoritative
sources do not choose.

**A3 — Span representation gap.** The child appears embedded among parent
instructions so the current contiguous export cannot preserve both spans or
parentage accurately.

**A4 — Source/zone uncertainty.** It is unclear whether the later text is active
as an ability of the card in another zone or exists only after the preceding
effect resolves.

**A5 — Rules ambiguity.** Competing structural readings remain after CR and
official rulings are consulted.

Ambiguous rows remain `ambiguous`; representation gaps remain `unsupported`
with `gap:<class>`. Neither is converted into a classifier guess.

## 4. Separation from related work

### 4.1 D19 versus D14

| Question | D19 | D14 |
|---|---|---|
| Does the later trigger exist because the preceding effect resolved? | Yes | No |
| Structural relation | Parent/child | Sibling or separate top-level ability |
| Key evidence | Referential/creation dependency, CR 603.7, ruling | Independent function and complete CR 603.1/113.3c ability |
| Is sentence adjacency sufficient? | No | No |
| Failure risk | Under-segmentation if child is missed | Under-segmentation if independent ability is merged; misattachment if treated as child |

The future investigation must measure both classes in one comparison set but
report them separately. A D19 proposal is rejected if its surface pattern also
captures unresolved D14 cases without an explicit abstention path.

### 4.2 D19 versus P-ATQ-1

P-ATQ-1 concerns invalid comma/colon boundaries inside a single sentence. It
preserves the whole ability because the former parent was a condition, cost, or
other fragment.

D19 concerns a later trigger-word sentence at an actual sentence boundary. Its
question is whether that complete sentence is a created child or an independent
ability. D19 must not reintroduce internal punctuation splitting or fragmentary
parents.

### 4.3 D19 versus P-ATQ-4

P-ATQ-4 marks a qualifying instant/sorcery delayed-trigger unit top-level when
the spell text itself is the trigger and no earlier sibling serves as parent.
D19 requires preceding creating effect text and tests nested attachment.

## 5. Applicable Comprehensive Rules questions

The future adjudication asks, in authority order:

1. **CR 113.3:** Is the surrounding text one spell, activated, triggered, or
   static ability, and what constitutes its complete reference unit?
2. **CR 113.6:** In which zone does the printed ability function? Could the later
   trigger be an off-stack ability of the card rather than a created child?
3. **CR 603.1:** Does the later sentence independently satisfy the complete
   trigger-condition/effect structure?
4. **CR 603.7:** Does the preceding effect create a delayed triggered ability?
5. **CR 603.7a:** At what point is the delayed trigger created, and could it have
   existed before resolution?
6. **CR 603.7b:** Does the absence of a stated duration imply a one-time delayed
   trigger rather than an independent recurring ability?
7. **CR 603.7c:** Does the later trigger refer to a particular object established
   by the creating effect?
8. **CR 603.7d-g:** What source/controller rule identifies whether the creator
   is a spell, activated/triggered ability, replacement effect, or static
   permission?
9. **CR 603.12:** Is the later text reflexive rather than an ordinary delayed or
   independent trigger?
10. **CR 607:** Are apparently related clauses linked abilities rather than a
    parent/created-child relation?

An official ruling may establish attachment explicitly. It supports the
disposition but cannot override the CR.

## 6. Future S8 search strategy

No search is run as part of this design. After the relevant development audit is
open and its baseline is frozen, S8 should proceed as follows.

### 6.1 Candidate inventory

Construct a deterministic inventory of non-held-out printed units that:

- contain at least two sentences after reminder text is masked;
- contain a later sentence beginning with `When`, `Whenever`, or `At`;
- are not inside quoted text;
- lack the already-supported scoped/reflexive markers in that later clause;
- retain the complete preceding and following sentence context;
- record type line, face, current kind/role/source, and parentage.

Held-out identities may contribute to an aggregate count only. They must be
filtered before any candidate text or metadata is displayed.

### 6.2 Nearest negative inventories

Search and sample separately for:

- later trigger-word sentences with no referential dependency on the preceding
  sentence;
- independent triggers sharing a paragraph (D14);
- the same trigger forms inside quotes or granted text;
- trigger-word clauses after a colon or internal comma (P-ATQ-1 negatives);
- top-level instant/sorcery trigger units with no preceding creator
  (P-ATQ-4 negatives);
- already-supported scoped delayed/reflexive trigger children;
- reminder/rules-supplied occurrences;
- units where a zone/self-reference suggests off-stack function.

### 6.3 Sampling and review

- Inspect all candidates if fewer than 20; otherwise inspect at least 20 across
  at least three first-printing decades, plus every rare surface template that
  occurs no more than twice.
- Inspect at least 20 nearest negatives under the same era/rarity rules.
- Use two independent attachment judgements before discussion.
- Record positive, negative, ambiguous, unsupported, and out-of-scope counts
  with denominators.
- Cite CR and rulings for every positive and every disputed negative.
- Preserve every counterexample, not only a representative sample.

### 6.4 Measurements

Report:

- candidate units and distinct surface templates;
- D19 positive, D14 negative, other negative, ambiguous, and unsupported counts;
- current true positives, false positives, false negatives, and correct refusals
  if evaluating an existing signal;
- parent kind/role/source and face-type breakdowns;
- anaphoric/dependency evidence classes;
- required context (`none`, `cr`, `type_line`, `game_state`, `card_specific`);
- inter-annotator and adjudicated agreement;
- held-out aggregate count and zero inspected held-out records.

## 7. Evidence required before a classifier proposal

No D19 classifier change may be proposed until all of the following exist:

1. one or more adjudicated non-held-out defect rows with valid parent and child
   reference spans;
2. a generic surface/dependency pattern stated without card names, set codes,
   Oracle IDs, or per-card branches;
3. CR rules establishing why the positive class is created and why the negative
   class is independent;
4. an S8 inventory and sample covering every declared positive, nearest
   negative, quoted, off-stack, P-ATQ-1, P-ATQ-4, and D14 class;
5. explicit ambiguous and unsupported outcomes when surface evidence cannot
   decide attachment;
6. a predicted parent/child representation, including handling of
   non-contiguous spans;
7. a rule-firing inventory and expected corpus before/after counts;
8. regression impact on `lea`, `leb`, `arn`, `atq`, and every later audited
   development set available at that time;
9. positive and negative regression-test classes plus a nesting/indexing test;
10. an acceptance record satisfying protocol S10-S12.

If the positive and D14 negative classes cannot be separated without semantic
execution, hidden state, or card-specific knowledge, the result is **no
classifier proposal**. The project should retain a review signal and explicit
ambiguous/unsupported attachment instead.

## 8. Stop and escalation rules

Stop the D19 investigation and escalate to the research lead if:

- a held-out record becomes visible;
- candidate generation drops surrounding sentence context needed for
  attachment;
- the export cannot represent the proposed parent/child spans;
- annotators are asked to decide from adjacency alone;
- D14 negatives are being absorbed into the D19 positive count;
- a card-specific exception is proposed;
- semantic or behavioral correctness is substituted for structural evidence;
- a code change is requested before the S8/adjudication package exists.

The escalation outcome may be a bounded tooling requirement, a protocol change
record, an unsupported structural class, or continued deferral. It is not
automatic authorization to broaden the audit.
