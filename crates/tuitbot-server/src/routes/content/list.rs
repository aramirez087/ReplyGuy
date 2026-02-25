//! List endpoints for tweets and threads.

use std::sync::Arc;

use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};
use tuitbot_core::storage::threads;

use crate::account::AccountContext;
use crate::error::ApiError;
use crate::state::AppState;

/// Query parameters for the tweets endpoint.
#[derive(Deserialize)]
pub struct TweetsQuery {
    /// Maximum number of tweets to return (default: 50).
    #[serde(default = "default_tweet_limit")]
    pub limit: u32,
}

fn default_tweet_limit() -> u32 {
    50
}

/// Query parameters for the threads endpoint.
#[derive(Deserialize)]
pub struct ThreadsQuery {
    /// Maximum number of threads to return (default: 20).
    #[serde(default = "default_thread_limit")]
    pub limit: u32,
}

fn default_thread_limit() -> u32 {
    20
}

/// `GET /api/content/tweets` — recent original tweets posted.
pub async fn list_tweets(
    State(state): State<Arc<AppState>>,
    ctx: AccountContext,
    Query(params): Query<TweetsQuery>,
) -> Result<Json<Value>, ApiError> {
    let tweets =
        threads::get_recent_original_tweets_for(&state.db, &ctx.account_id, params.limit).await?;
    Ok(Json(json!(tweets)))
}

/// `GET /api/content/threads` — recent threads posted.
pub async fn list_threads(
    State(state): State<Arc<AppState>>,
    ctx: AccountContext,
    Query(params): Query<ThreadsQuery>,
) -> Result<Json<Value>, ApiError> {
    let threads = threads::get_recent_threads_for(&state.db, &ctx.account_id, params.limit).await?;
    Ok(Json(json!(threads)))
}
