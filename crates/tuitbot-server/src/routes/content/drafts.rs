//! Draft content endpoints.

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};
use tuitbot_core::content::{tweet_weighted_len, MAX_TWEET_CHARS};
use tuitbot_core::storage::{approval_queue, scheduled_content};

use crate::account::{require_mutate, AccountContext};
use crate::error::ApiError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateDraftRequest {
    pub content_type: String,
    pub content: String,
    #[serde(default = "default_source")]
    pub source: String,
}

fn default_source() -> String {
    "manual".to_string()
}

pub async fn list_drafts(
    State(state): State<Arc<AppState>>,
    ctx: AccountContext,
) -> Result<Json<Vec<scheduled_content::ScheduledContent>>, ApiError> {
    let drafts = scheduled_content::list_drafts_for(&state.db, &ctx.account_id)
        .await
        .map_err(ApiError::Storage)?;
    Ok(Json(drafts))
}

pub async fn create_draft(
    State(state): State<Arc<AppState>>,
    ctx: AccountContext,
    Json(body): Json<CreateDraftRequest>,
) -> Result<Json<Value>, ApiError> {
    require_mutate(&ctx)?;

    // Validate content.
    if body.content.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "content must not be empty".to_string(),
        ));
    }

    if body.content_type == "tweet"
        && !tuitbot_core::content::validate_tweet_length(&body.content, MAX_TWEET_CHARS)
    {
        return Err(ApiError::BadRequest(format!(
            "Tweet exceeds {} characters (weighted length: {})",
            MAX_TWEET_CHARS,
            tweet_weighted_len(&body.content)
        )));
    }

    let id = scheduled_content::insert_draft_for(
        &state.db,
        &ctx.account_id,
        &body.content_type,
        &body.content,
        &body.source,
    )
    .await
    .map_err(ApiError::Storage)?;

    Ok(Json(json!({ "id": id, "status": "draft" })))
}

#[derive(Deserialize)]
pub struct EditDraftRequest {
    pub content: String,
}

pub async fn edit_draft(
    State(state): State<Arc<AppState>>,
    ctx: AccountContext,
    Path(id): Path<i64>,
    Json(body): Json<EditDraftRequest>,
) -> Result<Json<Value>, ApiError> {
    require_mutate(&ctx)?;

    if body.content.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "content must not be empty".to_string(),
        ));
    }

    scheduled_content::update_draft_for(&state.db, &ctx.account_id, id, &body.content)
        .await
        .map_err(ApiError::Storage)?;

    Ok(Json(json!({ "id": id, "status": "draft" })))
}

pub async fn delete_draft(
    State(state): State<Arc<AppState>>,
    ctx: AccountContext,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    require_mutate(&ctx)?;

    scheduled_content::delete_draft_for(&state.db, &ctx.account_id, id)
        .await
        .map_err(ApiError::Storage)?;

    Ok(Json(json!({ "id": id, "status": "cancelled" })))
}

#[derive(Deserialize)]
pub struct ScheduleDraftRequest {
    pub scheduled_for: String,
}

pub async fn schedule_draft(
    State(state): State<Arc<AppState>>,
    ctx: AccountContext,
    Path(id): Path<i64>,
    Json(body): Json<ScheduleDraftRequest>,
) -> Result<Json<Value>, ApiError> {
    require_mutate(&ctx)?;

    scheduled_content::schedule_draft_for(&state.db, &ctx.account_id, id, &body.scheduled_for)
        .await
        .map_err(ApiError::Storage)?;

    Ok(Json(
        json!({ "id": id, "status": "scheduled", "scheduled_for": body.scheduled_for }),
    ))
}

pub async fn publish_draft(
    State(state): State<Arc<AppState>>,
    ctx: AccountContext,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    require_mutate(&ctx)?;

    // Get the draft.
    let item = scheduled_content::get_by_id_for(&state.db, &ctx.account_id, id)
        .await
        .map_err(ApiError::Storage)?
        .ok_or_else(|| ApiError::NotFound(format!("Draft {id} not found")))?;

    if item.status != "draft" {
        return Err(ApiError::BadRequest(format!(
            "Item is in '{}' status, not 'draft'",
            item.status
        )));
    }

    // Queue into approval queue for immediate posting.
    let queue_id = approval_queue::enqueue_for(
        &state.db,
        &ctx.account_id,
        &item.content_type,
        "", // no target tweet
        "", // no target author
        &item.content,
        "",  // topic
        "",  // archetype
        0.0, // score
        "[]",
    )
    .await
    .map_err(ApiError::Storage)?;

    // Mark as approved immediately so the approval poster picks it up.
    approval_queue::update_status_for(&state.db, &ctx.account_id, queue_id, "approved")
        .await
        .map_err(ApiError::Storage)?;

    // Mark the draft as posted.
    scheduled_content::update_status_for(&state.db, &ctx.account_id, id, "posted", None)
        .await
        .map_err(ApiError::Storage)?;

    Ok(Json(
        json!({ "id": id, "approval_queue_id": queue_id, "status": "queued_for_posting" }),
    ))
}
