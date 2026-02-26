# Session 04: Spec Pack and Tool Generation Pipeline

Paste this into a new Claude Code session:

```md
Continue from Session 03 artifacts.

Mission:
Make API coverage scalable by introducing an internal X API spec pack and a generator that produces typed MCP tools from spec definitions.

Primary files and anchors:
- `crates/tuitbot-mcp/src/tools/mod.rs`
- `crates/tuitbot-mcp/src/tools/manifest.rs`
- `scripts/generate-mcp-manifests.sh`
- `docs/generated/mcp-manifest-*.json`
- new spec/generator directories as needed under `crates/tuitbot-mcp/`

Tasks:
1. Introduce a versioned internal spec pack (v2 + upload endpoints; include v1.1 only where required).
2. Implement generator pipeline producing typed tool schemas and handlers:
   - naming convention: `x_v2_<group>_<operation>`
   - typed parameters and defaults
   - consistent error mapping
3. Ensure generated artifacts are deterministic and reproducible.
4. Expose version triplet in manifests/metadata:
   - `mcp_schema_version`
   - `x_api_spec_version`
   - `tuitbot_mcp_version`

Deliverables:
1. Spec pack committed in-repo.
2. Generator implementation and integration into build/dev workflow.
3. Generated tool surface wired into manifest generation.
4. `docs/roadmap/x-api-surface-expansion/session-04-handoff.md` with:
   - generation architecture
   - versioning contract
   - how to extend for new endpoints

Quality gates:
- `cargo fmt --all && cargo fmt --all --check`
- `RUSTFLAGS="-D warnings" cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- `bash scripts/generate-mcp-manifests.sh` (or updated equivalent) succeeds

Exit criteria:
- Tool generation is pipeline-driven, not manual-only.
- Versioning and determinism are explicit and testable.
- Manifests reflect generated surface accurately.
```

