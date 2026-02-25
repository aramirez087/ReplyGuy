-- Add review metadata columns
ALTER TABLE approval_queue ADD COLUMN reviewed_by TEXT DEFAULT NULL;
ALTER TABLE approval_queue ADD COLUMN review_notes TEXT DEFAULT NULL;
ALTER TABLE approval_queue ADD COLUMN reason TEXT DEFAULT NULL;
ALTER TABLE approval_queue ADD COLUMN detected_risks TEXT DEFAULT '[]';

-- Edit history table
CREATE TABLE IF NOT EXISTS approval_edit_history (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    approval_id     INTEGER NOT NULL REFERENCES approval_queue(id),
    editor          TEXT NOT NULL DEFAULT 'dashboard',
    field           TEXT NOT NULL DEFAULT 'generated_content',
    old_value       TEXT NOT NULL,
    new_value       TEXT NOT NULL,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);
CREATE INDEX IF NOT EXISTS idx_approval_edit_history_approval_id ON approval_edit_history(approval_id);
CREATE INDEX IF NOT EXISTS idx_approval_edit_history_created_at ON approval_edit_history(created_at);
