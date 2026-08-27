use crate::rules::*;

#[test]
fn rules_parser_separates_numbered_rules_and_glossary() {
    let text = "Contents\n100. General\n\n100.1. A rule.\n\nGlossary\n\nAbility\nA definition.\n";
    let entries = parse_rules(text);
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].id.as_deref(), Some("100"));
    assert_eq!(entries[1].id.as_deref(), Some("100.1"));
    assert_eq!(entries[2].heading.as_deref(), Some("Ability"));
    assert_eq!(entries[2].kind, "glossary");
}
#[test]
fn rule_hierarchy_supports_numeric_and_lettered_subrules() {
    assert!(is_rule_within("603.1", "603.1"));
    assert!(is_rule_within("603.1a", "603.1"));
    assert!(is_rule_within("603.1a", "603"));
    assert!(!is_rule_within("603.10", "603.1"));
    assert!(!is_rule_within("603.2", "603.1"));
}
