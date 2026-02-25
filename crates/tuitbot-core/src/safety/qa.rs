//! Deterministic QA evaluator for generated content.
//!
//! This module builds a structured QA report that can be persisted alongside
//! drafts and approval items. It is intentionally rule-based and deterministic
//! so behavior is predictable and testable.

use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::config::{Config, EmojiPolicy, LanguagePolicyMode};

/// Severity used for QA flags.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QaSeverity {
    /// Blocks approval/publish unless explicitly overridden.
    Hard,
    /// Warning-level issue that should be reviewed.
    Soft,
}

/// QA category for scoring and grouping.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum QaCategory {
    /// Language and bilingual policy checks.
    Language,
    /// Brand voice style and glossary checks.
    Brand,
    /// Compliance checks (claims, links, UTM requirements).
    Compliance,
}

/// Structured issue emitted by the evaluator.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct QaFlag {
    /// Stable identifier for the rule that fired.
    pub code: String,
    /// Hard vs soft severity.
    pub severity: QaSeverity,
    /// Category used for score rollups.
    pub category: QaCategory,
    /// Human-readable summary.
    pub message: String,
    /// Optional excerpt/value that triggered the flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<String>,
    /// Optional remediation guidance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<String>,
}

/// Aggregate score rollup for UI display.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct QaScoreSummary {
    /// Overall score in [0, 100].
    pub overall: f32,
    /// Language-policy dimension score in [0, 100].
    pub language: f32,
    /// Brand dimension score in [0, 100].
    pub brand: f32,
    /// Compliance dimension score in [0, 100].
    pub compliance: f32,
}

/// Captures language detection context.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct QaLanguages {
    /// Detected source language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Detected output language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    /// Policy-selected target language.
    pub policy_target: String,
}

/// Complete QA report artifact.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct QaReport {
    /// Hard failures requiring override/edit.
    pub hard_flags: Vec<QaFlag>,
    /// Soft warnings for reviewer visibility.
    pub soft_flags: Vec<QaFlag>,
    /// Consolidated remediation recommendations.
    pub recommendations: Vec<String>,
    /// Quality score rollup.
    pub score: QaScoreSummary,
    /// Language metadata used by checks.
    pub languages: QaLanguages,
    /// `true` when hard flags exist.
    pub requires_override: bool,
}

impl Default for QaReport {
    fn default() -> Self {
        Self {
            hard_flags: Vec::new(),
            soft_flags: Vec::new(),
            recommendations: Vec::new(),
            score: QaScoreSummary {
                overall: 100.0,
                language: 100.0,
                brand: 100.0,
                compliance: 100.0,
            },
            languages: QaLanguages {
                source: None,
                output: None,
                policy_target: "en".to_string(),
            },
            requires_override: false,
        }
    }
}

#[derive(Debug, Clone)]
struct LanguageDetection {
    code: String,
    confidence: f32,
}

/// Rule-based QA evaluator.
pub struct QaEvaluator<'a> {
    config: &'a Config,
    similarity_threshold: f64,
    length_warning_buffer: usize,
}

impl<'a> QaEvaluator<'a> {
    /// Create a new evaluator using the provided config.
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            similarity_threshold: 0.8,
            length_warning_buffer: 15,
        }
    }

    /// Evaluate generated content against policy.
    ///
    /// `recent_outputs` is optional context for similarity warnings.
    pub fn evaluate(
        &self,
        source_text: &str,
        generated_text: &str,
        recent_outputs: &[String],
    ) -> QaReport {
        let source_lang = detect_language(source_text);
        let output_lang = detect_language(generated_text);
        let policy_target = self.resolve_target_language(source_lang.as_ref());

        let mut hard_flags = Vec::new();
        let mut soft_flags = Vec::new();

        self.evaluate_language_policy(
            source_lang.as_ref(),
            output_lang.as_ref(),
            &policy_target,
            &mut hard_flags,
            &mut soft_flags,
        );
        self.evaluate_glossary(
            source_text,
            generated_text,
            &mut hard_flags,
            &mut soft_flags,
        );
        self.evaluate_forbidden_terms(generated_text, &mut hard_flags);
        self.evaluate_claims(generated_text, &mut hard_flags);
        self.evaluate_links(generated_text, &mut hard_flags, &mut soft_flags);
        self.evaluate_length_and_emoji(generated_text, &mut soft_flags);
        self.evaluate_similarity(generated_text, recent_outputs, &mut soft_flags);

        let recommendations = collect_recommendations(&hard_flags, &soft_flags);
        let score = score_summary(&hard_flags, &soft_flags);

        QaReport {
            requires_override: !hard_flags.is_empty(),
            languages: QaLanguages {
                source: source_lang.map(|lang| lang.code),
                output: output_lang.map(|lang| lang.code),
                policy_target,
            },
            hard_flags,
            soft_flags,
            recommendations,
            score,
        }
    }

    fn resolve_target_language(&self, source_lang: Option<&LanguageDetection>) -> String {
        let supported: HashSet<String> = self
            .config
            .language_policy
            .supported_languages
            .iter()
            .map(|lang| normalize_language_code(lang))
            .collect();

        let default_lang =
            normalize_language_code(&self.config.language_policy.default_reply_language);
        match self.config.language_policy.mode {
            LanguagePolicyMode::FixedDefault => default_lang,
            LanguagePolicyMode::MatchSource => {
                if let Some(source_lang) = source_lang {
                    if supported.contains(&source_lang.code) {
                        return source_lang.code.clone();
                    }
                }
                default_lang
            }
        }
    }

    fn evaluate_language_policy(
        &self,
        source_lang: Option<&LanguageDetection>,
        output_lang: Option<&LanguageDetection>,
        policy_target: &str,
        hard_flags: &mut Vec<QaFlag>,
        soft_flags: &mut Vec<QaFlag>,
    ) {
        if let (LanguagePolicyMode::MatchSource, Some(source_lang)) =
            (&self.config.language_policy.mode, source_lang)
        {
            let supported: HashSet<String> = self
                .config
                .language_policy
                .supported_languages
                .iter()
                .map(|lang| normalize_language_code(lang))
                .collect();
            if !supported.contains(&source_lang.code) {
                soft_flags.push(QaFlag {
                    code: "source_language_not_supported".to_string(),
                    severity: QaSeverity::Soft,
                    category: QaCategory::Language,
                    message: format!(
                        "Source language '{}' is unsupported; defaulting to '{}'",
                        source_lang.code, policy_target
                    ),
                    evidence: Some(source_lang.code.clone()),
                    suggestion: Some(
                        "Add this language to supported_languages or keep default fallback"
                            .to_string(),
                    ),
                });
            }
        }

        match output_lang {
            Some(output_lang) => {
                if output_lang.code != policy_target {
                    if output_lang.confidence >= 0.6 {
                        hard_flags.push(QaFlag {
                            code: "language_mismatch".to_string(),
                            severity: QaSeverity::Hard,
                            category: QaCategory::Language,
                            message: format!(
                                "Output language '{}' does not match policy target '{}'",
                                output_lang.code, policy_target
                            ),
                            evidence: Some(output_lang.code.clone()),
                            suggestion: Some(format!(
                                "Regenerate in '{}' or switch language policy mode",
                                policy_target
                            )),
                        });
                    } else {
                        soft_flags.push(QaFlag {
                            code: "language_mismatch_low_confidence".to_string(),
                            severity: QaSeverity::Soft,
                            category: QaCategory::Language,
                            message: format!(
                                "Likely language mismatch: output '{}' vs target '{}'",
                                output_lang.code, policy_target
                            ),
                            evidence: Some(format!("{:.2}", output_lang.confidence)),
                            suggestion: Some(
                                "Review language manually before approval".to_string(),
                            ),
                        });
                    }
                }
            }
            None => {
                soft_flags.push(QaFlag {
                    code: "output_language_unknown".to_string(),
                    severity: QaSeverity::Soft,
                    category: QaCategory::Language,
                    message: "Could not confidently detect output language".to_string(),
                    evidence: None,
                    suggestion: Some("Review language manually before approval".to_string()),
                });
            }
        }
    }

    fn evaluate_glossary(
        &self,
        source_text: &str,
        generated_text: &str,
        hard_flags: &mut Vec<QaFlag>,
        soft_flags: &mut Vec<QaFlag>,
    ) {
        let source_lower = source_text.to_lowercase();
        let generated_lower = generated_text.to_lowercase();

        for term in &self.config.glossary_terms {
            let canonical = term.term.trim();
            if canonical.is_empty() {
                continue;
            }
            let canonical_lower = canonical.to_lowercase();
            if !source_lower.contains(&canonical_lower) {
                continue;
            }
            if generated_lower.contains(&canonical_lower) {
                continue;
            }

            let alias_hit = term
                .approved_aliases
                .iter()
                .map(|alias| alias.trim().to_lowercase())
                .any(|alias| !alias.is_empty() && generated_lower.contains(&alias));

            if term.preserve_exact {
                hard_flags.push(QaFlag {
                    code: "glossary_term_modified".to_string(),
                    severity: QaSeverity::Hard,
                    category: QaCategory::Brand,
                    message: format!("Glossary term '{}' must remain unchanged", canonical),
                    evidence: Some(canonical.to_string()),
                    suggestion: Some(format!("Restore exact term '{}'", canonical)),
                });
            } else if !alias_hit {
                soft_flags.push(QaFlag {
                    code: "glossary_term_low_confidence".to_string(),
                    severity: QaSeverity::Soft,
                    category: QaCategory::Brand,
                    message: format!(
                        "Glossary term '{}' may have been translated unexpectedly",
                        canonical
                    ),
                    evidence: Some(canonical.to_string()),
                    suggestion: Some(
                        "Use canonical term or one of its approved aliases".to_string(),
                    ),
                });
            }
        }
    }

    fn evaluate_forbidden_terms(&self, generated_text: &str, hard_flags: &mut Vec<QaFlag>) {
        let generated_lower = generated_text.to_lowercase();

        for phrase in &self.config.limits.banned_phrases {
            let needle = phrase.trim().to_lowercase();
            if !needle.is_empty() && generated_lower.contains(&needle) {
                hard_flags.push(QaFlag {
                    code: "banned_phrase".to_string(),
                    severity: QaSeverity::Hard,
                    category: QaCategory::Compliance,
                    message: "Content contains a globally banned phrase".to_string(),
                    evidence: Some(phrase.clone()),
                    suggestion: Some("Rewrite content without banned sales language".to_string()),
                });
            }
        }

        for word in &self.config.brand_voice_profile.forbidden_words {
            let needle = word.trim().to_lowercase();
            if !needle.is_empty() && generated_lower.contains(&needle) {
                hard_flags.push(QaFlag {
                    code: "forbidden_word".to_string(),
                    severity: QaSeverity::Hard,
                    category: QaCategory::Brand,
                    message: "Content contains a forbidden brand term".to_string(),
                    evidence: Some(word.clone()),
                    suggestion: Some("Replace forbidden terms to match brand policy".to_string()),
                });
            }
        }

        for phrase in &self.config.brand_voice_profile.forbidden_phrases {
            let needle = phrase.trim().to_lowercase();
            if !needle.is_empty() && generated_lower.contains(&needle) {
                hard_flags.push(QaFlag {
                    code: "forbidden_phrase".to_string(),
                    severity: QaSeverity::Hard,
                    category: QaCategory::Brand,
                    message: "Content contains a forbidden brand phrase".to_string(),
                    evidence: Some(phrase.clone()),
                    suggestion: Some("Rewrite phrase to align with brand policy".to_string()),
                });
            }
        }
    }

    fn evaluate_claims(&self, generated_text: &str, hard_flags: &mut Vec<QaFlag>) {
        let generated_lower = generated_text.to_lowercase();
        for claim in &self.config.brand_voice_profile.disallowed_claims {
            let needle = claim.trim().to_lowercase();
            if !needle.is_empty() && generated_lower.contains(&needle) {
                hard_flags.push(QaFlag {
                    code: "disallowed_claim".to_string(),
                    severity: QaSeverity::Hard,
                    category: QaCategory::Compliance,
                    message: "Content contains a disallowed claim".to_string(),
                    evidence: Some(claim.clone()),
                    suggestion: Some("Remove or soften claim per compliance policy".to_string()),
                });
            }
        }
    }

    fn evaluate_links(
        &self,
        generated_text: &str,
        hard_flags: &mut Vec<QaFlag>,
        soft_flags: &mut Vec<QaFlag>,
    ) {
        let urls = extract_urls(generated_text);
        if urls.is_empty() {
            return;
        }

        let allowlist: HashSet<String> = self
            .config
            .link_policy
            .allowlist
            .iter()
            .map(|domain| normalize_domain(domain))
            .collect();
        let denylist: HashSet<String> = self
            .config
            .link_policy
            .denylist
            .iter()
            .map(|domain| normalize_domain(domain))
            .collect();

        for url in urls {
            let Some(domain) = extract_domain(&url) else {
                continue;
            };

            if denylist.contains(&domain) {
                hard_flags.push(QaFlag {
                    code: "denied_domain".to_string(),
                    severity: QaSeverity::Hard,
                    category: QaCategory::Compliance,
                    message: format!("URL domain '{}' is denied by policy", domain),
                    evidence: Some(url.clone()),
                    suggestion: Some("Replace link with an approved domain".to_string()),
                });
            }

            if !allowlist.is_empty() && !allowlist.contains(&domain) {
                soft_flags.push(QaFlag {
                    code: "domain_not_in_allowlist".to_string(),
                    severity: QaSeverity::Soft,
                    category: QaCategory::Compliance,
                    message: format!("URL domain '{}' is not in allowlist", domain),
                    evidence: Some(url.clone()),
                    suggestion: Some("Use allowlisted domains when possible".to_string()),
                });
            }

            let query_keys = parse_query_keys(&url);
            for required in &self.config.link_policy.required_utm_params {
                let required = required.trim();
                if required.is_empty() {
                    continue;
                }
                if !query_keys.contains(required) {
                    hard_flags.push(QaFlag {
                        code: "missing_required_utm".to_string(),
                        severity: QaSeverity::Hard,
                        category: QaCategory::Compliance,
                        message: format!("URL is missing required query param '{}'", required),
                        evidence: Some(url.clone()),
                        suggestion: Some(format!(
                            "Add '{}' to all generated URLs per link policy",
                            required
                        )),
                    });
                }
            }
        }
    }

    fn evaluate_length_and_emoji(&self, generated_text: &str, soft_flags: &mut Vec<QaFlag>) {
        let length = generated_text.chars().count();
        if let Some(min_len) = self.config.brand_voice_profile.min_length_chars {
            if length < min_len {
                soft_flags.push(QaFlag {
                    code: "length_below_min".to_string(),
                    severity: QaSeverity::Soft,
                    category: QaCategory::Brand,
                    message: format!(
                        "Content length ({length}) is below min_length_chars ({min_len})"
                    ),
                    evidence: Some(length.to_string()),
                    suggestion: Some("Expand content with one concrete helpful detail".to_string()),
                });
            }
        }

        if let Some(max_len) = self.config.brand_voice_profile.max_length_chars {
            if length > max_len {
                soft_flags.push(QaFlag {
                    code: "length_above_max".to_string(),
                    severity: QaSeverity::Soft,
                    category: QaCategory::Brand,
                    message: format!(
                        "Content length ({length}) exceeds max_length_chars ({max_len})"
                    ),
                    evidence: Some(length.to_string()),
                    suggestion: Some("Trim wording while keeping core intent".to_string()),
                });
            } else if max_len.saturating_sub(length) <= self.length_warning_buffer {
                soft_flags.push(QaFlag {
                    code: "length_near_limit".to_string(),
                    severity: QaSeverity::Soft,
                    category: QaCategory::Brand,
                    message: format!(
                        "Content length ({length}) is near max_length_chars ({max_len})"
                    ),
                    evidence: Some(length.to_string()),
                    suggestion: Some(
                        "Consider shortening for safer delivery and edits".to_string(),
                    ),
                });
            }
        }

        let emoji_count = count_emoji(generated_text);
        match self.config.brand_voice_profile.emoji_policy {
            EmojiPolicy::Allow => {}
            EmojiPolicy::Avoid if emoji_count > 1 => soft_flags.push(QaFlag {
                code: "emoji_policy_avoid".to_string(),
                severity: QaSeverity::Soft,
                category: QaCategory::Brand,
                message: "Emoji usage is higher than avoid policy allows".to_string(),
                evidence: Some(emoji_count.to_string()),
                suggestion: Some("Reduce emoji usage to keep tone professional".to_string()),
            }),
            EmojiPolicy::Forbid if emoji_count > 0 => soft_flags.push(QaFlag {
                code: "emoji_policy_forbid".to_string(),
                severity: QaSeverity::Soft,
                category: QaCategory::Brand,
                message: "Emoji usage conflicts with forbid policy".to_string(),
                evidence: Some(emoji_count.to_string()),
                suggestion: Some("Remove emojis to satisfy brand voice constraints".to_string()),
            }),
            _ => {}
        }
    }

    fn evaluate_similarity(
        &self,
        generated_text: &str,
        recent_outputs: &[String],
        soft_flags: &mut Vec<QaFlag>,
    ) {
        let similarity = max_similarity(generated_text, recent_outputs);
        if similarity >= self.similarity_threshold {
            soft_flags.push(QaFlag {
                code: "high_similarity_recent_content".to_string(),
                severity: QaSeverity::Soft,
                category: QaCategory::Brand,
                message: "Output is very similar to recent content".to_string(),
                evidence: Some(format!("{:.2}", similarity)),
                suggestion: Some("Rewrite with a distinct angle or fresh wording".to_string()),
            });
        }
    }
}

fn normalize_language_code(code: &str) -> String {
    code.trim().to_lowercase()
}

fn normalize_domain(domain: &str) -> String {
    domain.trim().to_lowercase()
}

fn detect_language(text: &str) -> Option<LanguageDetection> {
    let cleaned = text.trim();
    if cleaned.is_empty() {
        return None;
    }

    let spanish_markers: HashSet<&'static str> = [
        "el", "la", "los", "las", "de", "que", "y", "en", "un", "una", "por", "para", "con",
        "como", "pero", "hola", "gracias", "porque", "cuando", "donde",
    ]
    .into_iter()
    .collect();
    let english_markers: HashSet<&'static str> = [
        "the", "and", "for", "with", "this", "that", "you", "your", "is", "are", "to", "from",
        "thanks", "when", "where", "because", "hello", "great", "build", "product",
    ]
    .into_iter()
    .collect();

    let mut es_score = 0usize;
    let mut en_score = 0usize;

    for raw_token in cleaned.split_whitespace() {
        let token = raw_token
            .trim_matches(|c: char| !c.is_alphanumeric() && c != 'ñ' && c != 'Ñ')
            .to_lowercase();
        if token.is_empty() {
            continue;
        }
        if spanish_markers.contains(token.as_str()) {
            es_score += 1;
        }
        if english_markers.contains(token.as_str()) {
            en_score += 1;
        }
    }

    for ch in cleaned.chars() {
        if "áéíóúÁÉÍÓÚñÑ¿¡".contains(ch) {
            es_score += 2;
        }
    }

    if es_score == 0 && en_score == 0 {
        return None;
    }
    if es_score == en_score {
        return None;
    }

    let (code, winner, loser) = if es_score > en_score {
        ("es", es_score, en_score)
    } else {
        ("en", en_score, es_score)
    };

    let confidence = ((winner - loser) as f32 / winner as f32).clamp(0.0, 1.0);
    Some(LanguageDetection {
        code: code.to_string(),
        confidence,
    })
}

fn url_regex() -> &'static Regex {
    static URL_RE: OnceLock<Regex> = OnceLock::new();
    URL_RE.get_or_init(|| Regex::new(r"https?://[^\s<>()]+").expect("valid URL regex"))
}

fn extract_urls(text: &str) -> Vec<String> {
    url_regex()
        .find_iter(text)
        .map(|m| {
            m.as_str()
                .trim_end_matches(|c: char| matches!(c, '.' | ',' | ';' | '!' | '?'))
                .to_string()
        })
        .collect()
}

fn extract_domain(url: &str) -> Option<String> {
    let without_scheme = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);

    let host_port = without_scheme
        .split('/')
        .next()?
        .split('?')
        .next()?
        .split('#')
        .next()?
        .trim();
    if host_port.is_empty() {
        return None;
    }

    let host = host_port
        .split('@')
        .next_back()
        .unwrap_or(host_port)
        .split(':')
        .next()
        .unwrap_or(host_port);
    let normalized = normalize_domain(host);
    if normalized.is_empty() {
        None
    } else {
        Some(normalized)
    }
}

fn parse_query_keys(url: &str) -> HashSet<String> {
    let Some(query) = url.split('?').nth(1) else {
        return HashSet::new();
    };
    let query = query.split('#').next().unwrap_or(query);

    query
        .split('&')
        .filter_map(|kv| {
            let key = kv.split('=').next()?.trim();
            if key.is_empty() {
                None
            } else {
                Some(key.to_string())
            }
        })
        .collect()
}

fn count_emoji(text: &str) -> usize {
    text.chars().filter(|ch| is_emoji(*ch)).count()
}

fn is_emoji(ch: char) -> bool {
    let code = ch as u32;
    (0x1F300..=0x1FAFF).contains(&code) || (0x2600..=0x27BF).contains(&code)
}

fn tokenize(text: &str) -> HashSet<String> {
    text.to_lowercase()
        .split_whitespace()
        .map(|word| {
            word.trim_matches(|c: char| !c.is_ascii_alphanumeric())
                .to_string()
        })
        .filter(|word| !word.is_empty())
        .collect()
}

fn jaccard_similarity(a: &HashSet<String>, b: &HashSet<String>) -> f64 {
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }
    let intersection = a.intersection(b).count() as f64;
    let union = a.union(b).count() as f64;
    if union == 0.0 {
        0.0
    } else {
        intersection / union
    }
}

fn max_similarity(candidate: &str, recent_outputs: &[String]) -> f64 {
    if candidate.trim().is_empty() {
        return 0.0;
    }
    let candidate_tokens = tokenize(candidate);
    let mut max_similarity = 0.0;
    for previous in recent_outputs {
        if previous == candidate {
            return 1.0;
        }
        let score = jaccard_similarity(&candidate_tokens, &tokenize(previous));
        if score > max_similarity {
            max_similarity = score;
        }
    }
    max_similarity
}

fn score_summary(hard_flags: &[QaFlag], soft_flags: &[QaFlag]) -> QaScoreSummary {
    let mut penalties: HashMap<QaCategory, f32> = HashMap::new();
    for flag in hard_flags {
        *penalties.entry(flag.category.clone()).or_insert(0.0) += 35.0;
    }
    for flag in soft_flags {
        *penalties.entry(flag.category.clone()).or_insert(0.0) += 12.0;
    }

    let language = (100.0 - penalties.get(&QaCategory::Language).copied().unwrap_or(0.0)).max(0.0);
    let brand = (100.0 - penalties.get(&QaCategory::Brand).copied().unwrap_or(0.0)).max(0.0);
    let compliance = (100.0
        - penalties
            .get(&QaCategory::Compliance)
            .copied()
            .unwrap_or(0.0))
    .max(0.0);
    let overall = ((language + brand + compliance) / 3.0).clamp(0.0, 100.0);

    QaScoreSummary {
        overall,
        language,
        brand,
        compliance,
    }
}

fn collect_recommendations(hard_flags: &[QaFlag], soft_flags: &[QaFlag]) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut recommendations = Vec::new();

    for flag in hard_flags.iter().chain(soft_flags.iter()) {
        if let Some(suggestion) = &flag.suggestion {
            if seen.insert(suggestion.clone()) {
                recommendations.push(suggestion.clone());
            }
        }
    }
    recommendations
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{
        BrandVoiceProfileConfig, GlossaryTermConfig, LanguagePolicyConfig, LinkPolicyConfig,
    };

    fn base_config() -> Config {
        let mut config = Config::default();
        config.business.product_name = "ReplyGuy".to_string();
        config.business.product_keywords = vec!["marketing".to_string()];
        config.llm.provider = "ollama".to_string();
        config.language_policy = LanguagePolicyConfig {
            supported_languages: vec!["en".to_string(), "es".to_string()],
            default_reply_language: "en".to_string(),
            mode: LanguagePolicyMode::MatchSource,
        };
        config
    }

    #[test]
    fn detects_english_and_spanish_language_policy_alignment() {
        let config = base_config();
        let qa = QaEvaluator::new(&config);

        let report_en = qa.evaluate(
            "The onboarding flow feels slow today",
            "Thanks for sharing this. We are improving onboarding speed this week.",
            &[],
        );
        assert!(report_en
            .hard_flags
            .iter()
            .all(|flag| flag.code != "language_mismatch"));
        assert_eq!(report_en.languages.policy_target, "en");

        let report_es = qa.evaluate(
            "La activacion se siente lenta hoy",
            "Gracias por compartir esto. Estamos mejorando la activacion esta semana.",
            &[],
        );
        assert!(report_es
            .hard_flags
            .iter()
            .all(|flag| flag.code != "language_mismatch"));
        assert_eq!(report_es.languages.policy_target, "es");
    }

    #[test]
    fn glossary_exact_preservation_is_hard_flag() {
        let mut config = base_config();
        config.glossary_terms = vec![GlossaryTermConfig {
            term: "ReplyGuy".to_string(),
            approved_aliases: vec!["Reply Guy".to_string()],
            preserve_exact: true,
        }];
        let qa = QaEvaluator::new(&config);

        let report = qa.evaluate(
            "ReplyGuy saves hours on outbound.",
            "Nuestro asistente ahorra horas en outbound.",
            &[],
        );
        assert!(report
            .hard_flags
            .iter()
            .any(|flag| flag.code == "glossary_term_modified"));
    }

    #[test]
    fn denied_domain_and_missing_utm_are_hard_flags() {
        let mut config = base_config();
        config.link_policy = LinkPolicyConfig {
            allowlist: vec![],
            denylist: vec!["bit.ly".to_string()],
            required_utm_params: vec!["utm_source".to_string(), "utm_campaign".to_string()],
        };

        let qa = QaEvaluator::new(&config);
        let report = qa.evaluate(
            "Can you share a link?",
            "Sure: https://bit.ly/abc123?utm_source=x",
            &[],
        );

        assert!(report
            .hard_flags
            .iter()
            .any(|flag| flag.code == "denied_domain"));
        assert!(report
            .hard_flags
            .iter()
            .any(|flag| flag.code == "missing_required_utm"));
    }

    #[test]
    fn hard_and_soft_flags_are_classified() {
        let mut config = base_config();
        config.brand_voice_profile = BrandVoiceProfileConfig {
            tone: vec![],
            emoji_policy: EmojiPolicy::Forbid,
            min_length_chars: None,
            max_length_chars: Some(80),
            forbidden_words: vec!["guaranteed".to_string()],
            forbidden_phrases: vec![],
            disallowed_claims: vec![],
        };

        let qa = QaEvaluator::new(&config);
        let report = qa.evaluate(
            "How does this work?",
            "Guaranteed growth 😄😄 with our playbook and templates available right now.",
            &[String::from(
                "Guaranteed growth 😄😄 with our playbook and templates available right now.",
            )],
        );

        assert!(report
            .hard_flags
            .iter()
            .any(|flag| flag.code == "forbidden_word"));
        assert!(report
            .soft_flags
            .iter()
            .any(|flag| flag.code == "emoji_policy_forbid"));
        assert!(report
            .soft_flags
            .iter()
            .any(|flag| flag.code == "high_similarity_recent_content"));
        assert!(report.requires_override);
    }

    #[test]
    fn fixed_default_language_mode_sets_policy_target() {
        let mut config = base_config();
        config.language_policy.mode = LanguagePolicyMode::FixedDefault;
        config.language_policy.default_reply_language = "en".to_string();

        let qa = QaEvaluator::new(&config);
        let report = qa.evaluate(
            "Necesito ayuda con activacion",
            "Thanks for sharing this, here is the fastest fix.",
            &[],
        );
        assert_eq!(report.languages.policy_target, "en");
        assert!(report
            .hard_flags
            .iter()
            .all(|flag| flag.code != "language_mismatch"));
    }
}
