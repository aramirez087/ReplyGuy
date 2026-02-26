# Session 02: Architecture and Config Redesign

Paste this into a new Claude Code session:

```md
Continue from Session 01 artifacts in `docs/roadmap/init-simplification/`.

Mission:
Design and implement a clean configuration architecture that supports:
- Minimal Quickstart defaults
- Optional advanced profile enrichment

Important:
- Backward compatibility is not required.
- Prioritize clarity and maintainability over preserving old structures.

Primary files:
- `crates/tuitbot-core/src/config/mod.rs`
- `crates/tuitbot-core/src/config/defaults.rs`
- `config.example.toml`
- `crates/tuitbot-cli/src/commands/init/wizard.rs`
- `crates/tuitbot-cli/src/commands/init/render.rs`

Tasks:
1. Redesign config boundaries so Quickstart requires only essential fields.
2. Move rich profile data (brand voice/persona/archetypes-related inputs) into clearly optional sections.
3. Update validation rules to align with the new contract.
4. Keep the schema easy to reason about and test.

Deliverables:
1. Implement code changes.
2. Add/update tests proving:
   - Minimal quickstart config validates and runs core flows.
   - Advanced fields remain optional and can be added later.
3. Create `docs/roadmap/init-simplification/session-02-handoff.md` with:
   - New config model summary
   - Validation changes
   - Follow-up requirements for CLI init rebuild

Quality gates:
- `cargo fmt --all && cargo fmt --all --check`
- `RUSTFLAGS="-D warnings" cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`

Exit criteria:
- Config model supports progressive setup by design.
- Tests pass.
- Handoff doc is complete and specific.
```
