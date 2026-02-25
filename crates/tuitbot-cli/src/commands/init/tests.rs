use super::helpers::{escape_toml, format_toml_array, parse_csv};
use super::render::render_config_toml;
use super::wizard::WizardResult;

#[test]
fn parse_csv_basic() {
    assert_eq!(parse_csv("rust, cli, tools"), vec!["rust", "cli", "tools"]);
}

#[test]
fn parse_csv_trims_and_filters_empty() {
    assert_eq!(parse_csv("  a , , b ,  "), vec!["a", "b"]);
}

#[test]
fn parse_csv_empty_string() {
    assert!(parse_csv("").is_empty());
    assert!(parse_csv("   ").is_empty());
    assert!(parse_csv(",,,").is_empty());
}

#[test]
fn escape_toml_special_chars() {
    assert_eq!(escape_toml(r#"hello "world""#), r#"hello \"world\""#);
    assert_eq!(escape_toml("back\\slash"), "back\\\\slash");
    assert_eq!(escape_toml("line\nbreak"), "line\\nbreak");
    assert_eq!(escape_toml("tab\there"), "tab\\there");
}

#[test]
fn escape_toml_plain_string() {
    assert_eq!(escape_toml("hello world"), "hello world");
}

#[test]
fn format_toml_array_basic() {
    let items = vec!["a".to_string(), "b".to_string()];
    assert_eq!(format_toml_array(&items), r#"["a", "b"]"#);
}

#[test]
fn format_toml_array_escapes() {
    let items = vec!["say \"hi\"".to_string()];
    assert_eq!(format_toml_array(&items), r#"["say \"hi\""]"#);
}

/// Helper to create a default WizardResult for tests.
fn test_wizard_result() -> WizardResult {
    WizardResult {
        client_id: "cid".to_string(),
        client_secret: None,
        product_name: "App".to_string(),
        product_description: "desc".to_string(),
        product_url: None,
        target_audience: "devs".to_string(),
        product_keywords: vec!["test".to_string()],
        industry_topics: vec!["topic".to_string()],
        brand_voice: None,
        reply_style: None,
        content_style: None,
        persona_opinions: vec![],
        persona_experiences: vec![],
        content_pillars: vec![],
        target_accounts: vec![],
        approval_mode: false,
        llm_provider: "ollama".to_string(),
        llm_api_key: None,
        llm_model: "llama3.2".to_string(),
        llm_base_url: None,
        timezone: "UTC".to_string(),
        active_hours_start: 8,
        active_hours_end: 22,
        active_days: vec![
            "Mon".into(),
            "Tue".into(),
            "Wed".into(),
            "Thu".into(),
            "Fri".into(),
            "Sat".into(),
            "Sun".into(),
        ],
    }
}

#[test]
fn render_config_toml_is_valid_toml() {
    let result = WizardResult {
        client_id: "test-client-id".to_string(),
        client_secret: Some("test-secret".to_string()),
        product_name: "TestProduct".to_string(),
        product_description: "A test product for devs".to_string(),
        product_url: Some("https://example.com".to_string()),
        target_audience: "developers".to_string(),
        product_keywords: vec!["rust".to_string(), "cli".to_string()],
        industry_topics: vec!["Rust development".to_string()],
        brand_voice: None,
        reply_style: None,
        content_style: None,
        persona_opinions: vec![],
        persona_experiences: vec![],
        content_pillars: vec![],
        target_accounts: vec![],
        approval_mode: false,
        llm_provider: "openai".to_string(),
        llm_api_key: Some("sk-test-key".to_string()),
        llm_model: "gpt-4o-mini".to_string(),
        llm_base_url: None,
        timezone: "UTC".to_string(),
        active_hours_start: 8,
        active_hours_end: 22,
        active_days: vec![
            "Mon".into(),
            "Tue".into(),
            "Wed".into(),
            "Thu".into(),
            "Fri".into(),
            "Sat".into(),
            "Sun".into(),
        ],
    };

    let toml_str = render_config_toml(&result);

    // Must parse as valid TOML
    let config: tuitbot_core::config::Config =
        toml::from_str(&toml_str).expect("rendered TOML should parse");

    // Roundtrip: verify key fields survive
    assert_eq!(config.x_api.client_id, "test-client-id");
    assert_eq!(config.x_api.client_secret, Some("test-secret".to_string()));
    assert_eq!(config.business.product_name, "TestProduct");
    assert_eq!(
        config.business.product_description,
        "A test product for devs"
    );
    assert_eq!(
        config.business.product_url,
        Some("https://example.com".to_string())
    );
    assert_eq!(config.business.target_audience, "developers");
    assert_eq!(config.business.product_keywords, vec!["rust", "cli"]);
    assert_eq!(config.business.industry_topics, vec!["Rust development"]);
    assert_eq!(config.llm.provider, "openai");
    assert_eq!(config.llm.api_key, Some("sk-test-key".to_string()));
    assert_eq!(config.llm.model, "gpt-4o-mini");
    assert!(config.llm.base_url.is_none());
}

#[test]
fn render_config_toml_ollama_with_base_url() {
    let result = WizardResult {
        llm_base_url: Some("http://localhost:11434/v1".to_string()),
        ..test_wizard_result()
    };

    let toml_str = render_config_toml(&result);
    let config: tuitbot_core::config::Config =
        toml::from_str(&toml_str).expect("rendered TOML should parse");

    assert_eq!(config.llm.provider, "ollama");
    assert!(config.llm.api_key.is_none());
    assert_eq!(
        config.llm.base_url,
        Some("http://localhost:11434/v1".to_string())
    );
    // client_secret should be None (was commented out)
    assert!(config.x_api.client_secret.is_none());
    // product_url should be None (was commented out)
    assert!(config.business.product_url.is_none());
}

#[test]
fn render_config_toml_escapes_special_chars() {
    let result = WizardResult {
        client_id: "id-with-\"quotes\"".to_string(),
        product_name: "My \"App\"".to_string(),
        product_description: "A tool for\\devs".to_string(),
        product_keywords: vec!["say \"hi\"".to_string()],
        ..test_wizard_result()
    };

    let toml_str = render_config_toml(&result);
    let config: tuitbot_core::config::Config =
        toml::from_str(&toml_str).expect("TOML with special chars should parse");

    assert_eq!(config.x_api.client_id, "id-with-\"quotes\"");
    assert_eq!(config.business.product_name, "My \"App\"");
    assert_eq!(config.business.product_description, "A tool for\\devs");
    assert_eq!(config.business.product_keywords, vec!["say \"hi\""]);
}

#[test]
fn render_config_toml_with_brand_voice() {
    let result = WizardResult {
        product_name: "VoiceApp".to_string(),
        brand_voice: Some("Friendly technical expert. Casual, occasionally witty.".to_string()),
        reply_style: Some("Lead with genuine help. Ask follow-up questions.".to_string()),
        content_style: Some("Share practical tips with real examples.".to_string()),
        ..test_wizard_result()
    };

    let toml_str = render_config_toml(&result);
    let config: tuitbot_core::config::Config =
        toml::from_str(&toml_str).expect("rendered TOML should parse");

    assert_eq!(
        config.business.brand_voice,
        Some("Friendly technical expert. Casual, occasionally witty.".to_string())
    );
    assert_eq!(
        config.business.reply_style,
        Some("Lead with genuine help. Ask follow-up questions.".to_string())
    );
    assert_eq!(
        config.business.content_style,
        Some("Share practical tips with real examples.".to_string())
    );
}

#[test]
fn render_config_toml_without_brand_voice() {
    let result = test_wizard_result();

    let toml_str = render_config_toml(&result);
    let config: tuitbot_core::config::Config =
        toml::from_str(&toml_str).expect("rendered TOML should parse");

    // When None, lines are commented out → deserialized as None
    assert!(config.business.brand_voice.is_none());
    assert!(config.business.reply_style.is_none());
    assert!(config.business.content_style.is_none());
}

#[test]
fn render_config_toml_with_persona() {
    let result = WizardResult {
        persona_opinions: vec!["Rust is the future".to_string(), "TDD matters".to_string()],
        persona_experiences: vec!["Built 3 startups".to_string()],
        content_pillars: vec!["Developer tools".to_string(), "Productivity".to_string()],
        ..test_wizard_result()
    };

    let toml_str = render_config_toml(&result);
    let config: tuitbot_core::config::Config =
        toml::from_str(&toml_str).expect("rendered TOML should parse");

    assert_eq!(
        config.business.persona_opinions,
        vec!["Rust is the future", "TDD matters"]
    );
    assert_eq!(
        config.business.persona_experiences,
        vec!["Built 3 startups"]
    );
    assert_eq!(
        config.business.content_pillars,
        vec!["Developer tools", "Productivity"]
    );
}

#[test]
fn render_config_toml_with_targets() {
    let result = WizardResult {
        target_accounts: vec!["elonmusk".to_string(), "levelsio".to_string()],
        ..test_wizard_result()
    };

    let toml_str = render_config_toml(&result);
    let config: tuitbot_core::config::Config =
        toml::from_str(&toml_str).expect("rendered TOML should parse");

    assert_eq!(config.targets.accounts, vec!["elonmusk", "levelsio"]);
}

#[test]
fn render_config_toml_with_approval_mode() {
    let result = WizardResult {
        approval_mode: true,
        ..test_wizard_result()
    };

    let toml_str = render_config_toml(&result);
    let config: tuitbot_core::config::Config =
        toml::from_str(&toml_str).expect("rendered TOML should parse");

    assert!(config.approval_mode);
}

#[test]
fn render_config_toml_updated_defaults() {
    let result = test_wizard_result();

    let toml_str = render_config_toml(&result);
    let config: tuitbot_core::config::Config =
        toml::from_str(&toml_str).expect("rendered TOML should parse");

    assert_eq!(config.limits.max_replies_per_day, 5);
    assert_eq!(config.limits.max_tweets_per_day, 6);
    assert_eq!(config.intervals.content_post_window_seconds, 10800);
    assert_eq!(config.logging.status_interval_seconds, 0);
    assert_eq!(config.limits.max_replies_per_author_per_day, 1);
    assert!((config.limits.product_mention_ratio - 0.2).abs() < f32::EPSILON);
    assert_eq!(
        config.limits.banned_phrases,
        vec!["check out", "you should try", "I recommend", "link in bio"]
    );
}
