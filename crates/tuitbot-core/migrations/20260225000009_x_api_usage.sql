-- X API usage tracking — records each API call with endpoint, method, status, and estimated cost.

CREATE TABLE IF NOT EXISTS x_api_usage (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    endpoint TEXT NOT NULL,
    method TEXT NOT NULL,
    status_code INTEGER NOT NULL,
    cost_usd REAL NOT NULL DEFAULT 0.0,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_x_api_usage_created_at ON x_api_usage(created_at);
CREATE INDEX IF NOT EXISTS idx_x_api_usage_endpoint ON x_api_usage(endpoint);
