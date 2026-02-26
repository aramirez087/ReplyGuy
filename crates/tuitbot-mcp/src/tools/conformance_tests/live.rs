//! Live conformance tests against real X API sandbox credentials.
//!
//! All tests in this module are `#[ignore]` and require env vars:
//!
//! | Var | Required for | Description |
//! |-----|-------------|-------------|
//! | `TUITBOT_TEST_BEARER_TOKEN` | app-only auth | App-only Bearer token |
//! | `TUITBOT_TEST_USER_ID` | user-context ops | Authenticated user's numeric ID |
//! | `TUITBOT_TEST_KNOWN_TWEET_ID` | read ops | A tweet ID known to exist |
//! | `TUITBOT_TEST_KNOWN_USERNAME` | read ops | A username known to exist |
//!
//! Run with: `cargo test -p tuitbot-mcp live -- --ignored`
//!
//! These tests exercise the full stack: MCP kernel → XApiClient → real HTTP.
//! They are separated from the deterministic mock-based conformance tests
//! and should only run in CI with sandbox credentials or locally by developers
//! who have configured the above env vars.

use std::env;

use serde_json::Value;

use crate::kernel::{engage, read, utils, write};
use crate::provider::x_api::XApiProvider;
use crate::tools::test_mocks::artifacts_dir;

/// Helper: skip test gracefully if required env var is missing.
fn require_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| {
        eprintln!("SKIP: {key} not set");
        String::new()
    })
}

fn has_env(key: &str) -> bool {
    env::var(key).is_ok()
}

/// Validates a JSON response string is a conformant success envelope.
fn assert_live_success(json: &str, label: &str) {
    let parsed: Value =
        serde_json::from_str(json).unwrap_or_else(|e| panic!("{label}: invalid JSON: {e}"));
    assert!(
        parsed["success"].as_bool().unwrap_or(false),
        "{label}: expected success=true, got: {json}"
    );
    assert!(parsed.get("data").is_some(), "{label}: missing 'data'");
    assert!(parsed.get("meta").is_some(), "{label}: missing 'meta'");
    assert_eq!(
        parsed["meta"]["tool_version"], "1.0",
        "{label}: tool_version mismatch"
    );
}

/// Validates a JSON response string is a conformant error envelope.
fn assert_live_error(json: &str, label: &str) {
    let parsed: Value =
        serde_json::from_str(json).unwrap_or_else(|e| panic!("{label}: invalid JSON: {e}"));
    assert!(
        !parsed["success"].as_bool().unwrap_or(true),
        "{label}: expected success=false"
    );
    assert!(parsed.get("error").is_some(), "{label}: missing 'error'");
    assert!(
        parsed["error"]["code"].is_string(),
        "{label}: error.code not a string"
    );
    assert!(
        parsed["error"]["retryable"].is_boolean(),
        "{label}: error.retryable not a boolean"
    );
}

// ── Auth Mode: App-Only (Bearer Token) ─────────────────────────────────

/// Build an XApiHttpClient using app-only bearer token auth.
/// Returns None if credentials aren't available.
async fn build_app_only_client() -> Option<tuitbot_core::x_api::XApiHttpClient> {
    let token = require_env("TUITBOT_TEST_BEARER_TOKEN");
    if token.is_empty() {
        return None;
    }
    Some(tuitbot_core::x_api::XApiHttpClient::new(token))
}

#[tokio::test]
#[ignore]
async fn live_app_only_get_user_by_username() {
    let Some(client) = build_app_only_client().await else {
        return;
    };
    let username = require_env("TUITBOT_TEST_KNOWN_USERNAME");
    if username.is_empty() {
        return;
    }
    let provider = XApiProvider::new(&client);
    let json = read::get_user_by_username(&provider, &username).await;
    assert_live_success(&json, "live/app_only/get_user_by_username");
}

#[tokio::test]
#[ignore]
async fn live_app_only_get_tweet() {
    let Some(client) = build_app_only_client().await else {
        return;
    };
    let tweet_id = require_env("TUITBOT_TEST_KNOWN_TWEET_ID");
    if tweet_id.is_empty() {
        return;
    }
    let provider = XApiProvider::new(&client);
    let json = read::get_tweet(&provider, &tweet_id).await;
    assert_live_success(&json, "live/app_only/get_tweet");
}

#[tokio::test]
#[ignore]
async fn live_app_only_search_tweets() {
    let Some(client) = build_app_only_client().await else {
        return;
    };
    let provider = XApiProvider::new(&client);
    let json = read::search_tweets(&provider, "rust lang", 5, None, None).await;
    assert_live_success(&json, "live/app_only/search_tweets");
}

// ── Auth Mode: User OAuth ───────────────────────────────────────────────

#[tokio::test]
#[ignore]
async fn live_user_auth_get_me() {
    let Some(client) = build_app_only_client().await else {
        return;
    };
    let provider = XApiProvider::new(&client);
    let json = utils::get_me(&provider).await;
    // get_me requires user-context auth; with app-only it should error
    // If the token is a user token, it should succeed
    let parsed: Value = serde_json::from_str(&json).unwrap();
    if parsed["success"].as_bool().unwrap_or(false) {
        assert_live_success(&json, "live/user_auth/get_me");
    } else {
        assert_live_error(
            &json,
            "live/user_auth/get_me (expected error with app-only)",
        );
    }
}

#[tokio::test]
#[ignore]
async fn live_user_auth_get_followers() {
    let Some(client) = build_app_only_client().await else {
        return;
    };
    let user_id = require_env("TUITBOT_TEST_USER_ID");
    if user_id.is_empty() {
        return;
    }
    let provider = XApiProvider::new(&client);
    let json = read::get_followers(&provider, &user_id, 5, None).await;
    let parsed: Value = serde_json::from_str(&json).unwrap();
    if parsed["success"].as_bool().unwrap_or(false) {
        assert_live_success(&json, "live/user_auth/get_followers");
    } else {
        assert_live_error(&json, "live/user_auth/get_followers");
    }
}

// ── Safe Write Suite: create + delete tweet ─────────────────────────────

#[tokio::test]
#[ignore]
async fn live_safe_write_tweet_lifecycle() {
    let Some(client) = build_app_only_client().await else {
        return;
    };
    // Post a tweet
    let text = format!(
        "[tuitbot conformance test] {}",
        chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ")
    );
    let post_json = write::post_tweet(&client, &text, None).await;
    let post_parsed: Value = serde_json::from_str(&post_json).unwrap();

    if !post_parsed["success"].as_bool().unwrap_or(false) {
        // May fail with auth or permissions — log and skip
        eprintln!(
            "SKIP: post_tweet failed (auth?): {}",
            post_parsed["error"]["code"]
        );
        assert_live_error(&post_json, "live/write/post_tweet (auth failure)");
        return;
    }

    assert_live_success(&post_json, "live/write/post_tweet");
    let tweet_id = post_parsed["data"]["id"]
        .as_str()
        .expect("posted tweet should have id");

    // Delete the tweet
    let del_json = write::delete_tweet(&client, tweet_id).await;
    assert_live_success(&del_json, "live/write/delete_tweet");
}

// ── Safe Engage Suite: like + unlike ────────────────────────────────────

#[tokio::test]
#[ignore]
async fn live_safe_engage_like_unlike() {
    let Some(client) = build_app_only_client().await else {
        return;
    };
    let user_id = require_env("TUITBOT_TEST_USER_ID");
    let tweet_id = require_env("TUITBOT_TEST_KNOWN_TWEET_ID");
    if user_id.is_empty() || tweet_id.is_empty() {
        return;
    }

    // Like
    let like_json = engage::like_tweet(&client, &user_id, &tweet_id).await;
    let like_parsed: Value = serde_json::from_str(&like_json).unwrap();

    if !like_parsed["success"].as_bool().unwrap_or(false) {
        eprintln!(
            "SKIP: like_tweet failed (auth?): {}",
            like_parsed["error"]["code"]
        );
        assert_live_error(&like_json, "live/engage/like_tweet (auth failure)");
        return;
    }

    assert_live_success(&like_json, "live/engage/like_tweet");

    // Unlike (cleanup)
    let unlike_json = engage::unlike_tweet(&client, &user_id, &tweet_id).await;
    assert_live_success(&unlike_json, "live/engage/unlike_tweet");
}

// ── Pagination Behavior ─────────────────────────────────────────────────

#[tokio::test]
#[ignore]
async fn live_pagination_search() {
    let Some(client) = build_app_only_client().await else {
        return;
    };
    let provider = XApiProvider::new(&client);

    // First page
    let page1 = read::search_tweets(&provider, "rust", 10, None, None).await;
    let p1: Value = serde_json::from_str(&page1).unwrap();

    if !p1["success"].as_bool().unwrap_or(false) {
        eprintln!("SKIP: search_tweets failed: {}", p1["error"]["code"]);
        return;
    }
    assert_live_success(&page1, "live/pagination/page1");

    // Extract next_token for page 2
    let next_token = p1["data"]["meta"]["next_token"].as_str();
    if let Some(token) = next_token {
        let page2 = read::search_tweets(&provider, "rust", 10, None, Some(token)).await;
        assert_live_success(&page2, "live/pagination/page2");
    }
}

// ── Rate-Limit Handling ─────────────────────────────────────────────────

#[tokio::test]
#[ignore]
async fn live_rate_limit_detection() {
    // This test verifies that when we hit a rate limit, the error envelope
    // correctly reports x_rate_limited with retry_after_ms.
    // We can't reliably trigger a rate limit, so we just validate that
    // the error path is wired up by checking the envelope structure on
    // any error response.
    let Some(client) = build_app_only_client().await else {
        return;
    };
    let provider = XApiProvider::new(&client);

    // Try to fetch a nonexistent user — should produce an error envelope
    let json = read::get_user_by_username(&provider, "________nonexistent_user_99999").await;
    let parsed: Value = serde_json::from_str(&json).unwrap();

    // Could be success (empty) or error — either way, envelope should be valid
    if parsed["success"].as_bool().unwrap_or(false) {
        assert_live_success(
            &json,
            "live/rate_limit/nonexistent_user (unexpected success)",
        );
    } else {
        assert_live_error(&json, "live/rate_limit/nonexistent_user");
        // Verify error code is a known variant
        let code = parsed["error"]["code"].as_str().unwrap_or("");
        let known_codes = [
            "x_rate_limited",
            "x_auth_expired",
            "x_forbidden",
            "x_network_error",
            "x_not_configured",
            "x_api_error",
        ];
        assert!(known_codes.contains(&code), "Unknown error code: {code}");
    }
}

// ── Aggregate Live Report ───────────────────────────────────────────────

#[tokio::test]
#[ignore]
async fn live_conformance_aggregate_report() {
    if !has_env("TUITBOT_TEST_BEARER_TOKEN") {
        eprintln!("SKIP: no credentials, skipping aggregate report");
        return;
    }

    let client = build_app_only_client().await.unwrap();
    let provider = XApiProvider::new(&client);
    let username = require_env("TUITBOT_TEST_KNOWN_USERNAME");
    let tweet_id = require_env("TUITBOT_TEST_KNOWN_TWEET_ID");

    let mut results: Vec<(&str, &str, bool, String)> = Vec::new();

    // Read endpoints
    if !tweet_id.is_empty() {
        let json = read::get_tweet(&provider, &tweet_id).await;
        let p: Value = serde_json::from_str(&json).unwrap_or_default();
        results.push((
            "get_tweet",
            "read",
            p["success"].as_bool().unwrap_or(false),
            json,
        ));
    }
    if !username.is_empty() {
        let json = read::get_user_by_username(&provider, &username).await;
        let p: Value = serde_json::from_str(&json).unwrap_or_default();
        results.push((
            "get_user_by_username",
            "read",
            p["success"].as_bool().unwrap_or(false),
            json,
        ));
    }
    {
        let json = read::search_tweets(&provider, "rust", 5, None, None).await;
        let p: Value = serde_json::from_str(&json).unwrap_or_default();
        results.push((
            "search_tweets",
            "read",
            p["success"].as_bool().unwrap_or(false),
            json,
        ));
    }
    {
        let json = utils::get_me(&provider).await;
        let p: Value = serde_json::from_str(&json).unwrap_or_default();
        results.push((
            "get_me",
            "read",
            p["success"].as_bool().unwrap_or(false),
            json,
        ));
    }

    // Write report
    let total = results.len();
    let passed = results.iter().filter(|(_, _, ok, _)| *ok).count();
    let mut md = String::from("# Live Conformance Results\n\n");
    md.push_str(&format!(
        "**Generated:** {}\n\n",
        chrono::Utc::now().format("%Y-%m-%d %H:%M UTC")
    ));
    md.push_str(&format!(
        "**Pass rate:** {passed}/{total} ({:.1}%)\n\n",
        if total > 0 {
            passed as f64 / total as f64 * 100.0
        } else {
            0.0
        }
    ));
    md.push_str("| Tool | Category | Result |\n");
    md.push_str("|------|----------|--------|\n");
    for (name, cat, ok, _) in &results {
        md.push_str(&format!(
            "| {} | {} | {} |\n",
            name,
            cat,
            if *ok { "PASS" } else { "FAIL" }
        ));
    }

    let dir = artifacts_dir();
    std::fs::create_dir_all(&dir).expect("create artifacts dir");
    std::fs::write(dir.join("session-09-live-conformance.md"), &md)
        .expect("write live conformance");
}
