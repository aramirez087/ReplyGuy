//! Replies endpoint.

use std::sync::Arc;

use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};
use tuitbot_core::storage::replies;

use crate::error::ApiError;
use crate::state::AppState;

/// Query parameters for the replies endpoint.
#[derive(Deserialize)]
pub struct RepliesQuery {
    /// Maximum number of replies to return (default: 50).
    #[serde(default = "default_limit")]
    pub limit: u32,
    /// Offset for pagination (default: 0).
    #[serde(default)]
    pub offset: u32,
}

fn default_limit() -> u32 {
    50
}

/// `GET /api/replies` — recent replies sent.
pub async fn list_replies(
    State(state): State<Arc<AppState>>,
    Query(params): Query<RepliesQuery>,
) -> Result<Json<Value>, ApiError> {
    let replies = replies::get_recent_replies(&state.db, params.limit, params.offset).await?;
    Ok(Json(json!(replies)))
}
