# Session 02: CLI Broken Pipe Hardening (Immediate Win)

Paste this into a new Claude Code session:

```md
Continue from Session 01 artifacts in `docs/roadmap/x-api-surface-expansion/`.

Mission:
Fix CLI broken-pipe behavior so `tuitbot mcp manifest` and related commands behave like robust Unix tooling when stdout is truncated or piped.

Primary files:
- `crates/tuitbot-cli/src/commands/mcp.rs`
- `crates/tuitbot-cli/src/main.rs`
- any shared output/printing helpers used by CLI commands

Tasks:
1. Reproduce and document the broken pipe case (for example: piping to `head`).
2. Handle SIGPIPE/broken pipe gracefully:
   - no panic
   - clean non-noisy exit behavior
3. Ensure this behavior is consistent for manifest and other stdout-heavy commands.
4. Add regression tests where practical (unit/integration).

Deliverables:
1. Code fix committed in CLI command/output path.
2. Test coverage or deterministic repro script.
3. `docs/roadmap/x-api-surface-expansion/session-02-handoff.md` with:
   - root cause
   - implementation summary
   - test evidence

Quality gates:
- `cargo fmt --all && cargo fmt --all --check`
- `RUSTFLAGS="-D warnings" cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`

Exit criteria:
- Broken pipe no longer panics.
- Behavior is explicitly tested or reproducibly validated.
- Handoff clearly states before/after behavior.
```

