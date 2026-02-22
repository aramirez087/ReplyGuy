//! Shared types and port traits for content and thread loops.
//!
//! Defines the trait interfaces (ports) that the content and thread loops
//! depend on. Concrete adapters are implemented in other work packages
//! and wired together during CLI startup.

use std::fmt;

/// Errors that can occur in the content/thread automation loops.
#[derive(Debug)]
pub enum ContentLoopError {
    /// LLM generation failed.
    LlmFailure(String),
    /// Posting to X failed.
    PostFailed(String),
    /// Storage/database error.
    StorageError(String),
    /// Network error.
    NetworkError(String),
    /// Other error.
    Other(String),
}

impl fmt::Display for ContentLoopError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LlmFailure(msg) => write!(f, "LLM failure: {msg}"),
            Self::PostFailed(msg) => write!(f, "Post failed: {msg}"),
            Self::StorageError(msg) => write!(f, "Storage error: {msg}"),
            Self::NetworkError(msg) => write!(f, "Network error: {msg}"),
            Self::Other(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for ContentLoopError {}

// --- Port traits ---

/// Generates individual tweets on a given topic.
#[async_trait::async_trait]
pub trait TweetGenerator: Send + Sync {
    /// Generate an educational tweet on the given topic.
    async fn generate_tweet(&self, topic: &str) -> Result<String, ContentLoopError>;
}

/// Checks safety limits for content posting.
#[async_trait::async_trait]
pub trait ContentSafety: Send + Sync {
    /// Check if a tweet can be posted (daily limit not reached).
    async fn can_post_tweet(&self) -> bool;
    /// Check if a thread can be posted (weekly limit not reached).
    async fn can_post_thread(&self) -> bool;
}

/// Storage operations for content and thread loops.
#[async_trait::async_trait]
pub trait ContentStorage: Send + Sync {
    /// Get the timestamp of the most recent posted tweet.
    async fn last_tweet_time(
        &self,
    ) -> Result<Option<chrono::DateTime<chrono::Utc>>, ContentLoopError>;

    /// Get the timestamp of the most recent posted thread.
    async fn last_thread_time(
        &self,
    ) -> Result<Option<chrono::DateTime<chrono::Utc>>, ContentLoopError>;

    /// Post a tweet (sends to posting queue and records in DB).
    async fn post_tweet(&self, topic: &str, content: &str) -> Result<(), ContentLoopError>;

    /// Create a thread record in the database. Returns the thread ID.
    async fn create_thread(
        &self,
        topic: &str,
        tweet_count: usize,
    ) -> Result<String, ContentLoopError>;

    /// Update thread status (pending, posting, sent, partial).
    async fn update_thread_status(
        &self,
        thread_id: &str,
        status: &str,
        tweet_count: usize,
        root_tweet_id: Option<&str>,
    ) -> Result<(), ContentLoopError>;

    /// Record a thread tweet (position in reply chain).
    async fn store_thread_tweet(
        &self,
        thread_id: &str,
        position: usize,
        tweet_id: &str,
        content: &str,
    ) -> Result<(), ContentLoopError>;

    /// Log an action to the audit trail.
    async fn log_action(
        &self,
        action_type: &str,
        status: &str,
        message: &str,
    ) -> Result<(), ContentLoopError>;
}

/// Posts tweets directly to X (for thread reply chains).
///
/// Thread tweets bypass the posting queue because reply chain
/// order must be maintained -- each tweet must reply to the previous.
#[async_trait::async_trait]
pub trait ThreadPoster: Send + Sync {
    /// Post a standalone tweet. Returns the tweet ID.
    async fn post_tweet(&self, content: &str) -> Result<String, ContentLoopError>;

    /// Reply to a tweet. Returns the new tweet ID.
    async fn reply_to_tweet(
        &self,
        in_reply_to: &str,
        content: &str,
    ) -> Result<String, ContentLoopError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_loop_error_display() {
        let err = ContentLoopError::LlmFailure("model down".to_string());
        assert_eq!(err.to_string(), "LLM failure: model down");

        let err = ContentLoopError::PostFailed("429".to_string());
        assert_eq!(err.to_string(), "Post failed: 429");

        let err = ContentLoopError::StorageError("disk full".to_string());
        assert_eq!(err.to_string(), "Storage error: disk full");

        let err = ContentLoopError::NetworkError("timeout".to_string());
        assert_eq!(err.to_string(), "Network error: timeout");

        let err = ContentLoopError::Other("unknown".to_string());
        assert_eq!(err.to_string(), "unknown");
    }
}
