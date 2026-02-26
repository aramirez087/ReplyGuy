# Session 09: Conformance Harness and Coverage Report

Paste this into a new Claude Code session:

```md
Continue from Session 08 artifacts.

Mission:
Prove quality claims with a conformance harness against a sandbox X account and publish a machine-readable coverage report.

Primary files:
- `crates/tuitbot-mcp/src/tools/conformance_tests/`
- `crates/tuitbot-mcp/src/tools/contract_tests.rs`
- `crates/tuitbot-mcp/src/tools/golden_fixtures/`
- `scripts/` (for harness wrapper)
- `docs/generated/`

Required test coverage:
1. Auth modes:
   - app-only auth
   - user OAuth auth
2. Read endpoint suite
3. Safe write suite on sandbox account:
   - create tweet + delete tweet
   - like + unlike
4. Pagination behavior
5. Rate-limit handling behavior

Tasks:
1. Build/expand harness infrastructure for deterministic execution.
2. Add clear separation between networked conformance tests and local deterministic tests.
3. Generate endpoint coverage report:
   - implemented vs not implemented
   - tier-gated/credential-gated areas
4. Publish report in repo and document how to run.

Deliverables:
1. Conformance harness updates.
2. Coverage report artifacts (markdown + machine-readable format).
3. `docs/roadmap/x-api-surface-expansion/session-09-handoff.md` with:
   - pass/fail summary
   - largest coverage gaps
   - recommended follow-up backlog

Quality gates:
- `cargo fmt --all && cargo fmt --all --check`
- `RUSTFLAGS="-D warnings" cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- run conformance suite against sandbox credentials where available

Exit criteria:
- Coverage and reliability are measurable, not asserted.
- Known unsupported areas are explicit and traceable.
- Harness can be rerun for release gating.
```

