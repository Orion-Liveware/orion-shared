//! Serde helpers for serializing `RecordId` as a plain string (e.g., "domain:abc123").
//!
//! SurrealDB v2's `RecordId` serde impl produces a complex structure,
//! but Tauri IPC and the frontend expect a flat string.
//!
//! Usage on struct fields:
//! ```ignore
//! #[serde(serialize_with = "orion_db::record_id_string::serialize")]
//! pub id: RecordId,
//! ```

use serde::Serializer;
use surrealdb::RecordId;

/// Serialize a `RecordId` as its Display string (e.g., "table:key").
pub fn serialize<S: Serializer>(id: &RecordId, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&id.to_string())
}

/// Serialize an `Option<RecordId>` as `Option<String>`.
pub mod option {
    use serde::Serializer;
    use surrealdb::RecordId;

    pub fn serialize<S: Serializer>(
        id: &Option<RecordId>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match id {
            Some(id) => serializer.serialize_some(&id.to_string()),
            None => serializer.serialize_none(),
        }
    }
}
