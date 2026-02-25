//! Compose endpoints for tweets, threads, and unified compose.

use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};
use tuitbot_core::content::{tweet_weighted_len, MAX_TWEET_CHARS};
use tuitbot_core::storage::{approval_queue, scheduled_content};

use crate::account::{require_mutate, AccountContext};
use crate::error::ApiError;
use crate::state::AppState;
use crate::ws::WsEvent;

use super::read_approval_mode;

/// Request body for composing a manual tweet.
#[derive(Deserialize)]
pub struct ComposeTweetRequest {
    /// The tweet text.
    pub text: String,
    /// Optional ISO 8601 timestamp to schedule the tweet.
    pub scheduled_for: Option<String>,
}

/// `POST /api/content/tweets` — compose and queue a manual tweet.
pub async fn compose_tweet(
    State(state): State<Arc<AppState>>,
    ctx: AccountContext,
    Json(body): Json<ComposeTweetRequest>,
) -> Result<Json<Value>, ApiError> {
    require_mutate(&ctx)?;

    let text = body.text.trim();
    if text.is_empty() {
        return Err(ApiError::BadRequest("text is required".to_string()));
    }

    // Check if approval mode is enabled.
    let approval_mode = read_approval_mode(&state)?;

    if approval_mode {
        let id = approval_queue::enqueue_for(
            &state.db,
            &ctx.account_id,
            "tweet",
            "", // no target tweet
            "", // no target author
            text,
            "", // no topic
            "", // no archetype
            0.0,
            "[]",
        )
        .await?;

        let _ = state.event_tx.send(WsEvent::ApprovalQueued {
            id,
            action_type: "tweet".to_string(),
            content: text.to_string(),
            media_paths: vec![],
        });

        Ok(Json(json!({
            "status": "queued_for_approval",
            "id": id,
        })))
    } else {
        // Without X API client in AppState, we can only acknowledge the intent.
        Ok(Json(json!({
            "status": "accepted",
            "text": text,
            "scheduled_for": body.scheduled_for,
        })))
    }
}

/// Request body for composing a manual thread.
#[derive(Deserialize)]
pub struct ComposeThreadRequest {
    /// The tweets forming the thread.
    pub tweets: Vec<String>,
    /// Optional ISO 8601 timestamp to schedule the thread.
    pub scheduled_for: Option<String>,
}

/// `POST /api/content/threads` — compose and queue a manual thread.
pub async fn compose_thread(
    State(state): State<Arc<AppState>>,
    ctx: AccountContext,
    Json(body): Json<ComposeThreadRequest>,
) -> Result<Json<Value>, ApiError> {
    require_mutate(&ctx)?;

    if body.tweets.is_empty() {
        return Err(ApiError::BadRequest(
            "tweets array must not be empty".to_string(),
        ));
    }

    let approval_mode = read_approval_mode(&state)?;
    let combined = body.tweets.join("\n---\n");

    if approval_mode {
        let id = approval_queue::enqueue_for(
            &state.db,
            &ctx.account_id,
            "thread",
            "",
            "",
            &combined,
            "",
            "",
            0.0,
            "[]",
        )
        .await?;

        let _ = state.event_tx.send(WsEvent::ApprovalQueued {
            id,
            action_type: "thread".to_string(),
            content: combined,
            media_paths: vec![],
        });

        Ok(Json(json!({
            "status": "queued_for_approval",
            "id": id,
        })))
    } else {
        Ok(Json(json!({
            "status": "accepted",
            "tweet_count": body.tweets.len(),
            "scheduled_for": body.scheduled_for,
        })))
    }
}

/// Request body for the unified compose endpoint.
#[derive(Deserialize)]
pub struct ComposeRequest {
    /// Content type: "tweet" or "thread".
    pub content_type: String,
    /// Content text (string for tweet, JSON array string for thread).
    pub content: String,
    /// Optional ISO 8601 timestamp to schedule the content.
    pub scheduled_for: Option<String>,
    /// Optional local media file paths to attach.
    #[serde(default)]
    pub media_paths: Option<Vec<String>>,
}

/// `POST /api/content/compose` — compose manual content (tweet or thread).
pub async fn compose(
    State(state): State<Arc<AppState>>,
    ctx: AccountContext,
    Json(body): Json<ComposeRequest>,
) -> Result<Json<Value>, ApiError> {
    require_mutate(&ctx)?;
    let content = body.content.trim().to_string();
    if content.is_empty() {
        return Err(ApiError::BadRequest("content is required".to_string()));
    }

    match body.content_type.as_str() {
        "tweet" => {
            if tweet_weighted_len(&content) > MAX_TWEET_CHARS {
                return Err(ApiError::BadRequest(
                    "tweet content must not exceed 280 characters".to_string(),
                ));
            }
        }
        "thread" => {
            // Validate that content is a JSON array of strings
            let tweets: Result<Vec<String>, _> = serde_json::from_str(&content);
            match tweets {
                Ok(ref t) if t.is_empty() => {
                    return Err(ApiError::BadRequest(
                        "thread must contain at least one tweet".to_string(),
                    ));
                }
                Ok(ref t) => {
                    for (i, tweet) in t.iter().enumerate() {
                        if tweet_weighted_len(tweet) > MAX_TWEET_CHARS {
                            return Err(ApiError::BadRequest(format!(
                                "tweet {} exceeds 280 characters",
                                i + 1
                            )));
                        }
                    }
                }
                Err(_) => {
                    return Err(ApiError::BadRequest(
                        "thread content must be a JSON array of strings".to_string(),
                    ));
                }
            }
        }
        _ => {
            return Err(ApiError::BadRequest(
                "content_type must be 'tweet' or 'thread'".to_string(),
            ));
        }
    }

    let approval_mode = read_approval_mode(&state)?;

    if approval_mode {
        let media_paths = body.media_paths.as_deref().unwrap_or(&[]);
        let media_json = serde_json::to_string(media_paths).unwrap_or_else(|_| "[]".to_string());
        let id = approval_queue::enqueue_for(
            &state.db,
            &ctx.account_id,
            &body.content_type,
            "",
            "",
            &content,
            "",
            "",
            0.0,
            &media_json,
        )
        .await?;

        let _ = state.event_tx.send(WsEvent::ApprovalQueued {
            id,
            action_type: body.content_type,
            content: content.clone(),
            media_paths: media_paths.to_vec(),
        });

        Ok(Json(json!({
            "status": "queued_for_approval",
            "id": id,
        })))
    } else {
        let id = scheduled_content::insert_for(
            &state.db,
            &ctx.account_id,
            &body.content_type,
            &content,
            body.scheduled_for.as_deref(),
        )
        .await?;

        let _ = state.event_tx.send(WsEvent::ContentScheduled {
            id,
            content_type: body.content_type,
            scheduled_for: body.scheduled_for,
        });

        Ok(Json(json!({
            "status": "scheduled",
            "id": id,
        })))
    }
}
