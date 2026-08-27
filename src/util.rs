use std::collections::BTreeMap;

use serde::Serialize;
use serde_json::{Value, json};

/// Serialize a label-keyed histogram as a JSON object keyed by the label's
/// serde name.
pub(crate) fn histogram<K: Serialize>(map: BTreeMap<K, u64>) -> Value {
    map.into_iter()
        .map(|(key, count)| {
            let label = serde_json::to_value(key)
                .ok()
                .and_then(|value| value.as_str().map(str::to_owned))
                .unwrap_or_default();
            (label, json!(count))
        })
        .collect::<serde_json::Map<_, _>>()
        .into()
}

pub(crate) fn percent(part: u64, total: u64) -> f64 {
    if total == 0 {
        0.0
    } else {
        ((part as f64 / total as f64) * 10_000.0).round() / 100.0
    }
}
