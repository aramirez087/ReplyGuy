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
    String,
    String,
    String,
    String,
    f64,
    i64,
    Option<String>,
    Option<String>,
    Option<String>,
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
    /// Full QA report payload as JSON.
    #[serde(serialize_with = "serialize_json_string")]
    pub qa_report: String,
    /// JSON-encoded hard QA flags.
    #[serde(serialize_with = "serialize_json_string")]
    pub qa_hard_flags: String,
    /// JSON-encoded soft QA flags.
    #[serde(serialize_with = "serialize_json_string")]
    pub qa_soft_flags: String,
    /// JSON-encoded QA recommendations.
    #[serde(serialize_with = "serialize_json_string")]
    pub qa_recommendations: String,
    /// QA score summary (0-100).
    pub qa_score: f64,
    /// Whether approval requires explicit hard-flag override.
    pub qa_requires_override: bool,
    /// Actor who performed override.
    pub qa_override_by: Option<String>,
    /// Required override note.
    pub qa_override_note: Option<String>,
    /// Timestamp of override action.
    pub qa_override_at: Option<String>,
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
            qa_report: r.15,
            qa_hard_flags: r.16,
            qa_soft_flags: r.17,
            qa_recommendations: r.18,
            qa_score: r.19,
            qa_requires_override: r.20 != 0,
            qa_override_by: r.21,
            qa_override_note: r.22,
            qa_override_at: r.23,
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
