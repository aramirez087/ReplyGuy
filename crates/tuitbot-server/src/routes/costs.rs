//! Cost tracking endpoints — LLM and X API usage summaries and breakdowns.

use std::sync::Arc;

use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;
use tuitbot_core::storage::{llm_usage, x_api_usage};

use crate::account::AccountContext;
use crate::error::ApiError;
use crate::state::AppState;

/// Query parameters for endpoints that accept a `days` window.
#[derive(Deserialize)]
pub struct DaysQuery {
    #[serde(default = "default_days")]
    pub days: u32,
}

fn default_days() -> u32 {
    30
}

/// `GET /api/costs/summary` — cost totals across time windows.
pub async fn summary(
    State(state): State<Arc<AppState>>,
    ctx: AccountContext,
) -> Result<Json<llm_usage::CostSummary>, ApiError> {
    let summary = llm_usage::get_cost_summary_for(&state.db, &ctx.account_id).await?;
    Ok(Json(summary))
}

/// `GET /api/costs/daily?days=30` — per-day cost data for charts.
pub async fn daily(
    State(state): State<Arc<AppState>>,
    ctx: AccountContext,
    Query(params): Query<DaysQuery>,
) -> Result<Json<Vec<llm_usage::DailyCostSummary>>, ApiError> {
    let data = llm_usage::get_daily_costs_for(&state.db, &ctx.account_id, params.days).await?;
    Ok(Json(data))
}

/// `GET /api/costs/by-model?days=30` — cost breakdown by provider + model.
pub async fn by_model(
    State(state): State<Arc<AppState>>,
    ctx: AccountContext,
    Query(params): Query<DaysQuery>,
) -> Result<Json<Vec<llm_usage::ModelCostBreakdown>>, ApiError> {
    let data = llm_usage::get_model_breakdown_for(&state.db, &ctx.account_id, params.days).await?;
    Ok(Json(data))
}

/// `GET /api/costs/by-type?days=30` — cost breakdown by generation type.
pub async fn by_type(
    State(state): State<Arc<AppState>>,
    ctx: AccountContext,
    Query(params): Query<DaysQuery>,
) -> Result<Json<Vec<llm_usage::TypeCostBreakdown>>, ApiError> {
    let data = llm_usage::get_type_breakdown_for(&state.db, &ctx.account_id, params.days).await?;
    Ok(Json(data))
}

// --- X API usage endpoints ---

/// `GET /api/costs/x-api/summary` — X API call totals across time windows.
pub async fn x_api_summary(
    State(state): State<Arc<AppState>>,
    ctx: AccountContext,
) -> Result<Json<x_api_usage::XApiUsageSummary>, ApiError> {
    let summary = x_api_usage::get_usage_summary_for(&state.db, &ctx.account_id).await?;
    Ok(Json(summary))
}

/// `GET /api/costs/x-api/daily?days=30` — per-day X API call data for charts.
pub async fn x_api_daily(
    State(state): State<Arc<AppState>>,
    ctx: AccountContext,
    Query(params): Query<DaysQuery>,
) -> Result<Json<Vec<x_api_usage::DailyXApiUsage>>, ApiError> {
    let data = x_api_usage::get_daily_usage_for(&state.db, &ctx.account_id, params.days).await?;
    Ok(Json(data))
}

/// `GET /api/costs/x-api/by-endpoint?days=30` — X API usage breakdown by endpoint.
pub async fn x_api_by_endpoint(
    State(state): State<Arc<AppState>>,
    ctx: AccountContext,
    Query(params): Query<DaysQuery>,
) -> Result<Json<Vec<x_api_usage::EndpointBreakdown>>, ApiError> {
    let data =
        x_api_usage::get_endpoint_breakdown_for(&state.db, &ctx.account_id, params.days).await?;
    Ok(Json(data))
}
