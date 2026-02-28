//! Posting queue adapter implementations.

use tokio::sync::mpsc;

use super::super::loop_helpers::{LoopError, PostSender};
use super::super::posting_queue::{ApprovalQueue, PostAction};
use crate::storage::{self, DbPool};

/// Adapts `mpsc::Sender<PostAction>` to the `PostSender` port trait.
pub struct PostSenderAdapter {
    tx: mpsc::Sender<PostAction>,
}

impl PostSenderAdapter {
    pub fn new(tx: mpsc::Sender<PostAction>) -> Self {
        Self { tx }
    }
}

#[async_trait::async_trait]
impl PostSender for PostSenderAdapter {
    async fn send_reply(&self, tweet_id: &str, content: &str) -> Result<(), LoopError> {
        let (result_tx, result_rx) = tokio::sync::oneshot::channel();
        self.tx
            .send(PostAction::Reply {
                tweet_id: tweet_id.to_string(),
                content: content.to_string(),
                media_ids: vec![],
                result_tx: Some(result_tx),
            })
            .await
            .map_err(|e| LoopError::Other(format!("posting queue send failed: {e}")))?;

        result_rx
            .await
            .map_err(|e| LoopError::Other(format!("posting queue result recv failed: {e}")))?
            .map_err(|e| LoopError::Other(format!("post action failed: {e}")))?;

        Ok(())
    }
}

/// Adapts `DbPool` to the `ApprovalQueue` port trait.
pub struct ApprovalQueueAdapter {
    pool: DbPool,
}

impl ApprovalQueueAdapter {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ApprovalQueue for ApprovalQueueAdapter {
    async fn queue_reply(
        &self,
        tweet_id: &str,
        content: &str,
        media_paths: &[String],
    ) -> Result<i64, String> {
        let media_json = serde_json::to_string(media_paths).unwrap_or_else(|_| "[]".to_string());
        storage::approval_queue::enqueue(
            &self.pool,
            "reply",
            tweet_id,
            "", // target_author not available here
            content,
            "",  // topic
            "",  // archetype
            0.0, // score
            &media_json,
        )
        .await
        .map_err(|e| e.to_string())
    }

    async fn queue_tweet(&self, content: &str, media_paths: &[String]) -> Result<i64, String> {
        let media_json = serde_json::to_string(media_paths).unwrap_or_else(|_| "[]".to_string());
        storage::approval_queue::enqueue(
            &self.pool,
            "tweet",
            "", // no target tweet
            "", // no target author
            content,
            "",  // topic
            "",  // archetype
            0.0, // score
            &media_json,
        )
        .await
        .map_err(|e| e.to_string())
    }
}
