//! `generate_thread_plan` — plan and generate a thread with analysis.

use std::sync::Arc;
use std::time::Instant;

use tuitbot_core::content::frameworks::ThreadStructure;
use tuitbot_core::content::ContentGenerator;

use crate::state::SharedState;
use crate::tools::response::{ErrorCode, ToolMeta, ToolResponse};
use crate::tools::workflow::content::ArcProvider;

/// Parse a structure string into a `ThreadStructure`.
fn parse_structure(s: &str) -> Option<ThreadStructure> {
    match s.to_lowercase().as_str() {
        "transformation" => Some(ThreadStructure::Transformation),
        "framework" => Some(ThreadStructure::Framework),
        "mistakes" => Some(ThreadStructure::Mistakes),
        "analysis" => Some(ThreadStructure::Analysis),
        _ => None,
    }
}

/// Analyze the hook type of the first tweet.
fn analyze_hook(first_tweet: &str) -> &'static str {
    let trimmed = first_tweet.trim();
    if trimmed.ends_with('?') {
        "question"
    } else if trimmed.starts_with("Most people")
        || trimmed.starts_with("Everyone")
        || trimmed.starts_with("most people")
    {
        "contrarian"
    } else if trimmed.starts_with("I ") || trimmed.starts_with("I'") {
        "story"
    } else {
        "statement"
    }
}

/// Execute the `generate_thread_plan` composite tool.
pub async fn execute(
    state: &SharedState,
    topic: &str,
    objective: Option<&str>,
    target_audience: Option<&str>,
    structure_str: Option<&str>,
) -> String {
    let start = Instant::now();

    // Require LLM provider
    if state.llm_provider.is_none() {
        let elapsed = start.elapsed().as_millis() as u64;
        return ToolResponse::error(
            ErrorCode::LlmNotConfigured,
            "No LLM provider configured. Set up the [llm] section in config.toml.",
        )
        .with_meta(ToolMeta::new(elapsed))
        .to_json();
    }

    let structure_override = structure_str.and_then(parse_structure);

    let provider = Box::new(ArcProvider {
        state: Arc::clone(state),
    });
    let gen = ContentGenerator::new(provider, state.config.business.clone());

    let thread = match gen
        .generate_thread_with_structure(topic, structure_override)
        .await
    {
        Ok(t) => t,
        Err(e) => {
            let elapsed = start.elapsed().as_millis() as u64;
            return ToolResponse::error(
                ErrorCode::LlmError,
                format!("Thread generation failed: {e}"),
            )
            .with_meta(ToolMeta::new(elapsed))
            .to_json();
        }
    };

    let tweet_count = thread.tweets.len();
    let hook_type = thread
        .tweets
        .first()
        .map(|t| analyze_hook(t))
        .unwrap_or("unknown");

    // Relevance heuristic: check if topic overlaps with configured industry topics
    let topic_lower = topic.to_lowercase();
    let relevance = state.config.business.industry_topics.iter().any(|t| {
        topic_lower.contains(&t.to_lowercase()) || t.to_lowercase().contains(&topic_lower)
    });

    let estimated_performance = if relevance { "high" } else { "medium" };

    let structure_used = structure_str.unwrap_or("auto_selected");

    let elapsed = start.elapsed().as_millis() as u64;
    crate::tools::workflow::telemetry::record(
        &state.pool,
        "generate_thread_plan",
        "composite",
        elapsed,
        true,
        None,
        None,
        None,
    )
    .await;
    ToolResponse::success(serde_json::json!({
        "thread_tweets": thread.tweets,
        "tweet_count": tweet_count,
        "structure_used": structure_used,
        "hook_analysis": {
            "type": hook_type,
            "first_tweet_preview": thread.tweets.first().cloned().unwrap_or_default(),
        },
        "estimated_performance": estimated_performance,
        "objective_alignment": objective.unwrap_or("general engagement"),
        "target_audience": target_audience.unwrap_or("general"),
        "topic_relevance": if relevance { "matches_industry_topics" } else { "novel_topic" },
    }))
    .with_meta(ToolMeta::new(elapsed).with_workflow(
        state.config.mode.to_string(),
        state.config.effective_approval_mode(),
    ))
    .to_json()
}
