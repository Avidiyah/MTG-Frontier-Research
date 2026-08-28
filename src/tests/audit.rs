use rusqlite::{Connection, params};

use crate::audit::*;
use crate::segment::*;

fn audit_card(name: &str, set: &str, released_at: &str, text: Option<&str>) -> AuditCard {
    AuditCard {
        oracle_id: format!("{set}-{name}"),
        name: name.to_owned(),
        type_line: None,
        first_set: set.to_owned(),
        first_released_at: Some(released_at.to_owned()),
        first_is_fallback: false,
        oracle_text: text.map(str::to_owned),
    }
}

#[test]
fn audit_records_are_sorted_and_flatten_nested_units_with_parents() {
    let cards = vec![
        audit_card(
            "Zombie Master",
            "lea",
            "1993-08-05",
            Some("Other Zombies have \"{B}: Regenerate this permanent.\""),
        ),
        audit_card(
            "Blue Elemental Blast",
            "lea",
            "1993-08-05",
            Some(
                "Choose one \u{2014}\n\u{2022} Counter target red spell.\n\u{2022} Destroy target red permanent.",
            ),
        ),
    ];
    let records = audit_records(&cards);
    assert_eq!(records[0].card_name, "Blue Elemental Blast");
    assert_eq!(records[0].unit_index, 0);
    assert_eq!(records[0].parent_index, None);
    assert_eq!(records[0].depth, 0);
    assert_eq!(records[1].parent_index, Some(0));
    assert_eq!(records[1].depth, 1);
    assert_eq!(records[1].role, StructuralRole::Mode);
    assert_eq!(records[2].parent_index, Some(0));
    assert_eq!(records[2].role, StructuralRole::Mode);

    let zombie_child = records
        .iter()
        .find(|record| {
            record.card_name == "Zombie Master" && record.role == StructuralRole::Granted
        })
        .expect("granted child");
    assert_eq!(zombie_child.parent_index, Some(0));
    assert_eq!(zombie_child.depth, 1);
    assert_eq!(zombie_child.normalized, "{M}: Regenerate ~.");
    validate_audit_records(&records).expect("valid stable keys and parents");
}

#[test]
fn audit_export_is_byte_deterministic_for_reordered_input_cards() {
    let mut cards = vec![
        audit_card("Zulu", "tst", "2000-01-01", Some("Flying")),
        audit_card("alpha", "tst", "2000-01-01", Some("Draw a card.")),
        audit_card("Alpha", "tst", "2000-01-01", Some("Trample")),
    ];
    let first = serde_json::to_vec_pretty(
        &audit_export_payload("TST", &cards, false).expect("first export"),
    )
    .expect("serialize first export");
    cards.reverse();
    let second = serde_json::to_vec_pretty(
        &audit_export_payload("tst", &cards, false).expect("second export"),
    )
    .expect("serialize second export");

    assert_eq!(first, second);
}

#[test]
fn audit_export_carries_segment_prefix_and_serializes_absence_as_null() {
    let cards = vec![
        audit_card(
            "Prefixed",
            "tst",
            "2000-01-01",
            Some("Heroic \u{2014} Whenever you cast a spell, draw a card."),
        ),
        audit_card("Plain", "tst", "2000-01-01", Some("Flying")),
    ];
    let payload = audit_export_payload("tst", &cards, false).expect("audit export");
    let records = payload["records"].as_array().expect("records array");
    let prefixed = records
        .iter()
        .find(|record| record["card_name"] == "Prefixed")
        .expect("prefixed record");
    let plain = records
        .iter()
        .find(|record| record["card_name"] == "Plain")
        .expect("plain record");

    assert_eq!(prefixed["prefix"], "Heroic");
    assert!(plain["prefix"].is_null());
}

#[test]
fn audit_export_rejects_duplicate_stable_keys() {
    let mut records =
        audit_records(&[audit_card("Duplicate", "tst", "2000-01-01", Some("Flying"))]);
    records.push(records[0].clone());

    let error = validate_audit_records(&records).expect_err("duplicate key must fail");
    assert!(error.to_string().contains("duplicate stable keys"));
}

#[test]
fn held_out_definition_keeps_historical_exceptions_and_fallbacks_out_of_pool() {
    assert!(is_held_out_identity(
        "f0000000-0000-0000-0000-000000000000",
        Some("Synthetic text."),
        false,
        "dev"
    ));
    assert!(!is_held_out_identity(
        "f0000000-0000-0000-0000-000000000000",
        Some("Synthetic text."),
        false,
        "lea"
    ));
    assert!(!is_held_out_identity(
        "f0000000-0000-0000-0000-000000000000",
        Some("Synthetic text."),
        true,
        "dev"
    ));
    assert!(!is_held_out_identity(
        "f0000000-0000-0000-0000-000000000000",
        None,
        false,
        "dev"
    ));
}

#[test]
fn audit_query_excludes_held_out_cards_before_segmentation() {
    let conn = Connection::open_in_memory().expect("in-memory database");
    conn.execute_batch(
        "CREATE TABLE cards (
                oracle_id TEXT PRIMARY KEY,
                name TEXT,
                type_line TEXT,
                first_set TEXT,
                first_released_at TEXT,
                first_is_fallback INTEGER,
                oracle_text TEXT
            );",
    )
    .expect("test schema");
    for row in [
        ("a-safe", "Safe", 0, Some("Flying")),
        ("f-held", "Held", 0, Some("Trample")),
        ("f-fallback", "Fallback", 1, Some("Haste")),
        ("f-empty", "No Text", 0, None),
    ] {
        conn.execute(
            "INSERT INTO cards VALUES (?1, ?2, NULL, 'dev', '2000-01-01', ?3, ?4)",
            params![row.0, row.1, row.2, row.3],
        )
        .expect("insert test card");
    }

    let unfiltered = load_audit_cards(&conn, "dev", false).expect("unfiltered cards");
    let filtered = load_audit_cards(&conn, "dev", true).expect("filtered cards");
    assert_eq!(unfiltered.len(), 4);
    assert_eq!(filtered.len(), 3);
    ensure_held_out_excluded(&filtered, true).expect("held-out postcondition");
    assert!(audit_export_payload("dev", &unfiltered, true).is_err());
}

#[test]
fn audit_summary_counts_match_template_inclusion_policy() {
    let cards = vec![
        audit_card(
            "Example",
            "tst",
            "2000-01-01",
            Some(
                "Flying\n({T}: Add {G}.)\nChoose one \u{2014}\n\u{2022} Draw a card.\n\u{2022} You gain 3 life.\nDo one thing. Do another thing.",
            ),
        ),
        audit_card("No Text", "tst", "2000-01-01", None),
    ];
    let summary = summarize_audit(&cards);
    assert_eq!(summary.cards, 2);
    assert_eq!(summary.cards_with_text, 1);
    assert_eq!(summary.printed_units, 5);
    assert_eq!(summary.rules_supplied_units, 1);
    assert_eq!(summary.distinct_printed_templates, 5);
    assert_eq!(summary.singleton_templates, 5);
    assert_eq!(summary.multi_sentence_units, 1);
    assert_eq!(summary.residual_spell_or_static_units, 4);
    assert_eq!(summary.uncited_rules_supplied_units, 0);
    assert_eq!(summary.sources.get(&TextSource::Printed), Some(&5));
    assert_eq!(summary.sources.get(&TextSource::RulesSupplied), Some(&1));
}

#[test]
fn novelty_classifies_units_and_templates_against_earlier_sets() {
    let earlier = vec![audit_card(
        "Earlier",
        "old",
        "1993-01-01",
        Some("Flying\nDraw a card."),
    )];
    let selected = vec![audit_card(
        "Selected",
        "new",
        "1993-02-01",
        Some("Flying\nDestroy target creature.\nDestroy target creature."),
    )];
    let report = novelty_report(&selected, &earlier);
    assert_eq!(report["total_printed_units"].as_u64(), Some(3));
    assert_eq!(report["units_seen_earlier"].as_u64(), Some(1));
    assert_eq!(report["novel_units"].as_u64(), Some(2));
    assert_eq!(report["unit_novelty_percent"].as_f64(), Some(66.67));
    assert_eq!(report["distinct_templates"].as_u64(), Some(2));
    assert_eq!(report["templates_seen_earlier"].as_u64(), Some(1));
    assert_eq!(report["novel_templates"].as_u64(), Some(1));
    assert_eq!(report["template_novelty_percent"].as_f64(), Some(50.0));
    assert_eq!(
        report["novel_template_records"][0]["count"].as_u64(),
        Some(2)
    );
}

#[test]
fn novelty_for_no_earlier_set_is_one_hundred_percent() {
    let selected = vec![audit_card(
        "Alpha Example",
        "lea",
        "1993-08-05",
        Some("Flying\nDraw a card."),
    )];
    let report = novelty_report(&selected, &[]);
    assert_eq!(report["total_printed_units"].as_u64(), Some(2));
    assert_eq!(report["units_seen_earlier"].as_u64(), Some(0));
    assert_eq!(report["novel_units"].as_u64(), Some(2));
    assert_eq!(report["unit_novelty_percent"].as_f64(), Some(100.0));
    assert_eq!(report["templates_seen_earlier"].as_u64(), Some(0));
    assert_eq!(report["novel_templates"].as_u64(), Some(2));
    assert_eq!(report["template_novelty_percent"].as_f64(), Some(100.0));
}

#[test]
fn suspicious_signals_include_positive_and_negative_cases() {
    let cards = vec![audit_card(
        "Signals",
        "sig",
        "2000-01-01",
        Some(
            "Do one thing. Do another thing.\n\
                 (Theme color: {W})\n\
                 Choose \"left\" or \"right\".\n\
                 Draw a card. Activate only as a sorcery.\n\
                 This creature enters tapped.\n\
                 At end of combat, draw a card.\n\
                 As long as this creature isn't attacking, its power and toughness are each equal to the number of Forests you control.\n\
                 Spend only black mana on X.\n\
                 Other creatures have \"{T}: Add {G}.\"",
        ),
    )];
    let records = audit_records(&cards);
    let has_signal = |name: &str| records.iter().any(|record| record.signals.contains(&name));
    assert!(has_signal("residual_multi_sentence_unit"));
    assert!(has_signal("uncited_rules_supplied_unit"));
    assert!(has_signal("quoted_text_not_extracted_candidate"));
    assert!(has_signal("activation_restriction_embedded_candidate"));
    assert!(has_signal("delayed_trigger_unattached_candidate"));
    assert!(has_signal("conditional_cda_candidate"));
    assert!(has_signal("payment_restriction_embedded_candidate"));

    let granted_parent = records
        .iter()
        .find(|record| record.normalized == "Other creatures have \"[ability]\"")
        .expect("granted parent");
    assert!(
        !granted_parent
            .signals
            .contains(&"quoted_text_not_extracted_candidate")
    );
    let cited_land = audit_records(&[audit_card(
        "Forest",
        "lea",
        "1993-08-05",
        Some("({T}: Add {G}.)"),
    )]);
    assert!(cited_land[0].signals.is_empty());
}

#[test]
fn suspicious_signals_flag_unresolved_single_sentence_delayed_trigger() {
    // P-ATQ-1's conservative fallback: when a single sentence has no
    // valid complete-effect-clause boundary before its delayed-trigger
    // phrase, the unit is kept whole (not split at a fabricated
    // comma/colon boundary) and the existing T8-style
    // `delayed_trigger_unattached_candidate` signal records the miss.
    let cards = vec![audit_card(
        "Whole Trigger",
        "sig",
        "2000-01-01",
        Some("Whenever this creature becomes blocked, destroy that creature at end of combat."),
    )];
    let records = audit_records(&cards);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].parent_index, None);
    assert!(
        records[0]
            .signals
            .contains(&"delayed_trigger_unattached_candidate")
    );

    let resolved = audit_records(&[audit_card(
        "Resolved Trigger",
        "sig",
        "2000-01-01",
        Some("Draw a card. At end of combat, destroy target creature."),
    )]);
    assert!(!resolved.iter().any(|record| {
        record
            .signals
            .contains(&"delayed_trigger_unattached_candidate")
    }));
}
