# X API Surface Expansion Session Pack

This folder contains an ordered set of prompts for running focused Claude Code sessions to make Tuitbot MCP best-in-class for maximum practical X API coverage while preserving agent safety and production reliability.

Project assumptions for this pack:

- Greenfield execution is allowed.
- Backward compatibility is not required.
- Rust-first implementation quality is mandatory.

## End-state target

1. Tuitbot supports both:
   - curated workflow/control-plane tools
   - a universal X API access layer for authorized endpoints
2. Coverage scales through an internal spec + generation pipeline, not manual tool curation alone.
3. Mutation safety is strong by construction:
   - profile-based tool registration
   - idempotency for writes
   - auditability and rollback guidance
4. Capability discovery is explicit for token scopes, auth mode, and likely tier constraints.
5. Media upload and thread creation workflows are deterministic and testable.
6. Conformance and coverage are provable via a sandbox test harness and published reports.

## Session order

1. `session-00-operator-rules.md`
2. `session-01-charter-and-scope-lock.md`
3. `session-02-cli-broken-pipe-hardening.md`
4. `session-03-universal-x-request-layer.md`
5. `session-04-spec-pack-and-tool-generation.md`
6. `session-05-generated-tool-registration-and-profiles.md`
7. `session-06-capability-discovery-and-auth-metadata.md`
8. `session-07-media-upload-and-thread-determinism.md`
9. `session-08-idempotency-audit-and-recent-writes.md`
10. `session-09-conformance-harness-and-coverage-report.md`
11. `session-10-admin-ads-dm-boundaries-and-positioning.md`
12. `session-11-release-readiness-go-no-go.md`

## Operator notes

- Run one session per Claude Code chat.
- Start each session by pasting the prompt from that file.
- Require a clean working implementation and explicit handoff doc at the end of every session.
- Do not start the next session if current session exit criteria are not met.
