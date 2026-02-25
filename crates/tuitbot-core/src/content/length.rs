//! URL-aware tweet length calculation.
//!
//! Twitter/X wraps every URL in a t.co short link (always 23 characters).
//! This module provides length functions that account for t.co normalization
//! so tweets containing URLs are not incorrectly rejected or truncated.

use regex::Regex;
use std::sync::OnceLock;

/// Length of a t.co shortened URL on X.
pub const TCO_URL_LENGTH: usize = 23;

/// Maximum characters allowed in a single tweet.
pub const MAX_TWEET_CHARS: usize = 280;

/// Compiled regex matching URLs that X will wrap in t.co links.
///
/// Matches two patterns:
/// 1. Protocol URLs: `https?://[^\s)>\]]+`
/// 2. Bare domains: `domain.tld` with common TLD allowlist, optional path
fn url_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r#"(?x)
            https?://[^\s)>\]]+
            |
            \b[a-zA-Z0-9](?:[a-zA-Z0-9-]*[a-zA-Z0-9])?
            (?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]*[a-zA-Z0-9])?)*
            \.(?:com|org|net|edu|gov|io|co|dev|app|me|info|biz|xyz|ai|tech|so|to|cc|gg|tv|fm|ly)
            (?:/[^\s)>\]]*)?
            "#,
        )
        .expect("URL regex is valid")
    })
}

/// Calculate the weighted length of a tweet accounting for t.co URL wrapping.
///
/// Every URL (protocol or bare domain) is counted as [`TCO_URL_LENGTH`] (23)
/// characters regardless of its actual length.
pub fn tweet_weighted_len(text: &str) -> usize {
    let re = url_regex();
    let mut length = text.len();

    for m in re.find_iter(text) {
        let url_len = m.as_str().len();
        // Replace actual URL length with t.co length
        length = length - url_len + TCO_URL_LENGTH;
    }

    length
}

/// Check if text is within the tweet character limit, accounting for t.co URLs.
///
/// Media attachments (images, GIFs, videos) do **not** affect the character
/// count — X attaches them via `media_ids` outside the tweet text, so this
/// function only considers the text content.
pub fn validate_tweet_length(text: &str, max_chars: usize) -> bool {
    tweet_weighted_len(text) <= max_chars
}

/// Truncate text at the last sentence boundary that fits within the limit.
///
/// Uses URL-aware length calculation. Looks for the last period, exclamation
/// mark, or question mark within the limit. Falls back to truncating at the
/// limit with "..." if no sentence boundary is found.
pub fn truncate_at_sentence(text: &str, max_chars: usize) -> String {
    if tweet_weighted_len(text) <= max_chars {
        return text.to_string();
    }

    // For truncation we need a byte-level cutoff. If the text has no URLs,
    // max_chars is exact. With URLs it's conservative but safe — we work
    // backwards from byte positions and recheck the weighted length.

    // Start from the raw byte limit (may be generous if URLs are present)
    let byte_limit = text.len().min(max_chars);
    let search_area = &text[..byte_limit];

    // Find the last sentence-ending punctuation
    let last_sentence_end = search_area
        .rfind('.')
        .max(search_area.rfind('!'))
        .max(search_area.rfind('?'));

    if let Some(pos) = last_sentence_end {
        if pos > 0 {
            let candidate = text[..=pos].trim().to_string();
            if tweet_weighted_len(&candidate) <= max_chars {
                return candidate;
            }
        }
    }

    // No valid sentence boundary; hard truncate with ellipsis.
    // Walk backwards to find a position that fits.
    let truncate_at = byte_limit.saturating_sub(3);
    let word_end = text[..truncate_at].rfind(' ').unwrap_or(truncate_at);
    let candidate = format!("{}...", &text[..word_end]);

    if tweet_weighted_len(&candidate) <= max_chars {
        return candidate;
    }

    // If still too long (many URLs), keep shrinking
    let mut end = word_end;
    while end > 0 {
        end = text[..end].rfind(' ').unwrap_or(0);
        let candidate = if end == 0 {
            "...".to_string()
        } else {
            format!("{}...", &text[..end])
        };
        if tweet_weighted_len(&candidate) <= max_chars {
            return candidate;
        }
    }

    "...".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_urls_plain_text() {
        let text = "Hello world, this is a simple tweet!";
        assert_eq!(tweet_weighted_len(text), text.len());
    }

    #[test]
    fn single_long_protocol_url() {
        let text = "Check out https://example.com/very/long/path/to/some/resource?query=value&other=param for more info!";
        let url = "https://example.com/very/long/path/to/some/resource?query=value&other=param";
        let expected = text.len() - url.len() + TCO_URL_LENGTH;
        assert_eq!(tweet_weighted_len(text), expected);
    }

    #[test]
    fn multiple_urls() {
        let text = "Visit https://example.com and https://another-site.org/page for details";
        let url1 = "https://example.com";
        let url2 = "https://another-site.org/page";
        let expected = text.len() - url1.len() - url2.len() + TCO_URL_LENGTH * 2;
        assert_eq!(tweet_weighted_len(text), expected);
    }

    #[test]
    fn http_url() {
        let text = "See http://example.com/path for info";
        let url = "http://example.com/path";
        let expected = text.len() - url.len() + TCO_URL_LENGTH;
        assert_eq!(tweet_weighted_len(text), expected);
    }

    #[test]
    fn bare_domain() {
        let text = "Check example.com for details";
        let url = "example.com";
        let expected = text.len() - url.len() + TCO_URL_LENGTH;
        assert_eq!(tweet_weighted_len(text), expected);
    }

    #[test]
    fn bare_domain_with_path() {
        let text = "Visit docs.example.io/getting-started today";
        let url = "docs.example.io/getting-started";
        let expected = text.len() - url.len() + TCO_URL_LENGTH;
        assert_eq!(tweet_weighted_len(text), expected);
    }

    #[test]
    fn url_in_parentheses() {
        let text = "Great resource (https://example.com/long/url/here) for learning";
        let url = "https://example.com/long/url/here";
        let expected = text.len() - url.len() + TCO_URL_LENGTH;
        assert_eq!(tweet_weighted_len(text), expected);
    }

    #[test]
    fn validate_with_url_under_limit() {
        // 250 chars of text + a 100-char URL = 350 raw bytes
        // but weighted = 250 + 23 = 273, under 280
        let padding = "a".repeat(250);
        let text = format!("{padding} https://example.com/{}", "x".repeat(76));
        assert!(text.len() > 280); // raw is over
        assert!(validate_tweet_length(&text, MAX_TWEET_CHARS)); // weighted is under
    }

    #[test]
    fn validate_with_url_over_limit() {
        // 260 chars of text + a URL = 260 + 23 = 283, over 280
        let padding = "a".repeat(260);
        let text = format!("{padding} https://example.com");
        assert!(!validate_tweet_length(&text, MAX_TWEET_CHARS));
    }

    #[test]
    fn validate_no_url_at_limit() {
        let text = "a".repeat(280);
        assert!(validate_tweet_length(&text, MAX_TWEET_CHARS));
    }

    #[test]
    fn validate_no_url_over_limit() {
        let text = "a".repeat(281);
        assert!(!validate_tweet_length(&text, MAX_TWEET_CHARS));
    }

    #[test]
    fn truncate_under_limit_unchanged() {
        let text = "Short sentence.";
        assert_eq!(
            truncate_at_sentence(text, MAX_TWEET_CHARS),
            "Short sentence."
        );
    }

    #[test]
    fn truncate_preserves_sentence_boundary() {
        let text = "First sentence. Second sentence. Third sentence is very long and goes over the limit and more and more text.";
        let result = truncate_at_sentence(text, 50);
        assert!(tweet_weighted_len(&result) <= 50);
        assert!(result.ends_with('.'));
    }

    #[test]
    fn truncate_no_sentence_boundary() {
        let text =
            "This is a very long sentence without any punctuation that keeps going and going";
        let result = truncate_at_sentence(text, 30);
        assert!(tweet_weighted_len(&result) <= 30);
        assert!(result.ends_with("..."));
    }

    #[test]
    fn media_does_not_affect_length() {
        // Media is attached via media_ids in the API request, not in the tweet
        // text body. The character counter should only look at text content.
        // This test documents that media_paths/media_ids are a separate field
        // and do not contribute to the weighted length calculation.
        let text = "Check out this photo!";
        let len = tweet_weighted_len(text);
        assert_eq!(len, text.len());
        assert!(validate_tweet_length(text, MAX_TWEET_CHARS));

        // Even at exactly 280 chars, adding media should still be valid.
        let text_280 = "a".repeat(280);
        assert!(validate_tweet_length(&text_280, MAX_TWEET_CHARS));
    }

    #[test]
    fn not_a_url_without_known_tld() {
        // ".rs" is not in the TLD allowlist, so this shouldn't be treated as a URL
        let text = "Check out foo.rs for Rust crates";
        assert_eq!(tweet_weighted_len(text), text.len());
    }
}
