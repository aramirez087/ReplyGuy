-- QA gates: persist structured QA artifacts on approval queue and drafts.

ALTER TABLE approval_queue ADD COLUMN qa_report TEXT DEFAULT '{}';
ALTER TABLE approval_queue ADD COLUMN qa_hard_flags TEXT DEFAULT '[]';
ALTER TABLE approval_queue ADD COLUMN qa_soft_flags TEXT DEFAULT '[]';
ALTER TABLE approval_queue ADD COLUMN qa_recommendations TEXT DEFAULT '[]';
ALTER TABLE approval_queue ADD COLUMN qa_score REAL DEFAULT 0;
ALTER TABLE approval_queue ADD COLUMN qa_requires_override INTEGER DEFAULT 0;
ALTER TABLE approval_queue ADD COLUMN qa_override_by TEXT DEFAULT NULL;
ALTER TABLE approval_queue ADD COLUMN qa_override_note TEXT DEFAULT NULL;
ALTER TABLE approval_queue ADD COLUMN qa_override_at TEXT DEFAULT NULL;

ALTER TABLE scheduled_content ADD COLUMN qa_report TEXT DEFAULT '{}';
ALTER TABLE scheduled_content ADD COLUMN qa_hard_flags TEXT DEFAULT '[]';
ALTER TABLE scheduled_content ADD COLUMN qa_soft_flags TEXT DEFAULT '[]';
ALTER TABLE scheduled_content ADD COLUMN qa_recommendations TEXT DEFAULT '[]';
ALTER TABLE scheduled_content ADD COLUMN qa_score REAL DEFAULT 0;
