//! Shared application state for the MCP server.
//!
//! Bundles the database pool, configuration, optional LLM provider,
//! and optional X API client so that all tool handlers can access
//! them through the server struct.
//!
//! Two state structs exist for two runtime profiles:
//! - [`AppState`] / [`SharedState`]: full workflow profile (DB + LLM + X client).
//! - [`ApiState`] / [`SharedApiState`]: lightweight API profile (X client only, no DB).

use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

use tuitbot_core::config::Config;
use tuitbot_core::llm::LlmProvider;
use tuitbot_core::storage::DbPool;
use tuitbot_core::x_api::XApiClient;

use crate::tools::idempotency::IdempotencyStore;

// ── Runtime profile ─────────────────────────────────────────────────

/// MCP server runtime profile.
///
/// - **`Api`** — generic X client tools only. No DB, no LLM, no policy gating. ~24 tools.
/// - **`Workflow`** — full TuitBot growth features. Default. All 60+ tools.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Profile {
    Api,
    Workflow,
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Api => write!(f, "api"),
            Self::Workflow => write!(f, "workflow"),
        }
    }
}

impl FromStr for Profile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "api" => Ok(Self::Api),
            "workflow" => Ok(Self::Workflow),
            other => Err(format!(
                "unknown profile '{other}'. Valid profiles: api, workflow"
            )),
        }
    }
}

// ── Workflow state (full) ───────────────────────────────────────────

/// Shared state accessible by all MCP tool handlers (workflow profile).
pub struct AppState {
    /// SQLite connection pool.
    pub pool: DbPool,
    /// Loaded and validated configuration.
    pub config: Config,
    /// Optional LLM provider (None if not configured or creation failed).
    pub llm_provider: Option<Box<dyn LlmProvider>>,
    /// Optional X API client (None if tokens not available).
    pub x_client: Option<Box<dyn XApiClient>>,
    /// Authenticated user ID from X API (cached from get_me on startup).
    pub authenticated_user_id: Option<String>,
    /// Idempotency guard for mutation dedup.
    pub idempotency: Arc<IdempotencyStore>,
}

/// Thread-safe reference to shared workflow state.
pub type SharedState = Arc<AppState>;

// ── API state (lightweight) ─────────────────────────────────────────

/// Lightweight state for the API profile (no DB, no LLM).
///
/// The X client is non-optional: an API profile with no X client has
/// zero usable tools, so `run_api_server` fails fast if tokens are missing.
pub struct ApiState {
    /// Loaded configuration.
    pub config: Config,
    /// X API client (required for API profile).
    pub x_client: Box<dyn XApiClient>,
    /// Authenticated user ID from X API (from get_me on startup).
    pub authenticated_user_id: String,
    /// Idempotency guard for mutation dedup.
    pub idempotency: Arc<IdempotencyStore>,
}

/// Thread-safe reference to shared API state.
pub type SharedApiState = Arc<ApiState>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_display() {
        assert_eq!(Profile::Api.to_string(), "api");
        assert_eq!(Profile::Workflow.to_string(), "workflow");
    }

    #[test]
    fn profile_from_str_valid() {
        assert_eq!(Profile::from_str("api").unwrap(), Profile::Api);
        assert_eq!(Profile::from_str("workflow").unwrap(), Profile::Workflow);
        assert_eq!(Profile::from_str("API").unwrap(), Profile::Api);
        assert_eq!(Profile::from_str("Workflow").unwrap(), Profile::Workflow);
    }

    #[test]
    fn profile_from_str_invalid() {
        let err = Profile::from_str("unknown").unwrap_err();
        assert!(err.contains("unknown profile"));
        assert!(err.contains("api, workflow"));
    }
}
