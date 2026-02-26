//! Shared tweet length validation.
//!
//! Re-exports from [`crate::kernel::utils`] for backward compatibility.

pub(crate) use crate::kernel::utils::check_tweet_length;

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;
    use crate::kernel::utils::compute_weighted_length;

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
        // 257 chars + URL (23 weighted) = 280, should be OK.
        let text = format!(
            "{} https://example.com/very/long/path/that/exceeds/23",
            "a".repeat(256)
        );
        let start = Instant::now();
        assert!(check_tweet_length(&text, start).is_none());
    }

    #[test]
    fn url_weighted_over_280() {
        // 258 chars + URL (23 weighted) = 281, should fail.
        let text = format!("{} https://example.com", "a".repeat(257));
        let start = Instant::now();
        assert!(check_tweet_length(&text, start).is_some());
    }

    #[test]
    fn compute_weighted_basic() {
        assert_eq!(compute_weighted_length("hello"), 5);
    }
}
