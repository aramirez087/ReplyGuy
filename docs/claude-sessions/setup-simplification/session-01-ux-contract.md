# Session 01: UX Contract and Scope Lock

Paste this into a new Claude Code session:

```md
You are the principal PM + senior Rust engineer for Tuitbot.

Mission:
Design the exact product contract for a setup experience that is fast for first-time users but still supports deep configuration later.

Critical context:
- We are solving setup complexity in `tuitbot init`.
- This is a new project phase; backward compatibility is NOT required.
- "Hello World" onboarding must be first-class.

Repository anchors:
- `crates/tuitbot-cli/src/commands/init/`
- `crates/tuitbot-cli/src/commands/mod.rs`
- `crates/tuitbot-core/src/config/mod.rs`
- `README.md`
- `docs/getting-started.md`
- `docs/cli-reference.md`

Tasks:
1. Audit current onboarding friction from code and docs.
2. Define a strict UX contract with two paths:
   - Quickstart (default)
   - Advanced setup
3. Decide max required inputs for Quickstart (hard cap).
4. Define measurable targets:
   - time to first successful run
   - number of required prompts
   - zero-conf defaults coverage
5. Produce a locked architecture decision record for the next sessions.

Deliverables:
1. Create `docs/roadmap/init-simplification/ux-contract.md` containing:
   - Problem statement
   - User archetypes (minimum: "Hello World", "Operator", "Power User")
   - Quickstart vs Advanced boundaries
   - Non-goals
   - Acceptance metrics
2. Create `docs/roadmap/init-simplification/session-01-handoff.md` with:
   - Risks
   - Open decisions resolved
   - Concrete implementation backlog for Session 02

Constraints:
- No code edits yet except lightweight doc scaffolding.
- Keep recommendations directly implementable in Rust CLI.
- Avoid vague language.

Exit criteria:
- Both files exist.
- Session-02 implementation backlog is explicit and sequenced.
- No unresolved "TBD" items remain in the UX contract.
```
