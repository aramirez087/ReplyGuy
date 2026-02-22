//! X API v2 client, authentication, and tier detection.
//!
//! Provides a trait-based client abstraction for all X API operations,
//! OAuth 2.0 PKCE authentication with token management, and API tier
//! detection for adaptive behavior.

pub mod auth;
pub mod client;
pub mod tier;
pub mod types;

pub use client::XApiHttpClient;
pub use types::*;

use crate::error::XApiError;

/// Trait abstracting all X API v2 operations.
///
/// Implementations include `XApiHttpClient` for real API calls and
/// mock implementations for testing.
#[async_trait::async_trait]
pub trait XApiClient: Send + Sync {
    /// Search recent tweets matching the given query.
    ///
    /// Returns up to `max_results` tweets. If `since_id` is provided,
    /// only returns tweets newer than that ID.
    async fn search_tweets(
        &self,
        query: &str,
        max_results: u32,
        since_id: Option<&str>,
    ) -> Result<SearchResponse, XApiError>;

    /// Get mentions for the authenticated user.
    ///
    /// If `since_id` is provided, only returns mentions newer than that ID.
    async fn get_mentions(
        &self,
        user_id: &str,
        since_id: Option<&str>,
    ) -> Result<MentionResponse, XApiError>;

    /// Post a new tweet.
    async fn post_tweet(&self, text: &str) -> Result<PostedTweet, XApiError>;

    /// Reply to an existing tweet.
    async fn reply_to_tweet(
        &self,
        text: &str,
        in_reply_to_id: &str,
    ) -> Result<PostedTweet, XApiError>;

    /// Get a single tweet by ID.
    async fn get_tweet(&self, tweet_id: &str) -> Result<Tweet, XApiError>;

    /// Get the authenticated user's profile.
    async fn get_me(&self) -> Result<User, XApiError>;
}
