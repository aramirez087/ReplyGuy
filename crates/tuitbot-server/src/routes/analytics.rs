//! Analytics endpoints.

use std::sync::Arc;

use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};
use tuitbot_core::storage::analytics;

use crate::error::ApiError;
use crate::state::AppState;

/// Query parameters for the followers endpoint.
#[derive(Deserialize)]
pub struct FollowersQuery {
    /// Number of days of follower snapshots to return (default: 7).
    #[serde(default = "default_days")]
    pub days: u32,
}

fn default_days() -> u32 {
    7
}

/// Query parameters for the topics endpoint.
#[derive(Deserialize)]
pub struct TopicsQuery {
    /// Maximum number of topics to return (default: 20).
    #[serde(default = "default_topic_limit")]
    pub limit: u32,
}

fn default_topic_limit() -> u32 {
    20
}

/// Query parameters for the recent-performance endpoint.
#[derive(Deserialize)]
pub struct RecentPerformanceQuery {
    /// Maximum number of items to return (default: 20).
    #[serde(default = "default_recent_limit")]
    pub limit: u32,
}

fn default_recent_limit() -> u32 {
    20
}

/// `GET /api/analytics/followers` — follower snapshots over time.
pub async fn followers(
    State(state): State<Arc<AppState>>,
    Query(params): Query<FollowersQuery>,
) -> Result<Json<Value>, ApiError> {
    let snapshots = analytics::get_follower_snapshots(&state.db, params.days).await?;
    Ok(Json(json!(snapshots)))
}

/// `GET /api/analytics/performance` — reply and tweet performance summaries.
pub async fn performance(State(state): State<Arc<AppState>>) -> Result<Json<Value>, ApiError> {
    let avg_reply = analytics::get_avg_reply_engagement(&state.db).await?;
    let avg_tweet = analytics::get_avg_tweet_engagement(&state.db).await?;
    let (reply_count, tweet_count) = analytics::get_performance_counts(&state.db).await?;

    Ok(Json(json!({
        "avg_reply_engagement": avg_reply,
        "avg_tweet_engagement": avg_tweet,
        "measured_replies": reply_count,
        "measured_tweets": tweet_count,
    })))
}

/// `GET /api/analytics/topics` — topic performance scores.
pub async fn topics(
    State(state): State<Arc<AppState>>,
    Query(params): Query<TopicsQuery>,
) -> Result<Json<Value>, ApiError> {
    let scores = analytics::get_top_topics(&state.db, params.limit).await?;
    Ok(Json(json!(scores)))
}

/// `GET /api/analytics/summary` — combined analytics dashboard summary.
pub async fn summary(State(state): State<Arc<AppState>>) -> Result<Json<Value>, ApiError> {
    let data = analytics::get_analytics_summary(&state.db).await?;
    Ok(Json(json!(data)))
}

/// `GET /api/analytics/recent-performance` — recent content with performance metrics.
pub async fn recent_performance(
    State(state): State<Arc<AppState>>,
    Query(params): Query<RecentPerformanceQuery>,
) -> Result<Json<Value>, ApiError> {
    let items = analytics::get_recent_performance_items(&state.db, params.limit).await?;
    Ok(Json(json!(items)))
}
