"""
MTG Card Data Pipeline
=======================
Step 1 of the "Effect Parsing" frontier: get every card's Oracle text into a
queryable local store, then run a first-pass template-frequency analysis to
see how much of the card pool collapses into a small number of recognizable
structural patterns (this operationalizes the roadmap's Level 0-6 ladder).

Run this LOCALLY (not in a sandboxed environment without internet access to
data.scryfall.io). Standard library only -- no third-party packages needed.

Usage:
    python mtg_card_pipeline.py fetch      # download bulk data, rulings, all printings
    python mtg_card_pipeline.py load       # load into SQLite
    python mtg_card_pipeline.py analyze    # template frequency report
    python mtg_card_pipeline.py all        # do all three in sequence
"""

import gzip
import json
import re
import sqlite3
import sys
import urllib.request
from collections import Counter
from pathlib import Path

# Bulk data and the SQLite store live in the repository root (two levels above
# scripts/python/), so paths hold regardless of the working directory you
# invoke from and the .gitignore patterns for the artifacts stay valid.
DATA_DIR = Path(__file__).resolve().parent.parent.parent

DB_PATH = DATA_DIR / "cards.sqlite"
ORACLE_BULK = DATA_DIR / "oracle-cards.jsonl.gz"
RULINGS_BULK = DATA_DIR / "rulings.jsonl.gz"
# Every printing of every card. Only used to derive each card's *first*
# printing: oracle_cards holds one arbitrary (usually recent) printing per
# card, so it cannot answer "which set introduced this card".
DEFAULT_BULK = DATA_DIR / "default-cards.jsonl.gz"

# Printings that do not count as a card's introduction to the game. Digital
# and promo printings are excluded via their own flags; these set types cover
# the remaining non-release products. Alchemy sets are digital and excluded
# by the flag, but listed here so the rule is explicit.
NON_RELEASE_SET_TYPES = {"promo", "token", "memorabilia", "minigame", "alchemy"}

BULK_DATA_ENDPOINT = "https://api.scryfall.com/bulk-data"
USER_AGENT = "MTGAIResearchPipeline/0.1 (personal research project)"


# ---------------------------------------------------------------------------
# Step 1: Fetch
# ---------------------------------------------------------------------------

def _urlopen(url: str, timeout: int, accept: str | None = None):
    """GET a URL with our User-Agent. Raises HTTPError on non-2xx."""
    headers = {"User-Agent": USER_AGENT}
    if accept:
        headers["Accept"] = accept
    return urllib.request.urlopen(
        urllib.request.Request(url, headers=headers), timeout=timeout
    )


def fetch_bulk_data():
    """Look up current bulk-data download URLs, then stream both files to disk."""
    with _urlopen(BULK_DATA_ENDPOINT, timeout=30, accept="application/json") as resp:
        entries = {e["type"]: e for e in json.load(resp)["data"]}

    for type_key, target_path in [
        ("oracle_cards", ORACLE_BULK),
        ("rulings", RULINGS_BULK),
        ("default_cards", DEFAULT_BULK),
    ]:
        entry = entries[type_key]
        # jsonl_download_uri serves gzipped line-delimited JSON (~24 MB for
        # oracle cards); download_uri serves the uncompressed array, which is
        # many times larger. Prefer the small one, fall back if it vanishes.
        url = entry.get("jsonl_download_uri") or entry["download_uri"]
        print(f"Downloading {type_key} from {url} ...")
        with _urlopen(url, timeout=120) as r:
            # Content-Length lets us show progress; these files are large
            # enough that a silent download looks like a hang on a phone.
            total = int(r.headers.get("Content-Length") or 0)
            done = 0
            with open(target_path, "wb") as f:
                while chunk := r.read(1 << 20):
                    f.write(chunk)
                    done += len(chunk)
                    if total:
                        print(f"\r  {100 * done / total:5.1f}%", end="", flush=True)
            if total:
                print()
        print(f"  saved to {target_path} ({target_path.stat().st_size / 1e6:.1f} MB)")


# ---------------------------------------------------------------------------
# Step 2: Load into SQLite
# ---------------------------------------------------------------------------

SCHEMA = """
DROP TABLE IF EXISTS cards;
CREATE TABLE cards (
    oracle_id TEXT PRIMARY KEY,
    name TEXT,
    mana_cost TEXT,
    cmc REAL,
    type_line TEXT,
    oracle_text TEXT,
    power TEXT,
    toughness TEXT,
    loyalty TEXT,
    keywords TEXT,       -- JSON array as text
    colors TEXT,         -- JSON array as text
    color_identity TEXT, -- JSON array as text
    legalities TEXT,     -- JSON object as text
    is_dfc INTEGER DEFAULT 0,
    -- Earliest paper, non-promo printing (see first_printings()). NULL when
    -- default_cards is unavailable or the card has no printing at all.
    first_set TEXT,
    first_set_name TEXT,
    first_set_type TEXT,
    first_released_at TEXT,
    -- 1 when no paper non-promo printing exists and the earliest printing of
    -- any kind was used instead.
    first_is_fallback INTEGER DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_cards_first_set ON cards(first_set);

CREATE TABLE IF NOT EXISTS rulings (
    oracle_id TEXT,
    published_at TEXT,
    comment TEXT
);

CREATE INDEX IF NOT EXISTS idx_rulings_oracle_id ON rulings(oracle_id);
"""


def _open_maybe_gzip(path: Path):
    """Open a bulk file whether or not it landed on disk gzip-compressed."""
    with open(path, "rb") as probe:
        is_gzip = probe.read(2) == b"\x1f\x8b"
    if is_gzip:
        return gzip.open(path, "rt", encoding="utf-8")
    return open(path, "rt", encoding="utf-8")


def _iter_bulk(path: Path):
    """Yield records from a Scryfall bulk file.

    The jsonl_download_uri endpoint serves gzip-compressed line-delimited
    JSON; download_uri serves a single uncompressed JSON array. Sniff both
    axes rather than assume, so either source loads. The line-delimited
    path streams record by record; the array path reads it all into memory,
    which matters for the very large sets (default_cards, all_cards).
    """
    with _open_maybe_gzip(path) as f:
        head = f.read(1)
        while head and head.isspace():
            head = f.read(1)
        f.seek(0)
        if head == "[":
            yield from json.load(f)
        else:
            for line in f:
                line = line.strip().rstrip(",")
                if line and line not in ("[", "]"):
                    yield json.loads(line)


def _record_oracle_id(card: dict) -> str | None:
    """Some layouts (e.g. reversible cards) carry oracle_id only on faces."""
    oracle_id = card.get("oracle_id")
    if oracle_id is None and card.get("card_faces"):
        oracle_id = card["card_faces"][0].get("oracle_id")
    return oracle_id


def first_printings() -> dict[str, tuple[str, str, str, str, int]]:
    """Map oracle_id -> (set, set_name, set_type, released_at, is_fallback).

    The first printing is the earliest-released printing that is paper
    (digital == False), not a promo (promo == False), and not in a
    NON_RELEASE_SET_TYPES set. Cards with no such printing (promo-only cards,
    digital-only cards) fall back to their earliest printing of any kind and
    are flagged so analyses can exclude them.
    """
    if not DEFAULT_BULK.exists():
        print(f"{DEFAULT_BULK.name} not found; first-printing columns will be NULL.")
        return {}
    best: dict[str, tuple[str, str, str, str]] = {}
    fallback: dict[str, tuple[str, str, str, str]] = {}
    n = 0
    for card in _iter_bulk(DEFAULT_BULK):
        n += 1
        oracle_id = _record_oracle_id(card)
        released = card.get("released_at")
        if not oracle_id or not released:
            continue
        entry = (card.get("set"), card.get("set_name"), card.get("set_type"), released)
        eligible = (
            not card.get("digital")
            and not card.get("promo")
            and card.get("set_type") not in NON_RELEASE_SET_TYPES
        )
        bucket = best if eligible else fallback
        current = bucket.get(oracle_id)
        # Tie-break on set code so the result is deterministic across runs.
        if current is None or (released, entry[0]) < (current[3], current[0]):
            bucket[oracle_id] = entry
    result = {oid: (*entry, 0) for oid, entry in best.items()}
    for oid, entry in fallback.items():
        if oid not in result:
            result[oid] = (*entry, 1)
    print(f"Scanned {n} printings; first printing known for {len(result)} cards "
          f"({len(result) - len(best)} fallback).")
    return result


def load_cards(conn: sqlite3.Connection):
    cur = conn.cursor()
    firsts = first_printings()
    n = 0
    missing = 0
    for card in _iter_bulk(ORACLE_BULK):
        # Double-faced / split cards store text per-face in card_faces
        # instead of at the top level.
        is_dfc = 0
        oracle_text = card.get("oracle_text")
        power = card.get("power")
        toughness = card.get("toughness")
        if oracle_text is None and "card_faces" in card:
            is_dfc = 1
            faces = card["card_faces"]
            oracle_text = "\n//\n".join(f.get("oracle_text", "") or "" for f in faces)
            power = faces[0].get("power")
            toughness = faces[0].get("toughness")

        oracle_id = _record_oracle_id(card)
        first = firsts.get(oracle_id)
        if first is None:
            first = (None, None, None, None, 0)
            if firsts:
                missing += 1
        cur.execute(
            """INSERT OR REPLACE INTO cards
               (oracle_id, name, mana_cost, cmc, type_line, oracle_text,
                power, toughness, loyalty, keywords, colors, color_identity,
                legalities, is_dfc, first_set, first_set_name, first_set_type,
                first_released_at, first_is_fallback)
               VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)""",
            (
                oracle_id,
                card.get("name"),
                card.get("mana_cost"),
                card.get("cmc"),
                card.get("type_line"),
                oracle_text,
                power,
                toughness,
                card.get("loyalty"),
                json.dumps(card.get("keywords", [])),
                json.dumps(card.get("colors", [])),
                json.dumps(card.get("color_identity", [])),
                json.dumps(card.get("legalities", {})),
                is_dfc,
                *first,
            ),
        )
        n += 1
    conn.commit()
    print(f"Loaded {n} cards ({missing} without a known first printing).")


def load_rulings(conn: sqlite3.Connection):
    cur = conn.cursor()
    cur.execute("DELETE FROM rulings")
    n = 0
    for ruling in _iter_bulk(RULINGS_BULK):
        cur.execute(
            "INSERT INTO rulings (oracle_id, published_at, comment) VALUES (?,?,?)",
            (ruling.get("oracle_id"), ruling.get("published_at"), ruling.get("comment")),
        )
        n += 1
    conn.commit()
    print(f"Loaded {n} rulings.")


def load_all():
    conn = sqlite3.connect(DB_PATH)
    conn.executescript(SCHEMA)
    load_cards(conn)
    load_rulings(conn)
    conn.close()


# ---------------------------------------------------------------------------
# Step 3: Template frequency analysis
# ---------------------------------------------------------------------------
# Goal: cheaply estimate what fraction of the card pool is "Level 1" template
# recognition (per the roadmap's generalization ladder) before investing in
# a real parser. This is intentionally crude — a normalization pass that
# strips card-specific tokens so structurally identical effects collapse
# into the same bucket.

NAME_TOKEN_RE = re.compile(r"\b[A-Z][a-zA-Z',\- ]{2,40}\b")  # rough proper-noun catcher, refined below
NUMBER_RE = re.compile(r"\b\d+\b")
MANA_SYMBOL_RE = re.compile(r"\{[^{}]+\}")
REMINDER_TEXT_RE = re.compile(r"\([^()]*\)")
ANALYSIS_FETCH_BATCH_SIZE = 1024


def normalize_oracle_text(text: str, card_name: str) -> str:
    """Collapse a card's oracle text down to a structural template."""
    if not text:
        return ""
    t = text
    # Self-references: Scryfall already uses ~ for the card's own name in
    # most cases, but be defensive.
    if card_name:
        t = t.replace(card_name, "~")
    t = REMINDER_TEXT_RE.sub("", t)         # strip reminder text in parens
    t = MANA_SYMBOL_RE.sub("{M}", t)        # collapse all mana symbols
    t = NUMBER_RE.sub("N", t)               # collapse specific quantities
    t = re.sub(r"\s+", " ", t).strip()
    return t


def analyze_templates(top_n: int = 40):
    conn = sqlite3.connect(DB_PATH)
    counter = Counter()
    total = 0
    try:
        cur = conn.cursor()
        cur.execute(
            "SELECT name, oracle_text FROM cards "
            "WHERE oracle_text IS NOT NULL AND oracle_text != ''"
        )
        while rows := cur.fetchmany(ANALYSIS_FETCH_BATCH_SIZE):
            for name, text in rows:
                # Multi-line abilities: treat each line as its own template unit
                # rather than the whole card, since composite cards (Level 2 in the
                # roadmap's ladder) are combinations of simpler line-level templates.
                for line in text.split("\n"):
                    line = line.strip()
                    if not line:
                        continue
                    template = normalize_oracle_text(line, name)
                    if template:
                        counter[template] += 1
                        total += 1
    finally:
        conn.close()

    print(f"\nTotal ability lines analyzed: {total}")
    print(f"Distinct normalized templates: {len(counter)}")
    print(f"\nTop {top_n} most common templates:\n")
    for template, count in counter.most_common(top_n):
        pct = 100 * count / total
        print(f"  {count:6d} ({pct:5.2f}%)  {template}")

    # Coverage curve: what fraction of all lines do the top-K templates cover?
    print("\nCoverage by template rank:")
    cumulative = 0
    checkpoints = {10, 25, 50, 100, 250, 500, 1000}
    for i, (template, count) in enumerate(counter.most_common(), start=1):
        cumulative += count
        if i in checkpoints:
            print(f"  top {i:5d} templates cover {100 * cumulative / total:5.2f}% of ability lines")


# ---------------------------------------------------------------------------

if __name__ == "__main__":
    cmd = sys.argv[1] if len(sys.argv) > 1 else "all"
    if cmd in ("fetch", "all"):
        fetch_bulk_data()
    if cmd in ("load", "all"):
        load_all()
    if cmd in ("analyze", "all"):
        analyze_templates()
    if cmd not in ("fetch", "load", "analyze", "all"):
        print(__doc__)
