# Architecture

## Workspace crates

- `tuitbot-core`: business logic, storage, API integrations, safety.
- `tuitbot-mcp`: MCP tool surface and transport wiring.
- `tuitbot-cli`: command-line UX and runtime entrypoints.

## Storage

- SQLite via SQLx
- Migrations embedded from crate-local migrations directory
- Single-process lock prevents overlapping run/tick instances

## Runtime loops

- discovery
- mentions
- target monitoring
- content posting
- thread publishing
- analytics snapshots

## Design principles

- conservative automation defaults
- explicit approval and guardrails
- deterministic CLI interfaces for scheduler and agent integration
