//! Target accounts endpoints.

use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};
use tuitbot_core::storage::target_accounts;

use crate::account::{require_mutate, AccountContext};
use crate::error::ApiError;
use crate::state::AppState;

/// `GET /api/targets` — list target accounts with enriched data.
pub async fn list_targets(
    State(state): State<Arc<AppState>>,
    ctx: AccountContext,
) -> Result<Json<Value>, ApiError> {
    let accounts =
        target_accounts::get_enriched_target_accounts_for(&state.db, &ctx.account_id).await?;
    Ok(Json(json!(accounts)))
}

/// Request body for adding a target account.
#[derive(Deserialize)]
pub struct AddTargetRequest {
    /// Username of the target account (without @).
    pub username: String,
}

/// `POST /api/targets` — add a new target account.
pub async fn add_target(
    State(state): State<Arc<AppState>>,
    ctx: AccountContext,
    Json(body): Json<AddTargetRequest>,
) -> Result<Json<Value>, ApiError> {
    require_mutate(&ctx)?;

    let username = body.username.trim().trim_start_matches('@');

    if username.is_empty() {
        return Err(ApiError::BadRequest("username is required".to_string()));
    }

    // Check if already exists and active.
    if let Some(existing) =
        target_accounts::get_target_account_by_username_for(&state.db, &ctx.account_id, username)
            .await?
    {
        if existing.status == "active" {
            return Err(ApiError::Conflict(format!(
                "target account @{username} already exists"
            )));
        }
    }

    // Use username as a placeholder account_id; the automation runtime will
    // resolve the real X user ID when it runs target monitoring.
    target_accounts::upsert_target_account_for(&state.db, &ctx.account_id, username, username)
        .await?;

    Ok(Json(
        json!({"status": "added", "username": username.to_string()}),
    ))
}

/// `DELETE /api/targets/:username` — deactivate a target account.
pub async fn remove_target(
    State(state): State<Arc<AppState>>,
    ctx: AccountContext,
    Path(username): Path<String>,
) -> Result<Json<Value>, ApiError> {
    require_mutate(&ctx)?;

    let removed =
        target_accounts::deactivate_target_account_for(&state.db, &ctx.account_id, &username)
            .await?;

    if !removed {
        return Err(ApiError::NotFound(format!(
            "active target account @{username} not found"
        )));
    }

    Ok(Json(json!({"status": "removed", "username": username})))
}

/// Query parameters for the timeline endpoint.
#[derive(Deserialize)]
pub struct TimelineQuery {
    /// Maximum number of timeline items to return (default: 50).
    pub limit: Option<i64>,
}

/// `GET /api/targets/:username/timeline` — interaction timeline for a target.
pub async fn target_timeline(
    State(state): State<Arc<AppState>>,
    ctx: AccountContext,
    Path(username): Path<String>,
    Query(params): Query<TimelineQuery>,
) -> Result<Json<Value>, ApiError> {
    let limit = params.limit.unwrap_or(50).min(200);
    let items =
        target_accounts::get_target_timeline_for(&state.db, &ctx.account_id, &username, limit)
            .await?;
    Ok(Json(json!(items)))
}

/// `GET /api/targets/:username/stats` — aggregated stats for a target.
pub async fn target_stats(
    State(state): State<Arc<AppState>>,
    ctx: AccountContext,
    Path(username): Path<String>,
) -> Result<Json<Value>, ApiError> {
    let stats =
        target_accounts::get_target_stats_for(&state.db, &ctx.account_id, &username).await?;

    match stats {
        Some(s) => Ok(Json(json!(s))),
        None => Err(ApiError::NotFound(format!(
            "active target account @{username} not found"
        ))),
    }
}
