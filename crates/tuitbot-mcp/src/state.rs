//! Shared application state for the MCP server.
//!
//! Bundles the database pool, configuration, optional LLM provider,
//! and optional X API client so that all tool handlers can access
//! them through the server struct.

use std::sync::Arc;

use tuitbot_core::config::Config;
use tuitbot_core::llm::LlmProvider;
use tuitbot_core::storage::DbPool;
use tuitbot_core::x_api::XApiClient;

/// Shared state accessible by all MCP tool handlers.
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
}

/// Thread-safe reference to shared state.
pub type SharedState = Arc<AppState>;
