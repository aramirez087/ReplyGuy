//! Boundary tests: verify structural isolation between shared and workflow tools.
//!
//! These tests enforce that the API profile server never imports workflow
//! modules, and that the manifest Lane assignments match actual module layout.

#[cfg(test)]
mod tests {
    use crate::tools::manifest::{generate_manifest, Lane, Profile};

    /// The API server source must not reference any workflow module paths.
    #[test]
    fn api_server_does_not_import_workflow_modules() {
        let source = include_str!("../server/api.rs");

        let workflow_modules = [
            "tools::workflow::",
            "tools::analytics::",
            "tools::actions::",
            "tools::approval::",
            "tools::capabilities::",
            "tools::content::",
            "tools::context::",
            "tools::discovery::",
            "tools::health::",
            "tools::policy_gate::",
            "tools::rate_limits::",
            "tools::replies::",
            "tools::targets::",
            "tools::telemetry::",
            "tools::composite::",
            "tools::x_actions::",
        ];

        for module in &workflow_modules {
            assert!(
                !source.contains(module),
                "api.rs imports workflow module: {module}"
            );
        }
    }

    /// Every workflow-only tool (WF profile only) must have Lane::Workflow.
    #[test]
    fn workflow_only_tools_have_workflow_lane() {
        let manifest = generate_manifest();
        for t in &manifest.tools {
            let wf_only = t.profiles == vec![Profile::Workflow];
            if wf_only {
                assert_eq!(
                    t.lane,
                    Lane::Workflow,
                    "tool {} is WF-only but has lane {:?}",
                    t.name,
                    t.lane
                );
            }
        }
    }

    /// Every tool available in the API profile must have Lane::Shared.
    #[test]
    fn shared_tools_have_shared_lane() {
        let manifest = generate_manifest();
        for t in &manifest.tools {
            let has_api = t.profiles.contains(&Profile::Api);
            if has_api {
                assert_eq!(
                    t.lane,
                    Lane::Shared,
                    "tool {} is in API profile but has lane {:?}",
                    t.name,
                    t.lane
                );
            }
        }
    }

    /// Regression: score_tweet is a pure function on &Config, no DB required.
    #[test]
    fn score_tweet_does_not_require_db() {
        let manifest = generate_manifest();
        let score = manifest.tools.iter().find(|t| t.name == "score_tweet");
        let t = score.expect("score_tweet must be in manifest");
        assert!(
            !t.requires_db,
            "score_tweet should not require DB (pure function)"
        );
        assert!(
            !t.possible_error_codes
                .iter()
                .any(|c| c.as_str() == "db_error"),
            "score_tweet should not list DbError"
        );
    }

    /// Drift guard: API profile tool count.
    #[test]
    fn api_profile_tool_count() {
        let manifest = generate_manifest();
        let api_count = manifest
            .tools
            .iter()
            .filter(|t| t.profiles.contains(&Profile::Api))
            .count();
        assert!(
            api_count >= 24 && api_count <= 35,
            "API profile has {api_count} tools (expected 24-35)"
        );
    }

    /// Drift guard: Workflow profile tool count.
    #[test]
    fn workflow_profile_tool_count() {
        let manifest = generate_manifest();
        let wf_count = manifest
            .tools
            .iter()
            .filter(|t| t.profiles.contains(&Profile::Workflow))
            .count();
        assert!(
            wf_count >= 50 && wf_count <= 70,
            "Workflow profile has {wf_count} tools (expected 50-70)"
        );
    }
}
