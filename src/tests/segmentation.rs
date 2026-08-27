use super::support::*;
use crate::segment::{AbilityKind, StructuralRole, TextSource};

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
fn instant_and_sorcery_spell_text_is_type_line_aware() {
    let disintegrate = segment_text_with_type(
        "Disintegrate deals X damage to any target. If it's a creature, it can't be regenerated this turn, and if it would die this turn, exile it instead.",
        "Disintegrate",
        "Sorcery",
    );
    assert_eq!(disintegrate[0].kind, AbilityKind::SpellOrStatic);

    let camouflage = segment_text_with_type(
        "This turn, instead of declaring blockers, each defending player chooses any number of creatures they control and divides them into piles.",
        "Camouflage",
        "Instant",
    );
    assert_eq!(camouflage[0].kind, AbilityKind::SpellOrStatic);

    let eye = segment_text_with_type(
        "The next time a source of your choice would deal damage to you this turn, instead that source deals that much damage to you and Eye for an Eye deals that much damage to that source's controller.",
        "Eye for an Eye",
        "Instant",
    );
    assert_eq!(eye[0].kind, AbilityKind::SpellOrStatic);
}

#[test]
fn instant_and_sorcery_static_exceptions_are_preserved() {
    assert_eq!(
        segment_text_with_type(
            "Cast this spell only before the combat damage step.",
            "Berserk",
            "Instant",
        )[0]
        .kind,
        AbilityKind::CastRestriction
    );
    assert_eq!(
        segment_text_with_type(
            "As an additional cost to cast this spell, sacrifice a creature.",
            "Sacrifice",
            "Sorcery",
        )[0]
        .kind,
        AbilityKind::AdditionalCost
    );
    assert_eq!(
        segment_text_with_type("This spell costs {1} less to cast.", "", "Instant")[0].kind,
        AbilityKind::SpellOrStatic
    );
    assert_eq!(
        segment_text_with_type("This spell can't be countered.", "", "Sorcery")[0].kind,
        AbilityKind::SpellOrStatic
    );
}

#[test]
fn multiface_cards_use_the_current_face_type_line() {
    let segments = segment_text_with_type(
        "If you would draw a card, draw two cards instead.\n//\nThe next time a source would deal damage to you this turn, prevent that damage.",
        "",
        "Enchantment // Instant",
    );
    assert_eq!(segments.len(), 2);
    assert_eq!(segments[0].face, 0);
    assert_eq!(segments[0].kind, AbilityKind::Replacement);
    assert_eq!(segments[1].face, 1);
    assert_eq!(segments[1].kind, AbilityKind::SpellOrStatic);
}

#[test]
fn static_prevention_effects_have_their_own_kind() {
    assert_eq!(
            segment_text(
                "For each 1 damage that would be dealt to this creature, if it has a +1/+1 counter on it, remove a +1/+1 counter from it and prevent that 1 damage.",
                "Rock Hydra",
            )[0]
            .kind,
            AbilityKind::Prevention
        );
    assert_eq!(
            segment_text(
                "As long as this creature is attacking, prevent all damage Deserts would deal to this creature and to creatures banded with this creature.",
                "Camel",
            )[0]
            .kind,
            AbilityKind::Prevention
        );
    assert_eq!(
        segment_text(
            "Prevent all damage that would be dealt to this creature by Deserts.",
            "Desert Nomads",
        )[0]
        .kind,
        AbilityKind::Prevention
    );
}

#[test]
fn prevention_in_activated_triggered_or_spell_text_keeps_precedence() {
    assert_eq!(
        segment_text(
            "{T}: Prevent the next 1 damage that would be dealt to target creature this turn.",
            "Oasis",
        )[0]
        .kind,
        AbilityKind::Activated
    );
    assert_eq!(
            segment_text(
                "Whenever a creature attacks, prevent all combat damage that would be dealt by it this turn.",
                "",
            )[0]
            .kind,
            AbilityKind::Triggered
        );
    assert_eq!(
        segment_text_with_type(
            "Prevent all combat damage that would be dealt this turn.",
            "Fog",
            "Instant",
        )[0]
        .kind,
        AbilityKind::SpellOrStatic
    );
}

#[test]
fn prevention_prohibition_is_not_classified_as_prevention_effect() {
    // P-ATQ-2: "can't/cannot be prevented" prohibits prevention (a
    // rule-modifying statement, CR 614/615 territory but not itself a
    // CR 615.1a prevention effect); it must fall through to the
    // existing residual static kind, not `prevention_effect`
    // (Antiquities corpus check §7 A, 9 misfires).
    assert_eq!(
        segment_text(
            "Damage that would be dealt to you by red sources can't be prevented.",
            "",
        )[0]
        .kind,
        AbilityKind::SpellOrStatic
    );
    // A distinct wording of the same prohibition class, to show the
    // exclusion is structural (the "can't be prevented" collocation),
    // not tied to one exact preceding phrase.
    assert_eq!(
        segment_text("Damage dealt by this creature can't be prevented.", "")[0].kind,
        AbilityKind::SpellOrStatic
    );
    // "cannot" (uncontracted) and a curly apostrophe both still count.
    assert_eq!(
        segment_text("Damage that would be dealt to you cannot be prevented.", "")[0].kind,
        AbilityKind::SpellOrStatic
    );
    assert_eq!(
        segment_text(
            "Damage that would be dealt to you by red sources can\u{2019}t be prevented.",
            "",
        )[0]
        .kind,
        AbilityKind::SpellOrStatic
    );
}

#[test]
fn prevention_prohibition_exclusion_does_not_regress_genuine_prevention() {
    // The exclusion is the narrow "can't/cannot be prevented"
    // collocation, not a blanket "contains 'prevented'" rule: a unit
    // that both commands genuine prevention and separately describes
    // damage as ("is") prevented must still classify as
    // `prevention_effect`.
    assert_eq!(
        segment_text(
            "If this creature would be dealt damage, that damage is prevented.",
            "",
        )[0]
        .kind,
        AbilityKind::Prevention
    );
    // Existing positive prevention cases (imperative "prevent") remain
    // unaffected, since they never contain "prevented" at all.
    assert_eq!(
        segment_text(
            "Prevent all damage that would be dealt to target creature this turn.",
            "",
        )[0]
        .kind,
        AbilityKind::Prevention
    );
}

// P-ATQ-3: strip a leading ability-word / Saga-chapter / named-mode
// prefix ("<prefix> — ", CR 207.2c, 714.2) before classification so it
// cannot hide the trigger word (or other classification evidence) that
// follows it. Synthetic fixtures below reproduce the wording *class*
// found by the Antiquities corpus check (8 prefixed prevention
// misfires); no card name or set code appears in production code.
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
