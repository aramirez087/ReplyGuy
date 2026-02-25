//! Activity feed endpoints.

use std::sync::Arc;

use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};
use tuitbot_core::storage::{action_log, rate_limits};

use crate::error::ApiError;
use crate::state::AppState;

/// Query parameters for the activity endpoint.
#[derive(Deserialize)]
pub struct ActivityQuery {
    /// Maximum number of actions to return (default: 50).
    #[serde(default = "default_limit")]
    pub limit: u32,
    /// Offset for pagination (default: 0).
    #[serde(default)]
    pub offset: u32,
    /// Filter by action type. Use "all" or omit for no filter.
    #[serde(rename = "type")]
    pub action_type: Option<String>,
    /// Filter by status (e.g. "failure" for errors).
    pub status: Option<String>,
}

fn default_limit() -> u32 {
    50
}

/// `GET /api/activity` — paginated, filterable action log.
pub async fn list_activity(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ActivityQuery>,
) -> Result<Json<Value>, ApiError> {
    let type_filter =
        params
            .action_type
            .as_deref()
            .and_then(|t| if t == "all" { None } else { Some(t) });
    let status_filter = params.status.as_deref();

    let actions = action_log::get_actions_paginated(
        &state.db,
        params.limit,
        params.offset,
        type_filter,
        status_filter,
    )
    .await?;

    let total = action_log::get_actions_count(&state.db, type_filter, status_filter).await?;

    Ok(Json(json!({
        "actions": actions,
        "total": total,
        "limit": params.limit,
        "offset": params.offset,
    })))
}

/// Query parameters for the activity export endpoint.
#[derive(Deserialize)]
pub struct ExportQuery {
    /// Export format: "csv" or "json" (default: "csv").
    #[serde(default = "default_csv")]
    pub format: String,
    /// Filter by action type.
    #[serde(rename = "type")]
    pub action_type: Option<String>,
    /// Filter by status.
    pub status: Option<String>,
}

fn default_csv() -> String {
    "csv".to_string()
}

/// `GET /api/activity/export` — export activity log as CSV or JSON.
pub async fn export_activity(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ExportQuery>,
) -> Result<axum::response::Response, ApiError> {
    use axum::response::IntoResponse;

    let type_filter =
        params
            .action_type
            .as_deref()
            .and_then(|t| if t == "all" { None } else { Some(t) });
    let status_filter = params.status.as_deref();

    let actions =
        action_log::get_actions_paginated(&state.db, 10_000, 0, type_filter, status_filter).await?;

    if params.format == "json" {
        let body = serde_json::to_string(&actions).unwrap_or_else(|_| "[]".to_string());
        Ok((
            [
                (
                    axum::http::header::CONTENT_TYPE,
                    "application/json; charset=utf-8",
                ),
                (
                    axum::http::header::CONTENT_DISPOSITION,
                    "attachment; filename=\"activity_export.json\"",
                ),
            ],
            body,
        )
            .into_response())
    } else {
        let mut csv = String::from("id,action_type,status,message,created_at\n");
        for a in &actions {
            csv.push_str(&format!(
                "{},{},{},{},{}\n",
                a.id,
                escape_csv(&a.action_type),
                escape_csv(&a.status),
                escape_csv(a.message.as_deref().unwrap_or("")),
                escape_csv(&a.created_at),
            ));
        }
        Ok((
            [
                (axum::http::header::CONTENT_TYPE, "text/csv; charset=utf-8"),
                (
                    axum::http::header::CONTENT_DISPOSITION,
                    "attachment; filename=\"activity_export.csv\"",
                ),
            ],
            csv,
        )
            .into_response())
    }
}

/// Escape a value for CSV output. Wraps in quotes if it contains commas,
/// quotes, or newlines.
fn escape_csv(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

/// `GET /api/activity/rate-limits` — current daily rate limit usage.
pub async fn rate_limit_usage(State(state): State<Arc<AppState>>) -> Result<Json<Value>, ApiError> {
    let usage = rate_limits::get_daily_usage(&state.db).await?;
    Ok(Json(json!(usage)))
}
