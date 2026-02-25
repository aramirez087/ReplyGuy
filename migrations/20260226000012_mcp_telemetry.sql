-- MCP execution telemetry for observability and eval harness.
-- Records every MCP tool invocation with latency, success, and policy decisions.

CREATE TABLE IF NOT EXISTS mcp_telemetry (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    tool_name       TEXT NOT NULL,
    category        TEXT NOT NULL DEFAULT 'unknown',
    latency_ms      INTEGER NOT NULL,
    success         INTEGER NOT NULL DEFAULT 1,
    error_code      TEXT,
    policy_decision TEXT,
    metadata        TEXT,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_mcp_telemetry_tool_name ON mcp_telemetry(tool_name);
CREATE INDEX IF NOT EXISTS idx_mcp_telemetry_created_at ON mcp_telemetry(created_at);
CREATE INDEX IF NOT EXISTS idx_mcp_telemetry_category ON mcp_telemetry(category);
CREATE INDEX IF NOT EXISTS idx_mcp_telemetry_success ON mcp_telemetry(success);
