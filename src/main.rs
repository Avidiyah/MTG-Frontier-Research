use std::collections::{BTreeMap, HashMap, HashSet};
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
    /// List first-printing sets in release order with card counts.
    Sets(SetsArgs),
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

    /// Restrict to cards whose first printing is this set code (e.g. lea).
    #[arg(long)]
    set: Option<String>,
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

    /// Restrict to cards whose first printing is this set code (e.g. lea).
    #[arg(long)]
    set: Option<String>,
}

#[derive(Args)]
struct SetsArgs {
    /// Restrict to this Scryfall set_type (core, expansion, commander, ...).
    #[arg(long = "type")]
    set_type: Option<String>,

    /// Only include sets released on or before this date (YYYY-MM-DD).
    #[arg(long)]
    until: Option<String>,
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

#[derive(Clone, Serialize)]
struct RuleEntry {
    id: Option<String>,
    heading: Option<String>,
    text: String,
    line: usize,
    kind: &'static str,
}

/// Heuristic Comprehensive-Rules category of a unit's text. Labels are
/// assigned from surface form (see `classify_kind`) and are measurement
/// instruments, not semantic parses.
#[derive(Serialize, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
enum AbilityKind {
    #[serde(rename = "keyword_ability")]
    Keyword,
    #[serde(rename = "activated_ability")]
    Activated,
    #[serde(rename = "triggered_ability")]
    Triggered,
    #[serde(rename = "replacement_effect")]
    Replacement,
    #[serde(rename = "cast_restriction")]
    CastRestriction,
    #[serde(rename = "additional_cost")]
    AdditionalCost,
    #[serde(rename = "characteristic_defining_ability")]
    CharacteristicDefining,
    #[serde(rename = "ante_instruction")]
    Ante,
    #[serde(rename = "spell_or_static_text")]
    SpellOrStatic,
}

/// Where a unit sits in the ability structure of its card.
#[derive(Serialize, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
#[serde(rename_all = "snake_case")]
enum StructuralRole {
    /// A top-level ability (or spell text) of the card.
    Ability,
    /// A `•` mode belonging to the parent modal ability (CR 700.2).
    Mode,
    /// A delayed triggered ability created by the parent's effect (CR 603.7).
    DelayedTrigger,
    /// A quoted ability that the parent grants, gains, or refers to.
    Granted,
}

/// Whether the unit's semantics come from printed text or are supplied by
/// the Comprehensive Rules and merely described in reminder text.
#[derive(Serialize, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
#[serde(rename_all = "snake_case")]
enum TextSource {
    Printed,
    RulesSupplied,
}

#[derive(Serialize, Clone, Debug)]
struct Segment {
    /// Pre-order position within the card, counting nested units.
    index: usize,
    face: usize,
    /// 1-based source line of the printed text this unit was derived from.
    line: usize,
    kind: AbilityKind,
    role: StructuralRole,
    source: TextSource,
    /// Comprehensive Rules citation for a rules-supplied unit, when inferable.
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<&'static str>,
    /// Printed text of the unit with reminder text removed (rules-supplied
    /// units keep the parenthetical because it is all that is printed).
    text: String,
    normalized: String,
    children: Vec<Segment>,
}

impl Segment {
    fn walk<'a>(&'a self, visit: &mut impl FnMut(&'a Segment)) {
        visit(self);
        for child in &self.children {
            child.walk(visit);
        }
    }

    fn set_origin(&mut self, face: usize, line: usize) {
        self.face = face;
        self.line = line;
        for child in &mut self.children {
            child.set_origin(face, line);
        }
    }

    fn assign_indices(&mut self, next: &mut usize) {
        self.index = *next;
        *next += 1;
        for child in &mut self.children {
            child.assign_indices(next);
        }
    }
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
        Command::Sets(args) => command_sets(&db_path, args)?,
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
    let set_filter = set_predicate("?5");
    let sql = format!(
        "SELECT oracle_id, name, mana_cost, cmc, type_line, oracle_text, power, \
         toughness, loyalty, keywords, colors, color_identity, legalities, is_dfc, \
         first_set, first_set_name, first_set_type, first_released_at, first_is_fallback \
         FROM cards WHERE {predicate}{set_filter} \
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
                args.set.as_deref().unwrap_or("").to_lowercase()
            ],
            card_from_row,
        )?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(json!({
        "query": args.query,
        "field": args.field,
        "set": args.set,
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
    let mut total_units = 0;
    for segment in &segments {
        segment.walk(&mut |_| total_units += 1);
    }
    Ok(json!({
        "name": if name.is_empty() { Value::Null } else { Value::String(name) },
        "source_text": text,
        "count": segments.len(),
        "total_units": total_units,
        "segments": segments
    }))
}

/// Split Oracle text into top-level units with nested sub-units.
///
/// One printed line usually yields one unit, but keyword lists yield one
/// unit per keyword, `•` lines attach to the preceding unit as modes, and a
/// line that is a delayed trigger created by the preceding unit's effect
/// attaches to that unit. Reminder-only lines become rules-supplied units.
fn segment_text(text: &str, card_name: &str) -> Vec<Segment> {
    let mut segments: Vec<Segment> = Vec::new();
    let mut face = 0;
    let mut face_start = 0;
    for (line_number, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if line == "//" {
            face += 1;
            face_start = segments.len();
            continue;
        }
        for mut unit in segment_line(line, card_name) {
            unit.set_origin(face, line_number + 1);
            let attach_to = match unit.role {
                StructuralRole::Mode => segments[face_start..].last_mut(),
                StructuralRole::DelayedTrigger => {
                    segments[face_start..].last_mut().filter(|parent| {
                        parent.source == TextSource::Printed && parent.kind != AbilityKind::Keyword
                    })
                }
                _ => None,
            };
            match attach_to {
                Some(parent) => parent.children.push(unit),
                None => segments.push(unit),
            }
        }
    }
    let mut next = 0;
    for segment in &mut segments {
        segment.assign_indices(&mut next);
    }
    segments
}

/// Units derived from one printed line. Roles other than `Ability` are
/// requests to the caller to attach the unit to the preceding unit.
fn segment_line(line: &str, card_name: &str) -> Vec<Segment> {
    let stripped = collapse_whitespace(&strip_reminder_text(line));
    if stripped.is_empty() {
        let inner = line
            .trim()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .trim();
        let normalized = normalize_text(inner, card_name);
        let mut unit = build_unit(inner, card_name);
        unit.source = TextSource::RulesSupplied;
        unit.rule = rules_supplied_rule(&normalized);
        unit.text = line.to_owned();
        return vec![unit];
    }
    if let Some(inner) = stripped.strip_prefix('•') {
        let mut unit = build_unit(inner.trim(), card_name);
        unit.role = StructuralRole::Mode;
        return vec![unit];
    }
    if let Some(keywords) = split_keyword_list(&stripped, card_name) {
        return keywords;
    }
    let mut unit = build_unit(&stripped, card_name);
    if unit.kind == AbilityKind::Triggered
        && unit.role == StructuralRole::Ability
        && delayed_trigger_start().is_match(&unit.normalized)
    {
        unit.role = StructuralRole::DelayedTrigger;
    }
    vec![unit]
}

/// Build a unit from reminder-stripped text, recursing into a trailing
/// delayed trigger (child role `DelayedTrigger`) and quoted abilities
/// (child role `Granted`).
fn build_unit(text: &str, card_name: &str) -> Segment {
    let text = text.trim();
    if let Some(split) = delayed_trigger_split(text) {
        let mut parent = build_unit(&text[..split], card_name);
        let mut child = build_unit(&text[split..], card_name);
        child.role = StructuralRole::DelayedTrigger;
        parent.children.push(child);
        return parent;
    }
    let (dequoted, quoted) = extract_quoted_abilities(text);
    let normalized = normalize_text(&dequoted, card_name);
    let kind = classify_kind(&normalized);
    let children = quoted
        .iter()
        .map(|quote| {
            let mut child = build_unit(quote, card_name);
            child.role = StructuralRole::Granted;
            child
        })
        .collect();
    Segment {
        index: 0,
        face: 0,
        line: 0,
        kind,
        role: StructuralRole::Ability,
        source: TextSource::Printed,
        rule: None,
        text: text.to_owned(),
        normalized,
        children,
    }
}

/// Split a keyword-only line such as `Flying, trample` or `Flying; banding`
/// into one keyword unit per item. Returns `None` for anything that is not
/// a comma/semicolon list of alphabetic keyword items.
fn split_keyword_list(stripped: &str, card_name: &str) -> Option<Vec<Segment>> {
    if !stripped.contains([',', ';'])
        || classify_kind(&normalize_text(stripped, card_name)) != AbilityKind::Keyword
    {
        return None;
    }
    // `Partner with <Name, Title>` carries a comma inside its argument.
    if stripped.to_lowercase().starts_with("partner with ") {
        return None;
    }
    let pieces: Vec<&str> = stripped.split([',', ';']).map(str::trim).collect();
    let is_keyword_item = |piece: &&str| {
        let first_word = piece.split_whitespace().next().unwrap_or_default();
        // Items of one keyword's argument list (`Protection from X, from Y,
        // and from Z`) continue with a conjunction or preposition.
        piece.chars().next().is_some_and(char::is_alphabetic)
            && !matches!(first_word, "and" | "or" | "from")
    };
    if !pieces.iter().all(is_keyword_item) {
        return None;
    }
    Some(
        pieces
            .into_iter()
            .map(|piece| {
                let mut unit = build_unit(piece, card_name);
                unit.kind = AbilityKind::Keyword;
                // List position alone lowercases later items (`Flying, trample`);
                // sentence-case the template so it matches the standalone keyword.
                let mut chars = unit.normalized.chars();
                if let Some(first) = chars.next() {
                    unit.normalized = first.to_uppercase().chain(chars).collect();
                }
                unit
            })
            .collect(),
    )
}

fn classify_kind(normalized: &str) -> AbilityKind {
    static REPLACEMENT: OnceLock<Regex> = OnceLock::new();
    static CDA: OnceLock<Regex> = OnceLock::new();
    // CR 614.1a-d: "instead", "skip", "As ~ enters", "enters with/as/tapped".
    let replacement = REPLACEMENT.get_or_init(|| {
        Regex::new(r"\binstead\b|\bskips?\b|^as ~ enters\b|\benters? (tapped|untapped|with|as|face down)\b|^you may have ~ enter\b")
            .expect("valid replacement regex")
    });
    // CR 604.3: a static ability that defines the object's own characteristics.
    let cda = CDA.get_or_init(|| {
        Regex::new(r"^~'s (power|toughness|power and toughness|colors?|mana value) (is|are) |^~ is (all colors|colorless|every creature type)\b")
            .expect("valid cda regex")
    });
    let lower = normalized.to_lowercase();
    if lower.starts_with("remove ~ from your deck before playing") {
        AbilityKind::Ante
    } else if lower.starts_with("cast ~ only") {
        AbilityKind::CastRestriction
    } else if lower.starts_with("as an additional cost to cast ~") {
        AbilityKind::AdditionalCost
    } else if lower.starts_with("when ")
        || lower.starts_with("whenever ")
        || lower.starts_with("at ")
    {
        AbilityKind::Triggered
    } else if normalized.contains(':') {
        AbilityKind::Activated
    } else if is_keyword_line(normalized) {
        AbilityKind::Keyword
    } else if replacement.is_match(&lower) {
        AbilityKind::Replacement
    } else if cda.is_match(&lower) {
        AbilityKind::CharacteristicDefining
    } else {
        AbilityKind::SpellOrStatic
    }
}

fn is_keyword_line(line: &str) -> bool {
    let words = line.split_whitespace().count();
    words <= 8
        && !line.contains('.')
        && !line.contains(':')
        && !line.contains('"')
        && !line.contains("—")
        && !line.starts_with('•')
}

/// A one-shot trigger phrase of the form `At the beginning of ... next ...`
/// (CR 603.7); the only delayed-trigger surface form detected so far.
fn delayed_trigger_start() -> &'static Regex {
    static START: OnceLock<Regex> = OnceLock::new();
    START.get_or_init(|| {
        Regex::new(r"^At the beginning of [^,]*\bnext\b").expect("valid delayed trigger regex")
    })
}

/// Byte offset at which a trailing delayed-trigger sentence begins, ignoring
/// matches inside quoted abilities.
fn delayed_trigger_split(text: &str) -> Option<usize> {
    static BOUNDARY: OnceLock<Regex> = OnceLock::new();
    let boundary = BOUNDARY.get_or_init(|| {
        Regex::new(r"\. (At the beginning of [^,]*\bnext\b)").expect("valid delayed boundary regex")
    });
    let masked = mask_quoted(text);
    boundary
        .captures(&masked)
        .and_then(|captures| captures.get(1))
        .map(|sentence| sentence.start())
}

/// Replace every character inside double quotes with `_`, preserving byte
/// offsets so positions found in the mask apply to the original text.
fn mask_quoted(text: &str) -> String {
    let mut masked = String::with_capacity(text.len());
    let mut inside = false;
    for ch in text.chars() {
        if ch == '"' {
            inside = !inside;
            masked.push(ch);
        } else if inside {
            masked.push_str(&"_".repeat(ch.len_utf8()));
        } else {
            masked.push(ch);
        }
    }
    masked
}

/// Replace quoted abilities with the placeholder `"[ability]"` and return
/// their contents. A quoted string counts as an ability if it has a cost
/// colon, opens with a trigger word, or is at least four words long (three
/// if it ends with a period). Short quotes such as `"left"` or `"destroy"`
/// are words being named, not abilities, and are left in place.
fn extract_quoted_abilities(text: &str) -> (String, Vec<String>) {
    static QUOTED: OnceLock<Regex> = OnceLock::new();
    let quoted = QUOTED.get_or_init(|| Regex::new(r#""([^"]+)""#).expect("valid quote regex"));
    let mut abilities = Vec::new();
    let dequoted = quoted.replace_all(text, |captures: &regex::Captures| {
        let inner = captures[1].trim();
        if looks_like_ability(inner) {
            abilities.push(inner.to_owned());
            "\"[ability]\"".to_owned()
        } else {
            captures[0].to_owned()
        }
    });
    (dequoted.into_owned(), abilities)
}

fn looks_like_ability(text: &str) -> bool {
    let lower = text.to_lowercase();
    let words = text.split_whitespace().count();
    text.contains(':')
        || lower.starts_with("when ")
        || lower.starts_with("whenever ")
        || lower.starts_with("at ")
        || words >= 4
        || (words >= 3 && text.ends_with('.'))
}

/// CR citation for a reminder-only line, when the reminder text has a form
/// whose rules source is known: a basic-land-type mana ability (CR 305.6).
fn rules_supplied_rule(normalized: &str) -> Option<&'static str> {
    static MANA_ABILITY: OnceLock<Regex> = OnceLock::new();
    let mana_ability = MANA_ABILITY.get_or_init(|| {
        Regex::new(r"^\{M\}: Add \{M\}( or \{M\})*\.?$").expect("valid mana ability regex")
    });
    mana_ability.is_match(normalized).then_some("305.6")
}

fn strip_reminder_text(text: &str) -> String {
    static REMINDER: OnceLock<Regex> = OnceLock::new();
    let reminder =
        REMINDER.get_or_init(|| Regex::new(r"\([^()]*\)").expect("valid reminder regex"));
    let mut stripped = text.to_owned();
    while reminder.is_match(&stripped) {
        stripped = reminder.replace_all(&stripped, "").into_owned();
    }
    stripped
}

fn collapse_whitespace(text: &str) -> String {
    static WHITESPACE: OnceLock<Regex> = OnceLock::new();
    let whitespace = WHITESPACE.get_or_init(|| Regex::new(r"\s+").expect("valid whitespace regex"));
    whitespace.replace_all(text.trim(), " ").into_owned()
}

/// Object words that follow `this` when a permanent or spell refers to itself.
const SELF_REFERENCE_TYPES: &str = "creature|artifact|enchantment|land|planeswalker|battle|permanent|card|spell|token|Aura|Equipment|Vehicle|Saga|Class|Fortification|Room|Case|scheme|conspiracy|Siege|Spacecraft|Contraption|Mount|emblem|phenomenon|plane|Attraction|Kindred";

fn normalize_text(text: &str, card_name: &str) -> String {
    static MANA: OnceLock<Regex> = OnceLock::new();
    static NUMBER: OnceLock<Regex> = OnceLock::new();
    static THIS_OBJECT: OnceLock<Regex> = OnceLock::new();
    let mana = MANA.get_or_init(|| Regex::new(r"\{[^{}]+\}").expect("valid mana regex"));
    let number = NUMBER.get_or_init(|| Regex::new(r"\b\d+\b").expect("valid number regex"));
    let this_object = THIS_OBJECT.get_or_init(|| {
        Regex::new(&format!(r"\b[Tt]his ({SELF_REFERENCE_TYPES})\b"))
            .expect("valid self-reference regex")
    });
    let mut normalized = text.to_owned();
    if !card_name.is_empty() {
        // `named <name>` is a predicate over other objects (CR 201.5), not a
        // self-reference; shield it before replacing the name.
        let mut names: Vec<&str> = vec![card_name];
        names.extend(card_name.split(" // "));
        let shields: Vec<(String, String)> = names
            .iter()
            .enumerate()
            .map(|(i, name)| (format!("named {name}"), format!("named \u{1}{i}\u{1}")))
            .collect();
        for (original, shield) in &shields {
            normalized = normalized.replace(original, shield);
        }
        for name in &names {
            normalized = normalized.replace(name, "~");
        }
        for (original, shield) in &shields {
            normalized = normalized.replace(shield, original);
        }
    }
    normalized = strip_reminder_text(&normalized);
    normalized = this_object.replace_all(&normalized, "~").into_owned();
    normalized = mana.replace_all(&normalized, "{M}").into_owned();
    normalized = number.replace_all(&normalized, "N").into_owned();
    collapse_whitespace(&normalized)
}

fn command_templates(db_path: &Path, args: TemplateArgs) -> Result<Value> {
    let conn = open_db(db_path)?;
    let set_filter = set_predicate("?1");
    let sql = format!(
        "SELECT name, oracle_text FROM cards \
         WHERE oracle_text IS NOT NULL AND oracle_text != ''{set_filter} ORDER BY oracle_id"
    );
    let mut statement = conn.prepare(&sql)?;
    let set_code = args.set.as_deref().unwrap_or("").to_lowercase();
    let rows = statement.query_map([&set_code], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;
    let mut counts: HashMap<String, u64> = HashMap::new();
    let mut kinds: BTreeMap<AbilityKind, u64> = BTreeMap::new();
    let mut roles: BTreeMap<StructuralRole, u64> = BTreeMap::new();
    let mut total = 0_u64;
    let mut rules_supplied = 0_u64;
    let mut empty = 0_u64;
    let mut cards = 0_u64;
    for row in rows {
        let (name, text) = row?;
        cards += 1;
        for segment in segment_text(&text, &name) {
            segment.walk(&mut |unit| {
                if unit.source == TextSource::RulesSupplied {
                    rules_supplied += 1;
                } else if unit.normalized.is_empty() {
                    empty += 1;
                } else {
                    *counts.entry(unit.normalized.clone()).or_default() += 1;
                    *kinds.entry(unit.kind).or_default() += 1;
                    *roles.entry(unit.role).or_default() += 1;
                    total += 1;
                }
            });
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
        "set": args.set,
        "cards": cards,
        "total_segments": total,
        "rules_supplied_units": rules_supplied,
        "empty_units": empty,
        "distinct_templates": distinct,
        "kinds": histogram(kinds),
        "roles": histogram(roles),
        "normalization": {
            "card_self_reference": "~ (card name and `this <object type>`; `named <name>` is preserved)",
            "mana_symbol": "{M}",
            "integer": "N",
            "reminder_text": "removed; reminder-only lines become rules_supplied units, counted separately",
            "keyword_lists": "split on , and ; into one keyword unit each",
            "mode_marker": "• removed; role = mode under the parent ability",
            "granted_ability": "quoted ability replaced by \"[ability]\" and counted as a child unit",
            "delayed_trigger": "`At the beginning of ... next ...` split off as a child of the creating unit",
            "face_separator": "excluded"
        },
        "coverage": coverage,
        "result_limit": args.limit,
        "minimum_count": args.min_count,
        "templates": templates
    }))
}

/// Serialize a label-keyed histogram as a JSON object keyed by the label's
/// serde name.
fn histogram<K: Serialize>(map: BTreeMap<K, u64>) -> Value {
    map.into_iter()
        .map(|(key, count)| {
            let label = serde_json::to_value(key)
                .ok()
                .and_then(|value| value.as_str().map(str::to_owned))
                .unwrap_or_default();
            (label, json!(count))
        })
        .collect::<serde_json::Map<_, _>>()
        .into()
}

/// SQL fragment restricting rows to a first-printing set, bound to `param`.
/// The parameter is always referenced so callers can bind it unconditionally;
/// an empty string (no set requested) matches every row.
fn set_predicate(param: &str) -> String {
    format!(" AND ({param} = '' OR lower(first_set) = {param})")
}

fn command_sets(db_path: &Path, args: SetsArgs) -> Result<Value> {
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
        assert_eq!(segments[0].kind, AbilityKind::Keyword);
        assert_eq!(segments[1].face, 1);
        assert_eq!(segments[1].line, 3);
        assert_eq!(segments[1].kind, AbilityKind::Triggered);
        assert_eq!(segments[1].role, StructuralRole::Ability);
    }

    fn kinds(segments: &[Segment]) -> Vec<AbilityKind> {
        segments.iter().map(|segment| segment.kind).collect()
    }

    fn normalized(segments: &[Segment]) -> Vec<&str> {
        segments
            .iter()
            .map(|segment| segment.normalized.as_str())
            .collect()
    }

    #[test]
    fn reminder_bearing_keyword_is_classified_on_stripped_text() {
        let segments = segment_text(
            "Vigilance (Attacking doesn't cause this creature to tap.)",
            "Serra Angel",
        );
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].kind, AbilityKind::Keyword);
        assert_eq!(segments[0].source, TextSource::Printed);
        assert_eq!(segments[0].text, "Vigilance");
        assert_eq!(segments[0].normalized, "Vigilance");
    }

    #[test]
    fn keyword_lists_split_on_comma_and_semicolon() {
        let comma = segment_text("Flying, trample", "Lord of the Pit");
        assert_eq!(normalized(&comma), ["Flying", "Trample"]);
        assert_eq!(kinds(&comma), [AbilityKind::Keyword, AbilityKind::Keyword]);

        let reminder = segment_text(
            "Defender, flying (This creature can't attack, and it can block creatures with flying.)",
            "Wall of Air",
        );
        assert_eq!(normalized(&reminder), ["Defender", "Flying"]);
        assert_eq!(reminder[1].text, "flying");
        assert_eq!(
            kinds(&reminder),
            [AbilityKind::Keyword, AbilityKind::Keyword]
        );

        let semicolon = segment_text(
            "Flying; banding (Any creatures with banding can attack in a band.)",
            "",
        );
        assert_eq!(normalized(&semicolon), ["Flying", "Banding"]);
        assert!(semicolon.iter().all(|unit| unit.line == 1));
    }

    #[test]
    fn ordinary_comma_text_is_not_split() {
        let sentence = segment_text(
            "Discard your hand, ante the top card of your library, then draw seven cards.",
            "",
        );
        assert_eq!(sentence.len(), 1);
        assert_eq!(sentence[0].kind, AbilityKind::SpellOrStatic);

        let trigger_fragment = segment_text("Whenever you attack with two or more creatures,", "");
        assert_eq!(trigger_fragment.len(), 1);
        assert_eq!(trigger_fragment[0].kind, AbilityKind::Triggered);

        let threshold = segment_text("9+ | Flying, trample", "");
        assert_eq!(threshold.len(), 1);

        let cost_list = segment_text("Kicker {2}, {R}", "");
        assert_eq!(cost_list.len(), 1);

        let named_argument = segment_text("Partner with Trynn, Champion of Freedom", "");
        assert_eq!(named_argument.len(), 1);
        assert_eq!(named_argument[0].kind, AbilityKind::Keyword);

        let argument_list = segment_text(
            "Protection from Vampires, from Werewolves, and from Zombies",
            "",
        );
        assert_eq!(argument_list.len(), 1);
        assert_eq!(argument_list[0].kind, AbilityKind::Keyword);
    }

    #[test]
    fn reminder_only_lands_are_rules_supplied_units() {
        let basic = segment_text("({T}: Add {G}.)", "Forest");
        assert_eq!(basic.len(), 1);
        assert_eq!(basic[0].source, TextSource::RulesSupplied);
        assert_eq!(basic[0].rule, Some("305.6"));
        assert_eq!(basic[0].kind, AbilityKind::Activated);
        assert_eq!(basic[0].text, "({T}: Add {G}.)");
        assert_eq!(basic[0].normalized, "{M}: Add {M}.");

        let dual = segment_text("({T}: Add {W} or {U}.)", "Tundra");
        assert_eq!(dual[0].source, TextSource::RulesSupplied);
        assert_eq!(dual[0].rule, Some("305.6"));
        assert_eq!(dual[0].normalized, "{M}: Add {M} or {M}.");

        let unknown = segment_text("(Theme color: {W})", "");
        assert_eq!(unknown[0].source, TextSource::RulesSupplied);
        assert_eq!(unknown[0].rule, None);
    }

    #[test]
    fn quoted_granted_ability_is_a_child_and_does_not_make_parent_activated() {
        let segments = segment_text(
            "Other Zombies have \"{B}: Regenerate this permanent.\"",
            "Zombie Master",
        );
        assert_eq!(segments.len(), 1);
        let parent = &segments[0];
        assert_eq!(parent.kind, AbilityKind::SpellOrStatic);
        assert_eq!(parent.role, StructuralRole::Ability);
        assert_eq!(parent.normalized, "Other Zombies have \"[ability]\"");
        assert_eq!(parent.children.len(), 1);
        let granted = &parent.children[0];
        assert_eq!(granted.kind, AbilityKind::Activated);
        assert_eq!(granted.role, StructuralRole::Granted);
        assert_eq!(granted.text, "{B}: Regenerate this permanent.");
        assert_eq!(granted.normalized, "{M}: Regenerate ~.");
        assert_eq!(granted.index, 1);
    }

    #[test]
    fn short_quoted_words_are_not_abilities() {
        let segments = segment_text(
            "Each player divides creatures into a \"left\" pile and a \"right\" pile. Then choose \"left\" or \"right.\"",
            "Raging River",
        );
        assert_eq!(segments.len(), 1);
        assert!(segments[0].children.is_empty());
        assert!(segments[0].normalized.contains("\"left\""));
    }

    #[test]
    fn named_predicate_survives_self_reference_normalization() {
        let text = "Plague Rats's power and toughness are each equal to the number of creatures named Plague Rats on the battlefield.";
        let segments = segment_text(text, "Plague Rats");
        assert_eq!(
            segments[0].normalized,
            "~'s power and toughness are each equal to the number of creatures named Plague Rats on the battlefield."
        );
        assert_eq!(segments[0].kind, AbilityKind::CharacteristicDefining);
    }

    #[test]
    fn this_object_self_references_normalize_to_tilde() {
        assert_eq!(
            normalize_text(
                "This creature's power is 3. Sacrifice this artifact: Exile this card.",
                ""
            ),
            "~'s power is N. Sacrifice ~: Exile ~."
        );
        assert_eq!(
            normalize_text("Until end of this turn, do this way.", ""),
            "Until end of this turn, do this way."
        );
    }

    #[test]
    fn modal_spell_is_one_ability_with_mode_children() {
        let segments = segment_text(
            "Choose one —\n• Counter target red spell.\n• Destroy target red permanent.",
            "Blue Elemental Blast",
        );
        assert_eq!(segments.len(), 1);
        let parent = &segments[0];
        assert_eq!(parent.normalized, "Choose one —");
        assert_eq!(parent.children.len(), 2);
        assert_eq!(parent.children[0].role, StructuralRole::Mode);
        assert_eq!(parent.children[0].normalized, "Counter target red spell.");
        assert_eq!(parent.children[0].text, "Counter target red spell.");
        assert_eq!(parent.children[1].line, 3);
        assert_eq!(parent.children[1].index, 2);
    }

    #[test]
    fn triggered_modal_header_keeps_its_kind() {
        let segments = segment_text(
            "When this creature enters, choose one —\n• Draw a card.\n• You gain 3 life.",
            "",
        );
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].kind, AbilityKind::Triggered);
        assert_eq!(segments[0].children.len(), 2);
        assert!(
            segments[0]
                .children
                .iter()
                .all(|mode| mode.role == StructuralRole::Mode)
        );
    }

    #[test]
    fn delayed_trigger_line_attaches_to_originating_spell_text() {
        let segments = segment_text(
            "Cast this spell only during an opponent's turn, before attackers are declared.\n\
             Creatures the active player controls attack this turn if able.\n\
             At the beginning of the next end step, destroy all non-Wall creatures that player controls that didn't attack this turn.",
            "Siren's Call",
        );
        assert_eq!(
            kinds(&segments),
            [AbilityKind::CastRestriction, AbilityKind::SpellOrStatic]
        );
        assert_eq!(
            segments[0].normalized,
            "Cast ~ only during an opponent's turn, before attackers are declared."
        );
        let delayed = &segments[1].children[0];
        assert_eq!(delayed.kind, AbilityKind::Triggered);
        assert_eq!(delayed.role, StructuralRole::DelayedTrigger);
        assert_eq!(delayed.line, 3);
        assert!(
            delayed
                .normalized
                .starts_with("At the beginning of the next end step,")
        );
    }

    #[test]
    fn inline_delayed_trigger_is_split_from_spell_effect() {
        let segments = segment_text(
            "Target creature gains trample and gets +X/+0 until end of turn, where X is its power. At the beginning of the next end step, destroy that creature if it attacked this turn.",
            "Berserk",
        );
        assert_eq!(segments.len(), 1);
        assert_eq!(
            segments[0].normalized,
            "Target creature gains trample and gets +X/+N until end of turn, where X is its power."
        );
        assert_eq!(segments[0].children.len(), 1);
        assert_eq!(segments[0].children[0].role, StructuralRole::DelayedTrigger);
        assert_eq!(
            segments[0].children[0].normalized,
            "At the beginning of the next end step, destroy that creature if it attacked this turn."
        );
    }

    #[test]
    fn recurring_upkeep_trigger_is_not_a_delayed_trigger() {
        let segments = segment_text(
            "Flying\nAt the beginning of your upkeep, sacrifice a creature other than this creature.",
            "Lord of the Pit",
        );
        assert_eq!(segments.len(), 2);
        assert_eq!(segments[1].role, StructuralRole::Ability);
        assert!(segments[1].children.is_empty());
    }

    #[test]
    fn delayed_trigger_phrase_inside_quotes_is_not_split() {
        let segments = segment_text(
            "Enchanted land has \"{T}: Add {G}. At the beginning of the next end step, sacrifice this land.\"",
            "",
        );
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].kind, AbilityKind::SpellOrStatic);
        assert_eq!(segments[0].children.len(), 1);
        let granted = &segments[0].children[0];
        assert_eq!(granted.role, StructuralRole::Granted);
        assert_eq!(granted.kind, AbilityKind::Activated);
        assert_eq!(granted.children[0].role, StructuralRole::DelayedTrigger);
    }

    #[test]
    fn structural_classes_are_detected_from_normalized_text() {
        let classify = |text: &str, name: &str| segment_text(text, name)[0].kind;
        assert_eq!(
            classify(
                "Remove this card from your deck before playing if you're not playing for ante.",
                "Darkpact"
            ),
            AbilityKind::Ante
        );
        assert_eq!(
            classify(
                "As an additional cost to cast this spell, sacrifice a creature.",
                "Sacrifice"
            ),
            AbilityKind::AdditionalCost
        );
        assert_eq!(
            classify(
                "Cast this spell only before the combat damage step.",
                "Berserk"
            ),
            AbilityKind::CastRestriction
        );
        assert_eq!(
            classify(
                "You may have this creature enter as a copy of any creature on the battlefield.",
                "Clone"
            ),
            AbilityKind::Replacement
        );
        assert_eq!(
            classify("This artifact enters tapped.", "Time Vault"),
            AbilityKind::Replacement
        );
        assert_eq!(
            classify(
                "If you would gain life, draw that many cards instead.",
                "Lich"
            ),
            AbilityKind::Replacement
        );
        assert_eq!(
            classify(
                "Nightmare's power and toughness are each equal to the number of Swamps you control.",
                "Nightmare"
            ),
            AbilityKind::CharacteristicDefining
        );
        assert_eq!(
            classify("Creatures you control get +1/+1.", ""),
            AbilityKind::SpellOrStatic
        );
        assert_eq!(
            classify("Whenever a creature enters, it enters tapped instead.", ""),
            AbilityKind::Triggered
        );
    }

    #[test]
    fn indices_are_preorder_across_nested_units() {
        let segments = segment_text(
            "Choose one —\n• Draw a card.\n• Discard a card.\nFlying, haste",
            "",
        );
        let mut seen = Vec::new();
        for segment in &segments {
            segment.walk(&mut |unit| seen.push(unit.index));
        }
        assert_eq!(seen, [0, 1, 2, 3, 4]);
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
    fn set_predicate_always_references_its_parameter() {
        assert_eq!(
            set_predicate("?3"),
            " AND (?3 = '' OR lower(first_set) = ?3)"
        );
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
