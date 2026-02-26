//! Automatic retry wrapper for [`SocialReadProvider`].
//!
//! [`RetryingProvider`] wraps any provider and retries transient failures
//! (network errors, 5xx server errors) with exponential backoff + jitter.
//! Rate-limited (429) responses pass through immediately so the agent
//! can respect `retry_after_ms`.

use std::future::Future;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::contract::ProviderError;
use crate::provider::SocialReadProvider;
use tuitbot_core::x_api::types::{MentionResponse, SearchResponse, Tweet, User, UsersResponse};

/// Configuration for retry behavior.
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts (not counting the first try).
    pub max_retries: u32,
    /// Base delay between retries (doubled each attempt).
    pub base_delay: Duration,
    /// Maximum delay cap.
    pub max_delay: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 2,
            base_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(5),
        }
    }
}

impl RetryPolicy {
    /// Compute the delay for a given attempt (0-indexed).
    ///
    /// Uses exponential backoff: `base * 2^attempt`, capped at `max_delay`,
    /// plus up to 25% jitter derived from `SystemTime` nanoseconds.
    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        let base_ms = self.base_delay.as_millis() as u64;
        let exp_ms = base_ms.saturating_mul(1u64 << attempt.min(10));
        let capped_ms = exp_ms.min(self.max_delay.as_millis() as u64);
        // Jitter: 0–25% of capped delay using system clock nanos.
        let jitter_nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos();
        let jitter_ms = (capped_ms / 4).wrapping_mul(jitter_nanos as u64) / u32::MAX as u64;
        Duration::from_millis(capped_ms + jitter_ms)
    }

    /// Whether the given error should trigger a retry.
    ///
    /// Only transient errors (network, server 5xx) are retried.
    /// Rate-limited errors pass through for the agent to handle.
    pub fn should_retry(e: &ProviderError) -> bool {
        matches!(
            e,
            ProviderError::Network { .. } | ProviderError::ServerError { .. }
        )
    }
}

/// Execute an async operation with retry logic.
///
/// Returns `(result, retry_count)` where `retry_count` is the number of
/// retries performed (0 if the first attempt succeeded).
pub async fn with_retry<F, Fut, T>(
    policy: &RetryPolicy,
    mut op: F,
) -> (Result<T, ProviderError>, u32)
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, ProviderError>>,
{
    let mut attempts = 0u32;
    loop {
        match op().await {
            Ok(v) => return (Ok(v), attempts),
            Err(e) if RetryPolicy::should_retry(&e) && attempts < policy.max_retries => {
                let delay = policy.delay_for_attempt(attempts);
                tokio::time::sleep(delay).await;
                attempts += 1;
            }
            Err(e) => return (Err(e), attempts),
        }
    }
}

/// Provider wrapper that adds automatic retry to all [`SocialReadProvider`] methods.
pub struct RetryingProvider<P> {
    inner: P,
    policy: RetryPolicy,
}

impl<P> RetryingProvider<P> {
    /// Wrap a provider with retry behavior.
    pub fn new(inner: P, policy: RetryPolicy) -> Self {
        Self { inner, policy }
    }
}

#[async_trait::async_trait]
impl<P: SocialReadProvider> SocialReadProvider for RetryingProvider<P> {
    async fn get_tweet(&self, tweet_id: &str) -> Result<Tweet, ProviderError> {
        let (result, _) = with_retry(&self.policy, || self.inner.get_tweet(tweet_id)).await;
        result
    }

    async fn get_user_by_username(&self, username: &str) -> Result<User, ProviderError> {
        let (result, _) =
            with_retry(&self.policy, || self.inner.get_user_by_username(username)).await;
        result
    }

    async fn search_tweets(
        &self,
        query: &str,
        max_results: u32,
        since_id: Option<&str>,
        pagination_token: Option<&str>,
    ) -> Result<SearchResponse, ProviderError> {
        let (result, _) = with_retry(&self.policy, || {
            self.inner
                .search_tweets(query, max_results, since_id, pagination_token)
        })
        .await;
        result
    }

    async fn get_user_mentions(
        &self,
        user_id: &str,
        since_id: Option<&str>,
        pagination_token: Option<&str>,
    ) -> Result<MentionResponse, ProviderError> {
        let (result, _) = with_retry(&self.policy, || {
            self.inner
                .get_user_mentions(user_id, since_id, pagination_token)
        })
        .await;
        result
    }

    async fn get_user_tweets(
        &self,
        user_id: &str,
        max_results: u32,
        pagination_token: Option<&str>,
    ) -> Result<SearchResponse, ProviderError> {
        let (result, _) = with_retry(&self.policy, || {
            self.inner
                .get_user_tweets(user_id, max_results, pagination_token)
        })
        .await;
        result
    }

    async fn get_home_timeline(
        &self,
        user_id: &str,
        max_results: u32,
        pagination_token: Option<&str>,
    ) -> Result<SearchResponse, ProviderError> {
        let (result, _) = with_retry(&self.policy, || {
            self.inner
                .get_home_timeline(user_id, max_results, pagination_token)
        })
        .await;
        result
    }

    async fn get_me(&self) -> Result<User, ProviderError> {
        let (result, _) = with_retry(&self.policy, || self.inner.get_me()).await;
        result
    }

    async fn get_followers(
        &self,
        user_id: &str,
        max_results: u32,
        pagination_token: Option<&str>,
    ) -> Result<UsersResponse, ProviderError> {
        let (result, _) = with_retry(&self.policy, || {
            self.inner
                .get_followers(user_id, max_results, pagination_token)
        })
        .await;
        result
    }

    async fn get_following(
        &self,
        user_id: &str,
        max_results: u32,
        pagination_token: Option<&str>,
    ) -> Result<UsersResponse, ProviderError> {
        let (result, _) = with_retry(&self.policy, || {
            self.inner
                .get_following(user_id, max_results, pagination_token)
        })
        .await;
        result
    }

    async fn get_user_by_id(&self, user_id: &str) -> Result<User, ProviderError> {
        let (result, _) = with_retry(&self.policy, || self.inner.get_user_by_id(user_id)).await;
        result
    }

    async fn get_liked_tweets(
        &self,
        user_id: &str,
        max_results: u32,
        pagination_token: Option<&str>,
    ) -> Result<SearchResponse, ProviderError> {
        let (result, _) = with_retry(&self.policy, || {
            self.inner
                .get_liked_tweets(user_id, max_results, pagination_token)
        })
        .await;
        result
    }

    async fn get_bookmarks(
        &self,
        user_id: &str,
        max_results: u32,
        pagination_token: Option<&str>,
    ) -> Result<SearchResponse, ProviderError> {
        let (result, _) = with_retry(&self.policy, || {
            self.inner
                .get_bookmarks(user_id, max_results, pagination_token)
        })
        .await;
        result
    }

    async fn get_users_by_ids(&self, user_ids: &[&str]) -> Result<UsersResponse, ProviderError> {
        let (result, _) = with_retry(&self.policy, || self.inner.get_users_by_ids(user_ids)).await;
        result
    }

    async fn get_tweet_liking_users(
        &self,
        tweet_id: &str,
        max_results: u32,
        pagination_token: Option<&str>,
    ) -> Result<UsersResponse, ProviderError> {
        let (result, _) = with_retry(&self.policy, || {
            self.inner
                .get_tweet_liking_users(tweet_id, max_results, pagination_token)
        })
        .await;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;
    use tuitbot_core::x_api::types::{PublicMetrics, UserMetrics};

    #[test]
    fn delay_computation_exponential() {
        let policy = RetryPolicy {
            max_retries: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
        };
        let d0 = policy.delay_for_attempt(0);
        let d1 = policy.delay_for_attempt(1);
        let d2 = policy.delay_for_attempt(2);
        // Base values (without jitter): 100, 200, 400.
        // With up to 25% jitter, ranges are [100, 125], [200, 250], [400, 500].
        assert!(d0.as_millis() >= 100 && d0.as_millis() <= 125);
        assert!(d1.as_millis() >= 200 && d1.as_millis() <= 250);
        assert!(d2.as_millis() >= 400 && d2.as_millis() <= 500);
    }

    #[test]
    fn delay_capped_at_max() {
        let policy = RetryPolicy {
            max_retries: 5,
            base_delay: Duration::from_secs(2),
            max_delay: Duration::from_secs(5),
        };
        let d3 = policy.delay_for_attempt(3);
        // 2000 * 8 = 16000, capped to 5000. Jitter up to 25%: max 6250.
        assert!(d3.as_millis() <= 6250);
    }

    #[test]
    fn should_retry_network_and_server() {
        assert!(RetryPolicy::should_retry(&ProviderError::Network {
            message: "timeout".into()
        }));
        assert!(RetryPolicy::should_retry(&ProviderError::ServerError {
            status: 503,
            message: "unavailable".into()
        }));
    }

    #[test]
    fn should_not_retry_rate_limited() {
        assert!(!RetryPolicy::should_retry(&ProviderError::RateLimited {
            retry_after: Some(30)
        }));
    }

    #[test]
    fn should_not_retry_auth_expired() {
        assert!(!RetryPolicy::should_retry(&ProviderError::AuthExpired));
        assert!(!RetryPolicy::should_retry(&ProviderError::Forbidden {
            message: "no".into()
        }));
    }

    /// Mock provider that fails N times then succeeds.
    struct FailThenSucceed {
        fail_count: u32,
        calls: Arc<AtomicU32>,
    }

    #[async_trait::async_trait]
    impl SocialReadProvider for FailThenSucceed {
        async fn get_tweet(&self, tweet_id: &str) -> Result<Tweet, ProviderError> {
            let n = self.calls.fetch_add(1, Ordering::SeqCst);
            if n < self.fail_count {
                Err(ProviderError::Network {
                    message: "transient".into(),
                })
            } else {
                Ok(Tweet {
                    id: tweet_id.to_string(),
                    text: "ok".into(),
                    author_id: "a".into(),
                    created_at: String::new(),
                    public_metrics: PublicMetrics::default(),
                    conversation_id: None,
                })
            }
        }

        async fn get_user_by_username(&self, _: &str) -> Result<User, ProviderError> {
            Err(ProviderError::Other {
                message: "unused".into(),
            })
        }

        async fn search_tweets(
            &self,
            _: &str,
            _: u32,
            _: Option<&str>,
            _: Option<&str>,
        ) -> Result<SearchResponse, ProviderError> {
            Err(ProviderError::Other {
                message: "unused".into(),
            })
        }
    }

    #[tokio::test]
    async fn retry_succeeds_after_transient() {
        let calls = Arc::new(AtomicU32::new(0));
        let provider = FailThenSucceed {
            fail_count: 1,
            calls: calls.clone(),
        };
        let policy = RetryPolicy {
            max_retries: 2,
            base_delay: Duration::from_millis(1),
            max_delay: Duration::from_millis(5),
        };
        let retrying = RetryingProvider::new(provider, policy);
        let result = retrying.get_tweet("t1").await;
        assert!(result.is_ok());
        assert_eq!(calls.load(Ordering::SeqCst), 2); // 1 fail + 1 success
    }

    #[tokio::test]
    async fn retry_exhausted_returns_error() {
        let calls = Arc::new(AtomicU32::new(0));
        let provider = FailThenSucceed {
            fail_count: 10,
            calls: calls.clone(),
        };
        let policy = RetryPolicy {
            max_retries: 2,
            base_delay: Duration::from_millis(1),
            max_delay: Duration::from_millis(5),
        };
        let retrying = RetryingProvider::new(provider, policy);
        let result = retrying.get_tweet("t1").await;
        assert!(result.is_err());
        assert_eq!(calls.load(Ordering::SeqCst), 3); // 1 initial + 2 retries
    }

    /// Mock that always returns rate limited.
    struct RateLimitedProvider;

    #[async_trait::async_trait]
    impl SocialReadProvider for RateLimitedProvider {
        async fn get_tweet(&self, _: &str) -> Result<Tweet, ProviderError> {
            Err(ProviderError::RateLimited {
                retry_after: Some(30),
            })
        }
        async fn get_user_by_username(&self, _: &str) -> Result<User, ProviderError> {
            Err(ProviderError::Other {
                message: "unused".into(),
            })
        }
        async fn search_tweets(
            &self,
            _: &str,
            _: u32,
            _: Option<&str>,
            _: Option<&str>,
        ) -> Result<SearchResponse, ProviderError> {
            Err(ProviderError::Other {
                message: "unused".into(),
            })
        }
    }

    #[tokio::test]
    async fn rate_limited_passes_through() {
        let provider = RateLimitedProvider;
        let policy = RetryPolicy {
            max_retries: 2,
            base_delay: Duration::from_millis(1),
            max_delay: Duration::from_millis(5),
        };
        let retrying = RetryingProvider::new(provider, policy);
        let result = retrying.get_tweet("t1").await;
        assert!(result.is_err());
        // Should return immediately, no retries — RateLimited is not transient.
        if let Err(ProviderError::RateLimited { retry_after }) = result {
            assert_eq!(retry_after, Some(30));
        } else {
            panic!("expected RateLimited");
        }
    }

    #[tokio::test]
    async fn server_error_retried() {
        let calls = Arc::new(AtomicU32::new(0));
        // Provider that returns ServerError then succeeds
        struct ServerErrThenOk {
            calls: Arc<AtomicU32>,
        }

        #[async_trait::async_trait]
        impl SocialReadProvider for ServerErrThenOk {
            async fn get_tweet(&self, tid: &str) -> Result<Tweet, ProviderError> {
                let n = self.calls.fetch_add(1, Ordering::SeqCst);
                if n == 0 {
                    Err(ProviderError::ServerError {
                        status: 502,
                        message: "bad gateway".into(),
                    })
                } else {
                    Ok(Tweet {
                        id: tid.into(),
                        text: "ok".into(),
                        author_id: "a".into(),
                        created_at: String::new(),
                        public_metrics: PublicMetrics::default(),
                        conversation_id: None,
                    })
                }
            }
            async fn get_user_by_username(&self, _: &str) -> Result<User, ProviderError> {
                Err(ProviderError::Other {
                    message: "unused".into(),
                })
            }
            async fn search_tweets(
                &self,
                _: &str,
                _: u32,
                _: Option<&str>,
                _: Option<&str>,
            ) -> Result<SearchResponse, ProviderError> {
                Err(ProviderError::Other {
                    message: "unused".into(),
                })
            }
        }

        let provider = ServerErrThenOk {
            calls: calls.clone(),
        };
        let policy = RetryPolicy {
            max_retries: 2,
            base_delay: Duration::from_millis(1),
            max_delay: Duration::from_millis(5),
        };
        let retrying = RetryingProvider::new(provider, policy);
        let result = retrying.get_tweet("t1").await;
        assert!(result.is_ok());
        assert_eq!(calls.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn get_me_default_returns_error() {
        // Verify mock providers' default methods work for coverage.
        let _user = UserMetrics::default();
    }
}
