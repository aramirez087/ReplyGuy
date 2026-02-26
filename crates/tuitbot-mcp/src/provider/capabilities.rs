//! Provider capability descriptors.
//!
//! [`ProviderCapabilities`] describes what a provider backend can and cannot do,
//! surfaced in `get_capabilities` so agents can plan accordingly.

use serde::Serialize;

use super::ProviderBackend;

/// Describes the capabilities and risk profile of a provider backend.
#[derive(Debug, Serialize)]
pub struct ProviderCapabilities {
    /// Which backend is active.
    pub backend: ProviderBackend,
    /// Whether mutation tools (write/engage/media) are available.
    pub mutations_available: bool,
    /// Risk level: `"standard"` for official API, `"elevated"` for scraper.
    pub risk_level: &'static str,
    /// Confidence in returned data: `"high"` for official API, `"medium"` for scraper.
    pub data_confidence: &'static str,
    /// Methods not supported by this backend.
    pub unsupported_methods: Vec<&'static str>,
    /// Human-readable note about this backend.
    pub note: &'static str,
}

impl ProviderCapabilities {
    /// Capabilities for the official X API backend.
    pub fn x_api() -> Self {
        Self {
            backend: ProviderBackend::XApi,
            mutations_available: true,
            risk_level: "standard",
            data_confidence: "high",
            unsupported_methods: vec![],
            note: "Official X API via OAuth 2.0.",
        }
    }

    /// Capabilities for the scraper backend.
    pub fn scraper(allow_mutations: bool) -> Self {
        Self {
            backend: ProviderBackend::Scraper,
            mutations_available: allow_mutations,
            risk_level: "elevated",
            data_confidence: "medium",
            unsupported_methods: vec![
                "get_user_mentions",
                "get_home_timeline",
                "get_me",
                "get_bookmarks",
            ],
            note: if allow_mutations {
                "Scraper backend with mutations enabled. Elevated risk of account restrictions."
            } else {
                "Scraper backend (read-only). Mutations blocked by default."
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x_api_capabilities() {
        let caps = ProviderCapabilities::x_api();
        assert_eq!(caps.backend, ProviderBackend::XApi);
        assert!(caps.mutations_available);
        assert_eq!(caps.risk_level, "standard");
        assert_eq!(caps.data_confidence, "high");
        assert!(caps.unsupported_methods.is_empty());
    }

    #[test]
    fn scraper_capabilities_no_mutations() {
        let caps = ProviderCapabilities::scraper(false);
        assert_eq!(caps.backend, ProviderBackend::Scraper);
        assert!(!caps.mutations_available);
        assert_eq!(caps.risk_level, "elevated");
        assert_eq!(caps.data_confidence, "medium");
        assert_eq!(caps.unsupported_methods.len(), 4);
        assert!(caps.note.contains("read-only"));
    }

    #[test]
    fn scraper_capabilities_with_mutations() {
        let caps = ProviderCapabilities::scraper(true);
        assert!(caps.mutations_available);
        assert!(caps.note.contains("mutations enabled"));
    }

    #[test]
    fn capabilities_serialize() {
        let caps = ProviderCapabilities::x_api();
        let json = serde_json::to_value(&caps).unwrap();
        assert_eq!(json["backend"], "x_api");
        assert_eq!(json["mutations_available"], true);
    }
}
