//! DB-free write functions taking `&dyn XApiClient` directly.
//!
//! These kernel write tools bypass policy gating and mutation recording,
//! making them suitable for the API profile where no DB is available.

use std::time::Instant;

use serde::Serialize;

use crate::contract::envelope::{ToolMeta, ToolResponse};
use crate::contract::error::provider_error_to_response;
use crate::contract::error_code::ErrorCode;
use crate::kernel::utils::check_tweet_length;
use crate::provider::x_api::map_x_error;
use tuitbot_core::x_api::XApiClient;

/// Post a new tweet, optionally with media.
pub async fn post_tweet(
    client: &dyn XApiClient,
    text: &str,
    media_ids: Option<&[String]>,
) -> String {
    let start = Instant::now();
    if let Some(err) = check_tweet_length(text, start) {
        return err;
    }

    let result = match media_ids {
        Some(ids) if !ids.is_empty() => client.post_tweet_with_media(text, ids).await,
        _ => client.post_tweet(text).await,
    };

    match result {
        Ok(tweet) => {
            let elapsed = start.elapsed().as_millis() as u64;
            ToolResponse::success(&tweet)
                .with_meta(ToolMeta::new(elapsed))
                .to_json()
        }
        Err(e) => provider_error_to_response(&map_x_error(&e), start),
    }
}

/// Reply to an existing tweet, optionally with media.
pub async fn reply_to_tweet(
    client: &dyn XApiClient,
    text: &str,
    in_reply_to_id: &str,
    media_ids: Option<&[String]>,
) -> String {
    let start = Instant::now();
    if let Some(err) = check_tweet_length(text, start) {
        return err;
    }

    let result = match media_ids {
        Some(ids) if !ids.is_empty() => {
            client
                .reply_to_tweet_with_media(text, in_reply_to_id, ids)
                .await
        }
        _ => client.reply_to_tweet(text, in_reply_to_id).await,
    };

    match result {
        Ok(tweet) => {
            let elapsed = start.elapsed().as_millis() as u64;
            ToolResponse::success(&tweet)
                .with_meta(ToolMeta::new(elapsed))
                .to_json()
        }
        Err(e) => provider_error_to_response(&map_x_error(&e), start),
    }
}

/// Post a quote tweet.
pub async fn quote_tweet(client: &dyn XApiClient, text: &str, quoted_tweet_id: &str) -> String {
    let start = Instant::now();
    if let Some(err) = check_tweet_length(text, start) {
        return err;
    }

    match client.quote_tweet(text, quoted_tweet_id).await {
        Ok(tweet) => {
            let elapsed = start.elapsed().as_millis() as u64;
            ToolResponse::success(&tweet)
                .with_meta(ToolMeta::new(elapsed))
                .to_json()
        }
        Err(e) => provider_error_to_response(&map_x_error(&e), start),
    }
}

/// Delete a tweet by ID.
pub async fn delete_tweet(client: &dyn XApiClient, tweet_id: &str) -> String {
    let start = Instant::now();

    match client.delete_tweet(tweet_id).await {
        Ok(deleted) => {
            let elapsed = start.elapsed().as_millis() as u64;
            #[derive(Serialize)]
            struct DeleteResult {
                deleted: bool,
                tweet_id: String,
            }
            ToolResponse::success(DeleteResult {
                deleted,
                tweet_id: tweet_id.to_string(),
            })
            .with_meta(ToolMeta::new(elapsed))
            .to_json()
        }
        Err(e) => provider_error_to_response(&map_x_error(&e), start),
    }
}

/// Post a thread (ordered sequence of tweets).
///
/// Validates all tweet lengths up front. On partial failure, returns posted IDs.
pub async fn post_thread(
    client: &dyn XApiClient,
    tweets: &[String],
    media_ids: Option<&[Vec<String>]>,
) -> String {
    let start = Instant::now();

    if tweets.is_empty() {
        let elapsed = start.elapsed().as_millis() as u64;
        return ToolResponse::error(
            ErrorCode::InvalidInput,
            "Thread must contain at least one tweet.",
        )
        .with_meta(ToolMeta::new(elapsed))
        .to_json();
    }

    // Validate all tweet lengths up front.
    for (i, tweet_text) in tweets.iter().enumerate() {
        if let Some(err_json) = check_tweet_length(tweet_text, start) {
            let mut parsed: serde_json::Value = serde_json::from_str(&err_json).unwrap_or_default();
            if let Some(err_obj) = parsed.get_mut("error") {
                err_obj["tweet_index"] = serde_json::json!(i);
            }
            return serde_json::to_string(&parsed).unwrap_or(err_json);
        }
    }

    let mut posted_ids: Vec<String> = Vec::with_capacity(tweets.len());

    for (i, tweet_text) in tweets.iter().enumerate() {
        let tweet_media = media_ids
            .and_then(|m| m.get(i))
            .cloned()
            .unwrap_or_default();

        let result = if i == 0 {
            if tweet_media.is_empty() {
                client.post_tweet(tweet_text).await
            } else {
                client.post_tweet_with_media(tweet_text, &tweet_media).await
            }
        } else {
            let prev_id = &posted_ids[i - 1];
            if tweet_media.is_empty() {
                client.reply_to_tweet(tweet_text, prev_id).await
            } else {
                client
                    .reply_to_tweet_with_media(tweet_text, prev_id, &tweet_media)
                    .await
            }
        };

        match result {
            Ok(posted) => posted_ids.push(posted.id),
            Err(e) => {
                let elapsed = start.elapsed().as_millis() as u64;
                let mut resp = ToolResponse::error(
                    ErrorCode::ThreadPartialFailure,
                    format!(
                        "Thread failed at tweet {i}: {e}. Successfully posted {}/{} tweets.",
                        posted_ids.len(),
                        tweets.len()
                    ),
                )
                .with_meta(ToolMeta::new(elapsed));
                resp.data = serde_json::json!({
                    "posted_tweet_ids": posted_ids,
                    "failed_at_index": i,
                });
                return resp.to_json();
            }
        }
    }

    let elapsed = start.elapsed().as_millis() as u64;
    #[derive(Serialize)]
    struct ThreadResult {
        thread_tweet_ids: Vec<String>,
        tweet_count: usize,
        root_tweet_id: String,
    }
    let root_id = posted_ids[0].clone();
    ToolResponse::success(ThreadResult {
        tweet_count: posted_ids.len(),
        thread_tweet_ids: posted_ids,
        root_tweet_id: root_id,
    })
    .with_meta(ToolMeta::new(elapsed))
    .to_json()
}
