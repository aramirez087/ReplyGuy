# Session 05: Generated Tool Registration and Profile Model

Paste this into a new Claude Code session:

```md
Continue from Session 04 artifacts.

Mission:
Wire generated/universal tools into a strict profile model so capability is enforced by registration, not soft policy checks.

Primary files:
- `crates/tuitbot-mcp/src/server/mod.rs`
- `crates/tuitbot-mcp/src/server/readonly.rs`
- `crates/tuitbot-mcp/src/server/api_readonly.rs`
- `crates/tuitbot-mcp/src/server/workflow.rs`
- `crates/tuitbot-mcp/src/tools/mod.rs`
- `crates/tuitbot-mcp/src/tools/manifest.rs`
- `docs/mcp-reference.md`

Target profile model:
1. `readonly`
2. `api-readonly`
3. `write`
4. `admin` (only when explicitly configured and supported)

Tasks:
1. Implement/update profile routing and tool registration boundaries.
2. Ensure mutation tools are absent from read profiles.
3. Ensure admin/ads tools are hidden unless configuration + credentials allow.
4. Keep manifests profile-specific and machine-verifiable.
5. Preserve clarity between curated workflow tools and generated X surface tools.

Deliverables:
1. Code changes to profile registration and manifest output.
2. Tests validating profile-specific tool visibility.
3. Updated generated manifests committed.
4. `docs/roadmap/x-api-surface-expansion/session-05-handoff.md` with:
   - profile-to-tool mapping table
   - notable policy decisions
   - migration notes for session 06

Quality gates:
- `cargo fmt --all && cargo fmt --all --check`
- `RUSTFLAGS="-D warnings" cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- `bash scripts/generate-mcp-manifests.sh` (or updated equivalent)

Exit criteria:
- Profile isolation is strict and test-backed.
- Tool registration aligns with declared security model.
- Documentation and manifests agree.
```

