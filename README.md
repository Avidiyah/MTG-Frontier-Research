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
| `.gitignore` | Keeps the bulk downloads and the SQLite store out of version control |

## Quick start

```
python mtg_card_pipeline.py all     # fetch, load, and analyze
python mtg_search.py                # interactive card lookup
```

`all` takes a while on first run — it downloads roughly 30 MB and then walks
every card in the pool. The three stages can also be run individually
(`fetch`, `load`, `analyze`), which matters if a stage fails: a completed
download does not need repeating.

## The pipeline

### `fetch`

Queries `https://api.scryfall.com/bulk-data` for the current download URLs,
then streams two datasets to the repository root:

- **`oracle-cards.jsonl.gz`** (~24 MB) — one entry per distinct card, keyed by
  `oracle_id`. This is the deduplicated set: every printing of Lightning Bolt
  collapses to a single record, which is what you want when studying effects
  rather than print runs.
- **`rulings.jsonl.gz`** (~5 MB) — official Oracle rulings, joined to cards by
  `oracle_id`.

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
      legalities, is_dfc)

rulings(oracle_id, published_at, comment)   -- indexed on oracle_id
```

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
