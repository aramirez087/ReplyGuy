# Session 00: Operator Rules (Run Before Session 01)

Paste this into Claude Code at the start of each session, before the session-specific prompt:

```md
Operating rules for this initiative:

1. Treat this as greenfield work. Backward compatibility is not required.
2. Optimize for fastest path to first successful user outcome.
3. Keep advanced power features available, but never in the default first-run critical path.
4. Avoid architecture sprawl: prefer small focused modules and clear ownership.
5. End every session with:
   - code changes complete
   - tests executed
   - explicit handoff markdown written under `docs/roadmap/init-simplification/`

Definition of done for every session:
- The repository builds.
- Relevant tests pass.
- Decisions are documented with no hidden assumptions.
- Next-session inputs are explicit.
```
