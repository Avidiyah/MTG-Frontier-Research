use super::support::*;
use crate::segment::{
    AbilityKind, StructuralRole, has_delayed_trigger_temporal_scope, has_off_stack_evidence,
    is_cast_or_resolve_trigger,
};

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
fn recurring_end_of_combat_trigger_after_replacement_is_not_delayed() {
    let segments = segment_text(
        "This creature enters with seven +1/+0 counters on it.\nAt end of combat, if this creature attacked or blocked this combat, remove a +1/+0 counter from it.",
        "",
    );
    assert_eq!(segments.len(), 2);
    assert_eq!(segments[0].kind, AbilityKind::Replacement);
    assert_eq!(segments[1].kind, AbilityKind::Triggered);
    assert_eq!(segments[1].role, StructuralRole::Ability);
    assert!(segments[0].children.is_empty());
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
fn sentence_level_delayed_trigger_still_splits_as_a_child() {
    // P-ATQ-1 preserves the validated P-ARN-1 sentence-level split: a
    // trailing sentence beginning with a delayed-trigger phrase, after a
    // complete preceding sentence, becomes a delayed_trigger child and
    // the parent ability remains valid.
    let segments = segment_text(
        "Perform an effect. At the beginning of the next end step, perform another effect.",
        "",
    );
    assert_eq!(segments.len(), 1);
    assert_eq!(segments[0].normalized, "Perform an effect.");
    assert_eq!(segments[0].children.len(), 1);
    assert_eq!(segments[0].children[0].role, StructuralRole::DelayedTrigger);
    assert_eq!(segments[0].children[0].kind, AbilityKind::Triggered);
    assert_eq!(
        segments[0].children[0].normalized,
        "At the beginning of the next end step, perform another effect."
    );
}

#[test]
fn inverted_next_step_delayed_trigger_in_a_single_sentence_stays_whole() {
    // P-ATQ-1: retracts the comma/colon fallback that used to split these
    // (Antiquities audit V3; Arabian Nights Rukh Egg #0, Nafs Asp #0) into
    // a bare trigger-condition parent (`When ~ dies,`) and a child. With
    // no sentence boundary before the delayed-trigger phrase, the whole
    // single-sentence ability is kept as one unit and the unattached
    // trigger is left as a signal (see
    // `suspicious_signals_flag_unresolved_single_sentence_delayed_trigger`).
    let rukh = segment_text(
        "When this creature dies, create a 4/4 red Bird creature token with flying at the beginning of the next end step.",
        "Rukh Egg",
    );
    assert_eq!(rukh.len(), 1);
    assert_eq!(rukh[0].kind, AbilityKind::Triggered);
    assert_eq!(rukh[0].role, StructuralRole::Ability);
    assert!(rukh[0].children.is_empty());
    assert_eq!(
        rukh[0].normalized,
        "When ~ dies, create a N/N red Bird creature token with flying at the beginning of the next end step."
    );

    let nafs = segment_text(
        "Whenever this creature deals damage to a player, that player loses 1 life at the beginning of their next draw step unless they pay {1} before that draw step.",
        "Nafs Asp",
    );
    assert_eq!(nafs.len(), 1);
    assert!(nafs[0].children.is_empty());
    assert_eq!(
        nafs[0].normalized,
        "Whenever ~ deals damage to a player, that player loses N life at the beginning of their next draw step unless they pay {M} before that draw step."
    );
}

#[test]
fn activation_cost_colon_is_not_split_into_its_own_parent() {
    // P-ATQ-1: never split at the activation-cost colon (CR 602.1a) —
    // `{T}:` must never be emitted as its own structural unit.
    let segments = segment_text(
        "{T}: Destroy target creature that blocked this creature this turn at end of combat.",
        "",
    );
    assert_eq!(segments.len(), 1);
    assert_eq!(segments[0].kind, AbilityKind::Activated);
    assert!(segments[0].children.is_empty());
    assert_ne!(segments[0].normalized, "{M}:");
    assert_eq!(
        segments[0].normalized,
        "{M}: Destroy target creature that blocked ~ this turn at end of combat."
    );
}

#[test]
fn delayed_trigger_split_preserves_conditional_leadin_and_compound_condition() {
    let dragon = segment_text(
        "{R}: This creature gets +1/+0 until end of turn. If this ability has been activated four or more times this turn, sacrifice this creature at the beginning of the next end step.",
        "Dragon Whelp",
    );
    assert_eq!(dragon[0].normalized, "{M}: ~ gets +N/+N until end of turn.");
    assert_eq!(dragon[0].children[0].role, StructuralRole::DelayedTrigger);
    assert_eq!(
        dragon[0].children[0].normalized,
        "If this ability has been activated four or more times this turn, sacrifice ~ at the beginning of the next end step."
    );

    let compound = segment_text(
        "Create a 1/1 creature token. Sacrifice it at the beginning of the next end step or if it would leave the battlefield.",
        "",
    );
    assert_eq!(compound[0].normalized, "Create a N/N creature token.");
    assert_eq!(
        compound[0].children[0].normalized,
        "Sacrifice it at the beginning of the next end step or if it would leave the battlefield."
    );
}

#[test]
fn end_of_combat_delayed_trigger_in_a_single_sentence_stays_whole() {
    // P-ATQ-1: this used to be split at the comma closing the leading
    // `Whenever CONDITION,` trigger clause (rule (c), rejected on
    // corpus evidence in `docs/findings/atq-structural-audit.md`,
    // Antiquities Battering Ram #1 class). The comma does not close a
    // complete effect clause, so the whole triggered ability now stays
    // one unit.
    let segments = segment_text(
        "Whenever this creature blocks or becomes blocked by a non-Wall creature, destroy that creature at end of combat.",
        "",
    );
    assert_eq!(segments.len(), 1);
    assert_eq!(segments[0].kind, AbilityKind::Triggered);
    assert!(segments[0].children.is_empty());
    assert_eq!(
        segments[0].normalized,
        "Whenever ~ blocks or becomes blocked by a non-Wall creature, destroy that creature at end of combat."
    );
}

#[test]
fn activation_instruction_after_delayed_trigger_stays_on_parent() {
    let segments = segment_text(
        "{T}: Choose target non-Wall creature. That creature attacks this turn if able. Destroy it at the beginning of the next end step if it didn't attack this turn. Activate only during an opponent's turn, before attackers are declared.",
        "",
    );
    assert_eq!(segments.len(), 1);
    assert_eq!(
        segments[0].normalized,
        "{M}: Choose target non-Wall creature. That creature attacks this turn if able. Activate only during an opponent's turn, before attackers are declared."
    );
    assert_eq!(segments[0].children.len(), 1);
    assert_eq!(
        segments[0].children[0].normalized,
        "Destroy it at the beginning of the next end step if it didn't attack this turn."
    );
}

#[test]
fn delayed_trigger_text_in_reminder_text_is_not_split() {
    let segments = segment_text(
        "Draw a card. (At the beginning of the next upkeep, this reminder explains timing.)",
        "",
    );
    assert_eq!(segments.len(), 1);
    assert!(segments[0].children.is_empty());
    assert_eq!(segments[0].normalized, "Draw a card.");
}

#[test]
fn delayed_trigger_inside_quoted_ability_stays_under_granted_child() {
    // The granted ability's own text is a single sentence with no
    // sentence boundary before "at end of combat", so P-ATQ-1 keeps it
    // whole (no comma-fallback split) rather than manufacturing a
    // trigger-condition-only child under the granted ability.
    let segments = segment_text(
        "Equipped creature has \"Whenever this creature attacks, sacrifice it at end of combat.\"",
        "",
    );
    assert_eq!(segments.len(), 1);
    assert_eq!(segments[0].children.len(), 1);
    let granted = &segments[0].children[0];
    assert_eq!(granted.role, StructuralRole::Granted);
    assert_eq!(granted.kind, AbilityKind::Triggered);
    assert!(granted.children.is_empty());
    assert_eq!(
        granted.normalized,
        "Whenever ~ attacks, sacrifice it at end of combat."
    );
}

#[test]
fn delayed_trigger_and_punctuation_inside_quotes_are_not_split() {
    // Structural properties, not the outer/inner quote status, drive the
    // split: a comma or colon inside a quoted granted ability must never
    // be mistaken for the outer or inner unit's structural boundary
    // (Antiquities corpus check: rule (c) used to `rfind` on unmasked
    // text and could split inside a quotation). Neither the outer
    // sentence (no visible delayed-trigger phrase once the quote is
    // masked) nor the granted ability itself (a single sentence, no
    // sentence boundary before "at end of combat") is split.
    let segments = segment_text(
        "Whenever this creature blocks, it deals damage equal to its power to any target. Equipped creature has \"{T}: Exile target creature, then return it to the battlefield tapped at end of combat.\"",
        "",
    );
    assert_eq!(segments.len(), 1);
    assert_eq!(
        segments[0].normalized,
        "Whenever ~ blocks, it deals damage equal to its power to any target. Equipped creature has \"[ability]\""
    );
    assert_eq!(segments[0].children.len(), 1);
    let granted = &segments[0].children[0];
    assert_eq!(granted.role, StructuralRole::Granted);
    assert_eq!(granted.kind, AbilityKind::Activated);
    assert!(granted.children.is_empty());
    assert_eq!(
        granted.normalized,
        "{M}: Exile target creature, then return it to the battlefield tapped at end of combat."
    );
}

#[test]
fn scoped_sentence_initial_when_forms_split_as_delayed_triggers() {
    let sandals = segment_text(
        "{2}, {T}: Target creature gains islandwalk until end of turn. When that creature dies this turn, destroy this artifact.",
        "Sandals of Abdallah",
    );
    assert_eq!(sandals.len(), 1);
    assert_eq!(sandals[0].kind, AbilityKind::Activated);
    assert_eq!(sandals[0].children[0].role, StructuralRole::DelayedTrigger);
    assert_eq!(
        sandals[0].children[0].normalized,
        "When that creature dies this turn, destroy ~."
    );

    let this_way = segment_text(
        "Exile target creature. Whenever a creature exiled this way dies this way, draw a card.",
        "",
    );
    assert_eq!(this_way[0].children[0].role, StructuralRole::DelayedTrigger);

    let reflexive = segment_text(
        "You may sacrifice a creature. When you do, draw a card.",
        "",
    );
    assert_eq!(
        reflexive[0].children[0].role,
        StructuralRole::DelayedTrigger
    );
}

#[test]
fn unscoped_sentence_initial_when_forms_are_not_delayed_trigger_children() {
    let independent = segment_text(
        "Remove a time counter from this permanent. When the last is removed, sacrifice it.",
        "",
    );
    assert_eq!(independent.len(), 1);
    assert!(independent[0].children.is_empty());

    let animate_dead = segment_text(
        "When this Aura enters, return enchanted creature card to the battlefield under your control and attach this Aura to it. When this Aura leaves the battlefield, that creature's controller sacrifices it.",
        "Animate Dead",
    );
    assert_eq!(animate_dead.len(), 1);
    assert!(animate_dead[0].children.is_empty());
}
#[test]
fn whenever_this_turn_on_an_instant_becomes_a_delayed_trigger() {
    // Class A.
    let segments = segment_text_with_type(
        "Whenever a creature attacks this turn, it gets +1/+0 until end of turn.",
        "",
        "Instant",
    );
    assert_eq!(segments[0].kind, AbilityKind::Triggered);
    assert_eq!(segments[0].role, StructuralRole::DelayedTrigger);
}

#[test]
fn when_you_next_cast_this_turn_on_a_sorcery_becomes_a_delayed_trigger() {
    // Class A: "next" scoping without "this turn"/"this combat" in the
    // trigger clause itself, guarding against a fix that only handles
    // the bare duration words.
    let segments = segment_text_with_type(
        "When you next cast an instant or sorcery spell this turn, copy that spell.",
        "",
        "Sorcery",
    );
    assert_eq!(segments[0].kind, AbilityKind::Triggered);
    assert_eq!(segments[0].role, StructuralRole::DelayedTrigger);
}

#[test]
fn at_the_beginning_of_combat_this_turn_on_a_sorcery_becomes_a_delayed_trigger() {
    // Class A, "At ..." trigger word.
    let segments = segment_text_with_type(
        "At the beginning of combat this turn, untap target creature.",
        "",
        "Sorcery",
    );
    assert_eq!(segments[0].kind, AbilityKind::Triggered);
    assert_eq!(segments[0].role, StructuralRole::DelayedTrigger);
}

#[test]
fn this_combat_scoping_on_an_instant_becomes_a_delayed_trigger() {
    // Class A, "this combat" duration.
    let segments = segment_text_with_type(
        "Whenever a creature blocks this combat, it gets -1/-1 until end of turn.",
        "",
        "Instant",
    );
    assert_eq!(segments[0].kind, AbilityKind::Triggered);
    assert_eq!(segments[0].role, StructuralRole::DelayedTrigger);
}

#[test]
fn next_end_step_on_a_single_line_spell_becomes_a_delayed_trigger_without_a_parent() {
    // A single-line instant/sorcery whose entire text is the delayed
    // trigger: before P-ATQ-4 this fell back to `role = ability`
    // because the pre-existing `delayed_trigger_start` mechanism only
    // keeps `delayed_trigger` when it can attach the unit as a child
    // of a preceding sibling, and a lone top-level unit has none. This
    // unit must stay top-level (`parent_index` has no representation
    // here, but nothing pushes it into another unit's `children`)
    // while still carrying the corrected role.
    let segments = segment_text_with_type(
        "At the beginning of the next end step, return that creature to the battlefield.",
        "",
        "Sorcery",
    );
    assert_eq!(segments.len(), 1);
    assert_eq!(segments[0].kind, AbilityKind::Triggered);
    assert_eq!(segments[0].role, StructuralRole::DelayedTrigger);
    assert!(segments[0].children.is_empty());
}

#[test]
fn cycling_trigger_stays_an_ordinary_card_ability() {
    // Class B: the ability functions from hand as part of cycling
    // (CR 113.6b), not created by the spell's resolution.
    let segments = segment_text_with_type("When you cycle this card, draw a card.", "", "Instant");
    assert_eq!(segments[0].kind, AbilityKind::Triggered);
    assert_eq!(segments[0].role, StructuralRole::Ability);
}

#[test]
fn graveyard_functioning_trigger_stays_an_ordinary_card_ability() {
    // Class B: "if this card is in your graveyard" is the ability's own
    // zone check (CR 113.6b), not a duration on a delayed trigger.
    let segments = segment_text_with_type(
        "Whenever an opponent gains life, if this card is in your graveyard, you may return it to your hand.",
        "",
        "Sorcery",
    );
    assert_eq!(segments[0].kind, AbilityKind::Triggered);
    assert_eq!(segments[0].role, StructuralRole::Ability);
}

#[test]
fn a_delayed_trigger_scoped_this_turn_that_returns_someone_elses_cards_from_a_graveyard_is_not_excluded()
 {
    // Guards the negative-evidence check itself: the mere word
    // "graveyard" must not disqualify an otherwise valid delayed
    // trigger when the graveyard is the destination/subject of the
    // effect rather than the ability's own zone check. Contrast with
    // the previous test, where "this card is in your graveyard" *is*
    // a self zone-check.
    let segments = segment_text_with_type(
        "Whenever a spell or ability an opponent controls causes you to discard cards this turn, return those cards from your graveyard to your hand.",
        "",
        "Instant",
    );
    assert_eq!(segments[0].kind, AbilityKind::Triggered);
    assert_eq!(segments[0].role, StructuralRole::DelayedTrigger);
}

#[test]
fn discard_trigger_stays_an_ordinary_card_ability() {
    // Class B: a trigger condition on the card's own movement out of
    // hand (CR 603.6c-style zone-change trigger), not a delayed
    // trigger created by resolving a spell.
    let segments = segment_text_with_type(
        "When you discard this card, you may pay {B}. If you do, return it to your hand.",
        "",
        "Instant",
    );
    assert_eq!(segments[0].kind, AbilityKind::Triggered);
    assert_eq!(segments[0].role, StructuralRole::Ability);
}

#[test]
fn suspend_off_stack_trigger_stays_an_ordinary_card_ability() {
    // Class B: a suspended card's own upkeep trigger (CR 702.61-style),
    // functioning from exile, not created by spell resolution.
    let segments = segment_text_with_type(
        "At the beginning of each upkeep, if this card is suspended, remove a time counter from it.",
        "",
        "Sorcery",
    );
    assert_eq!(segments[0].kind, AbilityKind::Triggered);
    assert_eq!(segments[0].role, StructuralRole::Ability);
}

#[test]
fn cast_trigger_stays_an_ordinary_card_ability_even_with_a_this_turn_phrase() {
    // Class C: the trigger condition is the spell's own casting, not an
    // event its resolution watches for afterward. Deliberately includes
    // a "this turn" phrase elsewhere in the effect text, to prove the
    // cast/resolve exclusion takes precedence over the temporal-scope
    // evidence rather than being redundant with it.
    let segments = segment_text_with_type(
        "When you cast this spell, copy it for each other instant and sorcery spell you've cast this turn.",
        "",
        "Sorcery",
    );
    assert_eq!(segments[0].kind, AbilityKind::Triggered);
    assert_eq!(segments[0].role, StructuralRole::Ability);
}

#[test]
fn resolution_trigger_stays_an_ordinary_card_ability() {
    // Class C: "When this spell resolves" is not itself the delayed
    // trigger CR 603.7d describes; it is the spell's own resolution
    // trigger.
    let segments = segment_text_with_type(
        "When this spell resolves, discard a card. Then draw a card.",
        "",
        "Instant",
    );
    assert_eq!(segments[0].kind, AbilityKind::Triggered);
    assert_eq!(segments[0].role, StructuralRole::Ability);
}

#[test]
fn identical_wording_on_a_permanent_face_does_not_become_a_delayed_trigger() {
    // Type-line context matters: the same "this turn" trigger text that
    // is P-ATQ-4 on an instant/sorcery face is an ordinary printed
    // ability when it appears on a permanent.
    let segments = segment_text_with_type(
        "Whenever a creature attacks this turn, it gets +1/+0 until end of turn.",
        "",
        "Creature — Human",
    );
    assert_eq!(segments[0].kind, AbilityKind::Triggered);
    assert_eq!(segments[0].role, StructuralRole::Ability);
}

#[test]
fn non_trigger_spell_text_with_this_turn_is_unaffected() {
    // The duration phrase alone is insufficient: this unit is not even
    // `kind = triggered_ability`, so P-ATQ-4 must never touch it.
    let segments = segment_text_with_type(
        "Target creature gets +2/+2 until end of turn.",
        "",
        "Instant",
    );
    assert_eq!(segments[0].kind, AbilityKind::SpellOrStatic);
    assert_eq!(segments[0].role, StructuralRole::Ability);
}

#[test]
fn p_atq_1_delayed_trigger_child_splitting_is_unaffected_by_p_atq_4() {
    // A multi-line spell that already produces a `delayed_trigger`
    // *child* via P-ATQ-1's sentence-boundary split must keep exactly
    // that shape: P-ATQ-4 only changes the role of a top-level unit in
    // place and must never reparent or duplicate an existing child.
    let segments = segment_text_with_type(
        "Do something. At the beginning of the next end step, do something else.",
        "",
        "Sorcery",
    );
    assert_eq!(segments.len(), 1);
    assert_eq!(segments[0].kind, AbilityKind::SpellOrStatic);
    assert_eq!(segments[0].role, StructuralRole::Ability);
    assert_eq!(segments[0].children.len(), 1);
    let child = &segments[0].children[0];
    assert_eq!(child.kind, AbilityKind::Triggered);
    assert_eq!(child.role, StructuralRole::DelayedTrigger);
    assert_eq!(
        child.normalized,
        "At the beginning of the next end step, do something else."
    );
}

#[test]
fn p_atq_2_prevention_prohibition_exclusion_is_unaffected_by_p_atq_4() {
    assert_eq!(
        segment_text("Damage can't be prevented.", "")[0].kind,
        AbilityKind::SpellOrStatic
    );
}

#[test]
fn p_atq_3_saga_chapter_and_ability_word_handling_is_unaffected_by_p_atq_4() {
    let saga = segment_text_with_type(
        "I, II — Prevent all damage that would be dealt to creatures you control this turn.",
        "",
        "Enchantment — Saga",
    );
    assert_eq!(saga[0].kind, AbilityKind::Triggered);
    assert_eq!(saga[0].prefix.as_deref(), Some("I, II"));
    // Not an instant/sorcery face, so P-ATQ-4 does not apply even
    // though the body reads as a delayed trigger; the Saga
    // chapter-symbol kind override from P-ATQ-3 is what makes it
    // `triggered_ability` here, unrelated to P-ATQ-4's role change.
    assert_eq!(saga[0].role, StructuralRole::Ability);

    let ability_word = segment_text(
        "Heroic — Whenever you cast a spell that targets this creature, do something.",
        "",
    );
    assert_eq!(ability_word[0].kind, AbilityKind::Triggered);
    assert_eq!(ability_word[0].prefix.as_deref(), Some("Heroic"));
    assert_eq!(ability_word[0].role, StructuralRole::Ability);
}

#[test]
fn p_atq_3_prefix_stripped_body_is_the_evidence_p_atq_4_reads() {
    // An ability-word prefix in front of an otherwise qualifying
    // Class A body must not hide the temporal-scope evidence from
    // P-ATQ-4, the same way it must not hide the trigger word from
    // `classify_kind` (P-ATQ-3).
    let segments = segment_text_with_type(
        "Adamant — Whenever a creature attacks this turn, it gets +1/+0 until end of turn.",
        "",
        "Instant",
    );
    assert_eq!(segments[0].kind, AbilityKind::Triggered);
    assert_eq!(segments[0].prefix.as_deref(), Some("Adamant"));
    assert_eq!(segments[0].role, StructuralRole::DelayedTrigger);
}

#[test]
fn has_delayed_trigger_temporal_scope_requires_a_stated_duration() {
    assert!(has_delayed_trigger_temporal_scope(
        "Whenever a creature blocks this turn, it gets -1/-1."
    ));
    assert!(has_delayed_trigger_temporal_scope(
        "When you next cast an instant or sorcery spell, copy it."
    ));
    assert!(!has_delayed_trigger_temporal_scope(
        "When you cycle this card, draw a card."
    ));
}

#[test]
fn has_off_stack_evidence_requires_self_reference_near_the_zone_word() {
    assert!(has_off_stack_evidence("if ~ is in your graveyard"));
    assert!(has_off_stack_evidence("When you cycle ~, draw a card."));
    assert!(has_off_stack_evidence("you discard ~"));
    // A zone word describing someone else's cards, with no self
    // reference nearby, is not off-stack evidence.
    assert!(!has_off_stack_evidence(
        "return those cards from your graveyard to your hand"
    ));
}

#[test]
fn is_cast_or_resolve_trigger_excludes_the_spells_own_casting_and_resolution() {
    assert!(is_cast_or_resolve_trigger(
        "When you cast ~, copy it for each spell you've cast this turn."
    ));
    assert!(is_cast_or_resolve_trigger(
        "When ~ resolves, discard a card."
    ));
    assert!(is_cast_or_resolve_trigger(
        "Whenever ~ is countered or fizzles, you may copy it."
    ));
    assert!(!is_cast_or_resolve_trigger(
        "Whenever a creature attacks this turn, it gets +1/+0."
    ));
}
