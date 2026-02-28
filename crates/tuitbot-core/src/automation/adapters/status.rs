//! Status reporter adapter implementation.

use chrono::{DateTime, Utc};

use super::super::status_reporter::{ActionCounts, StatusQuerier};
use crate::storage::{self, DbPool};

/// Adapts `DbPool` to the `StatusQuerier` port trait.
pub struct StatusQuerierAdapter {
    pool: DbPool,
}

impl StatusQuerierAdapter {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl StatusQuerier for StatusQuerierAdapter {
    async fn query_action_counts_since(
        &self,
        since: DateTime<Utc>,
    ) -> Result<ActionCounts, String> {
        let since_str = since.format("%Y-%m-%dT%H:%M:%SZ").to_string();
        let counts = storage::action_log::get_action_counts_since(&self.pool, &since_str)
            .await
            .map_err(|e| e.to_string())?;

        Ok(ActionCounts {
            tweets_scored: *counts.get("tweet_scored").unwrap_or(&0) as u64,
            replies_sent: *counts.get("reply_sent").unwrap_or(&0) as u64,
            tweets_posted: *counts.get("tweet_posted").unwrap_or(&0) as u64,
            threads_posted: *counts.get("thread_posted").unwrap_or(&0) as u64,
        })
    }
}
