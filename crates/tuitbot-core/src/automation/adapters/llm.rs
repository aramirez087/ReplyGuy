//! LLM adapter implementations.

use std::sync::Arc;

use super::super::loop_helpers::{ContentLoopError, LoopError, ReplyGenerator, TweetGenerator};
use super::super::thread_loop::ThreadGenerator;
use super::helpers::{llm_to_content_error, llm_to_loop_error};
use crate::content::ContentGenerator;
use crate::storage::DbPool;

/// Record LLM usage to the database (fire-and-forget).
pub(super) async fn record_llm_usage(
    pool: &DbPool,
    generation_type: &str,
    provider: &str,
    model: &str,
    input_tokens: u32,
    output_tokens: u32,
) {
    let pricing = crate::llm::pricing::lookup(provider, model);
    let cost = pricing.compute_cost(input_tokens, output_tokens);
    if let Err(e) = crate::storage::llm_usage::insert_llm_usage(
        pool,
        generation_type,
        provider,
        model,
        input_tokens,
        output_tokens,
        cost,
    )
    .await
    {
        tracing::warn!(error = %e, "Failed to record LLM usage");
    }
}

/// Adapts `ContentGenerator` to the `ReplyGenerator` port trait.
pub struct LlmReplyAdapter {
    generator: Arc<ContentGenerator>,
    pool: DbPool,
}

impl LlmReplyAdapter {
    pub fn new(generator: Arc<ContentGenerator>, pool: DbPool) -> Self {
        Self { generator, pool }
    }
}

#[async_trait::async_trait]
impl ReplyGenerator for LlmReplyAdapter {
    async fn generate_reply(
        &self,
        tweet_text: &str,
        author: &str,
        mention_product: bool,
    ) -> Result<String, LoopError> {
        let output = self
            .generator
            .generate_reply(tweet_text, author, mention_product)
            .await
            .map_err(llm_to_loop_error)?;
        record_llm_usage(
            &self.pool,
            "reply",
            &output.provider,
            &output.model,
            output.usage.input_tokens,
            output.usage.output_tokens,
        )
        .await;
        Ok(output.text)
    }
}

/// Adapts `ContentGenerator` to the `TweetGenerator` port trait.
pub struct LlmTweetAdapter {
    generator: Arc<ContentGenerator>,
    pool: DbPool,
}

impl LlmTweetAdapter {
    pub fn new(generator: Arc<ContentGenerator>, pool: DbPool) -> Self {
        Self { generator, pool }
    }
}

#[async_trait::async_trait]
impl TweetGenerator for LlmTweetAdapter {
    async fn generate_tweet(&self, topic: &str) -> Result<String, ContentLoopError> {
        let output = self
            .generator
            .generate_tweet(topic)
            .await
            .map_err(llm_to_content_error)?;
        record_llm_usage(
            &self.pool,
            "tweet",
            &output.provider,
            &output.model,
            output.usage.input_tokens,
            output.usage.output_tokens,
        )
        .await;
        Ok(output.text)
    }
}

/// Adapts `ContentGenerator` to the `ThreadGenerator` port trait.
pub struct LlmThreadAdapter {
    generator: Arc<ContentGenerator>,
    pool: DbPool,
}

impl LlmThreadAdapter {
    pub fn new(generator: Arc<ContentGenerator>, pool: DbPool) -> Self {
        Self { generator, pool }
    }
}

#[async_trait::async_trait]
impl ThreadGenerator for LlmThreadAdapter {
    async fn generate_thread(
        &self,
        topic: &str,
        _count: Option<usize>,
    ) -> Result<Vec<String>, ContentLoopError> {
        let output = self
            .generator
            .generate_thread(topic)
            .await
            .map_err(llm_to_content_error)?;
        record_llm_usage(
            &self.pool,
            "thread",
            &output.provider,
            &output.model,
            output.usage.input_tokens,
            output.usage.output_tokens,
        )
        .await;
        Ok(output.tweets)
    }
}
