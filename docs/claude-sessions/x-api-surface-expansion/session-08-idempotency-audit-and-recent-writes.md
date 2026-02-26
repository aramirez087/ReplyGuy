# Session 08: Idempotency, Audit Logging, and Recent Writes

Paste this into a new Claude Code session:

```md
Continue from Session 07 artifacts.

Mission:
Add cross-cutting idempotency and auditable mutation history so agent retries do not cause duplicate side effects.

Primary files:
- `crates/tuitbot-mcp/src/tools/idempotency.rs`
- `crates/tuitbot-core/src/storage/action_log.rs`
- `crates/tuitbot-core/src/storage/x_api_usage.rs`
- mutation tool handlers under `crates/tuitbot-mcp/src/tools/workflow/`
- migrations under `migrations/` and `crates/tuitbot-core/migrations/`

Tasks:
1. Enforce idempotency keys on all mutation-capable tools (including universal and generated writes).
2. Add correlation IDs and structured audit records for every mutation attempt/result.
3. Implement query tools:
   - "recent writes"
   - "what happened last"
4. Provide rollback guidance metadata where possible:
   - delete tweet
   - unlike
   - unbookmark
   - other reversible actions

Deliverables:
1. Code + migration updates for idempotency and audit persistence.
2. Tests for duplicate-call safety and audit query correctness.
3. `docs/roadmap/x-api-surface-expansion/session-08-handoff.md` with:
   - idempotency contract
   - audit schema summary
   - rollback guidance matrix

Quality gates:
- `cargo fmt --all && cargo fmt --all --check`
- `RUSTFLAGS="-D warnings" cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`

Exit criteria:
- Duplicate mutation retries are safe.
- Audit data makes incident review straightforward.
- Recent-write introspection is available to agents/operators.
```

