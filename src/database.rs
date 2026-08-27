use std::path::Path;

use anyhow::{Context, Result, bail};
use rusqlite::{Connection, OpenFlags};
use serde_json::{Value, json};

const HELD_OUT_POLICY: &str = "protocol-v1.0-section-6.3-2026-08-26";
pub(crate) const HELD_OUT_SQL: &str = "oracle_text IS NOT NULL AND oracle_text != '' \
    AND lower(substr(oracle_id, 1, 1)) = 'f' AND first_is_fallback = 0 \
    AND lower(coalesce(first_set, '')) NOT IN ('lea', 'leb', 'arn')";

pub(crate) fn open_db(path: &Path) -> Result<Connection> {
    if !path.is_file() {
        bail!("card database not found: {}", path.display());
    }
    Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .with_context(|| format!("failed to open card database: {}", path.display()))
}

pub(crate) fn held_out_exclusion_metadata(enabled: bool) -> Value {
    json!({
        "enabled": enabled,
        "policy": HELD_OUT_POLICY,
        "stable_identity_scope": "oracle_id (all faces excluded together)",
        "enforcement": "database predicate before card rows are segmented or serialized"
    })
}

/// SQL fragment restricting rows to a first-printing set, bound to `param`.
/// The parameter is always referenced so callers can bind it unconditionally;
/// an empty string (no set requested) matches every row.
pub(crate) fn set_predicate(param: &str) -> String {
    format!(" AND ({param} = '' OR lower(first_set) = {param})")
}

/// SQL fragment that removes protocol 6.3 pool rows when `param` is true.
/// Applying it in the database query prevents held-out identities and text
/// from reaching segmentation or any auditor-visible serializer.
pub(crate) fn held_out_exclusion_predicate(param: &str) -> String {
    format!(" AND ({param} = 0 OR NOT ({HELD_OUT_SQL}))")
}
