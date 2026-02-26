//! Boundary tests: verify structural isolation between profiles.
//!
//! These tests enforce that the read-only profile servers never import
//! mutation modules, and that the manifest Lane/Profile assignments are correct.

#[cfg(test)]
mod tests {
    use crate::tools::manifest::{generate_manifest, Lane, Profile};
    use std::collections::HashSet;

    // ── Source-level isolation ───────────────────────────────────────

    /// The readonly server source must not reference any workflow or mutation modules.
    #[test]
    fn readonly_server_does_not_import_workflow_modules() {
        let source = include_str!("../server/readonly.rs");

        let forbidden = [
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

        for module in &forbidden {
            assert!(
                !source.contains(module),
                "readonly.rs imports workflow module: {module}"
            );
        }
    }

    /// The api-readonly server source must not reference any workflow or mutation modules.
    #[test]
    fn api_readonly_server_does_not_import_workflow_modules() {
        let source = include_str!("../server/api_readonly.rs");

        let forbidden = [
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

        for module in &forbidden {
            assert!(
                !source.contains(module),
                "api_readonly.rs imports workflow module: {module}"
            );
        }
    }

    /// readonly.rs must not contain kernel::write, kernel::engage, or kernel::media imports.
    #[test]
    fn readonly_server_no_write_imports() {
        let source = include_str!("../server/readonly.rs");
        for path in ["kernel::write", "kernel::engage", "kernel::media"] {
            assert!(
                !source.contains(path),
                "readonly.rs contains mutation import: {path}"
            );
        }
    }

    /// api_readonly.rs must not contain kernel::write, kernel::engage, or kernel::media imports.
    #[test]
    fn api_readonly_server_no_write_imports() {
        let source = include_str!("../server/api_readonly.rs");
        for path in ["kernel::write", "kernel::engage", "kernel::media"] {
            assert!(
                !source.contains(path),
                "api_readonly.rs contains mutation import: {path}"
            );
        }
    }

    // ── Lane isolation ──────────────────────────────────────────────

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

    /// Every tool in a read-only profile must have Lane::Shared.
    #[test]
    fn readonly_tools_have_shared_lane() {
        let manifest = generate_manifest();
        for t in &manifest.tools {
            let in_ro = t.profiles.contains(&Profile::Readonly)
                || t.profiles.contains(&Profile::ApiReadonly);
            if in_ro {
                assert_eq!(
                    t.lane,
                    Lane::Shared,
                    "tool {} is in a read-only profile but has lane {:?}",
                    t.name,
                    t.lane
                );
            }
        }
    }

    // ── Profile tool counts ─────────────────────────────────────────

    /// Drift guard: Readonly profile tool count (expect 10).
    #[test]
    fn readonly_profile_tool_count() {
        let manifest = generate_manifest();
        let count = manifest
            .tools
            .iter()
            .filter(|t| t.profiles.contains(&Profile::Readonly))
            .count();
        assert_eq!(
            count, 10,
            "Readonly profile has {count} tools (expected 10)"
        );
    }

    /// Drift guard: ApiReadonly profile tool count (expect 20).
    #[test]
    fn api_readonly_profile_tool_count() {
        let manifest = generate_manifest();
        let count = manifest
            .tools
            .iter()
            .filter(|t| t.profiles.contains(&Profile::ApiReadonly))
            .count();
        assert_eq!(
            count, 20,
            "ApiReadonly profile has {count} tools (expected 20)"
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

    // ── Mutation safety ─────────────────────────────────────────────

    /// No tool with `mutation: true` may appear in the Readonly profile.
    #[test]
    fn readonly_has_no_mutation_tools() {
        let manifest = generate_manifest();
        for t in &manifest.tools {
            if t.profiles.contains(&Profile::Readonly) {
                assert!(
                    !t.mutation,
                    "mutation tool {} found in Readonly profile",
                    t.name
                );
            }
        }
    }

    /// No tool with `mutation: true` may appear in the ApiReadonly profile.
    #[test]
    fn api_readonly_has_no_mutation_tools() {
        let manifest = generate_manifest();
        for t in &manifest.tools {
            if t.profiles.contains(&Profile::ApiReadonly) {
                assert!(
                    !t.mutation,
                    "mutation tool {} found in ApiReadonly profile",
                    t.name
                );
            }
        }
    }

    /// Every Readonly tool name must also exist in ApiReadonly (subset relationship).
    #[test]
    fn readonly_is_subset_of_api_readonly() {
        let manifest = generate_manifest();
        let api_ro_names: HashSet<&str> = manifest
            .tools
            .iter()
            .filter(|t| t.profiles.contains(&Profile::ApiReadonly))
            .map(|t| t.name.as_str())
            .collect();

        for t in &manifest.tools {
            if t.profiles.contains(&Profile::Readonly) {
                assert!(
                    api_ro_names.contains(t.name.as_str()),
                    "Readonly tool {} is not in ApiReadonly profile",
                    t.name
                );
            }
        }
    }

    // ── Regression ──────────────────────────────────────────────────

    // ── Profile manifest contract ───────────────────────────────────

    /// For each profile: tool_count == tools.len().
    #[test]
    fn profile_manifest_count_matches_len() {
        use crate::state::Profile as StateProfile;
        for profile in [
            StateProfile::Full,
            StateProfile::Readonly,
            StateProfile::ApiReadonly,
        ] {
            let m = crate::tools::manifest::generate_profile_manifest(profile);
            assert_eq!(
                m.tool_count,
                m.tools.len(),
                "tool_count mismatch for profile {}: count={}, len={}",
                m.profile,
                m.tool_count,
                m.tools.len()
            );
        }
    }

    /// Readonly and ApiReadonly manifests have zero mutation tools.
    #[test]
    fn profile_manifest_readonly_no_mutations() {
        use crate::state::Profile as StateProfile;
        for profile in [StateProfile::Readonly, StateProfile::ApiReadonly] {
            let m = crate::tools::manifest::generate_profile_manifest(profile);
            for t in &m.tools {
                assert!(
                    !t.mutation,
                    "mutation tool {} found in {} profile manifest",
                    t.name, m.profile
                );
            }
        }
    }

    /// Profile field matches the input profile string.
    #[test]
    fn profile_manifest_field_matches_request() {
        use crate::state::Profile as StateProfile;
        let cases = [
            (StateProfile::Full, "full"),
            (StateProfile::Readonly, "readonly"),
            (StateProfile::ApiReadonly, "api-readonly"),
        ];
        for (profile, expected) in cases {
            let m = crate::tools::manifest::generate_profile_manifest(profile);
            assert_eq!(
                m.profile, expected,
                "profile field mismatch: got {}, expected {}",
                m.profile, expected
            );
        }
    }

    /// Tool names are in ascending alphabetical order.
    #[test]
    fn profile_manifest_tools_sorted() {
        use crate::state::Profile as StateProfile;
        for profile in [
            StateProfile::Full,
            StateProfile::Readonly,
            StateProfile::ApiReadonly,
        ] {
            let m = crate::tools::manifest::generate_profile_manifest(profile);
            let names: Vec<&str> = m.tools.iter().map(|t| t.name.as_str()).collect();
            let mut sorted = names.clone();
            sorted.sort();
            assert_eq!(names, sorted, "tools not sorted for profile {}", m.profile);
        }
    }

    /// Serialize → deserialize → assert equal.
    #[test]
    fn profile_manifest_serde_roundtrip() {
        use crate::state::Profile as StateProfile;
        for profile in [
            StateProfile::Full,
            StateProfile::Readonly,
            StateProfile::ApiReadonly,
        ] {
            let m = crate::tools::manifest::generate_profile_manifest(profile);
            let json = serde_json::to_string_pretty(&m).unwrap();
            let deserialized: crate::tools::manifest::ProfileManifest =
                serde_json::from_str(&json).unwrap();
            assert_eq!(
                m, deserialized,
                "roundtrip failed for profile {}",
                m.profile
            );
        }
    }

    /// Full→Workflow, Readonly→Readonly, ApiReadonly→ApiReadonly.
    #[test]
    fn state_profile_to_manifest_profile_conversion() {
        use crate::state::Profile as StateProfile;
        assert_eq!(Profile::from(StateProfile::Full), Profile::Workflow);
        assert_eq!(Profile::from(StateProfile::Readonly), Profile::Readonly);
        assert_eq!(
            Profile::from(StateProfile::ApiReadonly),
            Profile::ApiReadonly
        );
    }

    /// tuitbot_version contains '.', mcp_schema_version == "1.0".
    #[test]
    fn profile_manifest_version_is_semver() {
        use crate::state::Profile as StateProfile;
        let m = crate::tools::manifest::generate_profile_manifest(StateProfile::Full);
        assert!(
            m.tuitbot_version.contains('.'),
            "tuitbot_version '{}' does not look like semver",
            m.tuitbot_version
        );
        assert_eq!(m.mcp_schema_version, "1.0");
    }

    // ── Regression ──────────────────────────────────────────────────

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
}
