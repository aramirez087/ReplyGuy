//! Kernel utility functions: tweet length validation, user profile lookup.

use std::time::Instant;

use crate::contract::envelope::{ToolMeta, ToolResponse};
use crate::contract::error::provider_error_to_response;
use crate::contract::error_code::ErrorCode;
use crate::provider::SocialReadProvider;

/// URL-aware tweet length limit.
const MAX_TWEET_LENGTH: usize = 280;

/// X API counts each URL as 23 characters regardless of actual length.
const URL_WEIGHTED_LENGTH: usize = 23;

/// Check if tweet text exceeds the 280-char limit (URL-weighted).
///
/// Returns `Some(error_json)` if the text is too long, `None` if OK.
pub fn check_tweet_length(text: &str, start: Instant) -> Option<String> {
    let weighted_len = compute_weighted_length(text);
    if weighted_len > MAX_TWEET_LENGTH {
        let elapsed = start.elapsed().as_millis() as u64;
        Some(
            ToolResponse::error(
                ErrorCode::TweetTooLong,
                format!(
                    "Tweet text is {weighted_len} characters (URL-weighted), \
                     max is {MAX_TWEET_LENGTH}."
                ),
            )
            .with_meta(ToolMeta::new(elapsed))
            .to_json(),
        )
    } else {
        None
    }
}

/// Compute URL-weighted character length.
///
/// Any `http://` or `https://` URL is counted as 23 characters per X API rules.
pub fn compute_weighted_length(text: &str) -> usize {
    let mut total = 0;
    let mut remaining = text;

    while let Some(url_start) = remaining
        .find("https://")
        .or_else(|| remaining.find("http://"))
    {
        total += remaining[..url_start].chars().count();

        let url_rest = &remaining[url_start..];
        let url_end = url_rest
            .find(|c: char| c.is_whitespace())
            .unwrap_or(url_rest.len());

        total += URL_WEIGHTED_LENGTH;
        remaining = &remaining[url_start + url_end..];
    }

    total += remaining.chars().count();
    total
}

/// Get the authenticated user's profile via the provider.
pub async fn get_me(provider: &dyn SocialReadProvider) -> String {
    let start = Instant::now();
    match provider.get_me().await {
        Ok(user) => {
            let elapsed = start.elapsed().as_millis() as u64;
            ToolResponse::success(&user)
                .with_meta(ToolMeta::new(elapsed))
                .to_json()
        }
        Err(e) => provider_error_to_response(&e, start),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_text_ok() {
        let start = Instant::now();
        assert!(check_tweet_length("Hello world", start).is_none());
    }

    #[test]
    fn exactly_280_ok() {
        let start = Instant::now();
        let text = "a".repeat(280);
        assert!(check_tweet_length(&text, start).is_none());
    }

    #[test]
    fn over_280_rejected() {
        let start = Instant::now();
        let text = "a".repeat(281);
        assert!(check_tweet_length(&text, start).is_some());
    }

    #[test]
    fn url_counted_as_23() {
        let text = format!(
            "{} https://example.com/very/long/path/that/exceeds/23",
            "a".repeat(256)
        );
        let start = Instant::now();
        assert!(check_tweet_length(&text, start).is_none());
    }

    #[test]
    fn url_weighted_over_280() {
        let text = format!("{} https://example.com", "a".repeat(257));
        let start = Instant::now();
        assert!(check_tweet_length(&text, start).is_some());
    }

    #[test]
    fn compute_weighted_basic() {
        assert_eq!(compute_weighted_length("hello"), 5);
    }

    #[test]
    fn compute_weighted_url() {
        assert_eq!(
            compute_weighted_length("check https://example.com out"),
            // "check " = 6 + URL = 23 + " out" = 4 = 33
            33
        );
    }
}
