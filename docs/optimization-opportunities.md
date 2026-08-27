# Memory, speed, and small-scope architectural optimizations

**Status:** backlog, except O1 as noted below  
**Recorded:** 2026-08-26  
**Scope:** behavior-preserving improvements to the existing research tools

This document records measured or testable optimizations that may reduce
runtime, memory use, or repeated allocation without changing the project's
research object. These are not parser, IR, engine, storage-platform, or
infrastructure proposals. In particular, an optimization to segmentation or
normalization is acceptable only when the complete affected output remains
byte-for-byte identical.

## Decision rules

- Measure before changing code and compare against the same database, rules
  snapshot, build mode, command, and machine.
- Prefer a small local change over a new dependency or architectural rewrite.
- Compare complete output where practical, not only aggregate counts.
- Reject an optimization that changes unit boundaries, normalized text, kinds,
  roles, ordering guarantees, error behavior, or unsupported-case visibility.
- Keep changes only when the measured gain is material relative to their
  maintenance and validation cost.

## O1 — Stream the Python baseline query in batches

**Target:** `scripts/python/mtg_card_pipeline.py::analyze_templates`  
**Status:** implemented 2026-08-26  
**Risk:** low

### Previous behavior

The Python baseline queried every card with Oracle text and immediately called
`fetchall()`. It therefore materialized the full SQLite result before beginning
normalization:

```python
rows = cur.fetchall()
```

The corpus is not large enough to make this fail, but retaining every row is
unnecessary because template counting is incremental.

### Measured evidence

A five-run benchmark against the local `cards.sqlite` compared the original
implementation with `fetchmany(1024)`:

| Variant | Median runtime | Peak Python memory |
|---|---:|---:|
| `fetchall()` | 2829.8 ms | 19.21 MB |
| `fetchmany(1024)` | 2450.9 ms | 7.10 MB |

The streamed version was approximately 13% faster and used approximately 63%
less peak Python memory. Template counters and totals were identical.

The absolute runtime saving is only about 0.38 seconds, so this is primarily a
simple memory improvement rather than a major pipeline acceleration.

### Implementation

Process rows in batches of 1,024 while maintaining the same query and
normalization loop. Keep the connection open until iteration completes and
close it in `finally`, including when querying or normalization raises.

### Possible side effects

- The SQLite connection and read cursor remain open during normalization,
  whereas `fetchall()` allowed the connection to close before CPU processing.
- A longer-lived read transaction could temporarily delay a concurrent writer
  under rollback-journal SQLite. The pipeline is normally single-process, so
  the practical risk is low.
- The batch size affects performance and memory, not the result. A future
  benchmark may select another size without making it a semantic change.
- The query has no `ORDER BY`. Streaming preserves SQLite's delivery order
  within an execution, as `fetchall()` did, but tied `Counter.most_common()`
  presentation was already not formally stable across database rebuilds.
- Cleanup must remain exception-safe. A transaction context manager alone
  should not be mistaken for guaranteed immediate connection closure.

### Acceptance checks

Run:

```powershell
python scripts/python/mtg_card_pipeline.py analyze
```

Compare the complete output before and after, including total lines, distinct
templates, every reported template/count, and the coverage curve.

## O2 — Reuse card-name normalization context

**Target:** `src/main.rs::normalize_text` and the `segment_text` call tree  
**Status:** deferred pending benchmark  
**Risk:** medium

### Current repeated work

Every normalization call reconstructs:

- a vector containing the full card name and face names;
- each `named <name>` shield string;
- each sentinel replacement string.

This setup repeats for every structural unit and for recursively created
delayed-trigger and granted children. Cards with several lines, modes, quotes,
or multiple faces repeat the same allocations.

For an ordinary single-face card, the current construction also adds the same
name twice: once directly and once from `card_name.split(" // ")`. The duplicate
replacement passes are redundant, although removing them must still be proven
output-equivalent.

### Candidate design

Create a card-scoped context once in `segment_text`:

```rust
struct NormalizationContext<'a> {
    names: Vec<&'a str>,
    shields: Vec<(String, String)>,
}
```

Pass `&NormalizationContext` through `segment_line`, `build_unit`,
`split_keyword_list`, and recursive delayed/granted construction. Retain
`normalize_text(text, card_name)` as a compatibility wrapper for tests and
isolated callers.

The context must preserve the current name order: the complete card name first,
then face names. Recursive quoted abilities must use the parent card's
self-reference context.

### Expected benefit

This would eliminate repeated vector and shield-string allocation across the
corpus's roughly 71,000 printed units. The actual gain is unknown: reminder-text
processing, regex replacement, and repeated string reconstruction may dominate.
A prototype must be benchmarked before acceptance.

### Possible side effects

- Reusing the wrong context across cards could replace an unrelated name.
- Multi-face full-name and individual-face replacement order could change.
- `named <name>` shielding could cease to be byte-identical.
- Recursive quoted abilities could lose the parent card's self-reference.
- Direct normalization callers could behave differently from segmentation.
- Removing duplicate single-face names appears equivalent but is part of the
  measured normalization instrument and may not be assumed harmless.

### Acceptance checks

1. Run all normalization and segmentation tests.
2. Compare complete `templates` JSON before and after.
3. Compare every normalized unit in a full corpus dump, not only totals.
4. Exercise single-face names, multi-face full names, individual face names,
   `named <name>`, quoted granted abilities, delayed triggers, and names that
   are substrings of other text.
5. Keep the change only if output is byte-identical and the runtime or allocation
   reduction is material.

## O3 — Avoid normalized-string clones in Rust template counting

**Target:** `src/main.rs::command_templates`  
**Status:** deferred pending comparative benchmark  
**Risk:** low to medium, depending on implementation

### Current repeated work

The command visits each `Segment` by reference and therefore clones its owned
normalized string before inserting it into the template map:

```rust
*counts.entry(unit.normalized.clone()).or_default() += 1;
```

At the current baseline there are about 71,563 printed units and 37,299
distinct templates. An approach that clones only new keys could avoid roughly
34,000 string clones.

### Option A — Look up before cloning

```rust
if let Some(count) = counts.get_mut(unit.normalized.as_str()) {
    *count += 1;
} else {
    counts.insert(unit.normalized.clone(), 1);
}
```

This is a very small change and clones only the first instance of a template.
Its performance is not guaranteed: an existing key needs one lookup, but a new
key requires a failed lookup followed by a second hash during insertion. Since
approximately half the templates are distinct, extra hashing may offset saved
allocations.

Benchmark this option first because it is surgical. Revert it if neutral or
slower.

### Option B — Consume segments and move strings

A templates-specific owned traversal could consume each `Segment` and move
`unit.normalized` into `HashMap::entry`, avoiding both cloning and the second
lookup.

This is the technically stronger allocation optimization but requires more
care:

- preserve parent-before-child traversal;
- visit every child exactly once;
- leave existing borrowed `Segment::walk` callers unchanged;
- avoid exposing a misleading partially emptied `Segment` to a generic visitor;
- preserve the command's output ordering.

The command already sorts equal-frequency templates lexically after counting,
which limits ordering risk. An owned traversal should nevertheless be narrowly
scoped and tested rather than replacing the general borrowed traversal.

Do not add `hashbrown` or another dependency solely for raw-entry APIs.

### Acceptance checks

For each option:

1. Benchmark the release `templates` command repeatedly on the same corpus.
2. Compare the complete JSON output byte-for-byte.
3. Confirm identical printed/rules-supplied/empty totals, distinct count, kind
   and role histograms, rankings, and coverage.
4. If using owned traversal, add a focused nested-parent/child traversal test.
5. Prefer Option A only if it is measurably faster; evaluate Option B only if
   clone allocation is shown to be significant.

## Suggested future order

1. O1: retain the implemented batched Python query after output-equivalence
   validation.
2. O3 Option A: benchmark the minimal conditional-clone change.
3. O3 Option B: evaluate only if Option A is neutral and allocation remains a
   measured bottleneck.
4. O2: prototype last because it touches the shared normalization instrument;
   require complete corpus equivalence and a material gain.

