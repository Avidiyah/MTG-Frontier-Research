use crate::cards::escape_like;
use crate::database::*;

#[test]
fn set_predicate_always_references_its_parameter() {
    assert_eq!(
        set_predicate("?3"),
        " AND (?3 = '' OR lower(first_set) = ?3)"
    );
}

#[test]
fn held_out_predicate_is_gated_by_its_parameter() {
    let predicate = held_out_exclusion_predicate("?6");
    assert!(predicate.contains("?6 = 0"));
    assert!(predicate.contains(HELD_OUT_SQL));
}

#[test]
fn like_metacharacters_are_escaped() {
    assert_eq!(escape_like(r"100%_real\value"), r"100\%\_real\\value");
}
