# Session 05 Handoff: Contract Hardening and Tool Manifest

## What Was Done

Replaced all scattered string-literal error codes with a typed `ErrorCode` enum (27 variants), separated workflow metadata from API metadata, consolidated duplicate error-mapping logic, expanded test coverage, and generated a machine-readable tool manifest.

### ErrorCode Enum
- 27 variants covering X API, database, validation, LLM, media, thread, policy, context, resource, and internal errors
- Centralized `is_retryable()` — retry semantics derived from code, not caller-supplied
- `as_str()`, `Display`, serde round-trip all aligned
- `ALL` constant for enumeration in tests and manifest generation

### Envelope Hardening
- `ToolError.code`: `String` → `ErrorCode` (wire format unchanged)
- `ToolResponse::error()`: simplified from 3 args to 2 (retryable derived)
- `not_configured(what)` removed → typed `llm_not_configured()`, `x_not_configured()`
- `WorkflowContext` struct with `#[serde(flatten)]` for mode/approval_mode
- `with_mode()` renamed to `with_workflow()` across all call sites

### Error Path Consolidation
- `ProviderError::to_triple()` deleted → replaced by `error_code()` + `error_message()`
- `x_error_to_response()` in x_actions deleted (was duplicating provider error logic)
- Single error chain: `XApiError` → `map_x_error()` → `ProviderError` → `provider_error_to_response()`

### Tool Manifest
- `tools/manifest.rs`: 65 tools enumerated with category, mutation flag, dependency requirements, profiles, and possible error codes
- CI-gated snapshot test against `roadmap/artifacts/session-05-tool-manifest.json`
- 16 tool categories: Read, Write, Engage, Media, Analytics, Approval, Content, Discovery, Scoring, Config, Health, Policy, Telemetry, Context, Composite, Meta

## Files Modified/Created

### New Files (5)
- `crates/tuitbot-mcp/src/contract/error_code.rs` — ErrorCode enum (27 variants)
- `crates/tuitbot-mcp/src/tools/manifest.rs` — Tool manifest generation
- `roadmap/artifacts/session-05-tool-manifest.json` — Snapshot artifact
- `roadmap/artifacts/session-05-contract-spec.md` — Contract documentation
- `roadmap/artifacts/session-05-schema-validation-report.md` — Test coverage report

### Core Contract (3)
- `contract/mod.rs` — Added error_code module + re-exports
- `contract/envelope.rs` — Rewritten: ErrorCode integration, WorkflowContext, typed constructors
- `contract/error.rs` — Rewritten: error_code()/error_message() replacing to_triple()

### Tool Files (15)
- `tools/mod.rs` — Added manifest module
- `tools/response.rs` — Added ErrorCode/WorkflowContext re-exports
- `tools/x_actions/mod.rs` — Deleted x_error_to_response, uses provider chain
- `tools/x_actions/read.rs`, `write.rs`, `media.rs` — ErrorCode migration
- `tools/content.rs`, `context.rs`, `policy_gate.rs`, `telemetry.rs` — ErrorCode migration
- `tools/capabilities.rs`, `health.rs` — with_workflow migration
- `tools/actions.rs`, `config.rs`, `targets.rs`, `rate_limits.rs`, `discovery.rs`, `approval.rs`, `analytics.rs`, `replies.rs` — with_workflow migration

### Kernel Files (4)
- `kernel/utils.rs` — ErrorCode::TweetTooLong
- `kernel/write.rs` — ErrorCode::InvalidInput, ThreadPartialFailure
- `kernel/read.rs` — ErrorCode::InvalidInput
- `kernel/media.rs` — ErrorCode::UnsupportedMediaType, FileReadError, MediaUploadError

### Server Files (2)
- `server/workflow.rs` — llm_not_configured(), with_workflow
- `server/api.rs` — with_workflow

### Composite Files (4)
- `composite/find_opportunities.rs` — ErrorCode migration + with_workflow
- `composite/draft_replies.rs` — ErrorCode migration + with_workflow
- `composite/propose_queue.rs` — ErrorCode migration + with_workflow
- `composite/thread_plan.rs` — ErrorCode migration + with_workflow

### Test Files (2)
- `tools/contract_tests.rs` — Expanded: ErrorCode exhaustiveness, error path validation, API field isolation
- `kernel/tests.rs` — Updated XApiError retryable assertion

## Key Design Decisions

1. **ErrorCode is Copy** — no heap data, trivially passable, zero-cost in match arms
2. **Retryable derived from code** — eliminates inconsistency where same code had different retry behavior
3. **Wire format unchanged** — JSON consumers still see `"code": "db_error"`, no external breaking changes
4. **`#[serde(flatten)]` for WorkflowContext** — preserves JSON shape while adding compile-time type safety
5. **XApiError is retryable** — standardized across all tools (was inconsistent before)
6. **Manifest is test-generated** — cannot drift from source code; CI catches any tool addition/removal

## Tool Count

- **Workflow profile**: 64 tools (unchanged)
- **API profile**: 34 tools (unchanged)
- **Unique tools**: 65 (some shared across profiles)
- **Error codes**: 27 variants

## What Session 06 Should Address

1. **Centralized rate-limit handling** — Use `ErrorCode::XRateLimited` + `rate_limit_reset` field for automatic backoff
2. **Adaptive retry strategy** — 429/5xx/network errors with exponential backoff + jitter, leveraging `is_retryable()`
3. **Pagination normalization** — Consistent cursor handling across all list/search tools
4. **Idempotency safeguards** — Dedup guards for mutation tools to prevent double-posts on retry
