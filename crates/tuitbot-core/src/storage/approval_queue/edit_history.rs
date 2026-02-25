//! Edit history tracking for approval queue items.

use crate::error::StorageError;
use crate::storage::DbPool;

/// A single edit history entry for an approval item.
#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct EditHistoryEntry {
    pub id: i64,
    pub approval_id: i64,
    pub editor: String,
    pub field: String,
    pub old_value: String,
    pub new_value: String,
    pub created_at: String,
}

/// Record an edit to an approval item field.
pub async fn record_edit(
    pool: &DbPool,
    approval_id: i64,
    editor: &str,
    field: &str,
    old_value: &str,
    new_value: &str,
) -> Result<i64, StorageError> {
    let result = sqlx::query(
        "INSERT INTO approval_edit_history (approval_id, editor, field, old_value, new_value) \
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(approval_id)
    .bind(editor)
    .bind(field)
    .bind(old_value)
    .bind(new_value)
    .execute(pool)
    .await
    .map_err(|e| StorageError::Query { source: e })?;

    Ok(result.last_insert_rowid())
}

/// Get the edit history for an approval item, ordered by creation time.
pub async fn get_edit_history(
    pool: &DbPool,
    approval_id: i64,
) -> Result<Vec<EditHistoryEntry>, StorageError> {
    sqlx::query_as::<_, EditHistoryEntry>(
        "SELECT id, approval_id, editor, field, old_value, new_value, created_at \
         FROM approval_edit_history WHERE approval_id = ? ORDER BY created_at ASC",
    )
    .bind(approval_id)
    .fetch_all(pool)
    .await
    .map_err(|e| StorageError::Query { source: e })
}
