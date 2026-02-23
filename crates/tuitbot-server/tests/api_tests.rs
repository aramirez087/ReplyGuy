//! Integration tests for the tuitbot-server API routes.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;
use tuitbot_core::storage;

use tuitbot_server::state::AppState;

/// Create the test router backed by an in-memory SQLite database.
async fn test_router() -> axum::Router {
    let pool = storage::init_test_db().await.expect("init test db");

    let state = Arc::new(AppState {
        db: pool,
        config_path: std::path::PathBuf::from("/tmp/test-config.toml"),
    });

    tuitbot_server::build_router(state)
}

/// Helper: send a GET request and parse JSON from the response.
async fn get_json(router: axum::Router, path: &str) -> (StatusCode, serde_json::Value) {
    let req = Request::builder()
        .uri(path)
        .body(Body::empty())
        .expect("build request");

    let response = router.oneshot(req).await.expect("send request");
    let status = response.status();
    let body = response.into_body().collect().await.expect("read body");
    let json: serde_json::Value =
        serde_json::from_slice(&body.to_bytes()).expect("parse JSON");

    (status, json)
}

#[tokio::test]
async fn health_returns_ok() {
    let router = test_router().await;
    let (status, body) = get_json(router, "/api/health").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "ok");
    assert!(body["version"].is_string());
}

#[tokio::test]
async fn analytics_followers_returns_array() {
    let router = test_router().await;
    let (status, body) = get_json(router, "/api/analytics/followers").await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.is_array());
}

#[tokio::test]
async fn analytics_performance_returns_object() {
    let router = test_router().await;
    let (status, body) = get_json(router, "/api/analytics/performance").await;

    assert_eq!(status, StatusCode::OK);
    assert!(body["avg_reply_engagement"].is_number());
    assert!(body["avg_tweet_engagement"].is_number());
    assert!(body["measured_replies"].is_number());
    assert!(body["measured_tweets"].is_number());
}

#[tokio::test]
async fn analytics_topics_returns_array() {
    let router = test_router().await;
    let (status, body) = get_json(router, "/api/analytics/topics").await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.is_array());
}

#[tokio::test]
async fn approval_returns_array() {
    let router = test_router().await;
    let (status, body) = get_json(router, "/api/approval").await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.is_array());
}

#[tokio::test]
async fn activity_returns_array() {
    let router = test_router().await;
    let (status, body) = get_json(router, "/api/activity").await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.is_array());
}

#[tokio::test]
async fn replies_returns_array() {
    let router = test_router().await;
    let (status, body) = get_json(router, "/api/replies").await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.is_array());
}

#[tokio::test]
async fn content_tweets_returns_array() {
    let router = test_router().await;
    let (status, body) = get_json(router, "/api/content/tweets").await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.is_array());
}

#[tokio::test]
async fn content_threads_returns_array() {
    let router = test_router().await;
    let (status, body) = get_json(router, "/api/content/threads").await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.is_array());
}

#[tokio::test]
async fn targets_returns_array() {
    let router = test_router().await;
    let (status, body) = get_json(router, "/api/targets").await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.is_array());
}
