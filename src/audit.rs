use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;

use anyhow::{Result, bail};
use rusqlite::{Connection, params};
use serde::Serialize;
use serde_json::{Value, json};

use crate::cli::{AuditCommand, NoveltyAuditArgs, SetAuditArgs};
use crate::database::{held_out_exclusion_metadata, held_out_exclusion_predicate, open_db};
use crate::segment::{
    AbilityKind, Segment, StructuralRole, TextSource, delayed_trigger_start,
    delayed_trigger_unresolved, segment_text,
};
use crate::util::{histogram, percent};

#[derive(Clone, Debug)]
pub(crate) struct AuditCard {
    pub(crate) oracle_id: String,
    pub(crate) name: String,
    pub(crate) type_line: Option<String>,
    pub(crate) first_set: String,
    pub(crate) first_released_at: Option<String>,
    pub(crate) first_is_fallback: bool,
    pub(crate) oracle_text: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub(crate) struct AuditRecord {
    pub(crate) oracle_id: String,
    pub(crate) card_name: String,
    pub(crate) type_line: Option<String>,
    pub(crate) first_set: String,
    pub(crate) first_released_at: Option<String>,
    pub(crate) first_is_fallback: bool,
    pub(crate) face: usize,
    pub(crate) source_line: usize,
    pub(crate) unit_index: usize,
    pub(crate) parent_index: Option<usize>,
    pub(crate) depth: usize,
    pub(crate) source_line_text: Option<String>,
    pub(crate) unit_text: String,
    pub(crate) prefix: Option<String>,
    pub(crate) normalized: String,
    pub(crate) kind: AbilityKind,
    pub(crate) role: StructuralRole,
    pub(crate) source: TextSource,
    pub(crate) rule: Option<&'static str>,
    pub(crate) signals: Vec<&'static str>,
}

#[derive(Default)]
pub(crate) struct AuditSummary {
    pub(crate) cards: u64,
    pub(crate) cards_with_text: u64,
    pub(crate) printed_units: u64,
    pub(crate) rules_supplied_units: u64,
    pub(crate) empty_units: u64,
    pub(crate) distinct_printed_templates: usize,
    pub(crate) singleton_templates: u64,
    pub(crate) multi_sentence_units: u64,
    pub(crate) residual_spell_or_static_units: u64,
    pub(crate) uncited_rules_supplied_units: u64,
    pub(crate) templates: HashMap<String, u64>,
    pub(crate) kinds: BTreeMap<AbilityKind, u64>,
    pub(crate) roles: BTreeMap<StructuralRole, u64>,
    pub(crate) sources: BTreeMap<TextSource, u64>,
}

pub(crate) fn command_audit(db_path: &Path, command: AuditCommand) -> Result<Value> {
    match command {
        AuditCommand::Export(args) => command_audit_export(db_path, args),
        AuditCommand::Summary(args) => command_audit_summary(db_path, args),
        AuditCommand::Novelty(args) => command_audit_novelty(db_path, args),
        AuditCommand::Signals(args) => command_audit_signals(db_path, args),
    }
}

fn command_audit_export(db_path: &Path, args: SetAuditArgs) -> Result<Value> {
    let conn = open_db(db_path)?;
    let cards = load_audit_cards(&conn, &args.set, args.exclude_heldout)?;
    audit_export_payload(&args.set, &cards, args.exclude_heldout)
}

pub(crate) fn audit_export_payload(
    set: &str,
    cards: &[AuditCard],
    exclude_heldout: bool,
) -> Result<Value> {
    ensure_held_out_excluded(cards, exclude_heldout)?;
    let records = audit_records(cards);
    validate_audit_records(&records)?;
    Ok(json!({
        "schema_version": "audit-export-v1",
        "set": set.to_lowercase(),
        "ordering": "card name, oracle_id, face, pre-order unit_index",
        "stable_key": ["oracle_id", "face", "unit_index"],
        "heldout_exclusion": held_out_exclusion_metadata(exclude_heldout),
        "cards": cards.len(),
        "cards_with_text": cards.iter().filter(|card| card.oracle_text.as_ref().is_some_and(|text| !text.is_empty())).count(),
        "count": records.len(),
        "records": records
    }))
}

fn command_audit_summary(db_path: &Path, args: SetAuditArgs) -> Result<Value> {
    let conn = open_db(db_path)?;
    let cards = load_audit_cards(&conn, &args.set, args.exclude_heldout)?;
    ensure_held_out_excluded(&cards, args.exclude_heldout)?;
    let summary = summarize_audit(&cards);
    Ok(json!({
        "set": args.set.to_lowercase(),
        "inclusion_policy": audit_inclusion_policy(args.exclude_heldout),
        "cards": summary.cards,
        "cards_with_text": summary.cards_with_text,
        "printed_units": summary.printed_units,
        "rules_supplied_units": summary.rules_supplied_units,
        "empty_units": summary.empty_units,
        "distinct_printed_templates": summary.distinct_printed_templates,
        "singleton_templates": summary.singleton_templates,
        "kind_histogram": histogram(summary.kinds),
        "role_histogram": histogram(summary.roles),
        "source_histogram": histogram(summary.sources),
        "multi_sentence_unit_count": summary.multi_sentence_units,
        "residual_spell_static_count": summary.residual_spell_or_static_units,
        "uncited_rules_supplied_count": summary.uncited_rules_supplied_units
    }))
}

fn command_audit_novelty(db_path: &Path, args: NoveltyAuditArgs) -> Result<Value> {
    let conn = open_db(db_path)?;
    let set = args.set.to_lowercase();
    let selected = load_audit_cards(&conn, &set, false)?;
    let selected_release = selected_set_release(&selected);
    let mut earlier_set_codes: Vec<String> =
        args.earlier.iter().map(|set| set.to_lowercase()).collect();
    earlier_set_codes.sort();
    earlier_set_codes.dedup();
    let earlier_sets: HashSet<String> = earlier_set_codes.iter().cloned().collect();
    let earlier = load_earlier_audit_cards(
        &conn,
        selected_release.as_deref(),
        if earlier_sets.is_empty() {
            None
        } else {
            Some(&earlier_sets)
        },
    )?;
    let novelty = novelty_report(&selected, &earlier);
    Ok(json!({
        "set": set,
        "selected_released_at": selected_release,
        "earlier_sets": if earlier_sets.is_empty() {
            Value::Null
        } else {
            json!(earlier_set_codes)
        },
        "earlier_sets_policy": if earlier_sets.is_empty() {
            "All eligible sets with first_released_at strictly earlier than the selected set; same-date ties and missing dates are not earlier."
        } else {
            "Only the explicitly supplied audited sets, further restricted to first_released_at strictly earlier than the selected set."
        },
        "first_printing_policy": "Fallback first-printing records are excluded from the earlier comparison corpus.",
        "earlier_cards_with_text": earlier.iter().filter(|card| card.oracle_text.as_ref().is_some_and(|text| !text.is_empty())).count(),
        "total_printed_units": novelty["total_printed_units"],
        "units_seen_earlier": novelty["units_seen_earlier"],
        "novel_units": novelty["novel_units"],
        "unit_novelty_percent": novelty["unit_novelty_percent"],
        "distinct_templates": novelty["distinct_templates"],
        "templates_seen_earlier": novelty["templates_seen_earlier"],
        "novel_templates": novelty["novel_templates"],
        "template_novelty_percent": novelty["template_novelty_percent"],
        "novel_template_records": novelty["novel_template_records"]
    }))
}

fn command_audit_signals(db_path: &Path, args: SetAuditArgs) -> Result<Value> {
    let conn = open_db(db_path)?;
    let cards = load_audit_cards(&conn, &args.set, args.exclude_heldout)?;
    ensure_held_out_excluded(&cards, args.exclude_heldout)?;
    let records: Vec<_> = audit_records(&cards)
        .into_iter()
        .filter(|record| !record.signals.is_empty())
        .collect();
    let mut counts: BTreeMap<&'static str, u64> = BTreeMap::new();
    for record in &records {
        for signal in &record.signals {
            *counts.entry(signal).or_default() += 1;
        }
    }
    Ok(json!({
        "set": args.set.to_lowercase(),
        "heldout_exclusion": held_out_exclusion_metadata(args.exclude_heldout),
        "signal_policy": "Signals are surface-form audit candidates, not parser errors or ground-truth labels.",
        "signal_definitions": signal_definitions(),
        "signal_histogram": counts,
        "count": records.len(),
        "records": records
    }))
}

pub(crate) fn load_audit_cards(
    conn: &Connection,
    set: &str,
    exclude_heldout: bool,
) -> Result<Vec<AuditCard>> {
    let set = set.to_lowercase();
    let held_out_filter = held_out_exclusion_predicate("?2");
    let sql = format!(
        "SELECT oracle_id, name, type_line, first_set, first_released_at, first_is_fallback, oracle_text \
         FROM cards WHERE lower(first_set) = ?1{held_out_filter} \
         ORDER BY lower(name), name, oracle_id"
    );
    let mut statement = conn.prepare(&sql)?;
    let cards = statement
        .query_map(params![set, exclude_heldout], audit_card_from_row)?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    if cards.is_empty() {
        bail!("no cards found for first-printing set {:?}", set);
    }
    Ok(cards)
}

fn load_earlier_audit_cards(
    conn: &Connection,
    selected_release: Option<&str>,
    earlier_sets: Option<&HashSet<String>>,
) -> Result<Vec<AuditCard>> {
    let Some(selected_release) = selected_release else {
        return Ok(Vec::new());
    };
    let mut statement = conn.prepare(
        "SELECT oracle_id, name, type_line, first_set, first_released_at, first_is_fallback, oracle_text \
         FROM cards WHERE first_set IS NOT NULL AND first_released_at IS NOT NULL \
         AND first_released_at < ?1 AND oracle_text IS NOT NULL AND oracle_text != '' \
         AND first_is_fallback = 0 \
         ORDER BY first_released_at, lower(first_set), lower(name), name, oracle_id",
    )?;
    let mut cards = statement
        .query_map([selected_release], audit_card_from_row)?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    if let Some(earlier_sets) = earlier_sets {
        cards.retain(|card| earlier_sets.contains(&card.first_set.to_lowercase()));
    }
    Ok(cards)
}

fn audit_card_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<AuditCard> {
    Ok(AuditCard {
        oracle_id: row.get(0)?,
        name: row.get(1)?,
        type_line: row.get(2)?,
        first_set: row.get(3)?,
        first_released_at: row.get(4)?,
        first_is_fallback: row.get::<_, i64>(5)? != 0,
        oracle_text: row.get(6)?,
    })
}

pub(crate) fn selected_set_release(cards: &[AuditCard]) -> Option<String> {
    cards
        .iter()
        .filter_map(|card| card.first_released_at.as_deref())
        .min()
        .map(str::to_owned)
}

pub(crate) fn audit_records(cards: &[AuditCard]) -> Vec<AuditRecord> {
    let mut records = Vec::new();
    for card in cards {
        if let Some(text) = card.oracle_text.as_ref().filter(|text| !text.is_empty()) {
            let line_lookup = source_line_lookup(text);
            for segment in segment_text(text, &card.name, card.type_line.as_deref()) {
                flatten_audit_segment(card, &line_lookup, &segment, None, 0, &mut records);
            }
        }
    }
    records.sort_by(|a, b| {
        a.card_name
            .to_lowercase()
            .cmp(&b.card_name.to_lowercase())
            .then_with(|| a.card_name.cmp(&b.card_name))
            .then_with(|| a.oracle_id.cmp(&b.oracle_id))
            .then_with(|| a.face.cmp(&b.face))
            .then_with(|| a.unit_index.cmp(&b.unit_index))
    });
    records
}

pub(crate) fn flatten_audit_segment(
    card: &AuditCard,
    line_lookup: &BTreeMap<usize, String>,
    segment: &Segment,
    parent_index: Option<usize>,
    depth: usize,
    records: &mut Vec<AuditRecord>,
) {
    let source_line_text = line_lookup.get(&segment.line).cloned();
    let mut record = AuditRecord {
        oracle_id: card.oracle_id.clone(),
        card_name: card.name.clone(),
        type_line: card.type_line.clone(),
        first_set: card.first_set.clone(),
        first_released_at: card.first_released_at.clone(),
        first_is_fallback: card.first_is_fallback,
        face: segment.face,
        source_line: segment.line,
        unit_index: segment.index,
        parent_index,
        depth,
        source_line_text,
        unit_text: segment.text.clone(),
        prefix: segment.prefix.clone(),
        normalized: segment.normalized.clone(),
        kind: segment.kind,
        role: segment.role,
        source: segment.source,
        rule: segment.rule,
        signals: Vec::new(),
    };
    record.signals = suspicious_signals(&record, segment);
    records.push(record);
    for child in &segment.children {
        flatten_audit_segment(
            card,
            line_lookup,
            child,
            Some(segment.index),
            depth + 1,
            records,
        );
    }
}

pub(crate) fn source_line_lookup(text: &str) -> BTreeMap<usize, String> {
    text.lines()
        .enumerate()
        .map(|(index, line)| (index + 1, line.trim().to_owned()))
        .collect()
}

pub(crate) fn validate_audit_records(records: &[AuditRecord]) -> Result<()> {
    let keys: HashSet<(&str, usize, usize)> = records
        .iter()
        .map(|record| (record.oracle_id.as_str(), record.face, record.unit_index))
        .collect();
    if keys.len() != records.len() {
        bail!(
            "audit export contains duplicate stable keys: {} rows, {} unique keys",
            records.len(),
            keys.len()
        );
    }

    for record in records {
        if let Some(parent_index) = record.parent_index {
            if parent_index >= record.unit_index {
                bail!("audit export contains a non-preorder parent reference");
            }
            if !keys.contains(&(record.oracle_id.as_str(), record.face, parent_index)) {
                bail!("audit export contains a parent outside the stable card/face identity");
            }
        }
    }

    if records
        .windows(2)
        .any(|pair| audit_record_sort_key(&pair[0]) > audit_record_sort_key(&pair[1]))
    {
        bail!("audit export records do not satisfy the declared deterministic ordering");
    }
    Ok(())
}

fn audit_record_sort_key(record: &AuditRecord) -> (String, &str, &str, usize, usize) {
    (
        record.card_name.to_lowercase(),
        record.card_name.as_str(),
        record.oracle_id.as_str(),
        record.face,
        record.unit_index,
    )
}

pub(crate) fn is_held_out_identity(
    oracle_id: &str,
    oracle_text: Option<&str>,
    first_is_fallback: bool,
    first_set: &str,
) -> bool {
    oracle_text.is_some_and(|text| !text.is_empty())
        && oracle_id
            .chars()
            .next()
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case(&'f'))
        && !first_is_fallback
        && !matches!(
            first_set.to_ascii_lowercase().as_str(),
            "lea" | "leb" | "arn"
        )
}

pub(crate) fn ensure_held_out_excluded(cards: &[AuditCard], exclusion_enabled: bool) -> Result<()> {
    if exclusion_enabled
        && cards.iter().any(|card| {
            is_held_out_identity(
                &card.oracle_id,
                card.oracle_text.as_deref(),
                card.first_is_fallback,
                &card.first_set,
            )
        })
    {
        bail!("held-out exclusion invariant failed before audit serialization");
    }
    Ok(())
}

pub(crate) fn summarize_audit(cards: &[AuditCard]) -> AuditSummary {
    let mut summary = AuditSummary {
        cards: cards.len() as u64,
        cards_with_text: cards
            .iter()
            .filter(|card| {
                card.oracle_text
                    .as_ref()
                    .is_some_and(|text| !text.is_empty())
            })
            .count() as u64,
        ..Default::default()
    };
    for record in audit_records(cards) {
        *summary.sources.entry(record.source).or_default() += 1;
        if record.source == TextSource::RulesSupplied {
            summary.rules_supplied_units += 1;
            if record.rule.is_none() {
                summary.uncited_rules_supplied_units += 1;
            }
            continue;
        }
        if record.normalized.is_empty() {
            summary.empty_units += 1;
            continue;
        }
        summary.printed_units += 1;
        *summary
            .templates
            .entry(record.normalized.clone())
            .or_default() += 1;
        *summary.kinds.entry(record.kind).or_default() += 1;
        *summary.roles.entry(record.role).or_default() += 1;
        if is_multi_sentence(&record.unit_text) {
            summary.multi_sentence_units += 1;
        }
        if record.kind == AbilityKind::SpellOrStatic {
            summary.residual_spell_or_static_units += 1;
        }
    }
    summary.distinct_printed_templates = summary.templates.len();
    summary.singleton_templates = summary
        .templates
        .values()
        .filter(|count| **count == 1)
        .count() as u64;
    summary
}

pub(crate) fn novelty_report(selected: &[AuditCard], earlier: &[AuditCard]) -> Value {
    let earlier_templates: HashSet<String> = printed_template_units(earlier)
        .into_iter()
        .map(|unit| unit.0)
        .collect();
    let selected_units = printed_template_units(selected);
    let total_printed_units = selected_units.len() as u64;
    let units_seen_earlier = selected_units
        .iter()
        .filter(|unit| earlier_templates.contains(&unit.0))
        .count() as u64;
    let novel_units = total_printed_units - units_seen_earlier;

    let mut selected_templates: BTreeMap<String, (u64, Vec<String>)> = BTreeMap::new();
    for (template, card_name) in &selected_units {
        let entry = selected_templates
            .entry(template.clone())
            .or_insert_with(|| (0, Vec::new()));
        entry.0 += 1;
        if !entry.1.contains(card_name) {
            entry.1.push(card_name.clone());
            entry.1.sort();
        }
    }
    let distinct_templates = selected_templates.len() as u64;
    let templates_seen_earlier = selected_templates
        .keys()
        .filter(|template| earlier_templates.contains(*template))
        .count() as u64;
    let novel_templates = distinct_templates - templates_seen_earlier;
    let novel_template_records: Vec<_> = selected_templates
        .into_iter()
        .filter(|(template, _)| !earlier_templates.contains(template))
        .map(|(template, (count, mut cards))| {
            cards.truncate(5);
            json!({
                "template": template,
                "count": count,
                "representative_cards": cards
            })
        })
        .collect();

    json!({
        "total_printed_units": total_printed_units,
        "units_seen_earlier": units_seen_earlier,
        "novel_units": novel_units,
        "unit_novelty_percent": percent(novel_units, total_printed_units),
        "distinct_templates": distinct_templates,
        "templates_seen_earlier": templates_seen_earlier,
        "novel_templates": novel_templates,
        "template_novelty_percent": percent(novel_templates, distinct_templates),
        "novel_template_records": novel_template_records
    })
}

pub(crate) fn printed_template_units(cards: &[AuditCard]) -> Vec<(String, String)> {
    let mut units = Vec::new();
    for card in cards {
        if let Some(text) = card.oracle_text.as_ref().filter(|text| !text.is_empty()) {
            for segment in segment_text(text, &card.name, card.type_line.as_deref()) {
                segment.walk(&mut |unit| {
                    if unit.source == TextSource::Printed && !unit.normalized.is_empty() {
                        units.push((unit.normalized.clone(), card.name.clone()));
                    }
                });
            }
        }
    }
    units.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
    units
}

pub(crate) fn suspicious_signals(record: &AuditRecord, segment: &Segment) -> Vec<&'static str> {
    let mut signals = Vec::new();
    let lower = record.normalized.to_lowercase();
    if record.source == TextSource::Printed
        && record.kind == AbilityKind::SpellOrStatic
        && is_multi_sentence(&record.unit_text)
    {
        signals.push("residual_multi_sentence_unit");
    }
    if record.source == TextSource::RulesSupplied && record.rule.is_none() {
        signals.push("uncited_rules_supplied_unit");
    }
    if record.unit_text.contains('"')
        && !segment
            .children
            .iter()
            .any(|child| child.role == StructuralRole::Granted)
    {
        signals.push("quoted_text_not_extracted_candidate");
    }
    if lower.contains("activate only") || lower.contains("activate this ability only") {
        signals.push("activation_restriction_embedded_candidate");
    }
    if (delayed_trigger_start().is_match(&record.unit_text)
        || delayed_trigger_unresolved(&record.unit_text))
        && record.role != StructuralRole::DelayedTrigger
        && !segment
            .children
            .iter()
            .any(|child| child.role == StructuralRole::DelayedTrigger)
    {
        signals.push("delayed_trigger_unattached_candidate");
    }
    if record.source == TextSource::Printed
        && record.kind == AbilityKind::SpellOrStatic
        && is_short_punctuation_free(&record.normalized)
    {
        signals.push("short_punctuation_free_residual_candidate");
    }
    if lower.starts_with("as long as ~") && (lower.contains("power") || lower.contains("toughness"))
    {
        signals.push("conditional_cda_candidate");
    }
    if lower.contains("spend only") {
        signals.push("payment_restriction_embedded_candidate");
    }
    signals
}

pub(crate) fn is_multi_sentence(text: &str) -> bool {
    text.matches(". ").count() + text.matches("? ").count() + text.matches("! ").count() > 0
}

pub(crate) fn is_short_punctuation_free(text: &str) -> bool {
    let words = text.split_whitespace().count();
    words <= 8
        && !text.contains('.')
        && !text.contains(':')
        && !text.contains('"')
        && !text.contains("\u{2014}")
        && !text.starts_with('\u{2022}')
}

pub(crate) fn audit_inclusion_policy(exclude_heldout: bool) -> Value {
    json!({
        "set_selection": "cards whose derived first_set matches the selected set code",
        "card_count": "all cards in the selected first_set, including cards without Oracle text",
        "unit_count": "same as templates: printed units with non-empty normalized text; rules_supplied units counted separately",
        "fallback_first_printings": "included, matching existing sets/templates behavior; records expose first_is_fallback through the source database policy but audit rows are selected by first_set",
        "heldout_exclusion": held_out_exclusion_metadata(exclude_heldout),
        "ordering": "card name, oracle_id, face, pre-order unit_index"
    })
}

pub(crate) fn signal_definitions() -> Value {
    json!({
        "residual_multi_sentence_unit": "printed spell_or_static_text unit whose unit_text contains more than one sentence boundary",
        "uncited_rules_supplied_unit": "rules_supplied unit with no CR citation",
        "quoted_text_not_extracted_candidate": "unit_text contains double quotes but the unit has no granted child",
        "activation_restriction_embedded_candidate": "normalized text contains `activate only` or `activate this ability only`",
        "delayed_trigger_unattached_candidate": "unit_text contains a supported delayed-trigger split pattern but the unit is not a delayed_trigger and has no delayed_trigger child",
        "short_punctuation_free_residual_candidate": "printed spell_or_static_text unit with at most eight words and no period, colon, quote, bullet, or modal dash",
        "conditional_cda_candidate": "normalized text starts with `As long as ~` and mentions power or toughness",
        "payment_restriction_embedded_candidate": "normalized text contains `spend only`"
    })
}
