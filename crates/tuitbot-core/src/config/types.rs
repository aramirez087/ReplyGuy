//! Configuration section structs and their serde default functions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// X API
// ---------------------------------------------------------------------------

/// X API credentials.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XApiConfig {
    /// OAuth 2.0 client ID.
    #[serde(default)]
    pub client_id: String,

    /// OAuth 2.0 client secret (optional for public clients).
    #[serde(default)]
    pub client_secret: Option<String>,

    /// Provider backend: `"x_api"` (default) or `"scraper"`.
    #[serde(default)]
    pub provider_backend: String,

    /// Whether scraper backend is allowed to perform mutations.
    /// Only meaningful when `provider_backend = "scraper"`. Default: `false`.
    #[serde(default)]
    pub scraper_allow_mutations: bool,
}

// ---------------------------------------------------------------------------
// Auth
// ---------------------------------------------------------------------------

/// Authentication mode and callback settings.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthConfig {
    /// Auth mode: "manual" or "local_callback".
    #[serde(default = "default_auth_mode")]
    pub mode: String,

    /// Host for local callback server.
    #[serde(default = "default_callback_host")]
    pub callback_host: String,

    /// Port for local callback server.
    #[serde(default = "default_callback_port")]
    pub callback_port: u16,
}

// ---------------------------------------------------------------------------
// Business Profile
// ---------------------------------------------------------------------------

/// Business profile for content targeting and keyword matching.
///
/// Fields are grouped into two tiers:
///
/// **Quickstart fields** (required for a working config):
/// - `product_name`, `product_keywords`
///
/// **Optional context** (improve targeting but have sane defaults):
/// - `product_description`, `product_url`, `target_audience`,
///   `competitor_keywords`, `industry_topics`
///
/// **Enrichment fields** (shape voice/persona — unlocked via progressive setup):
/// - `brand_voice`, `reply_style`, `content_style`,
///   `persona_opinions`, `persona_experiences`, `content_pillars`
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BusinessProfile {
    // -- Quickstart fields --
    /// Name of the user's product.
    #[serde(default)]
    pub product_name: String,

    /// Keywords for tweet discovery.
    #[serde(default)]
    pub product_keywords: Vec<String>,

    // -- Optional context --
    /// One-line description of the product.
    #[serde(default)]
    pub product_description: String,

    /// URL to the product website.
    #[serde(default)]
    pub product_url: Option<String>,

    /// Description of the target audience.
    #[serde(default)]
    pub target_audience: String,

    /// Competitor-related keywords for discovery.
    #[serde(default)]
    pub competitor_keywords: Vec<String>,

    /// Topics for content generation. Defaults to `product_keywords` when empty
    /// (see [`Self::effective_industry_topics`]).
    #[serde(default)]
    pub industry_topics: Vec<String>,

    // -- Enrichment fields --
    /// Brand voice / personality description for all generated content.
    #[serde(default)]
    pub brand_voice: Option<String>,

    /// Style guidelines specific to replies.
    #[serde(default)]
    pub reply_style: Option<String>,

    /// Style guidelines specific to original tweets and threads.
    #[serde(default)]
    pub content_style: Option<String>,

    /// Opinions the persona holds (used to add variety to generated content).
    #[serde(default)]
    pub persona_opinions: Vec<String>,

    /// Experiences the persona can reference (keeps content authentic).
    #[serde(default)]
    pub persona_experiences: Vec<String>,

    /// Core content pillars (broad themes the account focuses on).
    #[serde(default)]
    pub content_pillars: Vec<String>,
}

impl BusinessProfile {
    /// Create a quickstart profile with only the required fields.
    ///
    /// Copies `product_keywords` into `industry_topics` so content loops
    /// have topics to work with even without explicit configuration.
    pub fn quickstart(product_name: String, product_keywords: Vec<String>) -> Self {
        Self {
            product_name,
            industry_topics: product_keywords.clone(),
            product_keywords,
            ..Default::default()
        }
    }

    /// Returns the effective industry topics for content generation.
    ///
    /// If `industry_topics` is non-empty, returns it directly.
    /// Otherwise falls back to `product_keywords`, so quickstart users
    /// never need to configure topics separately.
    pub fn effective_industry_topics(&self) -> &[String] {
        if self.industry_topics.is_empty() {
            &self.product_keywords
        } else {
            &self.industry_topics
        }
    }

    /// Returns `true` if any enrichment field has been set.
    ///
    /// Enrichment fields are: `brand_voice`, `reply_style`, `content_style`,
    /// `persona_opinions`, `persona_experiences`, `content_pillars`.
    /// Used by progressive enrichment to decide whether to show setup hints.
    pub fn is_enriched(&self) -> bool {
        self.brand_voice.as_ref().is_some_and(|v| !v.is_empty())
            || self.reply_style.as_ref().is_some_and(|v| !v.is_empty())
            || self.content_style.as_ref().is_some_and(|v| !v.is_empty())
            || !self.persona_opinions.is_empty()
            || !self.persona_experiences.is_empty()
            || !self.content_pillars.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Scoring
// ---------------------------------------------------------------------------

/// Scoring engine weights and threshold.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScoringConfig {
    /// Minimum score (0-100) to trigger a reply.
    #[serde(default = "default_threshold")]
    pub threshold: u32,

    /// Maximum points for keyword relevance.
    #[serde(default = "default_keyword_relevance_max")]
    pub keyword_relevance_max: f32,

    /// Maximum points for author follower count.
    #[serde(default = "default_follower_count_max")]
    pub follower_count_max: f32,

    /// Maximum points for tweet recency.
    #[serde(default = "default_recency_max")]
    pub recency_max: f32,

    /// Maximum points for engagement rate.
    #[serde(default = "default_engagement_rate_max")]
    pub engagement_rate_max: f32,

    /// Maximum points for reply count signal (fewer replies = higher score).
    #[serde(default = "default_reply_count_max")]
    pub reply_count_max: f32,

    /// Maximum points for content type signal (text-only originals score highest).
    #[serde(default = "default_content_type_max")]
    pub content_type_max: f32,
}

// ---------------------------------------------------------------------------
// Limits
// ---------------------------------------------------------------------------

/// Safety limits for API actions.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LimitsConfig {
    /// Maximum replies per day.
    #[serde(default = "default_max_replies_per_day")]
    pub max_replies_per_day: u32,

    /// Maximum original tweets per day.
    #[serde(default = "default_max_tweets_per_day")]
    pub max_tweets_per_day: u32,

    /// Maximum threads per week.
    #[serde(default = "default_max_threads_per_week")]
    pub max_threads_per_week: u32,

    /// Minimum delay between actions in seconds.
    #[serde(default = "default_min_action_delay_seconds")]
    pub min_action_delay_seconds: u64,

    /// Maximum delay between actions in seconds.
    #[serde(default = "default_max_action_delay_seconds")]
    pub max_action_delay_seconds: u64,

    /// Maximum replies to the same author per day.
    #[serde(default = "default_max_replies_per_author_per_day")]
    pub max_replies_per_author_per_day: u32,

    /// Phrases that should never appear in generated replies.
    #[serde(default = "default_banned_phrases")]
    pub banned_phrases: Vec<String>,

    /// Fraction of replies that may mention the product (0.0 - 1.0).
    #[serde(default = "default_product_mention_ratio")]
    pub product_mention_ratio: f32,
}

// ---------------------------------------------------------------------------
// Intervals
// ---------------------------------------------------------------------------

/// Automation interval settings.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntervalsConfig {
    /// Seconds between mention checks.
    #[serde(default = "default_mentions_check_seconds")]
    pub mentions_check_seconds: u64,

    /// Seconds between discovery searches.
    #[serde(default = "default_discovery_search_seconds")]
    pub discovery_search_seconds: u64,

    /// Seconds for content post window.
    #[serde(default = "default_content_post_window_seconds")]
    pub content_post_window_seconds: u64,

    /// Seconds between thread posts.
    #[serde(default = "default_thread_interval_seconds")]
    pub thread_interval_seconds: u64,
}

// ---------------------------------------------------------------------------
// Targets
// ---------------------------------------------------------------------------

/// Target account monitoring configuration.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetsConfig {
    /// Target account usernames to monitor (without @).
    #[serde(default)]
    pub accounts: Vec<String>,

    /// Maximum target account replies per day (separate from general limit).
    #[serde(default = "default_max_target_replies_per_day")]
    pub max_target_replies_per_day: u32,
}

fn default_max_target_replies_per_day() -> u32 {
    3
}

// ---------------------------------------------------------------------------
// LLM
// ---------------------------------------------------------------------------

/// LLM provider configuration.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LlmConfig {
    /// LLM provider name: "openai", "anthropic", or "ollama".
    #[serde(default)]
    pub provider: String,

    /// API key for the LLM provider (not needed for ollama).
    #[serde(default)]
    pub api_key: Option<String>,

    /// Provider-specific model name.
    #[serde(default)]
    pub model: String,

    /// Override URL for custom endpoints.
    #[serde(default)]
    pub base_url: Option<String>,
}

// ---------------------------------------------------------------------------
// Storage
// ---------------------------------------------------------------------------

/// Data storage configuration.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StorageConfig {
    /// Path to the SQLite database file.
    #[serde(default = "default_db_path")]
    pub db_path: String,

    /// Number of days to retain data.
    #[serde(default = "default_retention_days")]
    pub retention_days: u32,
}

// ---------------------------------------------------------------------------
// Logging
// ---------------------------------------------------------------------------

/// Logging and observability settings.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoggingConfig {
    /// Seconds between periodic status summaries (0 = disabled).
    #[serde(default)]
    pub status_interval_seconds: u64,
}

// ---------------------------------------------------------------------------
// Schedule
// ---------------------------------------------------------------------------

/// Active hours schedule configuration.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScheduleConfig {
    /// IANA timezone name (e.g. "America/New_York", "UTC").
    #[serde(default = "default_timezone")]
    pub timezone: String,

    /// Hour of day (0-23) when active posting window starts.
    #[serde(default = "default_active_hours_start")]
    pub active_hours_start: u8,

    /// Hour of day (0-23) when active posting window ends.
    #[serde(default = "default_active_hours_end")]
    pub active_hours_end: u8,

    /// Days of the week when posting is active (e.g. ["Mon", "Tue", ...]).
    #[serde(default = "default_active_days")]
    pub active_days: Vec<String>,

    /// Preferred posting times for tweets (HH:MM in 24h format, in configured timezone).
    /// When set, the content loop posts at these specific times instead of using interval mode.
    /// Use "auto" for research-backed defaults: 09:15, 12:30, 17:00.
    #[serde(default)]
    pub preferred_times: Vec<String>,

    /// Per-day overrides for preferred posting times.
    /// Keys are day abbreviations (Mon-Sun), values are lists of "HH:MM" times.
    /// Days not listed use the base `preferred_times`. Empty list = no posts that day.
    #[serde(default)]
    pub preferred_times_override: HashMap<String, Vec<String>>,

    /// Preferred day for weekly thread posting (Mon-Sun). None = interval mode.
    #[serde(default)]
    pub thread_preferred_day: Option<String>,

    /// Preferred time for weekly thread posting (HH:MM, 24h format).
    #[serde(default = "default_thread_preferred_time")]
    pub thread_preferred_time: String,
}

impl Default for ScheduleConfig {
    fn default() -> Self {
        Self {
            timezone: default_timezone(),
            active_hours_start: default_active_hours_start(),
            active_hours_end: default_active_hours_end(),
            active_days: default_active_days(),
            preferred_times: Vec::new(),
            preferred_times_override: HashMap::new(),
            thread_preferred_day: None,
            thread_preferred_time: default_thread_preferred_time(),
        }
    }
}

// ---------------------------------------------------------------------------
// MCP Policy
// ---------------------------------------------------------------------------

/// MCP mutation policy configuration.
///
/// Controls whether MCP mutation tools (post, reply, like, follow, etc.)
/// are gated by policy checks before execution.
///
/// v2 fields (`template`, `rules`, `rate_limits`) are additive — existing
/// v1 configs deserialize without changes.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct McpPolicyConfig {
    // --- v1 fields (unchanged) ---
    /// Master switch: when false, all mutations are allowed without checks.
    #[serde(default = "default_true")]
    pub enforce_for_mutations: bool,

    /// Tool names that require routing through the approval queue.
    #[serde(default = "default_require_approval_for")]
    pub require_approval_for: Vec<String>,

    /// Tool names that are completely blocked from execution.
    #[serde(default)]
    pub blocked_tools: Vec<String>,

    /// When true, mutations return a dry-run response without executing.
    #[serde(default)]
    pub dry_run_mutations: bool,

    /// Maximum MCP mutations allowed per hour (aggregate across all tools).
    #[serde(default = "default_max_mutations_per_hour")]
    pub max_mutations_per_hour: u32,

    // --- v2 fields ---
    /// Optional named template to apply as the baseline rule set.
    #[serde(default)]
    pub template: Option<crate::mcp_policy::types::PolicyTemplateName>,

    /// Explicit policy rules (user-defined). Evaluated by priority order.
    #[serde(default)]
    pub rules: Vec<crate::mcp_policy::types::PolicyRule>,

    /// Per-dimension rate limits (beyond the global `max_mutations_per_hour`).
    #[serde(default)]
    pub rate_limits: Vec<crate::mcp_policy::types::PolicyRateLimit>,
}

fn default_true() -> bool {
    true
}

fn default_require_approval_for() -> Vec<String> {
    vec![
        "post_tweet".to_string(),
        "reply_to_tweet".to_string(),
        "follow_user".to_string(),
        "like_tweet".to_string(),
    ]
}

fn default_max_mutations_per_hour() -> u32 {
    20
}

// ---------------------------------------------------------------------------
// Circuit Breaker
// ---------------------------------------------------------------------------

/// Circuit breaker configuration for X API rate-limit protection.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CircuitBreakerConfig {
    /// Number of errors within the window to trip the breaker.
    #[serde(default = "default_cb_error_threshold")]
    pub error_threshold: u32,

    /// Sliding window duration in seconds for counting errors.
    #[serde(default = "default_cb_window_seconds")]
    pub window_seconds: u64,

    /// How long (seconds) to stay Open before allowing a probe mutation.
    #[serde(default = "default_cb_cooldown_seconds")]
    pub cooldown_seconds: u64,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            error_threshold: default_cb_error_threshold(),
            window_seconds: default_cb_window_seconds(),
            cooldown_seconds: default_cb_cooldown_seconds(),
        }
    }
}

fn default_cb_error_threshold() -> u32 {
    5
}
fn default_cb_window_seconds() -> u64 {
    300
}
fn default_cb_cooldown_seconds() -> u64 {
    600
}

// ---------------------------------------------------------------------------
// Serde default value functions
// ---------------------------------------------------------------------------

fn default_auth_mode() -> String {
    "manual".to_string()
}
fn default_callback_host() -> String {
    "127.0.0.1".to_string()
}
fn default_callback_port() -> u16 {
    8080
}
fn default_threshold() -> u32 {
    60
}
fn default_keyword_relevance_max() -> f32 {
    25.0
}
fn default_follower_count_max() -> f32 {
    15.0
}
fn default_recency_max() -> f32 {
    10.0
}
fn default_engagement_rate_max() -> f32 {
    15.0
}
fn default_reply_count_max() -> f32 {
    15.0
}
fn default_content_type_max() -> f32 {
    10.0
}
fn default_max_replies_per_day() -> u32 {
    5
}
fn default_max_tweets_per_day() -> u32 {
    6
}
fn default_max_threads_per_week() -> u32 {
    1
}
fn default_min_action_delay_seconds() -> u64 {
    45
}
fn default_max_action_delay_seconds() -> u64 {
    180
}
fn default_mentions_check_seconds() -> u64 {
    300
}
fn default_discovery_search_seconds() -> u64 {
    900
}
fn default_content_post_window_seconds() -> u64 {
    10800
}
fn default_thread_interval_seconds() -> u64 {
    604800
}
fn default_max_replies_per_author_per_day() -> u32 {
    1
}
fn default_banned_phrases() -> Vec<String> {
    vec![
        "check out".to_string(),
        "you should try".to_string(),
        "I recommend".to_string(),
        "link in bio".to_string(),
    ]
}
fn default_product_mention_ratio() -> f32 {
    0.2
}
fn default_db_path() -> String {
    "~/.tuitbot/tuitbot.db".to_string()
}
fn default_retention_days() -> u32 {
    90
}
fn default_timezone() -> String {
    "UTC".to_string()
}
fn default_active_hours_start() -> u8 {
    8
}
fn default_active_hours_end() -> u8 {
    22
}
fn default_active_days() -> Vec<String> {
    vec![
        "Mon".to_string(),
        "Tue".to_string(),
        "Wed".to_string(),
        "Thu".to_string(),
        "Fri".to_string(),
        "Sat".to_string(),
        "Sun".to_string(),
    ]
}
fn default_thread_preferred_time() -> String {
    "10:00".to_string()
}
