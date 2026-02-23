//! Target accounts endpoint.

use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};
use tuitbot_core::storage::target_accounts;

use crate::error::ApiError;
use crate::state::AppState;

/// `GET /api/targets` — list target accounts and their state.
pub async fn list_targets(State(state): State<Arc<AppState>>) -> Result<Json<Value>, ApiError> {
    let accounts = target_accounts::get_active_target_accounts(&state.db).await?;
    Ok(Json(json!(accounts)))
}
