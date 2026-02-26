# Session 03: Universal X Request Layer

Paste this into a new Claude Code session:

```md
Continue from Session 02 artifacts.

Mission:
Implement a universal X API tool layer so MCP can call any authorized X endpoint safely and predictably.

Primary files:
- `crates/tuitbot-mcp/src/provider/x_api.rs`
- `crates/tuitbot-core/src/x_api/client.rs`
- `crates/tuitbot-mcp/src/tools/workflow/x_actions/`
- `crates/tuitbot-mcp/src/tools/mod.rs`
- `crates/tuitbot-mcp/src/contract/`
- `crates/tuitbot-mcp/src/tools/boundary_tests.rs`

Required capability:
- Add `x_request` (or a minimal family: `x_get`, `x_post`, `x_put`, `x_delete`) with:
  - method, path, query, body, restricted headers
  - structured response: status, selected headers, parsed JSON, raw text fallback
  - built-in retry/backoff for 429/5xx
  - pagination helpers (cursor/token)
  - rate-limit introspection and recommended wait metadata

Safety constraints:
1. Hard host allowlist (for example `api.x.com`, `upload.x.com`).
2. Block non-X domains and SSRF vectors.
3. Prevent caller override of auth-critical headers.
4. Enforce path validation to avoid malformed or ambiguous requests.

Deliverables:
1. Working tool implementation integrated into MCP tool catalog.
2. Deterministic error mapping through existing envelope/error-code model.
3. Tests for:
   - host and header guardrails
   - retry/backoff behavior
   - pagination helper behavior
   - JSON and non-JSON response handling
4. `docs/roadmap/x-api-surface-expansion/session-03-handoff.md` with:
   - final API contract
   - security controls
   - unresolved edge cases

Quality gates:
- `cargo fmt --all && cargo fmt --all --check`
- `RUSTFLAGS="-D warnings" cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`

Exit criteria:
- Universal endpoint calls work for authorized X hosts.
- Unsafe request classes are blocked by construction.
- Tests validate reliability and guardrails.
```

