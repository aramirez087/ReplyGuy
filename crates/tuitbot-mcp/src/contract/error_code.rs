//! Typed error codes for the MCP tool contract.
//!
//! Every error returned by an MCP tool uses an [`ErrorCode`] variant instead
//! of a free-form string. This gives compile-time exhaustiveness, centralized
//! retry semantics, and a stable wire format (snake_case JSON strings).

use std::fmt;

use serde::{Deserialize, Serialize};

/// Machine-readable error code attached to every [`ToolError`](super::envelope::ToolError).
///
/// Serializes to snake_case strings for JSON wire compatibility
/// (e.g. `XRateLimited` → `"x_rate_limited"`).
#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    // ── X API ───────────────────────────────────────────────────────
    #[serde(rename = "x_rate_limited")]
    XRateLimited,
    #[serde(rename = "x_auth_expired")]
    XAuthExpired,
    #[serde(rename = "x_forbidden")]
    XForbidden,
    #[serde(rename = "x_account_restricted")]
    XAccountRestricted,
    #[serde(rename = "x_network_error")]
    XNetworkError,
    #[serde(rename = "x_not_configured")]
    XNotConfigured,
    #[serde(rename = "x_api_error")]
    XApiError,

    // ── Database ────────────────────────────────────────────────────
    #[serde(rename = "db_error")]
    DbError,

    // ── Validation ──────────────────────────────────────────────────
    #[serde(rename = "validation_error")]
    ValidationError,
    #[serde(rename = "invalid_input")]
    InvalidInput,
    #[serde(rename = "tweet_too_long")]
    TweetTooLong,

    // ── LLM ─────────────────────────────────────────────────────────
    #[serde(rename = "llm_error")]
    LlmError,
    #[serde(rename = "llm_not_configured")]
    LlmNotConfigured,

    // ── Media ───────────────────────────────────────────────────────
    #[serde(rename = "unsupported_media_type")]
    UnsupportedMediaType,
    #[serde(rename = "file_read_error")]
    FileReadError,
    #[serde(rename = "media_upload_error")]
    MediaUploadError,

    // ── Thread ──────────────────────────────────────────────────────
    #[serde(rename = "thread_partial_failure")]
    ThreadPartialFailure,

    // ── Policy ──────────────────────────────────────────────────────
    #[serde(rename = "policy_error")]
    PolicyError,
    #[serde(rename = "policy_denied_blocked")]
    PolicyDeniedBlocked,
    #[serde(rename = "policy_denied_rate_limited")]
    PolicyDeniedRateLimited,
    #[serde(rename = "policy_denied_hard_rule")]
    PolicyDeniedHardRule,
    #[serde(rename = "policy_denied_user_rule")]
    PolicyDeniedUserRule,

    // ── Context ─────────────────────────────────────────────────────
    #[serde(rename = "context_error")]
    ContextError,
    #[serde(rename = "recommendation_error")]
    RecommendationError,
    #[serde(rename = "topic_error")]
    TopicError,

    // ── Resource ────────────────────────────────────────────────────
    #[serde(rename = "not_found")]
    NotFound,

    // ── Internal ────────────────────────────────────────────────────
    #[serde(rename = "serialization_error")]
    SerializationError,
}

impl ErrorCode {
    /// All error code variants, for enumeration in tests and manifest generation.
    pub const ALL: &'static [ErrorCode] = &[
        Self::XRateLimited,
        Self::XAuthExpired,
        Self::XForbidden,
        Self::XAccountRestricted,
        Self::XNetworkError,
        Self::XNotConfigured,
        Self::XApiError,
        Self::DbError,
        Self::ValidationError,
        Self::InvalidInput,
        Self::TweetTooLong,
        Self::LlmError,
        Self::LlmNotConfigured,
        Self::UnsupportedMediaType,
        Self::FileReadError,
        Self::MediaUploadError,
        Self::ThreadPartialFailure,
        Self::PolicyError,
        Self::PolicyDeniedBlocked,
        Self::PolicyDeniedRateLimited,
        Self::PolicyDeniedHardRule,
        Self::PolicyDeniedUserRule,
        Self::ContextError,
        Self::RecommendationError,
        Self::TopicError,
        Self::NotFound,
        Self::SerializationError,
    ];

    /// Whether a caller may retry the request that produced this error.
    pub fn is_retryable(self) -> bool {
        matches!(
            self,
            Self::XRateLimited
                | Self::XNetworkError
                | Self::XApiError
                | Self::DbError
                | Self::LlmError
                | Self::ThreadPartialFailure
                | Self::PolicyError
        )
    }

    /// The snake_case string used on the wire (matches serde rename).
    pub fn as_str(self) -> &'static str {
        match self {
            Self::XRateLimited => "x_rate_limited",
            Self::XAuthExpired => "x_auth_expired",
            Self::XForbidden => "x_forbidden",
            Self::XAccountRestricted => "x_account_restricted",
            Self::XNetworkError => "x_network_error",
            Self::XNotConfigured => "x_not_configured",
            Self::XApiError => "x_api_error",
            Self::DbError => "db_error",
            Self::ValidationError => "validation_error",
            Self::InvalidInput => "invalid_input",
            Self::TweetTooLong => "tweet_too_long",
            Self::LlmError => "llm_error",
            Self::LlmNotConfigured => "llm_not_configured",
            Self::UnsupportedMediaType => "unsupported_media_type",
            Self::FileReadError => "file_read_error",
            Self::MediaUploadError => "media_upload_error",
            Self::ThreadPartialFailure => "thread_partial_failure",
            Self::PolicyError => "policy_error",
            Self::PolicyDeniedBlocked => "policy_denied_blocked",
            Self::PolicyDeniedRateLimited => "policy_denied_rate_limited",
            Self::PolicyDeniedHardRule => "policy_denied_hard_rule",
            Self::PolicyDeniedUserRule => "policy_denied_user_rule",
            Self::ContextError => "context_error",
            Self::RecommendationError => "recommendation_error",
            Self::TopicError => "topic_error",
            Self::NotFound => "not_found",
            Self::SerializationError => "serialization_error",
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_constant_has_correct_count() {
        assert_eq!(ErrorCode::ALL.len(), 27);
    }

    #[test]
    fn roundtrip_serialization() {
        for &code in ErrorCode::ALL {
            let json = serde_json::to_string(&code).unwrap();
            let back: ErrorCode = serde_json::from_str(&json).unwrap();
            assert_eq!(back, code, "roundtrip failed for {code}");
        }
    }

    #[test]
    fn display_matches_serde() {
        for &code in ErrorCode::ALL {
            let display = code.to_string();
            let serde = serde_json::to_string(&code).unwrap();
            // serde wraps in quotes
            assert_eq!(
                format!("\"{display}\""),
                serde,
                "Display/serde mismatch for {code:?}"
            );
        }
    }

    #[test]
    fn is_retryable_consistency() {
        let retryable_codes = [
            ErrorCode::XRateLimited,
            ErrorCode::XNetworkError,
            ErrorCode::XApiError,
            ErrorCode::DbError,
            ErrorCode::LlmError,
            ErrorCode::ThreadPartialFailure,
            ErrorCode::PolicyError,
        ];
        for &code in ErrorCode::ALL {
            let expected = retryable_codes.contains(&code);
            assert_eq!(
                code.is_retryable(),
                expected,
                "{code:?}: expected retryable={expected}"
            );
        }
    }

    #[test]
    fn as_str_matches_display() {
        for &code in ErrorCode::ALL {
            assert_eq!(code.as_str(), &code.to_string());
        }
    }
}
