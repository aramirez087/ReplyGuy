# Session 08 — Handoff

## Completed

1. **Config** — Added `provider_backend` and `scraper_allow_mutations` fields to `XApiConfig`
2. **ErrorCode** — Added `ScraperMutationBlocked` variant (28 total, not retryable, not transient)
3. **ProviderBackend enum** — `XApi`/`Scraper` with serde, Display, Default, parse helper
4. **ProviderCapabilities** — Factory methods for x_api and scraper capability descriptors
5. **ScraperReadProvider** — Stub impl of `SocialReadProvider` (10 stub + 4 auth-gated methods)
6. **ToolMeta enrichment** — `provider_backend` field + `with_provider_backend()` builder
7. **ToolResponse convenience** — `scraper_mutation_blocked()` constructor
8. **Mutation gating (workflow)** — `scraper_mutation_guard()` wired into 14 mutation functions
9. **Mutation gating (API)** — `scraper_mutations_blocked()` wired into 14 API profile methods
10. **inject_provider_backend()** — JSON post-processor for adding provider info to responses
11. **Telemetry** — `record()` accepts optional `provider_backend` parameter
12. **Capabilities** — `get_capabilities` includes `provider` section
13. **Server init logging** — Logs provider backend at startup (info for x_api, warn for scraper)
14. **Manifest** — `ScraperMutationBlocked` added to write, engage, thread, and media error groups
15. **Tests** — Unit tests for ProviderBackend, ProviderCapabilities, ScraperReadProvider, inject_provider_backend

## Files Changed

| File | Action |
|------|--------|
| `tuitbot-core/src/config/mod.rs` | Edit |
| `tuitbot-mcp/src/contract/error_code.rs` | Edit |
| `tuitbot-mcp/src/contract/envelope.rs` | Edit |
| `tuitbot-mcp/src/provider/mod.rs` | Edit |
| `tuitbot-mcp/src/provider/capabilities.rs` | **New** |
| `tuitbot-mcp/src/provider/scraper.rs` | **New** |
| `tuitbot-mcp/src/tools/workflow/x_actions/mod.rs` | Edit |
| `tuitbot-mcp/src/tools/workflow/x_actions/write.rs` | Edit |
| `tuitbot-mcp/src/tools/workflow/x_actions/engage.rs` | Edit |
| `tuitbot-mcp/src/tools/workflow/x_actions/media.rs` | Edit |
| `tuitbot-mcp/src/tools/workflow/capabilities.rs` | Edit |
| `tuitbot-mcp/src/tools/workflow/telemetry.rs` | Edit |
| `tuitbot-mcp/src/tools/manifest.rs` | Edit |
| `tuitbot-mcp/src/server/api.rs` | Edit |
| `tuitbot-mcp/src/lib.rs` | Edit |
| `tuitbot-mcp/src/tools/workflow/composite/*.rs` | Edit (4 files) |
| `tuitbot-mcp/src/tools/workflow/policy_gate.rs` | Edit |

## Session 09 Preview

Session 09 must prove via conformance tests:
- Provider selection is deterministic from config
- Scraper mutations are gated by default across both profiles
- Telemetry records always include provider_backend
- `get_capabilities` correctly reflects active provider
- Swapping provider backend doesn't require contract changes
