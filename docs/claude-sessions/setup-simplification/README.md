# Setup Simplification Session Pack

This folder contains a sequence of prompts for running focused Claude Code sessions to solve this concern:

- `tuitbot init` currently feels too heavy for users who only want a quick "Hello World" style first run.

Project assumption for this pack:

- Greenfield execution is allowed.
- Backward compatibility is not required.

## End-state target

1. New users can complete a real first run in under 2 minutes.
2. The default setup path asks for only critical inputs.
3. Advanced profile fields (brand voice, persona, archetypes, targeting) are progressively unlocked.
4. Power-user depth still exists, but is no longer mandatory at first-run.

## Session order

1. `session-00-operator-rules.md`
2. `session-01-ux-contract.md`
3. `session-02-architecture-and-config.md`
4. `session-03-cli-init-rebuild.md`
5. `session-04-hello-world-flow.md`
6. `session-05-progressive-enrichment.md`
7. `session-06-testing-and-quality-gates.md`
8. `session-07-docs-and-positioning.md`
9. `session-08-release-readiness.md`

## Operator notes

- Run one session per Claude Code chat.
- Start each session by pasting the prompt from that file.
- Require a working commit at the end of every session.
- Do not start the next session if the current session acceptance criteria are not met.
