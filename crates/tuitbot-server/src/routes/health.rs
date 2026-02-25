//! Health check endpoints.

use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};

use crate::state::AppState;

/// `GET /api/health` — liveness probe (no auth required).
pub async fn health() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

/// `GET /api/health/detailed` — deep health check (requires auth).
pub async fn health_detailed(State(state): State<Arc<AppState>>) -> Json<Value> {
    // Database health
    let db_health = tuitbot_core::storage::health::check_db_health(&state.db).await;
    let db_healthy = db_health.reachable && db_health.wal_mode;

    // Runtime status
    let runtime_guard = state.runtime.lock().await;
    let runtime_running = runtime_guard.is_some();
    let runtime_tasks = runtime_guard.as_ref().map(|r| r.task_count()).unwrap_or(0);
    drop(runtime_guard);

    // Circuit breaker
    let (cb_state, cb_error_count, cb_cooldown) = if let Some(ref cb) = state.circuit_breaker {
        let s = cb.state().await;
        let count = cb.error_count().await;
        let cooldown = cb.cooldown_remaining_seconds().await;
        (s.to_string(), count, cooldown)
    } else {
        ("disabled".to_string(), 0, 0)
    };

    // Overall status
    let overall = if !db_health.reachable {
        "unhealthy"
    } else if !db_health.wal_mode || cb_state == "open" {
        "degraded"
    } else {
        "healthy"
    };

    Json(json!({
        "status": overall,
        "version": env!("CARGO_PKG_VERSION"),
        "checks": {
            "database": {
                "healthy": db_healthy,
                "reachable": db_health.reachable,
                "latency_ms": db_health.latency_ms,
                "wal_mode": db_health.wal_mode,
            },
            "runtime": {
                "healthy": runtime_running,
                "running": runtime_running,
                "task_count": runtime_tasks,
            },
            "circuit_breaker": {
                "healthy": cb_state != "open",
                "state": cb_state,
                "error_count": cb_error_count,
                "cooldown_remaining_seconds": cb_cooldown,
            },
        },
    }))
}
