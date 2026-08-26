# MTG Frontier Research

Tooling for the **"Effect Parsing" frontier**: getting every Magic: The Gathering
card's Oracle text into a queryable local store, then measuring how much of the
card pool collapses into a small number of recognizable structural patterns.

The motivating question is whether Magic's ~30,000 distinct cards are really
30,000 distinct *effects*, or whether they are a few thousand templates with
the nouns and numbers swapped out. If it's the latter, a parser only has to
learn the templates. The `analyze` step in this repo is a cheap first
measurement of that ratio, meant to be run before investing in a real parser.

Everything here is **standard library only** — no `pip install` step, which
also means it runs in constrained environments like Termux on Android.

## Contents

| File | What it is |
|---|---|
| `mtg_card_pipeline.py` | Fetch Scryfall bulk data → load into SQLite → template frequency report |
| `mtg_search.py` | Look up individual cards by name in the built database |
| `mtg-discover` | Rust CLI for agent-driven corpus and rules research |
| `.gitignore` | Keeps the bulk downloads and the SQLite store out of version control |

## Quick start

```
python mtg_card_pipeline.py all     # fetch, load, and analyze
python mtg_search.py                # interactive card lookup
```

## Rust discovery CLI

`mtg-discover` provides deterministic, structured research tools for terminal
agents. Every successful command writes one JSON document to standard output;
errors go to standard error with a nonzero exit status. Paths default to
`cards.sqlite` and `Magic-Comprehensive_Rules.md` in the repository root and
can be overridden globally with `--db` and `--rules`.

Build it once:

```powershell
cargo build --release
$mtg = ".\target\release\mtg-discover.exe"
```

Available discovery operations:

```powershell
& $mtg info
& $mtg cards "draw a card" --field text --limit 20
& $mtg cards "Phyrexian" --field type --limit 20
& $mtg card "Lightning Bolt" --rulings
& $mtg rules search "trigger condition" --limit 20
& $mtg rules show 603.1
& $mtg segment --card "Cryptic Command"
& $mtg segment --name "Example" --text "When Example enters, draw a card."
& $mtg templates --limit 100 --min-count 2
& $mtg sets --until 1995-12-31                 # first-printing sets in release order
& $mtg templates --set lea                     # restrict to one first-printing set
& $mtg cards "Regenerate" --field text --set lea
```

`cards` performs literal case-insensitive matching, so `%` and `_` are not
treated as SQL wildcards. It searches names, Oracle text, and type lines by
default; `--field` narrows that scope. `card` requires an exact name or Oracle
ID and can join official rulings. `rules search` searches both numbered rules
and glossary entries, while `rules show` returns a rule and all descendants.

`segment` is intentionally an observable baseline rather than a claim of full
parsing. It separates card faces and Oracle-text lines and emits a tree of
units. Each unit carries three independent labels: `kind` (heuristic CR
category: keyword, activated, triggered, replacement effect, cast
restriction, additional cost, characteristic-defining ability, ante
instruction, or residual spell/static text), `role` (`ability`, `mode`,
`delayed_trigger`, or `granted`), and `source` (`printed`, or
`rules_supplied` for reminder-only lines such as basic lands, with a CR
citation when inferable). Keyword lists (`Flying, trample`) become one unit
per keyword; `•` modes nest under their header; a `At the beginning of ...
next ...` delayed trigger nests under the unit that creates it; quoted
abilities nest under the granting unit and are replaced by `"[ability]"` in
the parent's template. `text` is the unit's printed text with reminder text
removed and `line` points back to the source line. `templates` applies that
same segmentation over the complete corpus, counts printed units (reporting
rules-supplied units separately, plus kind and role histograms), excludes face
separators, and reports a ranked coverage curve. `sets` lists
the sets in which cards were *first* printed, in release order, so the corpus
can be walked era by era; `--set <code>` on `cards` and `templates` restricts
them to cards first printed in that set.

### Suggested agent research loop

1. Run `info` to record the corpus and rules versions used by an experiment.
2. Form a narrow, falsifiable hypothesis about Oracle wording.
3. Use `cards` to locate examples and counterexamples.
4. Use `card --rulings` and `rules search/show` to inspect authoritative
   semantics rather than infer them from card text alone.
5. Use `segment` to test how the current structural baseline represents each
   example.
6. Run `templates` when changing normalization or segmentation and compare its
   total units, distinct templates, and coverage checkpoints.
7. Record both supporting and contradicting examples. Do not treat frequency
   as proof of semantic equivalence.

`all` takes a while on first run — it downloads roughly 110 MB and then walks
every card in the pool. The three stages can also be run individually
(`fetch`, `load`, `analyze`), which matters if a stage fails: a completed
download does not need repeating.

## The pipeline

### `fetch`

Queries `https://api.scryfall.com/bulk-data` for the current download URLs,
then streams three datasets to the repository root:

- **`oracle-cards.jsonl.gz`** (~24 MB) — one entry per distinct card, keyed by
  `oracle_id`. This is the deduplicated set: every printing of Lightning Bolt
  collapses to a single record, which is what you want when studying effects
  rather than print runs.
- **`rulings.jsonl.gz`** (~5 MB) — official Oracle rulings, joined to cards by
  `oracle_id`.
- **`default-cards.jsonl.gz`** (~78 MB) — every printing of every card. Used
  only to derive each card's *first* printing: the `oracle_cards` record is
  an arbitrary recent printing (Lightning Bolt's is a 2026 Commander deck),
  so it cannot say which set introduced a card.

Scryfall exposes each dataset at two URLs. This code prefers
`jsonl_download_uri`, which serves gzipped line-delimited JSON, over
`download_uri`, which serves the same data as one uncompressed JSON array many
times larger. Downloads print percentage progress when the server reports a
content length.

Files land in the repository root, resolved relative to the script rather than
the working directory, so the paths hold no matter where you invoke it from.

### `load`

Parses the bulk files into `cards.sqlite`.

The reader sniffs both the compression and the record framing rather than
assuming either: gzip magic bytes decide the opener, and the first
non-whitespace character decides whether to parse a JSON array or
line-delimited records. Line-delimited input streams record by record; the
array form has to be read into memory at once, which is why the compressed
JSONL endpoint is preferred.

Schema:

```sql
cards(oracle_id PRIMARY KEY, name, mana_cost, cmc, type_line, oracle_text,
      power, toughness, loyalty, keywords, colors, color_identity,
      legalities, is_dfc,
      first_set, first_set_name, first_set_type, first_released_at,
      first_is_fallback)                      -- indexed on first_set

rulings(oracle_id, published_at, comment)   -- indexed on oracle_id
```

`load` drops and recreates `cards` each run. The `first_*` columns hold the
earliest paper, non-promo printing outside promo/token/memorabilia/minigame/
alchemy sets; cards with no such printing (digital-only, promo-only, art
series) fall back to their earliest printing of any kind with
`first_is_fallback = 1` so analyses can exclude them. They are NULL only if
`default-cards.jsonl.gz` is absent.

`keywords`, `colors`, `color_identity`, and `legalities` are stored as JSON
text. SQLite's JSON functions can query them in place, so they stay queryable
without a normalized side table.

**Double-faced cards** (transform cards, modal DFCs, split cards) carry no
top-level `oracle_text` — their text lives per-face in `card_faces`. The loader
detects this, joins the faces' text with a `//` separator, sets `is_dfc = 1`,
and takes power/toughness from the front face.

### `analyze`

Reduces each card's Oracle text to a structural template and counts how often
each template appears.

Normalization, applied per line rather than per card:

| Transform | Effect |
|---|---|
| Card name → `~` | `Lightning Bolt deals` → `~ deals` |
| Reminder text stripped | `(This creature can't be blocked...)` → removed |
| Mana symbols → `{M}` | `{2}{U}{U}` → `{M}{M}{M}` |
| Numbers → `N` | `3 damage` → `N damage` |
| Whitespace collapsed | — |

Lines are counted separately because a card with three abilities is a
*composite* of three simpler templates, and the interesting question is how
often the atoms repeat, not how often whole cards do. Under this
normalization Lightning Bolt and Shock both become
`~ deals N damage to any target.` — the two cards are one template.

The report prints the most common templates with their share of all ability
lines, then a coverage curve: what fraction of every ability line in Magic the
top 10, 25, 50, 100, 250, 500, and 1000 templates account for. That curve is
the actual output of interest — it estimates how much of the card pool a
template-recognition approach would cover before any real parsing is needed.

## The search tool

```
python mtg_search.py                      # prompt for searches, blank line quits
python mtg_search.py lightning bolt       # multi-word, no quotes needed
python mtg_search.py bolt -e              # exact name match
python mtg_search.py jace -n 20           # raise the result cap (default 10)
python mtg_search.py black lotus -r       # include Oracle rulings
python mtg_search.py bolt -q              # names only, for piping
```

Results rank exact names first, then names beginning with the search term,
then names merely containing it, shortest name first within each tier. So
`bolt` returns Bolt, then Bolt Bend, then Lightning Bolt.

The database opens read-only — searching cannot modify the store. LIKE
wildcards in the search term are escaped, so a literal `%` or `_` matches
itself. Output wraps to the terminal width. Exit status is 0 on a hit, 1 on no
match, 2 if the database hasn't been built yet.

## Data files and version control

`oracle-cards.jsonl.gz`, `rulings.jsonl.gz`, and `cards.sqlite` are
gitignored. They are large, they are regenerable in one command, and they go
stale every time Scryfall publishes a new bulk drop — all three are reasons
not to put them in git history, which keeps every version forever. A clean
`git status` with those files present on disk is correct behavior.

## Known limitations

- **`fetch` needs real network access** to `data.scryfall.io`. It will not run
  in a sandboxed environment that blocks it.
- **The array-parsing path reads the whole file into memory.** Fine for
  `oracle_cards`; for the much larger `default_cards` or `all_cards` datasets,
  use the JSONL endpoint or switch to `ijson` for true streaming.
- **Double-faced cards lose back-face stats.** Only the front face's
  power/toughness and the top-level mana cost are stored, so a modal DFC's back
  side is not fully represented.
- **Name matching is ASCII case-insensitive only.** SQLite's built-in `LIKE`
  won't fold accents, so `aether` does not match `Æther`.
- **The template normalizer is deliberately crude.** It does not distinguish a
  creature name from a subtype, doesn't understand ability words, and treats
  every number as interchangeable — a `{1}` cost and "draw 1 card" both become
  `N`. It is a measurement instrument, not a parser.
- **`NAME_TOKEN_RE` in the pipeline is unused.** It was intended as a
  proper-noun catcher for normalization and is currently dead code.
- **Searching is by name only.** There is no way yet to query by template,
  which is the natural next tool given what `analyze` produces.
