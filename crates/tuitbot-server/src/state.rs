//! Shared application state for the tuitbot server.

use std::path::PathBuf;
use tuitbot_core::storage::DbPool;

/// Shared application state accessible by all route handlers.
pub struct AppState {
    /// SQLite connection pool.
    pub db: DbPool,
    /// Path to the configuration file.
    pub config_path: PathBuf,
}
