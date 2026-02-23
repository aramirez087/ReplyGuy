//! Activity feed endpoint.

use std::sync::Arc;

use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};
use tuitbot_core::storage::action_log;

use crate::error::ApiError;
use crate::state::AppState;

/// Query parameters for the activity endpoint.
#[derive(Deserialize)]
pub struct ActivityQuery {
    /// Maximum number of actions to return (default: 50).
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_limit() -> u32 {
    50
}

/// `GET /api/activity` — recent actions from the action log.
pub async fn list_activity(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ActivityQuery>,
) -> Result<Json<Value>, ApiError> {
    let actions = action_log::get_recent_actions(&state.db, params.limit).await?;
    Ok(Json(json!(actions)))
}
