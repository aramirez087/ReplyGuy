//! Machine-readable tool manifest for the MCP server.
//!
//! Provides a [`ToolManifest`] describing every registered tool: its name,
//! category, mutation flag, dependency requirements, available profiles, and
//! possible error codes. Generated programmatically — the snapshot test in this
//! module ensures the manifest JSON artifact never drifts from source.

use serde::Serialize;

use crate::contract::error_code::ErrorCode;

/// Top-level manifest containing all tool entries.
#[derive(Debug, Serialize)]
pub struct ToolManifest {
    /// Schema version for the manifest format.
    pub version: &'static str,
    /// All registered tools.
    pub tools: Vec<ToolEntry>,
}

/// Metadata for a single tool.
#[derive(Debug, Serialize)]
pub struct ToolEntry {
    /// Tool name as registered in the MCP server (e.g. `"x_post_tweet"`).
    pub name: &'static str,
    /// Functional category.
    pub category: ToolCategory,
    /// Whether this tool performs a mutation (write/engage).
    pub mutation: bool,
    /// Whether the tool requires an authenticated X API client.
    pub requires_x_client: bool,
    /// Whether the tool requires an LLM provider.
    pub requires_llm: bool,
    /// Whether the tool requires database access.
    pub requires_db: bool,
    /// Which profiles include this tool.
    pub profiles: Vec<Profile>,
    /// Error codes this tool may return.
    pub possible_error_codes: Vec<ErrorCode>,
}

/// Functional category for grouping tools.
#[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ToolCategory {
    Read,
    Write,
    Engage,
    Media,
    Analytics,
    Approval,
    Content,
    Discovery,
    Scoring,
    Config,
    Health,
    Policy,
    Telemetry,
    Context,
    Composite,
    Meta,
}

/// MCP server profile.
#[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Profile {
    Workflow,
    Api,
}

/// Build the complete tool manifest from the source-of-truth lookup table.
pub fn generate_manifest() -> ToolManifest {
    ToolManifest {
        version: "1.0",
        tools: all_tools(),
    }
}

// ── Helpers ────────────────────────────────────────────────────────────────

/// Shorthand constructors.
#[allow(clippy::too_many_arguments)]
fn tool(
    name: &'static str,
    category: ToolCategory,
    mutation: bool,
    requires_x_client: bool,
    requires_llm: bool,
    requires_db: bool,
    profiles: &[Profile],
    error_codes: &[ErrorCode],
) -> ToolEntry {
    ToolEntry {
        name,
        category,
        mutation,
        requires_x_client,
        requires_llm,
        requires_db,
        profiles: profiles.to_vec(),
        possible_error_codes: error_codes.to_vec(),
    }
}

const BOTH: &[Profile] = &[Profile::Workflow, Profile::Api];
const WF: &[Profile] = &[Profile::Workflow];
const API: &[Profile] = &[Profile::Api];

/// X API read errors.
const X_READ_ERR: &[ErrorCode] = &[
    ErrorCode::XNotConfigured,
    ErrorCode::XRateLimited,
    ErrorCode::XAuthExpired,
    ErrorCode::XForbidden,
    ErrorCode::XNetworkError,
    ErrorCode::XApiError,
];

/// X API read errors + no-user-id.
const X_READ_USER_ERR: &[ErrorCode] = &[
    ErrorCode::XNotConfigured,
    ErrorCode::XRateLimited,
    ErrorCode::XAuthExpired,
    ErrorCode::XForbidden,
    ErrorCode::XNetworkError,
    ErrorCode::XApiError,
];

/// X API write errors (includes policy).
const X_WRITE_ERR: &[ErrorCode] = &[
    ErrorCode::XNotConfigured,
    ErrorCode::XRateLimited,
    ErrorCode::XAuthExpired,
    ErrorCode::XForbidden,
    ErrorCode::XNetworkError,
    ErrorCode::XApiError,
    ErrorCode::TweetTooLong,
    ErrorCode::PolicyDeniedBlocked,
    ErrorCode::PolicyDeniedRateLimited,
    ErrorCode::PolicyDeniedHardRule,
    ErrorCode::PolicyDeniedUserRule,
    ErrorCode::PolicyError,
];

/// X API engage errors.
const X_ENGAGE_ERR: &[ErrorCode] = &[
    ErrorCode::XNotConfigured,
    ErrorCode::XRateLimited,
    ErrorCode::XAuthExpired,
    ErrorCode::XForbidden,
    ErrorCode::XNetworkError,
    ErrorCode::XApiError,
    ErrorCode::PolicyDeniedBlocked,
    ErrorCode::PolicyDeniedRateLimited,
    ErrorCode::PolicyDeniedHardRule,
    ErrorCode::PolicyDeniedUserRule,
    ErrorCode::PolicyError,
];

/// Database-only errors.
const DB_ERR: &[ErrorCode] = &[ErrorCode::DbError];

/// LLM errors.
const LLM_ERR: &[ErrorCode] = &[ErrorCode::LlmNotConfigured, ErrorCode::LlmError];

fn all_tools() -> Vec<ToolEntry> {
    vec![
        // ── Analytics ────────────────────────────────────────────────
        tool(
            "get_stats",
            ToolCategory::Analytics,
            false,
            false,
            false,
            true,
            WF,
            DB_ERR,
        ),
        tool(
            "get_follower_trend",
            ToolCategory::Analytics,
            false,
            false,
            false,
            true,
            WF,
            DB_ERR,
        ),
        // ── Action Log ───────────────────────────────────────────────
        tool(
            "get_action_log",
            ToolCategory::Analytics,
            false,
            false,
            false,
            true,
            WF,
            DB_ERR,
        ),
        tool(
            "get_action_counts",
            ToolCategory::Analytics,
            false,
            false,
            false,
            true,
            WF,
            DB_ERR,
        ),
        // ── Rate Limits ──────────────────────────────────────────────
        tool(
            "get_rate_limits",
            ToolCategory::Policy,
            false,
            false,
            false,
            true,
            WF,
            DB_ERR,
        ),
        // ── Replies ──────────────────────────────────────────────────
        tool(
            "get_recent_replies",
            ToolCategory::Analytics,
            false,
            false,
            false,
            true,
            WF,
            DB_ERR,
        ),
        tool(
            "get_reply_count_today",
            ToolCategory::Analytics,
            false,
            false,
            false,
            true,
            WF,
            DB_ERR,
        ),
        // ── Target Accounts ──────────────────────────────────────────
        tool(
            "list_target_accounts",
            ToolCategory::Discovery,
            false,
            false,
            false,
            true,
            WF,
            DB_ERR,
        ),
        // ── Discovery ────────────────────────────────────────────────
        tool(
            "list_unreplied_tweets",
            ToolCategory::Discovery,
            false,
            false,
            false,
            true,
            WF,
            DB_ERR,
        ),
        // ── Scoring ──────────────────────────────────────────────────
        tool(
            "score_tweet",
            ToolCategory::Scoring,
            false,
            false,
            false,
            true,
            BOTH,
            &[ErrorCode::DbError, ErrorCode::InvalidInput],
        ),
        // ── Approval Queue ───────────────────────────────────────────
        tool(
            "list_pending_approvals",
            ToolCategory::Approval,
            false,
            false,
            false,
            true,
            WF,
            DB_ERR,
        ),
        tool(
            "get_pending_count",
            ToolCategory::Approval,
            false,
            false,
            false,
            true,
            WF,
            DB_ERR,
        ),
        tool(
            "approve_item",
            ToolCategory::Approval,
            true,
            true,
            false,
            true,
            WF,
            &[
                ErrorCode::DbError,
                ErrorCode::NotFound,
                ErrorCode::XNotConfigured,
                ErrorCode::XApiError,
            ],
        ),
        tool(
            "reject_item",
            ToolCategory::Approval,
            true,
            false,
            false,
            true,
            WF,
            &[ErrorCode::DbError, ErrorCode::NotFound],
        ),
        tool(
            "approve_all",
            ToolCategory::Approval,
            true,
            true,
            false,
            true,
            WF,
            &[
                ErrorCode::DbError,
                ErrorCode::XNotConfigured,
                ErrorCode::XApiError,
            ],
        ),
        // ── Content Generation ───────────────────────────────────────
        tool(
            "generate_reply",
            ToolCategory::Content,
            false,
            false,
            true,
            true,
            WF,
            LLM_ERR,
        ),
        tool(
            "generate_tweet",
            ToolCategory::Content,
            false,
            false,
            true,
            true,
            WF,
            LLM_ERR,
        ),
        tool(
            "generate_thread",
            ToolCategory::Content,
            false,
            false,
            true,
            true,
            WF,
            LLM_ERR,
        ),
        // ── Config ───────────────────────────────────────────────────
        tool(
            "get_config",
            ToolCategory::Config,
            false,
            false,
            false,
            false,
            BOTH,
            &[],
        ),
        tool(
            "validate_config",
            ToolCategory::Config,
            false,
            false,
            false,
            false,
            BOTH,
            &[],
        ),
        // ── Capabilities & Health ────────────────────────────────────
        tool(
            "get_capabilities",
            ToolCategory::Meta,
            false,
            false,
            false,
            false,
            BOTH,
            &[],
        ),
        tool(
            "health_check",
            ToolCategory::Health,
            false,
            false,
            false,
            true,
            BOTH,
            DB_ERR,
        ),
        // ── Mode & Policy ────────────────────────────────────────────
        tool(
            "get_mode",
            ToolCategory::Meta,
            false,
            false,
            false,
            false,
            BOTH,
            &[],
        ),
        tool(
            "get_policy_status",
            ToolCategory::Policy,
            false,
            false,
            false,
            true,
            WF,
            DB_ERR,
        ),
        tool(
            "compose_tweet",
            ToolCategory::Write,
            true,
            true,
            false,
            true,
            WF,
            &[
                ErrorCode::XNotConfigured,
                ErrorCode::XApiError,
                ErrorCode::DbError,
                ErrorCode::InvalidInput,
                ErrorCode::TweetTooLong,
                ErrorCode::PolicyDeniedBlocked,
                ErrorCode::PolicyDeniedRateLimited,
                ErrorCode::PolicyDeniedHardRule,
                ErrorCode::PolicyDeniedUserRule,
                ErrorCode::PolicyError,
            ],
        ),
        // ── Discovery Feed & Topics ──────────────────────────────────
        tool(
            "get_discovery_feed",
            ToolCategory::Discovery,
            false,
            false,
            false,
            true,
            WF,
            DB_ERR,
        ),
        tool(
            "suggest_topics",
            ToolCategory::Content,
            false,
            false,
            false,
            true,
            WF,
            DB_ERR,
        ),
        // ── X API Read ───────────────────────────────────────────────
        tool(
            "get_tweet_by_id",
            ToolCategory::Read,
            false,
            true,
            false,
            false,
            BOTH,
            X_READ_ERR,
        ),
        tool(
            "x_get_user_by_username",
            ToolCategory::Read,
            false,
            true,
            false,
            false,
            BOTH,
            X_READ_ERR,
        ),
        tool(
            "x_search_tweets",
            ToolCategory::Read,
            false,
            true,
            false,
            false,
            BOTH,
            X_READ_ERR,
        ),
        tool(
            "x_get_user_mentions",
            ToolCategory::Read,
            false,
            true,
            false,
            false,
            BOTH,
            X_READ_USER_ERR,
        ),
        tool(
            "x_get_user_tweets",
            ToolCategory::Read,
            false,
            true,
            false,
            false,
            BOTH,
            X_READ_ERR,
        ),
        tool(
            "x_get_home_timeline",
            ToolCategory::Read,
            false,
            true,
            false,
            false,
            BOTH,
            X_READ_USER_ERR,
        ),
        tool(
            "x_get_followers",
            ToolCategory::Read,
            false,
            true,
            false,
            false,
            BOTH,
            X_READ_ERR,
        ),
        tool(
            "x_get_following",
            ToolCategory::Read,
            false,
            true,
            false,
            false,
            BOTH,
            X_READ_ERR,
        ),
        tool(
            "x_get_user_by_id",
            ToolCategory::Read,
            false,
            true,
            false,
            false,
            BOTH,
            X_READ_ERR,
        ),
        tool(
            "x_get_liked_tweets",
            ToolCategory::Read,
            false,
            true,
            false,
            false,
            BOTH,
            X_READ_ERR,
        ),
        tool(
            "x_get_bookmarks",
            ToolCategory::Read,
            false,
            true,
            false,
            false,
            BOTH,
            X_READ_USER_ERR,
        ),
        tool(
            "x_get_users_by_ids",
            ToolCategory::Read,
            false,
            true,
            false,
            false,
            BOTH,
            X_READ_ERR,
        ),
        tool(
            "x_get_tweet_liking_users",
            ToolCategory::Read,
            false,
            true,
            false,
            false,
            BOTH,
            X_READ_ERR,
        ),
        tool(
            "get_x_usage",
            ToolCategory::Analytics,
            false,
            false,
            false,
            true,
            WF,
            DB_ERR,
        ),
        // ── X API Read (API-only) ────────────────────────────────────
        tool(
            "x_get_me",
            ToolCategory::Read,
            false,
            true,
            false,
            false,
            API,
            X_READ_ERR,
        ),
        // ── X API Write ──────────────────────────────────────────────
        tool(
            "x_post_tweet",
            ToolCategory::Write,
            true,
            true,
            false,
            true,
            BOTH,
            X_WRITE_ERR,
        ),
        tool(
            "x_reply_to_tweet",
            ToolCategory::Write,
            true,
            true,
            false,
            true,
            BOTH,
            X_WRITE_ERR,
        ),
        tool(
            "x_quote_tweet",
            ToolCategory::Write,
            true,
            true,
            false,
            true,
            BOTH,
            X_WRITE_ERR,
        ),
        tool(
            "x_delete_tweet",
            ToolCategory::Write,
            true,
            true,
            false,
            true,
            BOTH,
            X_WRITE_ERR,
        ),
        tool(
            "x_post_thread",
            ToolCategory::Write,
            true,
            true,
            false,
            true,
            BOTH,
            &[
                ErrorCode::XNotConfigured,
                ErrorCode::XRateLimited,
                ErrorCode::XAuthExpired,
                ErrorCode::XForbidden,
                ErrorCode::XNetworkError,
                ErrorCode::XApiError,
                ErrorCode::TweetTooLong,
                ErrorCode::InvalidInput,
                ErrorCode::ThreadPartialFailure,
                ErrorCode::PolicyDeniedBlocked,
                ErrorCode::PolicyDeniedRateLimited,
                ErrorCode::PolicyDeniedHardRule,
                ErrorCode::PolicyDeniedUserRule,
                ErrorCode::PolicyError,
            ],
        ),
        // ── X API Engage ─────────────────────────────────────────────
        tool(
            "x_like_tweet",
            ToolCategory::Engage,
            true,
            true,
            false,
            true,
            BOTH,
            X_ENGAGE_ERR,
        ),
        tool(
            "x_unlike_tweet",
            ToolCategory::Engage,
            true,
            true,
            false,
            true,
            BOTH,
            X_ENGAGE_ERR,
        ),
        tool(
            "x_follow_user",
            ToolCategory::Engage,
            true,
            true,
            false,
            true,
            BOTH,
            X_ENGAGE_ERR,
        ),
        tool(
            "x_unfollow_user",
            ToolCategory::Engage,
            true,
            true,
            false,
            true,
            BOTH,
            X_ENGAGE_ERR,
        ),
        tool(
            "x_retweet",
            ToolCategory::Engage,
            true,
            true,
            false,
            true,
            BOTH,
            X_ENGAGE_ERR,
        ),
        tool(
            "x_unretweet",
            ToolCategory::Engage,
            true,
            true,
            false,
            true,
            BOTH,
            X_ENGAGE_ERR,
        ),
        tool(
            "x_bookmark_tweet",
            ToolCategory::Engage,
            true,
            true,
            false,
            true,
            BOTH,
            X_ENGAGE_ERR,
        ),
        tool(
            "x_unbookmark_tweet",
            ToolCategory::Engage,
            true,
            true,
            false,
            true,
            BOTH,
            X_ENGAGE_ERR,
        ),
        // ── X API Media ──────────────────────────────────────────────
        tool(
            "x_upload_media",
            ToolCategory::Media,
            true,
            true,
            false,
            false,
            BOTH,
            &[
                ErrorCode::XNotConfigured,
                ErrorCode::UnsupportedMediaType,
                ErrorCode::FileReadError,
                ErrorCode::MediaUploadError,
                ErrorCode::XApiError,
            ],
        ),
        // ── Context Intelligence ─────────────────────────────────────
        tool(
            "get_author_context",
            ToolCategory::Context,
            false,
            true,
            false,
            true,
            WF,
            &[
                ErrorCode::XNotConfigured,
                ErrorCode::ContextError,
                ErrorCode::DbError,
            ],
        ),
        tool(
            "recommend_engagement_action",
            ToolCategory::Context,
            false,
            true,
            false,
            true,
            WF,
            &[
                ErrorCode::XNotConfigured,
                ErrorCode::RecommendationError,
                ErrorCode::DbError,
            ],
        ),
        tool(
            "topic_performance_snapshot",
            ToolCategory::Context,
            false,
            false,
            false,
            true,
            WF,
            &[ErrorCode::TopicError, ErrorCode::DbError],
        ),
        // ── Telemetry ────────────────────────────────────────────────
        tool(
            "get_mcp_tool_metrics",
            ToolCategory::Telemetry,
            false,
            false,
            false,
            true,
            WF,
            DB_ERR,
        ),
        tool(
            "get_mcp_error_breakdown",
            ToolCategory::Telemetry,
            false,
            false,
            false,
            true,
            WF,
            DB_ERR,
        ),
        // ── Composite ────────────────────────────────────────────────
        tool(
            "find_reply_opportunities",
            ToolCategory::Composite,
            false,
            true,
            false,
            true,
            WF,
            &[
                ErrorCode::XNotConfigured,
                ErrorCode::InvalidInput,
                ErrorCode::XApiError,
                ErrorCode::DbError,
            ],
        ),
        tool(
            "draft_replies_for_candidates",
            ToolCategory::Composite,
            false,
            false,
            true,
            true,
            WF,
            &[
                ErrorCode::InvalidInput,
                ErrorCode::LlmNotConfigured,
                ErrorCode::LlmError,
                ErrorCode::DbError,
            ],
        ),
        tool(
            "propose_and_queue_replies",
            ToolCategory::Composite,
            true,
            true,
            false,
            true,
            WF,
            &[
                ErrorCode::InvalidInput,
                ErrorCode::XNotConfigured,
                ErrorCode::XApiError,
                ErrorCode::DbError,
                ErrorCode::PolicyDeniedBlocked,
                ErrorCode::PolicyDeniedRateLimited,
                ErrorCode::PolicyDeniedHardRule,
                ErrorCode::PolicyDeniedUserRule,
                ErrorCode::PolicyError,
            ],
        ),
        tool(
            "generate_thread_plan",
            ToolCategory::Composite,
            false,
            false,
            true,
            false,
            WF,
            &[
                ErrorCode::LlmNotConfigured,
                ErrorCode::LlmError,
                ErrorCode::InvalidInput,
            ],
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn manifest_generates_without_panic() {
        let manifest = generate_manifest();
        assert_eq!(manifest.version, "1.0");
        assert!(!manifest.tools.is_empty());
    }

    #[test]
    fn no_duplicate_tool_names() {
        let manifest = generate_manifest();
        let mut seen = HashSet::new();
        for t in &manifest.tools {
            assert!(seen.insert(t.name), "duplicate tool name: {}", t.name);
        }
    }

    #[test]
    fn all_tools_have_at_least_one_profile() {
        let manifest = generate_manifest();
        for t in &manifest.tools {
            assert!(!t.profiles.is_empty(), "tool {} has no profiles", t.name);
        }
    }

    #[test]
    fn mutation_tools_require_x_or_db() {
        let manifest = generate_manifest();
        for t in &manifest.tools {
            if t.mutation {
                assert!(
                    t.requires_x_client || t.requires_db,
                    "mutation tool {} requires neither x_client nor db",
                    t.name
                );
            }
        }
    }

    #[test]
    fn error_codes_are_valid_variants() {
        let all_codes: HashSet<ErrorCode> = ErrorCode::ALL.iter().copied().collect();
        let manifest = generate_manifest();
        for t in &manifest.tools {
            for &code in &t.possible_error_codes {
                assert!(
                    all_codes.contains(&code),
                    "tool {} references unknown error code {:?}",
                    t.name,
                    code
                );
            }
        }
    }

    #[test]
    fn category_counts() {
        let manifest = generate_manifest();
        let mut cats: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        for t in &manifest.tools {
            let cat = serde_json::to_string(&t.category).unwrap();
            *cats.entry(Box::leak(cat.into_boxed_str())).or_default() += 1;
        }
        // Sanity: we have tools in multiple categories
        assert!(
            cats.len() >= 10,
            "expected at least 10 categories, got {}",
            cats.len()
        );
    }

    #[test]
    fn manifest_snapshot() {
        let manifest = generate_manifest();
        let json = serde_json::to_string_pretty(&manifest).unwrap();
        let expected_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../roadmap/artifacts/session-05-tool-manifest.json"
        );
        let expected = std::fs::read_to_string(expected_path);
        match expected {
            Ok(content) => {
                assert_eq!(
                    json.trim(),
                    content.trim(),
                    "Tool manifest has drifted from snapshot. \
                     Regenerate with: cargo test -p tuitbot-mcp manifest -- --ignored"
                );
            }
            Err(_) => {
                // First run: write the snapshot.
                std::fs::write(expected_path, &json).unwrap();
            }
        }
    }
}
