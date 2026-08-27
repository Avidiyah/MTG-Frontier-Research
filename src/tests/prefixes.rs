use super::support::*;
use crate::segment::{AbilityKind, StructuralRole, extract_prefix, is_saga_chapter_prefix};

#[test]
fn ability_word_prefix_exposes_the_whenever_trigger_to_the_kind_classifier() {
    let segments = segment_text(
        "Heroic — Whenever you cast a spell that targets this creature, do something.",
        "",
    );
    assert_eq!(segments[0].kind, AbilityKind::Triggered);
    assert_eq!(segments[0].prefix.as_deref(), Some("Heroic"));
    // The original Oracle text is preserved verbatim on `text`.
    assert_eq!(
        segments[0].text,
        "Heroic — Whenever you cast a spell that targets this creature, do something."
    );
}

#[test]
fn ability_word_prefix_before_an_at_trigger_is_also_recovered() {
    // Guards against a fix that only handles "Whenever": a second
    // ability word over an "At the beginning of ..." trigger.
    let segments = segment_text(
        "Lieutenant — At the beginning of combat on your turn, if condition, effect.",
        "",
    );
    assert_eq!(segments[0].kind, AbilityKind::Triggered);
    assert_eq!(segments[0].prefix.as_deref(), Some("Lieutenant"));
}

#[test]
fn saga_chapter_prefix_is_a_triggered_ability_regardless_of_its_effect_verb() {
    // CR 714.2b: a chapter symbol "is a keyword ability that represents
    // a triggered ability" no matter what the printed effect after it
    // says. Naively stripping "I, II — " and running `classify_kind` on
    // "Prevent ..." would reproduce the P-ATQ-3 failure as
    // `prevention_effect` instead of `triggered_ability`; the chapter
    // case must not take that path.
    let segments = segment_text_with_type(
        "I, II — Prevent all damage that would be dealt to creatures you control this turn.",
        "",
        "Enchantment — Saga",
    );
    assert_eq!(segments[0].kind, AbilityKind::Triggered);
    assert_eq!(segments[0].prefix.as_deref(), Some("I, II"));
    assert_eq!(
        segments[0].text,
        "I, II — Prevent all damage that would be dealt to creatures you control this turn."
    );
}

#[test]
fn single_saga_chapter_marker_is_handled_consistently_with_multi_chapter_markers() {
    let segments = segment_text_with_type(
        "II — Prevent all damage that would be dealt by up to one target creature for as long as this Saga remains on the battlefield.",
        "",
        "Enchantment — Saga",
    );
    assert_eq!(segments[0].kind, AbilityKind::Triggered);
    assert_eq!(segments[0].prefix.as_deref(), Some("II"));
}

#[test]
fn roman_numeral_prefix_off_a_saga_is_not_treated_as_a_chapter_symbol() {
    // CR 714 chapter symbols exist only on Saga cards; the same
    // wording on a non-Saga permanent is a named-mode/label prefix
    // instead and falls through to ordinary body classification.
    let segments = segment_text_with_type(
        "II — Do something that is not a Comprehensive Rules chapter effect.",
        "",
        "Artifact Creature",
    );
    assert_eq!(segments[0].prefix.as_deref(), Some("II"));
    assert_eq!(segments[0].kind, AbilityKind::SpellOrStatic);
}

#[test]
fn named_mode_prefix_is_recorded_without_disturbing_mode_role_or_body_classification() {
    let segments = segment_text_with_type(
        "Choose one —\n• Run and Hide — Prevent all combat damage that would be dealt to you and creatures you control this turn.\n• Do something else.",
        "",
        "Instant",
    );
    let mode = &segments[0].children[0];
    assert_eq!(mode.role, StructuralRole::Mode);
    assert_eq!(mode.prefix.as_deref(), Some("Run and Hide"));
    // With the label out of the way the body is a plain CR 615.1a
    // static prevention effect, the same kind this repository's
    // existing prevention machinery already assigns to that wording
    // (mode `kind` is informational under the annotation protocol,
    // scored `n/a`; this is not a new claim about mode semantics).
    assert_eq!(mode.kind, AbilityKind::Prevention);
}

#[test]
fn prefix_with_an_early_colon_is_not_stripped() {
    // Guards against confusing activated-ability syntax (a cost colon)
    // with a label.
    let segments = segment_text(
        "Sacrifice a creature: Heroic — Whenever you cast a spell, draw a card.",
        "",
    );
    assert_eq!(segments[0].prefix, None);
    assert_eq!(segments[0].kind, AbilityKind::Activated);
}

#[test]
fn prefix_with_an_early_period_is_not_stripped() {
    let segments = segment_text(
        "Cast only as a sorcery. Heroic — Whenever you cast a spell, draw a card.",
        "",
    );
    assert_eq!(segments[0].prefix, None);
}

#[test]
fn overlong_prefix_is_not_stripped() {
    let segments = segment_text(
        "This ability word is deliberately far too long to count as a bounded prefix — Whenever you cast a spell, draw a card.",
        "",
    );
    assert_eq!(segments[0].prefix, None);
}

#[test]
fn mode_header_em_dash_with_no_following_body_is_not_treated_as_a_prefix() {
    // "Choose one —" (CR 700.2) is the single most common leading
    // em-dash construction in the corpus; it must never be read as a
    // prefix over its own bullet children.
    let segments = segment_text("Choose one —\n• Draw a card.\n• Discard a card.", "");
    assert_eq!(segments[0].prefix, None);
    assert_eq!(segments[0].normalized, "Choose one —");
}

#[test]
fn p_atq_2_prevention_prohibition_exclusion_is_unaffected_by_prefix_extraction() {
    assert_eq!(
        segment_text("Damage can't be prevented.", "")[0].kind,
        AbilityKind::SpellOrStatic
    );
}

#[test]
fn genuine_prevention_without_a_prefix_remains_prevention_effect() {
    assert_eq!(
        segment_text(
            "Prevent all damage that would be dealt to target creature this turn.",
            "",
        )[0]
        .kind,
        AbilityKind::Prevention
    );
}

#[test]
fn extract_prefix_rejects_a_body_less_em_dash_and_requires_leading_position() {
    assert_eq!(extract_prefix("Choose one —"), None);
    assert_eq!(
        extract_prefix("Heroic — Whenever you cast a spell, draw a card."),
        Some((
            "Heroic".to_owned(),
            "Whenever you cast a spell, draw a card."
        ))
    );
}

#[test]
fn saga_chapter_prefix_requires_pure_roman_numerals() {
    assert!(is_saga_chapter_prefix("I"));
    assert!(is_saga_chapter_prefix("I, II"));
    assert!(is_saga_chapter_prefix("III, IV"));
    assert!(!is_saga_chapter_prefix("N")); // a normalized Arabic numeral
    assert!(!is_saga_chapter_prefix("Heroic"));
    assert!(!is_saga_chapter_prefix(""));
}

// P-ATQ-4: a top-level instant/sorcery unit whose printed text *is* a
// delayed-trigger clause (CR 603.7d) gets `role = delayed_trigger`
// instead of the default `ability`, while `kind` stays
// `triggered_ability`. Synthetic fixtures reproduce the wording
// *classes* found in the historical corpus check (30 spell-created
// delayed triggers, 65 off-stack, 16 cast/resolve, out of 111 I/S
// top-level triggered_ability units); no card name appears in
// production code.
