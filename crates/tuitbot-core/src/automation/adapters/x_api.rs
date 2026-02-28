//! X API adapter implementations.

use std::sync::Arc;

use super::super::analytics_loop::{AnalyticsError, EngagementFetcher, ProfileFetcher};
use super::super::loop_helpers::{
    ContentLoopError, LoopError, LoopTweet, MentionsFetcher, ThreadPoster, TweetSearcher,
};
use super::super::posting_queue::PostExecutor;
use super::super::target_loop::{TargetTweetFetcher, TargetUserManager};
use super::helpers::{
    search_response_to_loop_tweets, toolkit_to_analytics_error, toolkit_to_content_error,
    toolkit_to_loop_error,
};
use crate::x_api::XApiClient;

/// Adapts `XApiClient` to the `TweetSearcher` port trait via toolkit.
pub struct XApiSearchAdapter {
    client: Arc<dyn XApiClient>,
}

impl XApiSearchAdapter {
    pub fn new(client: Arc<dyn XApiClient>) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl TweetSearcher for XApiSearchAdapter {
    async fn search_tweets(&self, query: &str) -> Result<Vec<LoopTweet>, LoopError> {
        let response = crate::toolkit::read::search_tweets(&*self.client, query, 20, None, None)
            .await
            .map_err(toolkit_to_loop_error)?;
        Ok(search_response_to_loop_tweets(response))
    }
}

/// Adapts `XApiClient` to the `MentionsFetcher` port trait via toolkit.
pub struct XApiMentionsAdapter {
    client: Arc<dyn XApiClient>,
    own_user_id: String,
}

impl XApiMentionsAdapter {
    pub fn new(client: Arc<dyn XApiClient>, own_user_id: String) -> Self {
        Self {
            client,
            own_user_id,
        }
    }
}

#[async_trait::async_trait]
impl MentionsFetcher for XApiMentionsAdapter {
    async fn get_mentions(&self, since_id: Option<&str>) -> Result<Vec<LoopTweet>, LoopError> {
        let response =
            crate::toolkit::read::get_mentions(&*self.client, &self.own_user_id, since_id, None)
                .await
                .map_err(toolkit_to_loop_error)?;
        Ok(search_response_to_loop_tweets(response))
    }
}

/// Adapts `XApiClient` to `TargetTweetFetcher` and `TargetUserManager` via toolkit.
pub struct XApiTargetAdapter {
    client: Arc<dyn XApiClient>,
}

impl XApiTargetAdapter {
    pub fn new(client: Arc<dyn XApiClient>) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl TargetTweetFetcher for XApiTargetAdapter {
    async fn fetch_user_tweets(&self, user_id: &str) -> Result<Vec<LoopTweet>, LoopError> {
        let response = crate::toolkit::read::get_user_tweets(&*self.client, user_id, 10, None)
            .await
            .map_err(toolkit_to_loop_error)?;
        Ok(search_response_to_loop_tweets(response))
    }
}

#[async_trait::async_trait]
impl TargetUserManager for XApiTargetAdapter {
    async fn lookup_user(&self, username: &str) -> Result<(String, String), LoopError> {
        let user = crate::toolkit::read::get_user_by_username(&*self.client, username)
            .await
            .map_err(toolkit_to_loop_error)?;
        Ok((user.id, user.username))
    }
}

/// Adapts `XApiClient` to `ProfileFetcher` and `EngagementFetcher` via toolkit.
pub struct XApiProfileAdapter {
    client: Arc<dyn XApiClient>,
}

impl XApiProfileAdapter {
    pub fn new(client: Arc<dyn XApiClient>) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl ProfileFetcher for XApiProfileAdapter {
    async fn get_profile_metrics(
        &self,
    ) -> Result<super::super::analytics_loop::ProfileMetrics, AnalyticsError> {
        let user = crate::toolkit::read::get_me(&*self.client)
            .await
            .map_err(toolkit_to_analytics_error)?;
        Ok(super::super::analytics_loop::ProfileMetrics {
            follower_count: user.public_metrics.followers_count as i64,
            following_count: user.public_metrics.following_count as i64,
            tweet_count: user.public_metrics.tweet_count as i64,
        })
    }
}

#[async_trait::async_trait]
impl EngagementFetcher for XApiProfileAdapter {
    async fn get_tweet_metrics(
        &self,
        tweet_id: &str,
    ) -> Result<super::super::analytics_loop::TweetMetrics, AnalyticsError> {
        let tweet = crate::toolkit::read::get_tweet(&*self.client, tweet_id)
            .await
            .map_err(toolkit_to_analytics_error)?;
        Ok(super::super::analytics_loop::TweetMetrics {
            likes: tweet.public_metrics.like_count as i64,
            retweets: tweet.public_metrics.retweet_count as i64,
            replies: tweet.public_metrics.reply_count as i64,
            impressions: tweet.public_metrics.impression_count as i64,
        })
    }
}

/// Adapts `XApiClient` to `PostExecutor` (for the posting queue) via toolkit.
pub struct XApiPostExecutorAdapter {
    client: Arc<dyn XApiClient>,
}

impl XApiPostExecutorAdapter {
    pub fn new(client: Arc<dyn XApiClient>) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl PostExecutor for XApiPostExecutorAdapter {
    async fn execute_reply(
        &self,
        tweet_id: &str,
        content: &str,
        media_ids: &[String],
    ) -> Result<String, String> {
        let media = if media_ids.is_empty() {
            None
        } else {
            Some(media_ids)
        };
        crate::toolkit::write::reply_to_tweet(&*self.client, content, tweet_id, media)
            .await
            .map(|posted| posted.id)
            .map_err(|e| e.to_string())
    }

    async fn execute_tweet(&self, content: &str, media_ids: &[String]) -> Result<String, String> {
        let media = if media_ids.is_empty() {
            None
        } else {
            Some(media_ids)
        };
        crate::toolkit::write::post_tweet(&*self.client, content, media)
            .await
            .map(|posted| posted.id)
            .map_err(|e| e.to_string())
    }
}

/// Adapts `XApiClient` to `ThreadPoster` (for direct thread posting) via toolkit.
pub struct XApiThreadPosterAdapter {
    client: Arc<dyn XApiClient>,
}

impl XApiThreadPosterAdapter {
    pub fn new(client: Arc<dyn XApiClient>) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl ThreadPoster for XApiThreadPosterAdapter {
    async fn post_tweet(&self, content: &str) -> Result<String, ContentLoopError> {
        crate::toolkit::write::post_tweet(&*self.client, content, None)
            .await
            .map(|posted| posted.id)
            .map_err(toolkit_to_content_error)
    }

    async fn reply_to_tweet(
        &self,
        in_reply_to: &str,
        content: &str,
    ) -> Result<String, ContentLoopError> {
        crate::toolkit::write::reply_to_tweet(&*self.client, content, in_reply_to, None)
            .await
            .map(|posted| posted.id)
            .map_err(toolkit_to_content_error)
    }
}
