//! Health check tool.

use std::time::Instant;

use serde::Serialize;

use tuitbot_core::config::Config;
use tuitbot_core::llm::LlmProvider;
use tuitbot_core::storage;
use tuitbot_core::storage::DbPool;

use crate::tools::response::{ToolMeta, ToolResponse};

#[derive(Serialize)]
struct HealthStatus {
    database: ComponentStatus,
    llm: ComponentStatus,
}

#[derive(Serialize)]
struct ComponentStatus {
    status: String,
    message: String,
}

/// Check system health: database connectivity and LLM provider status.
pub async fn health_check(
    pool: &DbPool,
    llm_provider: Option<&dyn LlmProvider>,
    config: &Config,
) -> String {
    let start = Instant::now();

    // Check database by running a simple query through the storage layer
    let db_status = match storage::analytics::get_follower_snapshots(pool, 1).await {
        Ok(_) => ComponentStatus {
            status: "ok".to_string(),
            message: "Database is accessible".to_string(),
        },
        Err(e) => ComponentStatus {
            status: "error".to_string(),
            message: format!("Database error: {e}"),
        },
    };

    // Check LLM provider
    let llm_status = match llm_provider {
        Some(provider) => match provider.health_check().await {
            Ok(()) => ComponentStatus {
                status: "ok".to_string(),
                message: format!("LLM provider '{}' is reachable", provider.name()),
            },
            Err(e) => ComponentStatus {
                status: "error".to_string(),
                message: format!("LLM provider '{}' error: {e}", provider.name()),
            },
        },
        None => ComponentStatus {
            status: "not_configured".to_string(),
            message: "No LLM provider configured. Content generation tools will not work."
                .to_string(),
        },
    };

    let out = HealthStatus {
        database: db_status,
        llm: llm_status,
    };

    let elapsed = start.elapsed().as_millis() as u64;
    let meta = ToolMeta::new(elapsed)
        .with_workflow(config.mode.to_string(), config.effective_approval_mode());

    ToolResponse::success(out).with_meta(meta).to_json()
}
