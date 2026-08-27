use std::path::Path;

use anyhow::{Result, bail};
use rusqlite::params;
use serde::Serialize;
use serde_json::{Value, json};

use crate::cli::{CardArgs, CardSearchArgs, SetsArgs};
use crate::database::{
    held_out_exclusion_metadata, held_out_exclusion_predicate, open_db, set_predicate,
};
use crate::rules::{parse_rules, read_rules};

#[derive(Serialize)]
struct Card {
    oracle_id: String,
    name: String,
    mana_cost: Option<String>,
    cmc: f64,
    type_line: Option<String>,
    oracle_text: Option<String>,
    power: Option<String>,
    toughness: Option<String>,
    loyalty: Option<String>,
    keywords: Value,
    colors: Value,
    color_identity: Value,
    legalities: Value,
    is_dfc: bool,
    first_set: Option<String>,
    first_set_name: Option<String>,
    first_set_type: Option<String>,
    first_released_at: Option<String>,
    first_is_fallback: bool,
}

#[derive(Serialize)]
struct Ruling {
    published_at: Option<String>,
    comment: Option<String>,
}

pub(crate) fn command_info(db_path: &Path, rules_path: &Path) -> Result<Value> {
    let conn = open_db(db_path)?;
    let cards: i64 = conn.query_row("SELECT count(*) FROM cards", [], |row| row.get(0))?;
    let with_text: i64 = conn.query_row(
        "SELECT count(*) FROM cards WHERE oracle_text IS NOT NULL AND oracle_text != ''",
        [],
        |row| row.get(0),
    )?;
    let dfcs: i64 = conn.query_row("SELECT count(*) FROM cards WHERE is_dfc = 1", [], |row| {
        row.get(0)
    })?;
    let rulings: i64 = conn.query_row("SELECT count(*) FROM rulings", [], |row| row.get(0))?;
    let with_first: i64 = conn.query_row(
        "SELECT count(*) FROM cards WHERE first_set IS NOT NULL",
        [],
        |row| row.get(0),
    )?;
    let first_sets: i64 = conn.query_row(
        "SELECT count(DISTINCT first_set) FROM cards WHERE first_set IS NOT NULL",
        [],
        |row| row.get(0),
    )?;
    let rules_text = read_rules(rules_path)?;
    let parsed_rules = parse_rules(&rules_text);
    let numbered = parsed_rules
        .iter()
        .filter(|entry| entry.id.is_some())
        .count();
    let glossary = parsed_rules
        .iter()
        .filter(|entry| entry.kind == "glossary")
        .count();

    Ok(json!({
        "database": db_path,
        "rules_file": rules_path,
        "cards": {
            "total": cards,
            "with_oracle_text": with_text,
            "without_oracle_text": cards - with_text,
            "multi_face": dfcs,
            "with_first_printing": with_first,
            "first_printing_sets": first_sets
        },
        "rulings": rulings,
        "rules": {
            "numbered_entries": numbered,
            "glossary_entries": glossary,
            "effective_date": rules_text.lines().nth(2)
        }
    }))
}

pub(crate) fn command_cards(db_path: &Path, args: CardSearchArgs) -> Result<Value> {
    let conn = open_db(db_path)?;
    let escaped = escape_like(&args.query);
    let pattern = format!("%{escaped}%");
    let predicate = match args.field.as_str() {
        "name" => "name LIKE ?1 ESCAPE '\\'",
        "text" => "oracle_text LIKE ?1 ESCAPE '\\'",
        "type" => "type_line LIKE ?1 ESCAPE '\\'",
        "all" => {
            "(name LIKE ?1 ESCAPE '\\' OR oracle_text LIKE ?1 ESCAPE '\\' \
             OR type_line LIKE ?1 ESCAPE '\\')"
        }
        _ => unreachable!(),
    };
    let set_filter = set_predicate("?5");
    let held_out_filter = held_out_exclusion_predicate("?6");
    let sql = format!(
        "SELECT oracle_id, name, mana_cost, cmc, type_line, oracle_text, power, \
         toughness, loyalty, keywords, colors, color_identity, legalities, is_dfc, \
         first_set, first_set_name, first_set_type, first_released_at, first_is_fallback \
         FROM cards WHERE {predicate}{set_filter}{held_out_filter} \
         ORDER BY CASE WHEN lower(name) = lower(?2) THEN 0 \
                       WHEN lower(name) LIKE lower(?2) || '%' THEN 1 ELSE 2 END, \
                  length(name), name LIMIT ?3 OFFSET ?4"
    );
    let mut statement = conn.prepare(&sql)?;
    let cards = statement
        .query_map(
            params![
                pattern,
                args.query,
                args.limit,
                args.offset,
                args.set.as_deref().unwrap_or("").to_lowercase(),
                args.exclude_heldout
            ],
            card_from_row,
        )?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(json!({
        "query": args.query,
        "field": args.field,
        "set": args.set,
        "heldout_exclusion": held_out_exclusion_metadata(args.exclude_heldout),
        "limit": args.limit,
        "offset": args.offset,
        "count": cards.len(),
        "cards": cards
    }))
}

pub(crate) fn command_card(db_path: &Path, args: CardArgs) -> Result<Value> {
    let conn = open_db(db_path)?;
    let mut statement = conn.prepare(
        "SELECT oracle_id, name, mana_cost, cmc, type_line, oracle_text, power, \
         toughness, loyalty, keywords, colors, color_identity, legalities, is_dfc, \
         first_set, first_set_name, first_set_type, first_released_at, first_is_fallback \
         FROM cards WHERE lower(name) = lower(?1) OR oracle_id = ?1 \
         ORDER BY CASE WHEN oracle_id = ?1 THEN 0 ELSE 1 END LIMIT 2",
    )?;
    let cards = statement
        .query_map([&args.query], card_from_row)?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    if cards.is_empty() {
        bail!("no exact card name or Oracle ID matched {:?}", args.query);
    }
    if cards.len() > 1 {
        bail!(
            "multiple cards matched {:?}; use the Oracle ID to disambiguate",
            args.query
        );
    }
    let card = &cards[0];
    let rulings = if args.rulings {
        let mut statement = conn.prepare(
            "SELECT published_at, comment FROM rulings \
             WHERE oracle_id = ?1 ORDER BY published_at, rowid",
        )?;
        statement
            .query_map([&card.oracle_id], |row| {
                Ok(Ruling {
                    published_at: row.get(0)?,
                    comment: row.get(1)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?
    } else {
        Vec::new()
    };

    Ok(json!({
        "card": card,
        "rulings_included": args.rulings,
        "rulings": rulings
    }))
}

fn card_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Card> {
    Ok(Card {
        oracle_id: row.get(0)?,
        name: row.get(1)?,
        mana_cost: row.get(2)?,
        cmc: row.get(3)?,
        type_line: row.get(4)?,
        oracle_text: row.get(5)?,
        power: row.get(6)?,
        toughness: row.get(7)?,
        loyalty: row.get(8)?,
        keywords: json_column(row.get::<_, String>(9)?),
        colors: json_column(row.get::<_, String>(10)?),
        color_identity: json_column(row.get::<_, String>(11)?),
        legalities: json_column(row.get::<_, String>(12)?),
        is_dfc: row.get::<_, i64>(13)? != 0,
        first_set: row.get(14)?,
        first_set_name: row.get(15)?,
        first_set_type: row.get(16)?,
        first_released_at: row.get(17)?,
        first_is_fallback: row.get::<_, i64>(18)? != 0,
    })
}

fn json_column(raw: String) -> Value {
    serde_json::from_str(&raw).unwrap_or(Value::String(raw))
}

pub(crate) fn command_sets(db_path: &Path, args: SetsArgs) -> Result<Value> {
    let conn = open_db(db_path)?;
    let mut statement = conn.prepare(
        "SELECT first_set, first_set_name, first_set_type, first_released_at, \
         count(*), sum(CASE WHEN oracle_text IS NOT NULL AND oracle_text != '' THEN 1 ELSE 0 END), \
         sum(first_is_fallback) \
         FROM cards WHERE first_set IS NOT NULL \
         AND (?1 = '' OR first_set_type = ?1) \
         AND (?2 = '' OR first_released_at <= ?2) \
         GROUP BY first_set ORDER BY first_released_at, first_set",
    )?;
    let sets = statement
        .query_map(
            params![
                args.set_type.as_deref().unwrap_or(""),
                args.until.as_deref().unwrap_or("")
            ],
            |row| {
                Ok(json!({
                    "set": row.get::<_, String>(0)?,
                    "name": row.get::<_, Option<String>>(1)?,
                    "type": row.get::<_, Option<String>>(2)?,
                    "released_at": row.get::<_, Option<String>>(3)?,
                    "cards": row.get::<_, i64>(4)?,
                    "cards_with_text": row.get::<_, i64>(5)?,
                    "fallback_cards": row.get::<_, i64>(6)?
                }))
            },
        )?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    let unknown: i64 = conn.query_row(
        "SELECT count(*) FROM cards WHERE first_set IS NULL",
        [],
        |row| row.get(0),
    )?;
    Ok(json!({
        "type": args.set_type,
        "until": args.until,
        "count": sets.len(),
        "cards_without_first_printing": unknown,
        "first_printing_rule": "earliest paper, non-promo printing outside promo/token/memorabilia/minigame/alchemy sets; fallback = earliest printing of any kind",
        "sets": sets
    }))
}

pub(crate) fn escape_like(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}
