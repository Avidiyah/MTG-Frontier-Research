use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;
use std::sync::OnceLock;

use anyhow::{Context, Result};
use regex::Regex;
use rusqlite::params;
use serde::Serialize;
use serde_json::{Value, json};

use crate::cli::{SegmentArgs, TemplateArgs};
use crate::database::{
    held_out_exclusion_metadata, held_out_exclusion_predicate, open_db, set_predicate,
};
use crate::util::{histogram, percent};

/// Heuristic Comprehensive-Rules category of a unit's text. Labels are
/// assigned from surface form (see `classify_kind`) and are measurement
/// instruments, not semantic parses.
#[derive(Serialize, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub(crate) enum AbilityKind {
    #[serde(rename = "keyword_ability")]
    Keyword,
    #[serde(rename = "activated_ability")]
    Activated,
    #[serde(rename = "triggered_ability")]
    Triggered,
    #[serde(rename = "replacement_effect")]
    Replacement,
    #[serde(rename = "prevention_effect")]
    Prevention,
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
pub(crate) enum StructuralRole {
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
pub(crate) enum TextSource {
    Printed,
    RulesSupplied,
}

#[derive(Serialize, Clone, Debug)]
pub(crate) struct Segment {
    /// Pre-order position within the card, counting nested units.
    pub(crate) index: usize,
    pub(crate) face: usize,
    /// 1-based source line of the printed text this unit was derived from.
    pub(crate) line: usize,
    pub(crate) kind: AbilityKind,
    pub(crate) role: StructuralRole,
    pub(crate) source: TextSource,
    /// Comprehensive Rules citation for a rules-supplied unit, when inferable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) rule: Option<&'static str>,
    /// Printed text of the unit with reminder text removed (rules-supplied
    /// units keep the parenthetical because it is all that is printed).
    pub(crate) text: String,
    pub(crate) normalized: String,
    /// A leading structural prefix (ability word, Saga chapter symbol, or
    /// named mode/label) detected and stripped before classification
    /// (P-ATQ-3; CR 207.2c, 714.2). `None` when no such prefix was found.
    /// The classification text fed to `classify_kind` is not stored
    /// separately: it is `normalized` with `"<prefix> — "` removed from the
    /// front, recoverable from these two fields whenever `prefix` is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) prefix: Option<String>,
    pub(crate) children: Vec<Segment>,
}

impl Segment {
    pub(crate) fn walk<'a>(&'a self, visit: &mut impl FnMut(&'a Segment)) {
        visit(self);
        for child in &self.children {
            child.walk(visit);
        }
    }

    pub(crate) fn set_origin(&mut self, face: usize, line: usize) {
        self.face = face;
        self.line = line;
        for child in &mut self.children {
            child.set_origin(face, line);
        }
    }

    pub(crate) fn assign_indices(&mut self, next: &mut usize) {
        self.index = *next;
        *next += 1;
        for child in &mut self.children {
            child.assign_indices(next);
        }
    }
}

pub(crate) fn command_segment(db_path: &Path, args: SegmentArgs) -> Result<Value> {
    let exclude_heldout = args.exclude_heldout;
    let (name, text, type_line) = if let Some(card_name) = args.card {
        let conn = open_db(db_path)?;
        let held_out_filter = held_out_exclusion_predicate("?2");
        let sql = format!(
            "SELECT name, oracle_text, type_line FROM cards \
             WHERE lower(name) = lower(?1){held_out_filter} LIMIT 1"
        );
        conn.query_row(&sql, params![card_name, exclude_heldout], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, Option<String>>(1)?,
                row.get::<_, Option<String>>(2)?,
            ))
        })
        .with_context(|| format!("card not found: {card_name:?}"))?
    } else {
        (args.name.unwrap_or_default(), args.text, args.type_line)
    };
    let text = text.unwrap_or_default();
    let segments = segment_text(&text, &name, type_line.as_deref());
    let mut total_units = 0;
    for segment in &segments {
        segment.walk(&mut |_| total_units += 1);
    }
    Ok(json!({
        "name": if name.is_empty() { Value::Null } else { Value::String(name) },
        "type_line": type_line,
        "source_text": text,
        "heldout_exclusion": held_out_exclusion_metadata(exclude_heldout),
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
pub(crate) fn segment_text(text: &str, card_name: &str, type_line: Option<&str>) -> Vec<Segment> {
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
        let face_type_line = type_line_for_face(type_line, face);
        for mut unit in segment_line(line, card_name, face_type_line) {
            unit.set_origin(face, line_number + 1);
            let attach_to = match unit.role {
                StructuralRole::Mode => segments[face_start..].last_mut(),
                StructuralRole::DelayedTrigger => {
                    segments[face_start..].last_mut().filter(|parent| {
                        parent.source == TextSource::Printed
                            && !matches!(
                                parent.kind,
                                AbilityKind::Keyword
                                    | AbilityKind::Replacement
                                    | AbilityKind::Prevention
                                    | AbilityKind::CharacteristicDefining
                            )
                    })
                }
                _ => None,
            };
            match attach_to {
                Some(parent) => parent.children.push(unit),
                None => {
                    if unit.role == StructuralRole::DelayedTrigger {
                        unit.role = StructuralRole::Ability;
                    }
                    segments.push(unit);
                }
            }
        }
    }
    apply_spell_created_delayed_triggers(&mut segments, type_line);
    let mut next = 0;
    for segment in &mut segments {
        segment.assign_indices(&mut next);
    }
    segments
}

/// P-ATQ-4: a top-level instant/sorcery unit whose printed text *is* a
/// delayed-trigger clause (CR 603.7d: "If a spell creates a delayed
/// triggered ability, the source of that delayed triggered ability is that
/// spell") gets `role = DelayedTrigger` instead of the default `Ability`.
/// This runs once, after every line of the card has already been segmented
/// and attached, and only ever *changes the role of an existing top-level
/// unit in place* — it never attaches it as anyone's child (unlike the
/// `delayed_trigger_start` mechanism above, which is for a delayed-trigger
/// phrase on its own line that continues a *preceding* unit's effect).
/// Resolving the spell and the delayed trigger it creates are the same
/// printed unit here, so `parent_index` stays `None`: the unit's own
/// top-level position on this face already represents "created by this
/// spell" (CR 603.7d) without inventing a face-as-parent `Segment` the
/// current schema has no room for.
fn apply_spell_created_delayed_triggers(segments: &mut [Segment], type_line: Option<&str>) {
    for segment in segments {
        if segment.role != StructuralRole::Ability
            || segment.source != TextSource::Printed
            || segment.kind != AbilityKind::Triggered
        {
            continue;
        }
        let face_type_line = type_line_for_face(type_line, segment.face);
        if is_spell_created_delayed_trigger(classification_text(segment), face_type_line) {
            segment.role = StructuralRole::DelayedTrigger;
        }
    }
}

/// The text `classify_kind` actually judged this unit on: the P-ATQ-3
/// prefix-stripped body when a prefix was found, otherwise the full
/// normalized text. Re-derives this from `extract_prefix` rather than
/// storing it, so P-ATQ-4 reads exactly the same evidence P-ATQ-3's
/// classification used instead of duplicating or bypassing that logic.
fn classification_text(segment: &Segment) -> &str {
    match extract_prefix(&segment.normalized) {
        Some((_, body)) => body,
        None => &segment.normalized,
    }
}

/// CR 603.7d: resolving an instant or sorcery may itself create a delayed
/// triggered ability ("Whenever a creature blocks this turn, ..."), distinct
/// from (1) an ordinary triggered ability of the card that functions from
/// another zone under CR 113.6b (cycling, discard, graveyard/exile/suspend/
/// haunt abilities) and (2) a cast- or resolution-trigger of the spell
/// itself. All three surface as `kind = triggered_ability` on an instant or
/// sorcery face; only the first is a P-ATQ-4 role correction. Positive
/// evidence (not just the leading trigger word) is required: an explicit
/// future/duration temporal scope (CR 603.7b: "unless it has a stated
/// duration, such as 'this turn'"), with no evidence the ability instead
/// functions off the stack or is about the spell's own casting/resolution.
fn is_spell_created_delayed_trigger(classification_text: &str, type_line: Option<&str>) -> bool {
    is_instant_or_sorcery(type_line)
        && has_delayed_trigger_temporal_scope(classification_text)
        && !is_cast_or_resolve_trigger(classification_text)
        && !has_off_stack_evidence(classification_text)
}

/// CR 603.7b's "stated duration" evidence that resolving the spell scopes
/// the delayed trigger to the rest of the current turn, combat, or a named
/// future event ("this turn", "this combat", "next end step", "you next
/// cast ..."), rather than the ordinary one-shot phrasing every other kind
/// of triggered ability also uses.
pub(crate) fn has_delayed_trigger_temporal_scope(text: &str) -> bool {
    static TEMPORAL: OnceLock<Regex> = OnceLock::new();
    let temporal = TEMPORAL.get_or_init(|| {
        Regex::new(r"(?i)\bthis turn\b|\bthis combat\b|\bnext\b")
            .expect("valid temporal scope regex")
    });
    temporal.is_match(text)
}

/// The trigger condition is about the spell's own casting or resolution
/// (self-reference `~`, since `normalize_text` already collapses "this
/// spell"/the card's own name), not about an event the spell's resolution
/// watches for afterward. This must exclude even when a temporal phrase is
/// also present (e.g. "When you cast ~, copy it for each ... spell you've
/// cast this turn." is still a cast trigger, not a 603.7d delayed trigger).
pub(crate) fn is_cast_or_resolve_trigger(text: &str) -> bool {
    static CAST_OR_RESOLVE: OnceLock<Regex> = OnceLock::new();
    let cast_or_resolve = CAST_OR_RESOLVE.get_or_init(|| {
        Regex::new(r"(?i)^when you cast ~|\bcast ~ from\b|~ is countered\b|^when ~ resolves\b")
            .expect("valid cast/resolve trigger regex")
    });
    cast_or_resolve.is_match(text)
}

/// Evidence the ability instead functions from another zone under CR
/// 113.6b: a CR-defined off-stack keyword mechanic (cycling, suspend,
/// haunt — always about the object bearing them), or the unit's own
/// self-reference (`~`) near a graveyard/exile/discard zone word in the
/// same sentence. The zone words alone are not used as a blacklist: "if
/// this card is in your graveyard" (self, excluded) and "return those
/// cards from your graveyard" (someone else's cards, not excluded) both
/// contain "graveyard", but only the first is evidence this ability
/// functions from that zone rather than being created by the spell's
/// resolution.
pub(crate) fn has_off_stack_evidence(text: &str) -> bool {
    static KEYWORD: OnceLock<Regex> = OnceLock::new();
    static ZONE_SELF: OnceLock<Regex> = OnceLock::new();
    let keyword = KEYWORD.get_or_init(|| {
        Regex::new(r"(?i)\bcycl(?:e|ing)\b|\bsuspended\b|\bhaunts?\b")
            .expect("valid off-stack keyword regex")
    });
    let zone_self = ZONE_SELF.get_or_init(|| {
        Regex::new(
            r"(?i)~[^.]{0,30}\b(?:graveyard|exiled?|discard(?:ed|s)?)\b|\b(?:graveyard|exiled?|discard(?:ed|s)?)\b[^.]{0,30}~",
        )
        .expect("valid off-stack zone-self-reference regex")
    });
    keyword.is_match(text) || zone_self.is_match(text)
}

/// Units derived from one printed line. Roles other than `Ability` are
/// requests to the caller to attach the unit to the preceding unit.
fn segment_line(line: &str, card_name: &str, type_line: Option<&str>) -> Vec<Segment> {
    let stripped = collapse_whitespace(&strip_reminder_text(line));
    if stripped.is_empty() {
        let inner = line
            .trim()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .trim();
        let normalized = normalize_text(inner, card_name);
        let mut unit = build_unit(inner, card_name, type_line, true, true);
        unit.source = TextSource::RulesSupplied;
        unit.rule = rules_supplied_rule(&normalized);
        unit.text = line.to_owned();
        return vec![unit];
    }
    if let Some(inner) = stripped.strip_prefix('•') {
        let mut unit = build_unit(inner.trim(), card_name, type_line, false, true);
        unit.role = StructuralRole::Mode;
        return vec![unit];
    }
    if let Some(keywords) = split_keyword_list(&stripped, card_name, type_line) {
        return keywords;
    }
    let mut unit = build_unit(&stripped, card_name, type_line, true, true);
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
fn build_unit(
    text: &str,
    card_name: &str,
    type_line: Option<&str>,
    allow_spell_text_override: bool,
    allow_delayed_split: bool,
) -> Segment {
    let text = text.trim();
    if allow_delayed_split && let Some(split) = delayed_trigger_split(text) {
        let mut parent_text = text[..split].trim_end().to_owned();
        let mut child_text = text[split..].trim_start();
        if let Some(instruction) = activation_instruction_sentence_split(child_text) {
            let trailing_instruction = child_text[instruction..].trim_start();
            child_text = child_text[..instruction].trim_end();
            parent_text.push(' ');
            parent_text.push_str(trailing_instruction);
        }
        let mut parent = build_unit(
            parent_text.trim(),
            card_name,
            type_line,
            allow_spell_text_override,
            false,
        );
        let mut child = build_unit(child_text, card_name, type_line, false, false);
        child.role = StructuralRole::DelayedTrigger;
        child.kind = AbilityKind::Triggered;
        parent.children.push(child);
        return parent;
    }
    let (dequoted, quoted) = extract_quoted_abilities(text);
    let normalized = normalize_text(&dequoted, card_name);
    let prefix = extract_prefix(&normalized);
    let kind = match &prefix {
        // CR 714.2: a Saga chapter symbol "is a keyword ability that
        // represents a triggered ability" regardless of the verb the
        // printed effect after it starts with, so the body is never run
        // through `classify_kind` for this case (P-ATQ-3).
        Some((marker, _)) if is_saga_chapter_prefix(marker) && is_saga(type_line) => {
            AbilityKind::Triggered
        }
        Some((_, body)) => classify_kind(body, type_line, allow_spell_text_override),
        None => classify_kind(&normalized, type_line, allow_spell_text_override),
    };
    let prefix = prefix.map(|(marker, _)| marker);
    let children = quoted
        .iter()
        .map(|quote| {
            let mut child = build_unit(quote, card_name, None, false, true);
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
        prefix,
        children,
    }
}

/// Detect a leading `<prefix> — ` structural marker before classification:
/// an ability word (CR 207.2c), a Saga chapter symbol (CR 714.2), or a
/// named mode/label (e.g. a modal spell's flavor-named mode). Bounded to
/// the P-ATQ-3 hypothesis so it cannot absorb an ordinary leading clause:
/// the delimiter is an em dash, the prefix has no period or colon, and it
/// is at most 45 characters. Returns the prefix and the remaining body,
/// both on the already-`normalize_text`-processed unit text; `None` when no
/// such prefix is present (including when the "prefix" would be everything
/// up to a mid-sentence em dash with no room left for a bounded, punctuation-
/// free marker before it).
pub(crate) fn extract_prefix(normalized: &str) -> Option<(String, &str)> {
    static PREFIX: OnceLock<Regex> = OnceLock::new();
    let prefix_pattern = PREFIX.get_or_init(|| {
        Regex::new(r"^([^.:]{1,45}?) \u{2014} (\S.*)$").expect("valid prefix regex")
    });
    let captures = prefix_pattern.captures(normalized)?;
    let prefix = captures.get(1)?.as_str().to_owned();
    let body = captures.get(2)?.as_str();
    Some((prefix, body))
}

/// A Saga chapter symbol (CR 714.2a-c): one or more comma-separated Roman
/// numerals. Unlike an ability word or a named mode/label, this is
/// structural vocabulary the Comprehensive Rules itself defines, so it is
/// recognized without a card-name or set-code exception list.
pub(crate) fn is_saga_chapter_prefix(prefix: &str) -> bool {
    static ROMAN: OnceLock<Regex> = OnceLock::new();
    let roman =
        ROMAN.get_or_init(|| Regex::new(r"^[IVXLCDM]+$").expect("valid roman numeral regex"));
    prefix
        .split(',')
        .map(str::trim)
        .all(|part| !part.is_empty() && roman.is_match(part))
}

/// Whether the (per-face) type line carries the Saga enchantment subtype
/// (CR 205.3h, 714), gating the chapter-symbol kind override to cards the
/// Comprehensive Rules actually defines chapter symbols for.
fn is_saga(type_line: Option<&str>) -> bool {
    type_line.is_some_and(|line| {
        line.split([' ', '\u{2014}', '/'])
            .any(|word| word == "Saga")
    })
}

fn activation_instruction_sentence_split(text: &str) -> Option<usize> {
    static ACTIVATION_INSTRUCTION: OnceLock<Regex> = OnceLock::new();
    let activation_instruction = ACTIVATION_INSTRUCTION.get_or_init(|| {
        Regex::new(
            r"(?i)\. (activate only\b|activate this ability only\b|any player may activate\b)",
        )
        .expect("valid activation-instruction regex")
    });
    activation_instruction
        .captures(text)
        .and_then(|captures| captures.get(1))
        .map(|instruction| instruction.start())
}

/// Split a keyword-only line such as `Flying, trample` or `Flying; banding`
/// into one keyword unit per item. Returns `None` for anything that is not
/// a comma/semicolon list of alphabetic keyword items.
fn split_keyword_list(
    stripped: &str,
    card_name: &str,
    type_line: Option<&str>,
) -> Option<Vec<Segment>> {
    if !stripped.contains([',', ';'])
        || classify_kind(&normalize_text(stripped, card_name), type_line, false)
            != AbilityKind::Keyword
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
                let mut unit = build_unit(piece, card_name, type_line, false, true);
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

fn classify_kind(
    normalized: &str,
    type_line: Option<&str>,
    allow_spell_text_override: bool,
) -> AbilityKind {
    static REPLACEMENT: OnceLock<Regex> = OnceLock::new();
    static PREVENTION: OnceLock<Regex> = OnceLock::new();
    static PREVENTION_PROHIBITION: OnceLock<Regex> = OnceLock::new();
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
    let prevention = PREVENTION
        .get_or_init(|| Regex::new(r"\bprevent(s|ed|ing)?\b").expect("valid prevention regex"));
    // P-ATQ-2 (CR 615.1a defines a prevention effect by *preventing* an
    // event; "can't/cannot be prevented" instead prohibits prevention — a
    // rule-modifying statement, not the effect it describes). Apostrophe
    // optional/either form since normalized text is not apostrophe-folded.
    let prevention_prohibition = PREVENTION_PROHIBITION.get_or_init(|| {
        Regex::new(r"\bcan(?:['\u{2019}])?t be prevented\b|\bcannot be prevented\b")
            .expect("valid prevention-prohibition regex")
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
    } else if allow_spell_text_override && is_instant_or_sorcery(type_line) {
        AbilityKind::SpellOrStatic
    } else if prevention.is_match(&lower) && !prevention_prohibition.is_match(&lower) {
        AbilityKind::Prevention
    } else if replacement.is_match(&lower) {
        AbilityKind::Replacement
    } else if cda.is_match(&lower) {
        AbilityKind::CharacteristicDefining
    } else {
        AbilityKind::SpellOrStatic
    }
}

fn type_line_for_face(type_line: Option<&str>, face: usize) -> Option<&str> {
    let type_line = type_line?.trim();
    type_line
        .split(" // ")
        .nth(face)
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .or(Some(type_line))
}

fn is_instant_or_sorcery(type_line: Option<&str>) -> bool {
    type_line.is_some_and(|line| {
        line.split([' ', '\u{2014}', '/'])
            .any(|word| matches!(word, "Instant" | "Sorcery"))
    })
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

/// One-shot trigger starts that can be created by a preceding effect (CR 603.7).
pub(crate) fn delayed_trigger_start() -> &'static Regex {
    static START: OnceLock<Regex> = OnceLock::new();
    START.get_or_init(|| {
        Regex::new(r"(?i)^(at the beginning of ([^,]*\bnext\b|the next\b)|at end of combat\b)")
            .expect("valid delayed trigger regex")
    })
}

/// A delayed-trigger phrase occurring anywhere in the text (CR 603.7a),
/// matched outside quoted spans.
fn inverted_delayed_trigger() -> &'static Regex {
    static INVERTED_DELAYED: OnceLock<Regex> = OnceLock::new();
    INVERTED_DELAYED.get_or_init(|| {
        Regex::new(r"(?i)(at the beginning of ([^,.]*\bnext\b|the next\b)|at end of combat\b)")
            .expect("valid inverted delayed trigger regex")
    })
}

/// Byte offset at which a trailing delayed-trigger sentence begins, ignoring
/// matches inside quoted abilities. Splits only at a complete sentence
/// boundary: P-ARN-1's generic and inverted `next`/`at end of combat` forms,
/// and P-ARN-2's scoped `When`/`Whenever ... this turn`/`this way`/`When you
/// do` forms.
///
/// P-ATQ-1 retracted the single-sentence fallback that used to search
/// backward from the phrase for the nearest comma or colon: corpus-wide that
/// heuristic produced parents that were bare trigger conditions (`When ~
/// dies,`) or bare activation costs (`{T}:`), not reference units, and it
/// searched the unmasked text so it could also split inside a quoted
/// ability. When a delayed-trigger phrase appears in a single sentence with
/// no preceding sentence boundary, this now returns `None` and the unit is
/// left whole; `delayed_trigger_unresolved` records that case as a signal
/// instead of fabricating a boundary.
fn delayed_trigger_split(text: &str) -> Option<usize> {
    static SENTENCE_BOUNDARY: OnceLock<Regex> = OnceLock::new();
    let sentence_boundary = SENTENCE_BOUNDARY.get_or_init(|| {
        Regex::new(
            r"(?i)\. ((at the beginning of ([^,]*\bnext\b|the next\b)|at end of combat\b|when you do\b|(when|whenever) [^.]*\b(this turn|this way)\b))",
        )
        .expect("valid delayed sentence boundary regex")
    });
    let masked = mask_quoted(text);
    if let Some(sentence) = sentence_boundary
        .captures(&masked)
        .and_then(|captures| captures.get(1))
    {
        return Some(sentence.start());
    }
    let inverted_delayed = inverted_delayed_trigger();
    for (period, _) in masked.match_indices(". ") {
        let sentence_start = period + 2;
        let sentence = &masked[sentence_start..];
        let sentence_end = sentence.find(". ").unwrap_or(sentence.len());
        if inverted_delayed.is_match(&sentence[..sentence_end]) {
            return Some(sentence_start);
        }
    }
    None
}

/// True when `text` contains a delayed-trigger phrase (outside quotes) that
/// `delayed_trigger_split` could not resolve to a sentence boundary. This is
/// the P-ATQ-1 conservative fallback: an unresolved single-sentence trigger
/// (no valid complete-effect-clause boundary) is reported through the
/// existing `delayed_trigger_unattached_candidate` signal rather than split.
pub(crate) fn delayed_trigger_unresolved(text: &str) -> bool {
    if delayed_trigger_split(text).is_some() {
        return true;
    }
    let masked = mask_quoted(text);
    inverted_delayed_trigger()
        .find(&masked)
        .is_some_and(|delayed| delayed.start() > 0)
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

pub(crate) fn normalize_text(text: &str, card_name: &str) -> String {
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

pub(crate) fn command_templates(db_path: &Path, args: TemplateArgs) -> Result<Value> {
    let conn = open_db(db_path)?;
    let set_filter = set_predicate("?1");
    let sql = format!(
        "SELECT name, oracle_text, type_line FROM cards \
         WHERE oracle_text IS NOT NULL AND oracle_text != ''{set_filter} ORDER BY oracle_id"
    );
    let mut statement = conn.prepare(&sql)?;
    let set_code = args.set.as_deref().unwrap_or("").to_lowercase();
    let rows = statement.query_map([&set_code], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, Option<String>>(2)?,
        ))
    })?;
    let mut counts: HashMap<String, u64> = HashMap::new();
    let mut kinds: BTreeMap<AbilityKind, u64> = BTreeMap::new();
    let mut roles: BTreeMap<StructuralRole, u64> = BTreeMap::new();
    let mut total = 0_u64;
    let mut rules_supplied = 0_u64;
    let mut empty = 0_u64;
    let mut cards = 0_u64;
    for row in rows {
        let (name, text, type_line) = row?;
        cards += 1;
        for segment in segment_text(&text, &name, type_line.as_deref()) {
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
            "delayed_trigger": "supported sentence-level `next`/`at end of combat` and scoped `When` forms split off as children; qualifying spell-created delayed triggers remain top-level with role delayed_trigger",
            "type_line_context": "per-face type line keeps instant/sorcery spell text out of replacement/prevention ability kinds except recognized static exceptions",
            "prevention_effect": "static CR 615 prevention text is reported separately from replacement_effect",
            "face_separator": "excluded"
        },
        "coverage": coverage,
        "result_limit": args.limit,
        "minimum_count": args.min_count,
        "templates": templates
    }))
}
