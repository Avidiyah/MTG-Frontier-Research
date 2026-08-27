# Minimum work to open the Legends structural audit

- Prepared: 2026-08-26 (America/Chicago)
- Repository state reviewed: `8307c94` (`Add candidate v1.0 structural
  annotation guide for Legends passes`), clean worktree before this document
  was added
- Objective: permit the first eligible Legends (`leg`) development row to be
  opened under the existing preregistration
- Status: execution plan only; Legends remains closed
- Non-observation statement: this plan was prepared from control documents,
  source and test contracts, aggregate-only evidence, and earlier audited
  sets. No Legends card or export row and no held-out identity was opened,
  printed, quoted, or annotated.

## 1. The exact endpoint

For this plan, **“get to Legends” means open the structural audit and allow the
two assigned annotators to begin reading the frozen, non-held-out Legends
development export**. It does not mean finish both annotation passes,
adjudicate the set, accept a classifier proposal, pass Gate 1, choose an IR, or
build an engine.

The opening condition is already fixed by
`docs/findings/leg-structural-audit-preregistration.md` §12 and tracked in
`docs/gates/legends-entry-record.md` §3. Items 1, 2, 3, and 10 are satisfied.
The earlier P-ATQ work and the T7 export-safety capability are complete. The
remaining work is:

1. make the preregistered prefix measurement possible in the exact annotation
   export;
2. turn the candidate annotation guide into one frozen set of instructions;
3. assign two independent annotation passes and a separate adjudicator, and
   record non-observation/independence attestations;
4. choose and verify the clean freeze commit;
5. retain and hash the final held-out-safe export and populate the frozen-input
   record without viewing rows;
6. record the program-owner authorization.

When those six actions are complete, update the entry record so all eleven
items are `satisfied`. The audit may then open. No additional research pass is
an opening requirement.

## 2. Why any pre-opening work remains

The remaining work protects three claims the Legends audit is intended to
make:

- **The two passes judged the same immutable units.** A clean commit, snapshot
  identities, export hash, and stable keys distinguish annotator disagreement
  from changing software or data.
- **No held-out evidence leaked into development work.** T7 has already proved
  the safe path exists, but the row-bearing output from that check was
  deliberately discarded. The actual annotation artifact still has to be
  retained and bound to that path.
- **Agreement measures independent application of fixed instructions.** The
  preregistration's H8 compares exact judgment fields. If the guide changes
  after a pass sees rows, or the passes exchange row-level judgments, that
  number no longer measures independent agreement.

These are not generic “best practices” added after the fact. They are direct
consequences of preregistration §§3, 7, 11, and 12, protocol §§2–3 and 6.3,
and the role/signature requirements already recorded in
`docs/gates/legends-entry-record.md`.

## 3. Critical path

```text
prefix export contract
        +
guide decisions
        |
        v
clean candidate freeze + aggregate-only verification hash
        |
        v
role assignments + attestations bound to that hash
        |
        v
retain identical TSV + populate frozen inputs
        |
        v
program-owner authorization
        |
        v
OPEN LEGENDS
```

The work below follows this dependency order. Do not open the retained TSV
while executing it.

## 4. Work item A — expose `prefix` in the frozen annotation export

### Required work

Add the segmenter's existing `prefix: Option<String>` value to the native
audit record and protocol TSV:

1. `src/audit.rs`
   - carry `segment.prefix` into each `AuditRecord`;
   - serialize it deterministically as `prefix`, null/absent or empty when no
     prefix exists, consistent with the chosen export contract;
   - preserve the existing stable key `(oracle_id, face, unit_index)` and
     ordering.
2. `scripts/python/export_units.py`
   - add `prefix` to `COLUMNS` and to `tsv_rows`;
   - preserve every existing column and stable key.
3. `scripts/python/test_export_units.py` and focused Rust export tests
   - prove a prefixed unit carries the expected value;
   - prove a non-prefixed unit carries the documented empty representation;
   - keep held-out rejection, parent integrity, deterministic order, and JSON/
     TSV key agreement passing.
4. `docs/protocol/structural-investigation-protocol.md` §4.1
   - add `prefix` to the T2 export-column contract and define it as structural
     metadata already emitted by the segmenter, not a new label or semantic
     claim.
5. `docs/protocol/structural-annotation-guide-v1.0.md`
   - close U2 by pointing annotators to the frozen `prefix` column;
   - do not change H5 or broaden the prefix classifier.

Do not change segmentation, prefix detection, normalization, kinds, roles, or
card selection. This is propagation of an existing field, not a new research
proposal.

### Why this is required

Legends preregistration H5 defines its denominator as **all development units
with a non-null `prefix`**, reported by prefix class, kind, role, face type,
and outcome. The segmenter stores `prefix` in `src/segment.rs`, but
`AuditRecord` and the TSV contract currently omit it. The frozen TSV therefore
cannot identify H5's exact denominator or show annotators what prefix the
classifier extracted.

Using a separate, unbound JSON query later would create a second source of row
state and make drift possible. Adding the already-existing field to the one
frozen export keeps H5 auditable against the same stable rows used for all
other judgments. This closes a specific preregistered measurement gap; it does
not improve the parser or add an ontology.

### Completion evidence

- focused Rust and Python tests pass;
- the T2 contract and guide agree with the emitted columns;
- `verify_export_safety.py` still reports byte-identical JSON and TSV runs,
  426/426 unique keys on the current snapshot, and zero held-out records;
- aggregate counts are unchanged unless a documented source/input change
  explains the difference.

## 5. Work item B — freeze one annotation-guide decision set

### Required work

The research lead should review §17 of
`docs/protocol/structural-annotation-guide-v1.0.md` once and either ratify or
replace each interim convention. No new corpus research is needed for these
operational choices. The shortest evidence-consistent disposition is:

| Issue | Legends v1.0 decision | Existing basis |
|---|---|---|
| U1, CR citations | Keep C7: `cr_ref` is never blank; use the guide's default citations for surface-obvious rows. | Protocol S7 says a boundary or kind disposition must cite a CR rule. Earlier blank citations are historical practice, not the frozen requirement. |
| U2, `prefix` | Resolve through work item A. | H5 explicitly requires the non-null-prefix denominator. |
| U3, non-contiguous parent spans | Keep the ATQ representation when all text and the correct parent link remain represented; pause only if the schema actually loses span or attachment information. | The ATQ adjudicated precedent accepts the representation; preregistration §11.2 pauses when the schema *cannot preserve* the observed span or attachment. |
| U4, residual classes | Keep the guide's bounded residual-accepted list and mandatory structure tags. | Gate 1's matrix records the accepted P-ATQ-2 residual treatment while requiring the limitation to remain visible rather than silently absorbed. |
| U5, `granted` | Keep C5. | The live enum explicitly includes granted, gained, lost, and referenced quoted abilities; this is the vocabulary the export asks annotators to judge. |
| U6, sentence terminators | Use `.`, `!`, and `?` in exported `text`; keep `multi_sentence` descriptive rather than dispositive. | Protocol §4.4 defines the tag mechanically as at least two sentence terminators but does not define the characters. The convention completes that mechanical definition without changing boundaries. |
| U7, multiple normalization issues | Permit `;`-separated values and state this in the schema. | Existing committed annotations already carry multiple issues; the delimiter preserves them without adding a new issue class. |
| U8, ruling context | A ruling about the annotated card is `card_specific` when it is required for the judgment; a ruling about another card is corroboration in `note`, not authority replacing the CR. | Protocol S5 defines `card_specific` as a ruling specific to the card, and S7 says rulings clarify but do not override the CR. |
| U9, guide length | Keep one authoritative guide and one hash for Legends. Do not split it before tonight. | The entry record and attestations bind one guide version/hash; splitting now adds another identity without changing a judgment. |

Then:

1. change the guide status from candidate to frozen v1.0 for the Legends
   passes;
2. turn §17 from unresolved questions into a decision record, retaining the
   alternatives and basis rather than deleting history;
3. clean up skipped convention identifiers or explicitly mark them reserved;
4. commit the guide, protocol/export-contract change, tests, and decision
   record;
5. do not edit the guide after either annotator can see the export.

### Why this is required

H8 counts a row as agreement only when eight judgment fields match exactly,
including `context` and `disposition`. The candidate guide currently labels
its U1–U9 items as requiring a lead decision. Leaving them open lets two
annotators apply different citation, context, residual, tag, and attachment
conventions while both believe they followed the instructions. That would
turn a documentation difference into a measured research disagreement.

The preregistration also says annotation must pause if instructions would need
to change after one pass has seen affected rows. Freezing these conventions
now avoids that stop condition. It is a single bounded decision over already
documented alternatives, not another model-driven research cycle.

### Completion evidence

- the guide says frozen v1.0 and contains no unresolved instruction that can
  change an H8 judgment field;
- its SHA-256 is recorded outside the guide in the preregistration freeze
  block and both annotator attestations;
- protocol, guide, and export columns agree;
- no Legends row was used to select a convention.

## 6. Work item C — assign the minimum independent roles

### Required work

Record these identities using the templates already present in
`docs/gates/legends-entry-record.md` §5:

- pass-1 annotator;
- pass-2 annotator;
- adjudicator, distinct from both annotators;
- research lead and program owner approving the assignments.

The repository defines an identity as an **agent/session or person label**.
It does not require different model vendors. Two isolated sessions using the
same model can be independent if neither sees the other's row-level work. The
adjudicator must be a third identity and may open the sealed passes only after
both are complete. The research lead and program owner are not stated to be
separate people; one project owner may fill both governance roles if that is
recorded plainly.

Each annotator must attest before receiving row access that they:

- have not inspected eligible Legends text in a heuristic-design context;
- will use only the frozen export and guide;
- will not read or discuss the other pass before both are sealed;
- will preserve `unsure`, `unsupported`, `ambiguous`, and `adjudicate` rather
  than guess.

The adjudicator signs the assignment note and the same non-observation
condition. Use the aggregate-only verifier's expected TSV hash in the
attestations; no one needs to open the file to receive or confirm its hash.

### Why this is required

The preregistration requires two complete independent passes and uses their
pre-discussion agreement as H8 evidence. One pass copied or influenced by the
other cannot support that measurement. The adjudicator is separate because
the final reference must preserve disagreements and reasons rather than let
one annotator silently overwrite the other.

This is about information separation, not spending tokens across many models.
The minimum is three named, isolated review identities. Guide drafting,
technical verification, program ownership, and research-lead decisions do not
need additional model calls merely for diversity.

### Completion evidence

- two completed §5.1 attestations;
- one completed §5.2 adjudicator note;
- exact identities copied into preregistration §3;
- no exception indicating prior eligible Legends inspection.

## 7. Work item D — choose and verify the clean freeze commit

### Required work

After work items A and B are committed, select that clean commit as the freeze
candidate. Run and record:

```powershell
git rev-parse HEAD
git status --short
cargo build --release
cargo test
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
python -m unittest scripts.python.test_audit_metrics
python -m unittest scripts.python.test_export_units scripts.python.test_manifests
python scripts/python/verify_manifests.py `
  docs/manifests/snapshot-scryfall-2026-08-25.json
```

Then run the aggregate-only export verifier without displaying rows:

```powershell
python scripts/python/verify_export_safety.py leg `
  --mtg .\target\release\mtg-discover.exe --runs 2
```

Record its aggregate counts and expected TSV SHA-256. Because work item A
changes the export bytes, the old TSV hash
`ca7f95e55a20916dd15e1107a2951b4138e93edb3c2a1babf90e4e5dba8a1202`
must not be copied forward. The expected current-snapshot counts are comparison
points, not values to force: 293 cards after exclusion, 273 with text, 426
records, 426 unique keys, 17 held-out identities excluded, and zero held-out
export records. Any unexplained difference is a stop condition.

Preserve
`docs/manifests/experiment-pre-legends-export-gate-2026-08-26.json` as the
historical record of the earlier T7 capability check. It binds the old source
layout and output hashes, so do not rewrite it or present it as the final
freeze manifest after modularization and work item A.

### Why this is required

The last full technical entry evidence was produced before the modularization
commit and before the guide/export-contract work. A successful check at an
older commit does not prove the final binary emits the same boundaries,
parents, roles, or stable keys. Tests at the actual freeze commit establish
which code produced the rows the annotators will judge.

Hashes for the database, Scryfall inputs, CR file, protocol, guide, and earlier
exports make the result reproducible and prevent later source drift from being
mistaken for a Legends finding. The aggregate-only run supplies the export hash
needed for attestations without exposing a row.

### Completion evidence

- clean recorded commit, with any later signature-only governance artifacts
  explicitly listed;
- release build, Rust tests, formatting, Clippy, and Python suites pass;
- the snapshot manifest validates against locally available artifacts; the
  final experiment manifest is created and validated in work item E;
- repeated export hashes match and all aggregate safety checks pass.

## 8. Work item E — retain the exact export and populate frozen inputs

### Required work

After the roles and attestations are recorded, generate the exact TSV at the
verified freeze commit without opening it:

```powershell
New-Item -ItemType Directory -Force -Path docs/audits/leg
python scripts/python/export_units.py leg `
  --mtg .\target\release\mtg-discover.exe `
  --exclude-heldout > docs/audits/leg/units-export.tsv
Get-FileHash -Algorithm SHA256 docs/audits/leg/units-export.tsv
```

The retained hash must equal the expected hash printed by the immediately
preceding aggregate-only verifier. If it does not, stop without opening the
file.

Copy the frozen export into two unopened pass files only after the hash
matches. When each pass is initialized for annotation, preserve every export
column and stable key and add only the annotation columns defined by protocol
§4.2.

Create a new final experiment manifest, for example
`docs/manifests/experiment-legends-freeze-2026-08-26.json`, modeled on the
existing experiment-manifest schema. Bind the actual freeze commit, current
source files and hashes, snapshot manifest, held-out policy/digest, commands,
environment, aggregate verification result, and the retained TSV path/hash.
Mark that TSV as retained. Validate the new manifest together with the
snapshot manifest:

```powershell
python scripts/python/verify_manifests.py `
  docs/manifests/snapshot-scryfall-2026-08-25.json `
  docs/manifests/experiment-legends-freeze-2026-08-26.json
```

Populate preregistration §3 from live command output, including:

- P-ATQ acceptance path/decision/commit;
- freeze commit and status;
- protocol, preregistration, and guide identities/hashes;
- release build and exact test results;
- three Scryfall snapshot identities and `cards.sqlite` identity;
- CR effective date and hash;
- live corpus information and aggregate Legends summary;
- earlier `lea`, `leb`, `arn`, and `atq` export/annotation hashes;
- held-out pool digest and incident registry;
- exact export command and retained TSV hash;
- the three review identities.

Record the pre-audit baseline using aggregate-only commands before anyone
opens a row:

```powershell
.\target\release\mtg-discover.exe info
.\target\release\mtg-discover.exe sets
.\target\release\mtg-discover.exe audit summary leg --exclude-heldout
```

Validate that `docs/findings/leg-structural-audit.md` remains an empty outline.
Update `docs/gates/legends-entry-record.md` items 4–10 with the actual evidence;
do not copy expected counts or hashes when live commands provide them.

### Why this is required

The T7 gate proved that a safe export can be produced, but deliberately
discarded the row-bearing bytes. Annotators cannot work from a capability
proof; they need one immutable artifact. Matching the retained file to the
aggregate-only hash proves it is the same unseen object that passed held-out,
determinism, stable-key, and parent-integrity checks.

The frozen-input block makes the denominators and source identities known
before card inspection. That is what prevents later observations from changing
the baseline, guide, or sample and makes future drift measurable rather than
silent.

### Completion evidence

- retained `docs/audits/leg/units-export.tsv` with matching SHA-256;
- two unopened pass copies with the same stable keys;
- validated final experiment manifest binding the retained export and current
  freeze inputs;
- populated preregistration §3 and aggregate baseline;
- entry items 4–10 marked satisfied with paths and live values;
- empty Legends findings outline confirmed.

## 9. Work item F — authorize opening

### Required work

The program owner reviews items 1–10 in
`docs/gates/legends-entry-record.md` and completes the §5.3 authorization with
the actual commit, test result, export hash, aggregate exclusion result, role
identities, and registry binding. Then mark checklist item 11 satisfied and
change the entry record's readiness statement to **authorized to open**.

Only after that signature may pass 1 and pass 2 open their copies. Populate
the Legends findings document's frozen-input and scope sections at that point;
empirical findings still wait for sealed passes and adjudication.

### Why this is required

The authorization is the preregistered boundary between preparation and data
observation. It makes it unambiguous whether a row was read before the fixed
inputs, instructions, roles, and held-out exclusion were in place. It does not
approve a classifier change or Gate 1; it only opens the declared development
audit.

### Completion evidence

- signed §5.3 authorization;
- all eleven entry items satisfied;
- absolute opening date and role identities recorded.

## 10. Work explicitly not required before opening Legends

Do not put the following on tonight's critical path:

- **A calibration exercise.** It may improve the guide later, but neither the
  protocol nor the Legends entry checklist requires it. The lead can ratify
  the guide's already documented conventions directly.
- **A shorter guide or separate quick-reference document.** One frozen guide
  is already drafted. Editing for convenience does not satisfy another entry
  item.
- **Resolution of D14 or D19.** The preregistration explicitly keeps them
  observational and separate. Legends is intended to collect evidence about
  them.
- **Another Antiquities audit or P-ATQ acceptance pass.** Those prerequisites
  are recorded as satisfied. Reopen them only for a concrete counterexample.
- **Gate 1 completion, a cross-era gold set, or held-out annotation.** The Gate
  1 matrix explicitly says these block Gate 1, not Legends.
- **Parser, semantic IR, engine, simulation, or ML work.** These remain outside
  the active research frontier.
- **Roadmap prose cleanup or stale post-modularization path references.** They
  should be corrected separately, but they do not change the frozen export or
  any Legends entry condition.
- **Additional literature or web research.** The remaining decisions concern
  the repository's already frozen procedure and live export contract.
- **Different model vendors for each role.** The recorded requirement is
  independent passes and a separate adjudicator, not model diversity.

## 11. Final opening checklist

- [ ] `prefix` is present in native and TSV audit exports, documented, and
      tested; H5 can be measured from the frozen rows.
- [ ] Guide U1–U9 have one recorded Legends-v1.0 disposition.
- [ ] Guide is frozen and its hash recorded externally.
- [ ] Pass 1, pass 2, and adjudicator identities are assigned.
- [ ] Independence/non-observation attestations are signed and bound to the
      expected export hash.
- [ ] Clean freeze commit is recorded.
- [ ] Release build, Rust tests, fmt, Clippy, and Python suites pass there.
- [ ] Snapshot, CR, protocol, preregistration, guide, database, and earlier
      exports are hashed or manifest-bound.
- [ ] Aggregate-only T7 verification passes twice with zero held-out records.
- [ ] Exact TSV is retained without inspection and matches the verified hash.
- [ ] Preregistration §3 and the aggregate baseline are populated from live
      output.
- [ ] Entry record items 1–10 are satisfied.
- [ ] Program-owner authorization is signed and item 11 is satisfied.

At the completion of this checklist, stop preparing and begin the two Legends
annotation passes. Further planning at that point would delay the evidence the
project needs next.
