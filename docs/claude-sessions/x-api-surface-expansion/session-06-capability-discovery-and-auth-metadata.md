# Session 06: Capability Discovery and Auth Metadata

Paste this into a new Claude Code session:

```md
Continue from Session 05 artifacts.

Mission:
Implement truthful capability discovery so users and agents can see exactly what their credentials can do before calls fail.

Primary files:
- `crates/tuitbot-mcp/src/provider/capabilities.rs`
- `crates/tuitbot-core/src/x_api/scopes.rs`
- `crates/tuitbot-core/src/x_api/tier.rs`
- `crates/tuitbot-core/src/x_api/auth.rs`
- `crates/tuitbot-mcp/src/tools/workflow/capabilities.rs`
- generated tool metadata outputs from Session 04/05

Tasks:
1. Add/expand `x_capabilities` tool to report:
   - endpoint group availability (read/write/media/ads/etc)
   - required scopes and which are missing
   - auth mode requirements (user vs app-only)
   - tier limitations when detectable
2. Add metadata on generated tools:
   - `requires_scope`
   - `requires_user_auth`
   - `requires_elevated_access` (or equivalent)
3. Make capability responses deterministic and easy for agents to reason about.
4. Add clear actionable guidance for failed capability checks.

Deliverables:
1. Capability tool implementation and metadata wiring.
2. Tests for representative scope/tier/auth combinations.
3. `docs/roadmap/x-api-surface-expansion/session-06-handoff.md` with:
   - capability model
   - known unknowns (where X does not expose clear tier signals)
   - caller UX guidance

Quality gates:
- `cargo fmt --all && cargo fmt --all --check`
- `RUSTFLAGS="-D warnings" cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`

Exit criteria:
- Capability truth is visible before risky operations.
- Metadata is available for generated tool planning.
- Failures are reduced from opaque to explainable.
```

