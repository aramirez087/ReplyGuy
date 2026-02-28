//! Safety adapter implementations.

use std::sync::Arc;

use chrono::Utc;

use super::super::loop_helpers::{ContentSafety, LoopError, SafetyChecker};
use super::helpers::storage_to_loop_error;
use crate::safety::SafetyGuard;
use crate::storage::{self, DbPool};

/// Adapts `SafetyGuard` to the `SafetyChecker` port trait.
pub struct SafetyAdapter {
    guard: Arc<SafetyGuard>,
    pool: DbPool,
}

impl SafetyAdapter {
    pub fn new(guard: Arc<SafetyGuard>, pool: DbPool) -> Self {
        Self { guard, pool }
    }
}

#[async_trait::async_trait]
impl SafetyChecker for SafetyAdapter {
    async fn can_reply(&self) -> bool {
        match self.guard.can_reply_to("__check__", None).await {
            Ok(Ok(())) => true,
            Ok(Err(reason)) => {
                tracing::debug!(reason = %reason, "Safety check denied reply");
                false
            }
            Err(e) => {
                tracing::warn!(error = %e, "Safety check error, denying reply");
                false
            }
        }
    }

    async fn has_replied_to(&self, tweet_id: &str) -> bool {
        match self.guard.dedup_checker().has_replied_to(tweet_id).await {
            Ok(replied) => replied,
            Err(e) => {
                tracing::warn!(error = %e, "Dedup check error, assuming already replied");
                true
            }
        }
    }

    async fn record_reply(&self, tweet_id: &str, reply_content: &str) -> Result<(), LoopError> {
        // Insert a reply record for dedup tracking.
        let reply = storage::replies::ReplySent {
            id: 0,
            target_tweet_id: tweet_id.to_string(),
            reply_tweet_id: None,
            reply_content: reply_content.to_string(),
            llm_provider: None,
            llm_model: None,
            created_at: Utc::now().to_rfc3339(),
            status: "pending".to_string(),
            error_message: None,
        };
        storage::replies::insert_reply(&self.pool, &reply)
            .await
            .map_err(storage_to_loop_error)?;

        // Increment rate limit counter.
        self.guard
            .record_reply()
            .await
            .map_err(storage_to_loop_error)?;

        Ok(())
    }
}

/// Adapts `SafetyGuard` to the `ContentSafety` port trait.
pub struct ContentSafetyAdapter {
    guard: Arc<SafetyGuard>,
}

impl ContentSafetyAdapter {
    pub fn new(guard: Arc<SafetyGuard>) -> Self {
        Self { guard }
    }
}

#[async_trait::async_trait]
impl ContentSafety for ContentSafetyAdapter {
    async fn can_post_tweet(&self) -> bool {
        match self.guard.can_post_tweet().await {
            Ok(Ok(())) => true,
            Ok(Err(reason)) => {
                tracing::debug!(reason = %reason, "Safety check denied tweet");
                false
            }
            Err(e) => {
                tracing::warn!(error = %e, "Safety check error, denying tweet");
                false
            }
        }
    }

    async fn can_post_thread(&self) -> bool {
        match self.guard.can_post_thread().await {
            Ok(Ok(())) => true,
            Ok(Err(reason)) => {
                tracing::debug!(reason = %reason, "Safety check denied thread");
                false
            }
            Err(e) => {
                tracing::warn!(error = %e, "Safety check error, denying thread");
                false
            }
        }
    }
}
