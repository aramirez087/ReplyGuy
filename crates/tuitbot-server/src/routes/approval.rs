//! Approval queue endpoint.

use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};
use tuitbot_core::storage::approval_queue;

use crate::error::ApiError;
use crate::state::AppState;

/// `GET /api/approval` — list pending approval items.
pub async fn list_pending(State(state): State<Arc<AppState>>) -> Result<Json<Value>, ApiError> {
    let items = approval_queue::get_pending(&state.db).await?;
    Ok(Json(json!(items)))
}
