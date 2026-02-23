//! Health check endpoint.

use axum::Json;
use serde_json::{json, Value};

/// `GET /api/health` — confirms the server is running and the DB is reachable.
pub async fn health() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}
