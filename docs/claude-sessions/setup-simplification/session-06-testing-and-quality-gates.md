# Session 06: Testing and Quality Gates

Paste this into a new Claude Code session:

```md
Continue from Session 05 artifacts.

Mission:
Harden the new setup architecture with explicit test coverage, reliability checks, and developer-facing quality gates.

Focus:
- Prevent regression back to setup complexity.
- Ensure quickstart remains fast and stable.

Tasks:
1. Add/expand unit and integration tests for:
   - Quickstart mode
   - Advanced mode
   - Progressive enrichment
   - First-success command path
2. Add a lightweight smoke test script for local verification.
3. Define CI expectations that block regressions in prompt count and first-run success behavior.

Deliverables:
1. Test additions in Rust crates.
2. Optional helper script under `scripts/` if useful.
3. `docs/roadmap/init-simplification/session-06-quality-report.md` including:
   - Test matrix
   - Remaining risk list
   - Recommended CI checks

Quality gates:
- `cargo fmt --all && cargo fmt --all --check`
- `RUSTFLAGS="-D warnings" cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`

Exit criteria:
- New flows are covered by deterministic tests.
- Quality report documents residual risk clearly.
- All checks pass.
```
