//! MCP telemetry tools: get_mcp_tool_metrics, get_mcp_error_breakdown.
//!
//! Time-windowed aggregation tools for observability into MCP tool
//! execution patterns, latency, success rates, and error distribution.

use std::time::Instant;

use tuitbot_core::storage;
use tuitbot_core::storage::DbPool;

use crate::tools::response::{ErrorCode, ToolMeta, ToolResponse};

/// Get time-windowed metrics aggregated per tool.
pub async fn get_mcp_tool_metrics(pool: &DbPool, since_hours: u32) -> String {
    let start = Instant::now();
    let since = chrono::Utc::now() - chrono::Duration::hours(i64::from(since_hours));
    let since_str = since.format("%Y-%m-%dT%H:%M:%SZ").to_string();

    match storage::mcp_telemetry::get_metrics_since(pool, &since_str).await {
        Ok(metrics) => {
            let summary = match storage::mcp_telemetry::get_summary(pool, &since_str).await {
                Ok(s) => serde_json::to_value(s).unwrap_or_default(),
                Err(_) => serde_json::Value::Null,
            };

            let elapsed = start.elapsed().as_millis() as u64;
            ToolResponse::success(serde_json::json!({
                "window_hours": since_hours,
                "since": since_str,
                "summary": summary,
                "tools": metrics,
            }))
            .with_meta(ToolMeta::new(elapsed))
            .to_json()
        }
        Err(e) => {
            let elapsed = start.elapsed().as_millis() as u64;
            ToolResponse::error(ErrorCode::DbError, format!("Failed to fetch metrics: {e}"))
                .with_meta(ToolMeta::new(elapsed))
                .to_json()
        }
    }
}

/// Get error distribution grouped by tool and error code.
pub async fn get_mcp_error_breakdown(pool: &DbPool, since_hours: u32) -> String {
    let start = Instant::now();
    let since = chrono::Utc::now() - chrono::Duration::hours(i64::from(since_hours));
    let since_str = since.format("%Y-%m-%dT%H:%M:%SZ").to_string();

    match storage::mcp_telemetry::get_error_breakdown(pool, &since_str).await {
        Ok(errors) => {
            let total_errors: i64 = errors.iter().map(|e| e.count).sum();
            let elapsed = start.elapsed().as_millis() as u64;
            ToolResponse::success(serde_json::json!({
                "window_hours": since_hours,
                "since": since_str,
                "total_errors": total_errors,
                "breakdown": errors,
            }))
            .with_meta(ToolMeta::new(elapsed))
            .to_json()
        }
        Err(e) => {
            let elapsed = start.elapsed().as_millis() as u64;
            ToolResponse::error(
                ErrorCode::DbError,
                format!("Failed to fetch error breakdown: {e}"),
            )
            .with_meta(ToolMeta::new(elapsed))
            .to_json()
        }
    }
}

/// Record a telemetry entry for an MCP tool invocation.
///
/// Best-effort — failures are silently ignored to avoid disrupting
/// the tool's primary operation.
pub async fn record(
    pool: &DbPool,
    tool_name: &str,
    category: &str,
    latency_ms: u64,
    success: bool,
    error_code: Option<&str>,
    policy_decision: Option<&str>,
) {
    let _ = storage::mcp_telemetry::log_telemetry(
        pool,
        &storage::mcp_telemetry::TelemetryParams {
            tool_name,
            category,
            latency_ms,
            success,
            error_code,
            policy_decision,
            metadata: None,
        },
    )
    .await;
}
