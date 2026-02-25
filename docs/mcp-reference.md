# MCP Reference

Tuitbot ships with an MCP server so AI agents can call tools with typed inputs.

## Run MCP server

```bash
tuitbot mcp serve
```

With custom config:

```bash
tuitbot -c /path/to/config.toml mcp serve
```

## Tool categories

- Analytics
- Action log
- Rate limits
- Replies and discovery
- Targets
- Scoring
- Approval queue
- Content generation
- Config and health
- Composer mode

## Claude Code example

```json
{
  "mcpServers": {
    "tuitbot": {
      "command": "tuitbot",
      "args": ["mcp", "serve"]
    }
  }
}
```

## Composer mode tools

These tools support user-driven workflows in Composer mode:

| Tool | Description | Parameters |
|---|---|---|
| `get_mode` | Returns the current operating mode (`autopilot` or `composer`) | None |
| `compose_tweet` | Generate a tweet using AI Assist | `topic` (required), `format` (optional) |
| `get_discovery_feed` | Retrieve scored tweets from the Discovery Feed | `limit` (optional), `min_score` (optional) |
| `suggest_topics` | Get topic suggestions based on profile and performance data | `count` (optional) |

## Response Envelope (v1.0)

Migrated tools wrap their output in a unified JSON envelope. Non-migrated tools
continue to return their original JSON shape. Agents can detect the envelope by
checking for the top-level `"success"` key.

### Example

```json
{
  "success": true,
  "data": {
    "tier": "Basic",
    "can_reply": true
  },
  "meta": {
    "tool_version": "1.0",
    "elapsed_ms": 12,
    "mode": "autopilot",
    "approval_mode": false
  }
}
```

### Field reference

| Field | Type | Description |
|-------|------|-------------|
| `success` | `bool` | Whether the tool call succeeded |
| `data` | `any` | Tool payload (object, array, or null on error) |
| `error` | `object?` | Present only on failure |
| `error.code` | `string` | Machine-readable code (e.g. `db_error`) |
| `error.message` | `string` | Human-readable description |
| `error.retryable` | `bool` | Whether the caller may retry |
| `meta` | `object?` | Execution metadata (optional) |
| `meta.tool_version` | `string` | Envelope schema version |
| `meta.elapsed_ms` | `u64` | Wall-clock execution time in ms |
| `meta.mode` | `string?` | Operating mode (`autopilot` / `composer`) |
| `meta.approval_mode` | `bool?` | Effective approval mode flag |

### Detection strategy

Check for the top-level `"success"` key. If present, the response uses the
envelope schema. If absent, treat it as a legacy (non-migrated) response.

### Migrated tools (v1.0)

| Tool | Error codes |
|------|-------------|
| `get_capabilities` | — (always succeeds) |
| `health_check` | — (always succeeds; degradation in data) |
| `get_stats` | `db_error` |
| `list_pending_approvals` | `db_error` |
| `get_discovery_feed` | `db_error` |

## Operational notes

- MCP server uses same config and DB as CLI.
- Use approval mode if agent autonomy should be constrained. In Composer mode, approval mode is always on.
- Prefer Composer mode for agents that should assist rather than act autonomously.
- Prefer JSON outputs for deterministic agent behavior.
