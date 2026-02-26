# Session 01: Charter and Scope Lock

Paste this into a new Claude Code session:

```md
You are the principal PM + senior Rust engineer for Tuitbot.

Mission:
Lock the technical charter and execution scope for making Tuitbot MCP best-in-class for "do anything the X API can do" while staying safe for autonomous agents.

Critical context:
- This is a new project phase. Backward compatibility is NOT required.
- Existing curated tools are strong; we now need systematic endpoint coverage.

Repository anchors:
- `crates/tuitbot-mcp/src/server/`
- `crates/tuitbot-mcp/src/tools/mod.rs`
- `crates/tuitbot-mcp/src/tools/manifest.rs`
- `crates/tuitbot-mcp/src/provider/x_api.rs`
- `crates/tuitbot-core/src/x_api/`
- `docs/mcp-reference.md`
- `docs/generated/mcp-manifest-full.json`
- `docs/generated/mcp-manifest-readonly.json`
- `docs/generated/mcp-manifest-api-readonly.json`

Tasks:
1. Audit current MCP surface and profile model from code + manifests.
2. Define target architecture with two layers:
   - curated workflow/control-plane tools
   - universal + generated X API surface tools
3. Define hard safety constraints:
   - host allowlist
   - SSRF prevention
   - restricted header policy
4. Define quality metrics:
   - endpoint coverage ratio
   - mutation idempotency coverage
   - conformance test pass criteria
5. Lock sequencing assumptions for sessions 02-11.

Deliverables:
1. Create `docs/roadmap/x-api-surface-expansion/charter.md` containing:
   - Problem statement
   - Goals and non-goals
   - Architecture overview
   - Coverage and reliability KPIs
   - Explicit scope for Ads and DMs
2. Create `docs/roadmap/x-api-surface-expansion/session-01-handoff.md` containing:
   - Risk register
   - Decision log
   - Session-by-session implementation backlog

Constraints:
- This session is docs/decision heavy; avoid large code edits.
- No vague placeholders; no unresolved "TBD".

Exit criteria:
- Both docs exist and are internally consistent.
- Session 02 starts from concrete, testable requirements.
```

