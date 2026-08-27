use super::support::segment_text;
use crate::segment::{AbilityKind, normalize_text};

#[test]
fn normalization_handles_nested_reminder_text_and_self_references() {
    let text = "Example deals 3 damage. (Use {R} (not {G}).)";
    assert_eq!(normalize_text(text, "Example"), "~ deals N damage.");
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
