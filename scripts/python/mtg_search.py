#!/usr/bin/env python3
"""
MTG Card Search
===============
Query the local card store built by mtg_card_pipeline.py. Name search only --
this is the lookup tool you reach for while reading template output and
wondering "wait, what does that card actually say?"

Standard library only. Run `python scripts/python/mtg_card_pipeline.py all`
from the repository root first to build the database.

Run it with no search term for an interactive prompt -- type a name, press
Enter, repeat. A blank line quits. Multi-word names need no quoting.

Usage:
    python scripts/python/mtg_search.py                      # prompt for searches
    python scripts/python/mtg_search.py bolt                 # substring match
    python scripts/python/mtg_search.py lightning bolt       # quoting optional
    python scripts/python/mtg_search.py lightning bolt -e    # exact name only
    python scripts/python/mtg_search.py jace -n 5            # result limit
    python scripts/python/mtg_search.py black lotus -r       # include rulings
    python scripts/python/mtg_search.py bolt -q              # names only
"""

import argparse
import shutil
import sqlite3
import sys
import textwrap
from pathlib import Path

# Resolve generated data from the repository root regardless of the caller's
# working directory.
DB_PATH = Path(__file__).resolve().parent.parent.parent / "cards.sqlite"

COLUMNS = "oracle_id, name, mana_cost, type_line, oracle_text, power, toughness, loyalty, is_dfc"


def escape_like(term: str) -> str:
    """Neutralize LIKE wildcards so a literal % or _ in a name still matches."""
    return term.replace("\\", "\\\\").replace("%", "\\%").replace("_", "\\_")


def search(conn: sqlite3.Connection, term: str, exact: bool, limit: int):
    if exact:
        return conn.execute(
            f"SELECT {COLUMNS} FROM cards WHERE name = ? COLLATE NOCASE "
            f"ORDER BY name LIMIT ?",
            (term, limit),
        ).fetchall()

    # Rank exact hits first, then names starting with the term, then names
    # merely containing it; shorter names win within a tier. So "bolt" gives
    # Bolt, then Bolt Bend, then Lightning Bolt. SQLite's LIKE is already
    # case-insensitive for ASCII (though not for accented names like Aether).
    pattern = escape_like(term)
    return conn.execute(
        f"""SELECT {COLUMNS} FROM cards
            WHERE name LIKE ? ESCAPE '\\'
            ORDER BY CASE
                         WHEN name = ? COLLATE NOCASE THEN 0
                         WHEN name LIKE ? ESCAPE '\\' THEN 1
                         ELSE 2
                     END,
                     LENGTH(name), name
            LIMIT ?""",
        (f"%{pattern}%", term, f"{pattern}%", limit),
    ).fetchall()


def fetch_rulings(conn: sqlite3.Connection, oracle_id: str):
    return conn.execute(
        "SELECT published_at, comment FROM rulings WHERE oracle_id = ? "
        "ORDER BY published_at",
        (oracle_id,),
    ).fetchall()


def format_card(row, width: int, conn=None) -> str:
    oracle_id, name, mana_cost, type_line, oracle_text, power, toughness, loyalty, is_dfc = row

    header = name + (f"   {mana_cost}" if mana_cost else "")
    if is_dfc:
        header += "   [double-faced]"

    out = [header, type_line or ""]

    if oracle_text:
        for line in oracle_text.split("\n"):
            # "//" separates the faces of a double-faced card.
            out.append("" if line == "//" else textwrap.fill(
                line, width=width, initial_indent="  ", subsequent_indent="  "
            ))

    if power is not None:
        out.append(f"  {power}/{toughness}")
    elif loyalty is not None:
        out.append(f"  Loyalty: {loyalty}")

    if conn is not None:
        rulings = fetch_rulings(conn, oracle_id)
        if rulings:
            out.append("")
            out.append("  Rulings:")
            for published_at, comment in rulings:
                out.append(textwrap.fill(
                    f"{published_at}: {comment}", width=width,
                    initial_indent="    - ", subsequent_indent="      ",
                ))

    return "\n".join(out)


def run_query(conn: sqlite3.Connection, term: str, args) -> bool:
    """Search and print. Returns whether anything matched."""
    rows = search(conn, term, args.exact, args.limit)

    if not rows:
        print(f"No cards matching {term!r}.", file=sys.stderr)
        return False

    if args.quiet:
        for row in rows:
            print(row[1])
        return True

    width = max(40, min(shutil.get_terminal_size((80, 24)).columns, 100))
    for i, row in enumerate(rows):
        if i:
            print("-" * width)
        print(format_card(row, width, conn if args.rulings else None))

    if len(rows) == args.limit:
        print(f"\n(stopped at {args.limit}; raise it with -n)", file=sys.stderr)
    return True


def interactive(conn: sqlite3.Connection, args) -> int:
    """Prompt for search terms until a blank line or EOF."""
    if sys.stdin.isatty():
        print("Search cards by name. Blank line or Ctrl-D to quit.")
    while True:
        try:
            term = input("\nSearch: ").strip()
        except (EOFError, KeyboardInterrupt):
            print()
            return 0
        if not term:
            return 0
        run_query(conn, term, args)


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Search the local MTG card store by name."
    )
    parser.add_argument("term", nargs="*",
                        help="name or fragment to search for; multiple words are "
                             "joined, so quoting is optional. Omit to be prompted.")
    parser.add_argument("-e", "--exact", action="store_true",
                        help="match the full name instead of a substring")
    parser.add_argument("-n", "--limit", type=int, default=10,
                        help="maximum results to show (default: 10)")
    parser.add_argument("-r", "--rulings", action="store_true",
                        help="include Oracle rulings for each result")
    parser.add_argument("-q", "--quiet", action="store_true",
                        help="print matching names only, one per line")
    parser.add_argument("--db", type=Path, default=DB_PATH,
                        help=f"path to the card database (default: {DB_PATH.name})")
    args = parser.parse_args()

    if not args.db.exists():
        print(f"No card database at {args.db}\n"
              "Build it first:  python scripts/python/mtg_card_pipeline.py all",
              file=sys.stderr)
        return 2

    # Read-only: searching should never be able to modify the store.
    conn = sqlite3.connect(f"file:{args.db}?mode=ro", uri=True)
    try:
        term = " ".join(args.term).strip()
        if not term:
            return interactive(conn, args)
        return 0 if run_query(conn, term, args) else 1
    finally:
        conn.close()


if __name__ == "__main__":
    sys.exit(main())
