use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use anyhow::{Context, Result, bail};
use clap::{Args, Parser, Subcommand};
use regex::Regex;
use rusqlite::{Connection, OpenFlags, params};
use serde::Serialize;
use serde_json::{Value, json};

const DEFAULT_DB: &str = "cards.sqlite";
const DEFAULT_RULES: &str = "Magic-Comprehensive_Rules.md";

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Path to cards.sqlite. Defaults to the repository root.
    #[arg(long, global = true)]
    db: Option<PathBuf>,

    /// Path to the Comprehensive Rules Markdown file.
    #[arg(long, global = true)]
    rules: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Report corpus sizes and input-file metadata.
    Info,
    /// Search cards by name, Oracle text, or type line.
    Cards(CardSearchArgs),
    /// Retrieve one card and, optionally, all of its rulings.
    Card(CardArgs),
    /// Search or retrieve numbered rules and glossary paragraphs.
    Rules {
        #[command(subcommand)]
        command: RulesCommand,
    },
    /// Split card text into agent-friendly structural units.
    Segment(SegmentArgs),
    /// Measure normalized ability-template frequency and coverage.
    Templates(TemplateArgs),
}

#[derive(Args)]
struct CardSearchArgs {
    /// Literal case-insensitive search query.
    query: String,

    /// Fields to search: name, text, type, or all.
    #[arg(long, default_value = "all", value_parser = ["name", "text", "type", "all"])]
    field: String,

    #[arg(long, default_value_t = 20, value_parser = clap::value_parser!(u32).range(1..=500))]
    limit: u32,

    #[arg(long, default_value_t = 0)]
    offset: u32,
}

#[derive(Args)]
struct CardArgs {
    /// Exact card name (case-insensitive) or Oracle ID.
    query: String,

    /// Include official rulings in chronological order.
    #[arg(long)]
    rulings: bool,
}

#[derive(Subcommand)]
enum RulesCommand {
    /// Search numbered rules and glossary paragraphs.
    Search(RuleSearchArgs),
    /// Retrieve a numbered rule and all of its subrules.
    Show(RuleShowArgs),
}

#[derive(Args)]
struct RuleSearchArgs {
    /// Literal case-insensitive search query.
    query: String,

    #[arg(long, default_value_t = 20, value_parser = clap::value_parser!(u32).range(1..=500))]
    limit: u32,
}

#[derive(Args)]
struct RuleShowArgs {
    /// Rule number, such as 603.1 or 704.5.
    id: String,
}

#[derive(Args)]
struct SegmentArgs {
    /// Exact card name to read from the database.
    #[arg(long, conflicts_with = "text", required_unless_present = "text")]
    card: Option<String>,

    /// Raw Oracle text to segment directly.
    #[arg(long, conflicts_with = "card", required_unless_present = "card")]
    text: Option<String>,

    /// Card name used to normalize self-references with --text.
    #[arg(long, requires = "text")]
    name: Option<String>,
}

#[derive(Args)]
struct TemplateArgs {
    #[arg(long, default_value_t = 50, value_parser = clap::value_parser!(u32).range(1..=5000))]
    limit: u32,

    /// Omit templates occurring fewer than this many times from the result list.
    #[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u32).range(1..))]
    min_count: u32,
}

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
}

#[derive(Serialize)]
struct Ruling {
    published_at: Option<String>,
    comment: Option<String>,
}

#[derive(Clone, Serialize)]
struct RuleEntry {
    id: Option<String>,
    heading: Option<String>,
    text: String,
    line: usize,
    kind: &'static str,
}

#[derive(Serialize)]
struct Segment {
    index: usize,
    face: usize,
    kind: &'static str,
    text: String,
    normalized: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let db_path = cli.db.unwrap_or_else(|| root.join(DEFAULT_DB));
    let rules_path = cli.rules.unwrap_or_else(|| root.join(DEFAULT_RULES));

    let output = match cli.command {
        Command::Info => command_info(&db_path, &rules_path)?,
        Command::Cards(args) => command_cards(&db_path, args)?,
        Command::Card(args) => command_card(&db_path, args)?,
        Command::Rules { command } => command_rules(&rules_path, command)?,
        Command::Segment(args) => command_segment(&db_path, args)?,
        Command::Templates(args) => command_templates(&db_path, args)?,
    };

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

fn open_db(path: &Path) -> Result<Connection> {
    if !path.is_file() {
        bail!("card database not found: {}", path.display());
    }
    Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .with_context(|| format!("failed to open card database: {}", path.display()))
}

fn command_info(db_path: &Path, rules_path: &Path) -> Result<Value> {
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
            "multi_face": dfcs
        },
        "rulings": rulings,
        "rules": {
            "numbered_entries": numbered,
            "glossary_entries": glossary,
            "effective_date": rules_text.lines().nth(2)
        }
    }))
}

fn command_cards(db_path: &Path, args: CardSearchArgs) -> Result<Value> {
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
    let sql = format!(
        "SELECT oracle_id, name, mana_cost, cmc, type_line, oracle_text, power, \
         toughness, loyalty, keywords, colors, color_identity, legalities, is_dfc \
         FROM cards WHERE {predicate} \
         ORDER BY CASE WHEN lower(name) = lower(?2) THEN 0 \
                       WHEN lower(name) LIKE lower(?2) || '%' THEN 1 ELSE 2 END, \
                  length(name), name LIMIT ?3 OFFSET ?4"
    );
    let mut statement = conn.prepare(&sql)?;
    let cards = statement
        .query_map(
            params![pattern, args.query, args.limit, args.offset],
            card_from_row,
        )?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(json!({
        "query": args.query,
        "field": args.field,
        "limit": args.limit,
        "offset": args.offset,
        "count": cards.len(),
        "cards": cards
    }))
}

fn command_card(db_path: &Path, args: CardArgs) -> Result<Value> {
    let conn = open_db(db_path)?;
    let mut statement = conn.prepare(
        "SELECT oracle_id, name, mana_cost, cmc, type_line, oracle_text, power, \
         toughness, loyalty, keywords, colors, color_identity, legalities, is_dfc \
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
    })
}

fn json_column(raw: String) -> Value {
    serde_json::from_str(&raw).unwrap_or(Value::String(raw))
}

fn command_rules(path: &Path, command: RulesCommand) -> Result<Value> {
    let text = read_rules(path)?;
    let entries = parse_rules(&text);
    match command {
        RulesCommand::Search(args) => {
            let query = args.query.to_lowercase();
            let matches: Vec<_> = entries
                .into_iter()
                .filter(|entry| {
                    entry.text.to_lowercase().contains(&query)
                        || entry
                            .heading
                            .as_ref()
                            .is_some_and(|heading| heading.to_lowercase().contains(&query))
                        || entry.id.as_ref().is_some_and(|id| id == &query)
                })
                .take(args.limit as usize)
                .collect();
            Ok(json!({
                "query": args.query,
                "limit": args.limit,
                "count": matches.len(),
                "matches": matches
            }))
        }
        RulesCommand::Show(args) => {
            let requested = args.id.trim_end_matches('.');
            let matches: Vec<_> = entries
                .into_iter()
                .filter(|entry| {
                    entry
                        .id
                        .as_ref()
                        .is_some_and(|id| is_rule_within(id, requested))
                })
                .collect();
            if matches.is_empty() {
                bail!("rule {:?} was not found", args.id);
            }
            Ok(json!({"rule": args.id, "entries": matches}))
        }
    }
}

fn is_rule_within(candidate: &str, requested: &str) -> bool {
    if candidate == requested {
        return true;
    }
    candidate
        .strip_prefix(requested)
        .and_then(|suffix| suffix.chars().next())
        .is_some_and(|first| first == '.' || first.is_ascii_lowercase())
}

fn read_rules(path: &Path) -> Result<String> {
    fs::read_to_string(path)
        .with_context(|| format!("failed to read Comprehensive Rules: {}", path.display()))
}

fn parse_rules(text: &str) -> Vec<RuleEntry> {
    let numbered =
        Regex::new(r"^(\d{3}(?:\.\d+[a-z]?)*)\.?\s+(.+)$").expect("valid numbered-rule regex");
    let lines: Vec<_> = text.lines().collect();
    let glossary_start = lines
        .iter()
        .rposition(|line| *line == "Glossary")
        .unwrap_or(lines.len());
    let mut entries = Vec::new();

    for (index, line) in lines[..glossary_start].iter().enumerate() {
        let trimmed = line.trim();
        if let Some(captures) = numbered.captures(trimmed) {
            entries.push(RuleEntry {
                id: Some(captures[1].to_owned()),
                heading: None,
                text: captures[2].to_owned(),
                line: index + 1,
                kind: "rule",
            });
        }
    }

    let mut index = glossary_start + 1;
    while index < lines.len() {
        while index < lines.len() && lines[index].trim().is_empty() {
            index += 1;
        }
        if index >= lines.len() {
            break;
        }
        let heading_line = index;
        let heading = lines[index].trim().to_owned();
        index += 1;
        let mut paragraphs = Vec::new();
        while index < lines.len() && !lines[index].trim().is_empty() {
            paragraphs.push(lines[index].trim());
            index += 1;
        }
        entries.push(RuleEntry {
            id: None,
            heading: Some(heading),
            text: paragraphs.join(" "),
            line: heading_line + 1,
            kind: "glossary",
        });
    }
    entries
}

fn command_segment(db_path: &Path, args: SegmentArgs) -> Result<Value> {
    let (name, text) = if let Some(card_name) = args.card {
        let conn = open_db(db_path)?;
        conn.query_row(
            "SELECT name, oracle_text FROM cards WHERE lower(name) = lower(?1) LIMIT 1",
            [&card_name],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, Option<String>>(1)?)),
        )
        .with_context(|| format!("card not found: {card_name:?}"))?
    } else {
        (args.name.unwrap_or_default(), args.text)
    };
    let text = text.unwrap_or_default();
    let segments = segment_text(&text, &name);
    Ok(json!({
        "name": if name.is_empty() { Value::Null } else { Value::String(name) },
        "source_text": text,
        "count": segments.len(),
        "segments": segments
    }))
}

fn segment_text(text: &str, card_name: &str) -> Vec<Segment> {
    let mut segments = Vec::new();
    let mut face = 0;
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if line == "//" {
            face += 1;
            continue;
        }
        let kind = classify_segment(line);
        segments.push(Segment {
            index: segments.len(),
            face,
            kind,
            text: line.to_owned(),
            normalized: normalize_text(line, card_name),
        });
    }
    segments
}

fn classify_segment(line: &str) -> &'static str {
    let lower = line.to_lowercase();
    if line.starts_with('•') {
        "mode"
    } else if lower.starts_with("choose ") {
        "modal_header"
    } else if lower.starts_with("when ")
        || lower.starts_with("whenever ")
        || lower.starts_with("at ")
    {
        "triggered_ability"
    } else if line.contains(':') {
        "activated_ability"
    } else if is_keyword_line(line) {
        "keyword_ability"
    } else {
        "spell_or_static_text"
    }
}

fn is_keyword_line(line: &str) -> bool {
    let words = line.split_whitespace().count();
    words <= 8
        && !line.contains('.')
        && !line.contains(':')
        && !line.contains("—")
        && !line.starts_with('•')
}

fn normalize_text(text: &str, card_name: &str) -> String {
    static MANA: OnceLock<Regex> = OnceLock::new();
    static NUMBER: OnceLock<Regex> = OnceLock::new();
    static REMINDER: OnceLock<Regex> = OnceLock::new();
    static WHITESPACE: OnceLock<Regex> = OnceLock::new();
    let mana = MANA.get_or_init(|| Regex::new(r"\{[^{}]+\}").expect("valid mana regex"));
    let number = NUMBER.get_or_init(|| Regex::new(r"\b\d+\b").expect("valid number regex"));
    let reminder =
        REMINDER.get_or_init(|| Regex::new(r"\([^()]*\)").expect("valid reminder regex"));
    let whitespace = WHITESPACE.get_or_init(|| Regex::new(r"\s+").expect("valid whitespace regex"));
    let mut normalized = text.to_owned();
    if !card_name.is_empty() {
        normalized = normalized.replace(card_name, "~");
        for face_name in card_name.split(" // ") {
            normalized = normalized.replace(face_name, "~");
        }
    }
    while reminder.is_match(&normalized) {
        normalized = reminder.replace_all(&normalized, "").into_owned();
    }
    normalized = mana.replace_all(&normalized, "{M}").into_owned();
    normalized = number.replace_all(&normalized, "N").into_owned();
    whitespace.replace_all(normalized.trim(), " ").into_owned()
}

fn command_templates(db_path: &Path, args: TemplateArgs) -> Result<Value> {
    let conn = open_db(db_path)?;
    let mut statement = conn.prepare(
        "SELECT name, oracle_text FROM cards \
         WHERE oracle_text IS NOT NULL AND oracle_text != '' ORDER BY oracle_id",
    )?;
    let rows = statement.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;
    let mut counts: HashMap<String, u64> = HashMap::new();
    let mut total = 0_u64;
    for row in rows {
        let (name, text) = row?;
        for segment in segment_text(&text, &name) {
            if !segment.normalized.is_empty() {
                *counts.entry(segment.normalized).or_default() += 1;
                total += 1;
            }
        }
    }
    let distinct = counts.len();
    let mut ranked: Vec<_> = counts.into_iter().collect();
    ranked.sort_unstable_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    let checkpoints: HashSet<usize> = [10, 25, 50, 100, 250, 500, 1000, 2500, 5000]
        .into_iter()
        .collect();
    let mut cumulative = 0_u64;
    let mut coverage = Vec::new();
    for (index, (_, count)) in ranked.iter().enumerate() {
        cumulative += count;
        let rank = index + 1;
        if checkpoints.contains(&rank) {
            coverage.push(json!({
                "rank": rank,
                "lines": cumulative,
                "percent": percent(cumulative, total)
            }));
        }
    }
    let templates: Vec<_> = ranked
        .into_iter()
        .filter(|(_, count)| *count >= args.min_count as u64)
        .take(args.limit as usize)
        .enumerate()
        .map(|(index, (template, count))| {
            json!({
                "rank": index + 1,
                "count": count,
                "percent": percent(count, total),
                "template": template
            })
        })
        .collect();

    Ok(json!({
        "total_segments": total,
        "distinct_templates": distinct,
        "normalization": {
            "card_self_reference": "~",
            "mana_symbol": "{M}",
            "integer": "N",
            "reminder_text": "removed",
            "face_separator": "excluded"
        },
        "coverage": coverage,
        "result_limit": args.limit,
        "minimum_count": args.min_count,
        "templates": templates
    }))
}

fn percent(part: u64, total: u64) -> f64 {
    if total == 0 {
        0.0
    } else {
        ((part as f64 / total as f64) * 10_000.0).round() / 100.0
    }
}

fn escape_like(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalization_handles_nested_reminder_text_and_self_references() {
        let text = "Example deals 3 damage. (Use {R} (not {G}).)";
        assert_eq!(normalize_text(text, "Example"), "~ deals N damage.");
    }

    #[test]
    fn segmentation_excludes_face_separator_and_tracks_faces() {
        let segments = segment_text("Flying\n//\nWhen this enters, draw a card.", "");
        assert_eq!(segments.len(), 2);
        assert_eq!(segments[0].face, 0);
        assert_eq!(segments[0].kind, "keyword_ability");
        assert_eq!(segments[1].face, 1);
        assert_eq!(segments[1].kind, "triggered_ability");
    }

    #[test]
    fn rules_parser_separates_numbered_rules_and_glossary() {
        let text =
            "Contents\n100. General\n\n100.1. A rule.\n\nGlossary\n\nAbility\nA definition.\n";
        let entries = parse_rules(text);
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].id.as_deref(), Some("100"));
        assert_eq!(entries[1].id.as_deref(), Some("100.1"));
        assert_eq!(entries[2].heading.as_deref(), Some("Ability"));
        assert_eq!(entries[2].kind, "glossary");
    }

    #[test]
    fn like_metacharacters_are_escaped() {
        assert_eq!(escape_like(r"100%_real\value"), r"100\%\_real\\value");
    }

    #[test]
    fn rule_hierarchy_supports_numeric_and_lettered_subrules() {
        assert!(is_rule_within("603.1", "603.1"));
        assert!(is_rule_within("603.1a", "603.1"));
        assert!(is_rule_within("603.1a", "603"));
        assert!(!is_rule_within("603.10", "603.1"));
        assert!(!is_rule_within("603.2", "603.1"));
    }
}
