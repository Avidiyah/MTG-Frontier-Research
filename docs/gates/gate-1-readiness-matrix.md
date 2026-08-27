# Gate 1 readiness matrix

- Date assessed: 2026-08-26
- Gate: Phase 1 — Structural discovery
- Status: **readiness assessment, not a Gate 1 decision**
- Decision basis: repository state at `8e83221` plus the P-ATQ
  research acceptance report; Claude's technical P-ATQ package remains pending

## 1. Reading the matrix

Statuses mean:

- **satisfied:** current committed evidence meets the criterion as written;
- **partial:** relevant evidence exists, but the full Gate 1 requirement is not
  yet demonstrated;
- **missing:** the required artifact or measurement does not yet exist;
- **deferred:** deliberately scheduled for the Gate 1 review or a later phase.

“Blocks Legends” asks whether the gap prevents the next development-set audit
from opening after P-ATQ closes. “Blocks Gate 1” asks whether it prevents the
eventual written Gate 1 pass.

## 2. Formal Gate 1 exit criteria

| # | Exact Gate 1 criterion (`docs/roadmap.md` §6) | Current repository evidence | Status | Evidence still required | Responsible role | Earliest appropriate action | Blocks Legends? | Blocks Gate 1 review/pass? |
|---|---|---|---|---|---|---|---|---|
| G1.1 | “The gold set spans release eras, card layouts, ability categories, frequent templates, rare templates, and known adversarial cases.” | Adjudicated or reviewed development annotations exist for `lea`, `leb`, `arn`, and `atq`; structure tags, novelty, template frequency, and known failure classes are recorded. All audited sets are from the earliest era, and `docs/current-state.md` explicitly says they are development/regression sets, not gold. | **partial** | A declared structural-gold sampling frame and frozen manifest spanning eras, layouts, categories, frequency strata, and adversarial classes; adjudicated annotations for that manifest; coverage table showing every stratum. | Research lead designs strata; program owner approves; annotators produce evidence. | Continue Phase 1 era walk now; freeze the gold manifest only after sufficient strata exist and before parser tuning. | **No.** Legends contributes needed development evidence. | **Yes.** |
| G1.2 | “Two independent passes or an equivalent adjudication process establish a documented reference segmentation.” | Antiquities has two complete independent passes with 125/125 agreement. Alpha and Arabian Nights are single-annotator with lead review; Beta is a small appendix. The protocol defines an adjudication path, but there is no multi-era gold reference yet. | **partial** | Two independent passes or equivalent documented adjudication on the eventual structural-gold set, with sealed pass identities, disagreement report, and final reference segmentation. The Legends preregistration requires two complete independent passes for its eligible development partition. | Independent annotators; research lead/adjudicator. | Assign annotators after P-ATQ closes; execute the Legends workflow after the baseline freeze; repeat across later gold strata. | **No**, provided two annotators are assigned before Legends opens. | **Yes.** |
| G1.3 | “Structural metrics report boundary precision, recall, and exact-card agreement rather than template frequency alone.” | Per-set `metrics.json` files report boundary precision/recall plus kind, role, source, context, novelty, and dispositions. Template coverage is explicitly described as non-correctness. Exact-card structural correctness/agreement is not currently reported. | **partial** | A frozen exact-card definition and numerator/denominator; segmenter-vs-reference exact-card correctness; inter-annotator exact-card agreement; aggregate and stratum breakdowns in the Gate 1 package. | Research lead defines metric; technical measurement owner computes reproducibly; adjudicator validates. | Definition is frozen in the Legends preregistration; compute for Legends after adjudication and for the final gold set at Gate 1. | **No.** | **Yes.** |
| G1.4 | “Remaining failures are categorized rather than silently absorbed into `spell_or_static_text`.” | Protocol S13 defines `unsupported`, `ambiguous`, and `gap:<class>`; prior audits record defects, ambiguity, structure tags, D14, D19, and normalization risks. P-ATQ-2 deliberately uses residual static text because no prohibition kind exists, while documenting that limitation. The residual kind still contains heterogeneous structures and there is no cross-era failure matrix. | **partial** | A versioned error taxonomy covering every final-gold non-accept row and sampled residual class; counts by boundary/kind/role/context; explicit list of vocabulary gaps and residual fallbacks; no unexplained catch-all errors. | Research lead and adjudicators. | Categorize during every development audit; consolidate after additional eras and before Gate 1 review. | **No.** | **Yes.** |
| G1.5 | “The team can state which structures require rules/type-line context.” | Every annotation row has a `context` field. Aggregate reports distinguish `none`, `cr`, `type_line`, `game_state`, and `card_specific`; early audits already show CR and type-line dependencies and no accepted card-specific heuristic. Coverage remains early-era only. | **partial** | Cross-era context-by-structure matrix on the structural-gold set, with CR citations, per-face/type-line cases, game-state dependencies, and preserved unsupported/ambiguous rows. | Research lead defines categories; annotators record; adjudicator consolidates. | Continue recording in Legends; publish the cross-era matrix when the gold strata are frozen. | **No.** | **Yes.** |
| G1.6 | “The held-out structural set has been frozen before parser tuning.” | Protocol §6.3 freezes a 2,096-card pool by Oracle-ID rule and prohibits inspection; it has not been sampled or annotated. The P-ATQ report logs four accidentally exposed identities that must be excluded from future samples. No parser tuning is authorized. | **partial** | A Gate 1 structural held-out sample manifest with stable keys/hash, all incident exclusions applied, linked faces/variants co-partitioned, sampling rationale, annotation plan, and proof it was frozen before any parser tuning. | Research lead/data steward prepares; program owner approves; independent held-out annotators execute only at review. | Preserve the pool throughout Legends; select and hash the structural held-out sample at the declared Gate 1 preparation point, before parser tuning. | **No**, if Legends uses only a verified non-held-out development export. | **Yes.** |

No formal Gate 1 criterion is currently fully satisfied at program scope. That is
expected during Phase 1: the audited early sets provide development evidence,
not the final cross-era gold and held-out package.

## 3. Required Phase 1 deliverables

| Required deliverable (`docs/roadmap.md` §6) | Current evidence | Status | Evidence still required | Blocks Legends? | Blocks Gate 1? |
|---|---|---|---|---|---|
| Versioned structural taxonomy | Protocol v1.0 defines export/annotation fields and a controlled tag vocabulary; the CLI exposes kind, role, source, parentage, and prefix metadata. Known gaps remain. | **partial** | Freeze a Gate 1 taxonomy version with definitions, additions/change records, deprecated values, and unsupported classes. | No | Yes |
| Era-stratified failure matrix | Early-set findings and current-state limitations exist, but no matrix spans eras. | **missing** | Rows by era/layout/structure and columns for boundary, kind, role, context, ambiguity, and unsupported rates. | No | Yes |
| Corpus novelty measurements with documented denominator | Unit/template novelty definitions are frozen; `arn` and `atq` have committed measurements against earlier audited exports. | **partial** | Continue across sets/strata; bind every measure to export hashes and earlier-set denominator. | No | Yes |
| Candidate segmentation specification | Protocol, current-state, CLI metadata, tests, and proposal records collectively describe current heuristics. There is no single frozen candidate specification. | **partial** | Consolidated surface rules, precedence, parent/child topology, exclusions, signals, unsupported cases, and revocation evidence. | No | Yes |
| Frozen structural annotation guide | Protocol S5/S13 and schema form an initial guide; the external report calls for positive, negative, and unresolved examples. No cross-era frozen guide artifact exists. | **partial** | Versioned guide used unchanged by independent gold annotators, with change-control and agreement evidence. | No, if the Legends guide/version is frozen before its passes | Yes |
| Reviewed structural gold set with development and held-out partitions | Early annotated sets are explicitly development/regression data. The held-out pool is unsampled. | **missing** | Frozen manifests, adjudicated gold annotations, held-out annotation at review, hashes, datasheet/limitations, and leakage record. | No | Yes |

## 4. Supporting methodological readiness

The external stack report identifies minimum Gate 1 evidence beyond the six
formal exit statements. Current readiness is:

| Supporting evidence | Current state | Status | Next evidence |
|---|---|---|---|
| Double-annotated adjudicated structural examples across strata | Present for Antiquities only; Legends preregisters two full passes. | **partial** | Additional layout/era strata and final gold sample. |
| Published annotation guidelines | Protocol v1.0 exists, but is not a complete example-rich cross-era guide. | **partial** | Versioned frozen guide with positive, negative, unsupported, and ambiguous examples drawn only from development data. |
| Per-layer agreement | Antiquities reports aggregate 125/125 judgement-field agreement. Boundary/kind/role/source confusion and exact-card agreement are not separately published. | **partial** | Per-layer row and exact-card agreement on Legends and final gold. |
| Frozen manifest | Pool rule is frozen; structural held-out sample and gold manifest are not. | **partial** | Stable-key manifests and hashes before parser tuning. |
| Error taxonomy | Protocol dispositions and known failure classes exist. | **partial** | Cross-era quantified matrix and complete final-gold unsupported inventory. |

## 5. Pre-Legends entry dependencies

These are not Gate 1 exit criteria; they decide whether the next development
audit may open.

| Entry dependency | Current status | Required action | Responsible role | Blocks Legends? |
|---|---|---|---|---|
| P-ATQ technical acceptance package | **partial** — research acceptance exists; Claude's S8/regression/evidence package is pending. | Complete technical package; surface any contradiction; close only affected proposal if needed. | Claude/technical validator; research lead adjudicates conflict. | **Yes.** |
| Live baseline reconciled in `docs/current-state.md` | **partial** — current state still contains pre-acceptance wording in places. | Reconcile after P-ATQ closure and record final commit. | Technical documentation owner; research lead verifies. | **Yes.** |
| Legends preregistration and empty outline | **satisfied** by `docs/findings/leg-structural-audit-preregistration.md` and `docs/findings/leg-structural-audit.md`. | Program-owner countersignature after P-ATQ closure. | Research lead; program owner. | **Yes until countersigned.** |
| Held-out-safe development export | **missing as an attested audit input**; protocol T7 remains a tooling requirement. | Produce deterministic non-held-out export and verify exclusions by aggregate/key checks before any row is displayed. No classifier change is involved. | Technical measurement owner; research lead verifies. | **Yes.** |
| Frozen source/build/rules/export identities | **deferred until P-ATQ closes** so the actual accepted commit is bound. | Record section 3 of the preregistration verbatim from live commands and hashes. | Technical measurement owner; research lead. | **Yes.** |
| Two independent annotators and adjudicator assigned | **missing** in current artifacts. | Assign named/identified roles and confirm no pre-freeze Legends inspection. | Research lead/program owner. | **Yes.** |

## 6. Readiness conclusion

- **Ready to preregister:** yes.
- **Ready to inspect Legends:** no; P-ATQ closure and the pre-Legends entry
  dependencies remain open.
- **Does incomplete Gate 1 evidence block Legends after entry conditions pass:**
  no. Legends is Phase 1 development work needed to improve Gate 1 readiness.
- **Ready for Gate 1 review/pass:** no. Cross-era gold coverage, a held-out
  structural sample, exact-card metrics, a frozen guide, and a quantified error
  matrix remain incomplete.
- **Ready to select a parser, semantic IR, engine, or infrastructure:** no; all
  remain outside this phase and this matrix.
