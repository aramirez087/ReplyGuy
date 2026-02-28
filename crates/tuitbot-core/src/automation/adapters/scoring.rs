//! Scoring adapter implementation.

use std::sync::Arc;

use super::super::loop_helpers::{LoopTweet, ScoreResult, TweetScorer};
use crate::scoring::{self, ScoringEngine, TweetData};

/// Adapts `ScoringEngine` to the `TweetScorer` port trait.
pub struct ScoringAdapter {
    engine: Arc<ScoringEngine>,
}

impl ScoringAdapter {
    pub fn new(engine: Arc<ScoringEngine>) -> Self {
        Self { engine }
    }
}

impl TweetScorer for ScoringAdapter {
    fn score(&self, tweet: &LoopTweet) -> ScoreResult {
        let data = TweetData {
            text: tweet.text.clone(),
            created_at: tweet.created_at.clone(),
            likes: tweet.likes,
            retweets: tweet.retweets,
            replies: tweet.replies,
            author_username: tweet.author_username.clone(),
            author_followers: tweet.author_followers,
            has_media: false,
            is_quote_tweet: false,
        };

        let score = self.engine.score_tweet(&data);
        let matched_keywords = scoring::find_matched_keywords(&tweet.text, self.engine.keywords());

        ScoreResult {
            total: score.total,
            meets_threshold: score.meets_threshold,
            matched_keywords,
        }
    }
}
