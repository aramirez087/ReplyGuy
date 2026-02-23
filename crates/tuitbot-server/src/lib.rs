//! Tuitbot HTTP API server.
//!
//! Exposes `tuitbot-core`'s storage layer as a read-only REST API.
//! The server owns zero business logic — only routing, serialization,
//! and CORS/tracing middleware.

pub mod error;
pub mod routes;
pub mod state;

use std::sync::Arc;

use axum::routing::get;
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::state::AppState;

/// Build the complete axum router with all API routes and middleware.
pub fn build_router(state: Arc<AppState>) -> Router {
    let api = Router::new()
        .route("/health", get(routes::health::health))
        // Analytics
        .route("/analytics/followers", get(routes::analytics::followers))
        .route(
            "/analytics/performance",
            get(routes::analytics::performance),
        )
        .route("/analytics/topics", get(routes::analytics::topics))
        // Approval
        .route("/approval", get(routes::approval::list_pending))
        // Activity
        .route("/activity", get(routes::activity::list_activity))
        // Replies
        .route("/replies", get(routes::replies::list_replies))
        // Content
        .route("/content/tweets", get(routes::content::list_tweets))
        .route("/content/threads", get(routes::content::list_threads))
        // Targets
        .route("/targets", get(routes::targets::list_targets));

    Router::new()
        .nest("/api", api)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
