use crate::segment::{self, AbilityKind, Segment};

pub(super) fn kinds(segments: &[Segment]) -> Vec<AbilityKind> {
    segments.iter().map(|segment| segment.kind).collect()
}

pub(super) fn segment_text(text: &str, card_name: &str) -> Vec<Segment> {
    segment::segment_text(text, card_name, None)
}

pub(super) fn segment_text_with_type(text: &str, card_name: &str, type_line: &str) -> Vec<Segment> {
    segment::segment_text(text, card_name, Some(type_line))
}

pub(super) fn normalized(segments: &[Segment]) -> Vec<&str> {
    segments
        .iter()
        .map(|segment| segment.normalized.as_str())
        .collect()
}
