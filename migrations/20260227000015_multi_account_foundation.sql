-- Multi-account foundation: account registry, role-based access, per-account data isolation.

-- Account registry
CREATE TABLE IF NOT EXISTS accounts (
    id TEXT PRIMARY KEY,
    label TEXT NOT NULL DEFAULT '',
    x_user_id TEXT,
    x_username TEXT,
    config_overrides TEXT NOT NULL DEFAULT '{}',
    token_path TEXT,
    status TEXT NOT NULL DEFAULT 'active',
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Seed default account (backward compat sentinel)
INSERT OR IGNORE INTO accounts (id, label, status)
VALUES ('00000000-0000-0000-0000-000000000000', 'Default', 'active');

-- Role-based access per account
CREATE TABLE IF NOT EXISTS account_roles (
    account_id TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    actor TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'viewer',
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    PRIMARY KEY (account_id, actor)
);

-- Grant admin to all default actors on the default account
INSERT OR IGNORE INTO account_roles (account_id, actor, role)
VALUES ('00000000-0000-0000-0000-000000000000', 'dashboard', 'admin');
INSERT OR IGNORE INTO account_roles (account_id, actor, role)
VALUES ('00000000-0000-0000-0000-000000000000', 'mcp', 'admin');

-- Add account_id to all account-scoped tables.
-- Default value ensures existing rows belong to the default account.

ALTER TABLE discovered_tweets ADD COLUMN account_id TEXT NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000';
ALTER TABLE replies_sent ADD COLUMN account_id TEXT NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000';
ALTER TABLE original_tweets ADD COLUMN account_id TEXT NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000';
ALTER TABLE threads ADD COLUMN account_id TEXT NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000';
ALTER TABLE thread_tweets ADD COLUMN account_id TEXT NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000';
ALTER TABLE action_log ADD COLUMN account_id TEXT NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000';
ALTER TABLE approval_queue ADD COLUMN account_id TEXT NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000';
ALTER TABLE approval_edit_history ADD COLUMN account_id TEXT NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000';
ALTER TABLE scheduled_content ADD COLUMN account_id TEXT NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000';
ALTER TABLE follower_snapshots ADD COLUMN account_id TEXT NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000';
ALTER TABLE reply_performance ADD COLUMN account_id TEXT NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000';
ALTER TABLE tweet_performance ADD COLUMN account_id TEXT NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000';
ALTER TABLE content_scores ADD COLUMN account_id TEXT NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000';
ALTER TABLE llm_usage ADD COLUMN account_id TEXT NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000';
ALTER TABLE x_api_usage ADD COLUMN account_id TEXT NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000';
ALTER TABLE mcp_telemetry ADD COLUMN account_id TEXT NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000';
ALTER TABLE author_interactions ADD COLUMN account_id TEXT NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000';
ALTER TABLE strategy_reports ADD COLUMN account_id TEXT NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000';

-- target_accounts and target_tweets already have account_id (refers to the target X account).
-- Add owner_account_id to distinguish the agent account from the monitored target.
ALTER TABLE target_accounts ADD COLUMN owner_account_id TEXT NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000';
ALTER TABLE target_tweets ADD COLUMN owner_account_id TEXT NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000';

-- Rebuild rate_limits with composite PK (account_id, action_type).
CREATE TABLE rate_limits_new (
    account_id TEXT NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000',
    action_type TEXT NOT NULL,
    request_count INTEGER NOT NULL DEFAULT 0,
    period_start TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    max_requests INTEGER NOT NULL,
    period_seconds INTEGER NOT NULL,
    PRIMARY KEY (account_id, action_type)
);

INSERT INTO rate_limits_new (account_id, action_type, request_count, period_start, max_requests, period_seconds)
SELECT '00000000-0000-0000-0000-000000000000', action_type, request_count, period_start, max_requests, period_seconds
FROM rate_limits;

DROP TABLE rate_limits;
ALTER TABLE rate_limits_new RENAME TO rate_limits;

-- Rebuild cursors with composite PK (account_id, key).
CREATE TABLE cursors_new (
    account_id TEXT NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000',
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (account_id, key)
);

INSERT INTO cursors_new (account_id, key, value, updated_at)
SELECT '00000000-0000-0000-0000-000000000000', key, value, updated_at
FROM cursors;

DROP TABLE cursors;
ALTER TABLE cursors_new RENAME TO cursors;

-- Indexes for high-query tables on account_id
CREATE INDEX IF NOT EXISTS idx_discovered_tweets_account ON discovered_tweets(account_id);
CREATE INDEX IF NOT EXISTS idx_replies_sent_account ON replies_sent(account_id);
CREATE INDEX IF NOT EXISTS idx_original_tweets_account ON original_tweets(account_id);
CREATE INDEX IF NOT EXISTS idx_threads_account ON threads(account_id);
CREATE INDEX IF NOT EXISTS idx_action_log_account ON action_log(account_id);
CREATE INDEX IF NOT EXISTS idx_approval_queue_account ON approval_queue(account_id);
CREATE INDEX IF NOT EXISTS idx_scheduled_content_account ON scheduled_content(account_id);
CREATE INDEX IF NOT EXISTS idx_follower_snapshots_account ON follower_snapshots(account_id);
CREATE INDEX IF NOT EXISTS idx_llm_usage_account ON llm_usage(account_id);
CREATE INDEX IF NOT EXISTS idx_x_api_usage_account ON x_api_usage(account_id);
CREATE INDEX IF NOT EXISTS idx_target_accounts_owner ON target_accounts(owner_account_id);
CREATE INDEX IF NOT EXISTS idx_author_interactions_account ON author_interactions(account_id);
CREATE INDEX IF NOT EXISTS idx_strategy_reports_account ON strategy_reports(account_id);
