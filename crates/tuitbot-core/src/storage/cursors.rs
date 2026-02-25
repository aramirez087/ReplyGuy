//! CRUD operations for the key-value `cursors` table.
//!
//! Used by automation loops to persist pagination cursors (e.g., `since_id`)
//! and by the MCP layer to store metadata like the detected API tier.

use super::accounts::DEFAULT_ACCOUNT_ID;
use super::DbPool;
use crate::error::StorageError;

/// Read a cursor value by key for a specific account. Returns `None` if the key does not exist.
pub async fn get_cursor_for(
    pool: &DbPool,
    account_id: &str,
    key: &str,
) -> Result<Option<String>, StorageError> {
    let row: Option<(String,)> =
        sqlx::query_as("SELECT value FROM cursors WHERE account_id = ?1 AND key = ?2")
            .bind(account_id)
            .bind(key)
            .fetch_optional(pool)
            .await
            .map_err(|e| StorageError::Query { source: e })?;
    Ok(row.map(|(v,)| v))
}

/// Read a cursor value by key. Returns `None` if the key does not exist.
pub async fn get_cursor(pool: &DbPool, key: &str) -> Result<Option<String>, StorageError> {
    get_cursor_for(pool, DEFAULT_ACCOUNT_ID, key).await
}

/// Read a cursor value and its `updated_at` timestamp for a specific account. Returns `None` if missing.
pub async fn get_cursor_with_timestamp_for(
    pool: &DbPool,
    account_id: &str,
    key: &str,
) -> Result<Option<(String, String)>, StorageError> {
    let row: Option<(String, String)> =
        sqlx::query_as("SELECT value, updated_at FROM cursors WHERE account_id = ?1 AND key = ?2")
            .bind(account_id)
            .bind(key)
            .fetch_optional(pool)
            .await
            .map_err(|e| StorageError::Query { source: e })?;
    Ok(row)
}

/// Read a cursor value and its `updated_at` timestamp. Returns `None` if missing.
pub async fn get_cursor_with_timestamp(
    pool: &DbPool,
    key: &str,
) -> Result<Option<(String, String)>, StorageError> {
    get_cursor_with_timestamp_for(pool, DEFAULT_ACCOUNT_ID, key).await
}

/// Write a cursor value for a specific account, creating or updating the row atomically.
pub async fn set_cursor_for(
    pool: &DbPool,
    account_id: &str,
    key: &str,
    value: &str,
) -> Result<(), StorageError> {
    sqlx::query(
        "INSERT INTO cursors (account_id, key, value, updated_at) VALUES (?1, ?2, ?3, datetime('now')) \
         ON CONFLICT(account_id, key) DO UPDATE SET value = excluded.value, updated_at = datetime('now')",
    )
    .bind(account_id)
    .bind(key)
    .bind(value)
    .execute(pool)
    .await
    .map_err(|e| StorageError::Query { source: e })?;
    Ok(())
}

/// Write a cursor value, creating or updating the row atomically.
pub async fn set_cursor(pool: &DbPool, key: &str, value: &str) -> Result<(), StorageError> {
    set_cursor_for(pool, DEFAULT_ACCOUNT_ID, key, value).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::init_test_db;

    #[tokio::test]
    async fn get_cursor_returns_none_for_missing_key() {
        let pool = init_test_db().await.expect("init db");
        let val = get_cursor(&pool, "nonexistent").await.expect("get");
        assert_eq!(val, None);
    }

    #[tokio::test]
    async fn set_and_get_cursor() {
        let pool = init_test_db().await.expect("init db");
        set_cursor(&pool, "test_key", "test_value")
            .await
            .expect("set");
        let val = get_cursor(&pool, "test_key").await.expect("get");
        assert_eq!(val, Some("test_value".to_string()));
    }

    #[tokio::test]
    async fn set_cursor_upserts() {
        let pool = init_test_db().await.expect("init db");
        set_cursor(&pool, "key", "v1").await.expect("set 1");
        set_cursor(&pool, "key", "v2").await.expect("set 2");
        let val = get_cursor(&pool, "key").await.expect("get");
        assert_eq!(val, Some("v2".to_string()));
    }
}
