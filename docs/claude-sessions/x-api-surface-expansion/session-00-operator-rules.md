# Session 00: Operator Rules (Run Before Session 01)

Paste this into Claude Code at the start of each session, before the session-specific prompt:

```md
Operating rules for this initiative:

1. Treat this as greenfield work. Backward compatibility is not required.
2. Primary goal: maximum practical X API coverage plus strong agent safety.
3. Never expose arbitrary outbound HTTP. Only approved X hosts are allowed.
4. Tool access control must be enforced by registration (tools not allowed must not be registered).
5. All mutation flows must be idempotent and auditable.
6. Prefer small Rust modules with explicit ownership and testable seams.
7. End every session with:
   - code/docs changes complete
   - tests executed
   - a handoff markdown under `docs/roadmap/x-api-surface-expansion/`

Definition of done for every session:
- Repository builds.
- Relevant tests pass.
- Decisions and tradeoffs are documented with no hidden assumptions.
- Next-session inputs are explicit and actionable.
```
