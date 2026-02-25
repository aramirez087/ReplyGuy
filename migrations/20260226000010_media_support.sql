-- Add media_paths column to approval_queue for storing local media file paths.
-- JSON-encoded array of strings, e.g. '["~/.tuitbot/media/uuid1.jpg"]'
ALTER TABLE approval_queue ADD COLUMN media_paths TEXT DEFAULT '[]';
