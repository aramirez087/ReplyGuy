# Session 05 Contract Specification

## Overview

The MCP tool contract defines the JSON envelope every tool returns. Session 05 upgrades the contract from soft string-based error codes to a typed `ErrorCode` enum, adds clean metadata boundaries between workflow and API profiles, and introduces a machine-readable tool manifest.

## Envelope Schema

```json
{
  "success": true | false,
  "data": <any>,
  "error": {                          // present only when success=false
    "code": "<error_code>",           // ErrorCode enum variant (snake_case)
    "message": "<human-readable>",
    "retryable": true | false,        // derived from error code
    "rate_limit_reset": "<optional>", // ISO-8601 or epoch
    "policy_decision": "<optional>"   // e.g. "denied", "routed_to_approval"
  },
  "meta": {                           // present when tool attaches metadata
    "tool_version": "1.0",
    "elapsed_ms": <u64>,
    "mode": "<optional>",             // workflow profile only
    "approval_mode": <optional bool>  // workflow profile only
  }
}
```

## Error Codes (27 variants)

| Code | Category | Retryable | Description |
|------|----------|-----------|-------------|
| `x_rate_limited` | X API | Yes | Rate limit exceeded |
| `x_auth_expired` | X API | No | OAuth token expired |
| `x_forbidden` | X API | No | Insufficient permissions |
| `x_account_restricted` | X API | No | Account suspended/restricted |
| `x_network_error` | X API | Yes | Network/timeout error |
| `x_not_configured` | X API | No | X client not initialized |
| `x_api_error` | X API | Yes | Generic X API error |
| `db_error` | Database | Yes | Database operation failed |
| `validation_error` | Validation | No | Input validation failed |
| `invalid_input` | Validation | No | Invalid parameter value |
| `tweet_too_long` | Validation | No | Tweet exceeds 280 weighted chars |
| `llm_error` | LLM | Yes | LLM provider call failed |
| `llm_not_configured` | LLM | No | No LLM provider configured |
| `unsupported_media_type` | Media | No | File extension not supported |
| `file_read_error` | Media | No | Could not read media file |
| `media_upload_error` | Media | No | Upload to X API failed |
| `thread_partial_failure` | Thread | Yes | Some tweets posted, then failure |
| `policy_error` | Policy | Yes | Policy evaluation error |
| `policy_denied_blocked` | Policy | No | Blocked by policy rule |
| `policy_denied_rate_limited` | Policy | No | Policy rate limit exceeded |
| `policy_denied_hard_rule` | Policy | No | Hard safety rule violation |
| `policy_denied_user_rule` | Policy | No | User-defined rule violation |
| `context_error` | Context | No | Author context lookup failed |
| `recommendation_error` | Context | No | Engagement recommendation failed |
| `topic_error` | Context | No | Topic analysis failed |
| `not_found` | Resource | No | Requested resource not found |
| `serialization_error` | Internal | No | JSON serialization failed |

## Profile Metadata Boundaries

### API Profile
- `meta.mode` and `meta.approval_mode` are **never present**
- Tools use `ToolMeta::new(elapsed)` only

### Workflow Profile
- `meta.mode` and `meta.approval_mode` are **always present** (via `with_workflow()`)
- Values reflect runtime operating mode (autopilot/composer) and effective approval state

## Retry Semantics

Retryable flag is **derived from the error code** — callers cannot set it independently. This prevents inconsistencies where the same error code reports different retry behavior in different tools.

Retryable codes: `x_rate_limited`, `x_network_error`, `x_api_error`, `db_error`, `llm_error`, `thread_partial_failure`, `policy_error`.

## Tool Manifest

A machine-readable JSON manifest at `roadmap/artifacts/session-05-tool-manifest.json` describes every registered tool with:
- `name`: Tool name as registered in the MCP server
- `category`: Functional category (read, write, engage, media, analytics, etc.)
- `mutation`: Whether the tool performs a write operation
- `requires_x_client`, `requires_llm`, `requires_db`: Dependency flags
- `profiles`: Which profiles (workflow, api) include this tool
- `possible_error_codes`: Error codes the tool may return

The manifest is generated from source code and validated by a snapshot test — CI fails if tools are added/removed without updating the manifest.
