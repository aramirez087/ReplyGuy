//! Storage operations for the approval queue.
//!
//! Provides CRUD operations for queuing posts for human review
//! when `approval_mode` is enabled.

mod edit_history;
mod queries;
#[cfg(test)]
mod tests;

pub use edit_history::{get_edit_history, record_edit, EditHistoryEntry};
pub use queries::*;

/// Row type for approval queue queries (expanded with review metadata).
type ApprovalRow = (
    i64,
    String,
    String,
    String,
    String,
    String,
    String,
    f64,
    String,
    String,
    String,
    Option<String>,
    Option<String>,
    Option<String>,
    String,
);

/// A pending item in the approval queue.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ApprovalItem {
    pub id: i64,
    pub action_type: String,
    pub target_tweet_id: String,
    pub target_author: String,
    pub generated_content: String,
    pub topic: String,
    pub archetype: String,
    pub score: f64,
    pub status: String,
    pub created_at: String,
    /// JSON-encoded list of local media file paths.
    #[serde(serialize_with = "serialize_json_string")]
    pub media_paths: String,
    pub reviewed_by: Option<String>,
    pub review_notes: Option<String>,
    pub reason: Option<String>,
    /// JSON-encoded list of detected risks.
    #[serde(serialize_with = "serialize_json_string")]
    pub detected_risks: String,
}

/// Serialize a JSON-encoded string as a raw JSON value.
///
/// The database stores `media_paths` and `detected_risks` as JSON strings.
/// This serializer emits them as actual JSON arrays in the API response.
fn serialize_json_string<S: serde::Serializer>(
    value: &str,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    use serde::Serialize;
    let parsed: serde_json::Value =
        serde_json::from_str(value).unwrap_or(serde_json::Value::Array(vec![]));
    parsed.serialize(serializer)
}

impl From<ApprovalRow> for ApprovalItem {
    fn from(r: ApprovalRow) -> Self {
        Self {
            id: r.0,
            action_type: r.1,
            target_tweet_id: r.2,
            target_author: r.3,
            generated_content: r.4,
            topic: r.5,
            archetype: r.6,
            score: r.7,
            status: r.8,
            created_at: r.9,
            media_paths: r.10,
            reviewed_by: r.11,
            review_notes: r.12,
            reason: r.13,
            detected_risks: r.14,
        }
    }
}

/// Counts of approval items grouped by status.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ApprovalStats {
    pub pending: i64,
    pub approved: i64,
    pub rejected: i64,
}

/// Optional review metadata for approve/reject actions.
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct ReviewAction {
    pub actor: Option<String>,
    pub notes: Option<String>,
}
