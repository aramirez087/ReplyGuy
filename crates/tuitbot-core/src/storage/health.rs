//! Database health check for the deep health endpoint.

use super::DbPool;
use serde::Serialize;
use std::time::Instant;

/// Database health check result.
#[derive(Debug, Clone, Serialize)]
pub struct DbHealth {
    /// Whether the database is reachable.
    pub reachable: bool,
    /// Query latency in milliseconds.
    pub latency_ms: u64,
    /// Whether WAL journal mode is active.
    pub wal_mode: bool,
}

/// Check database health by running a probe query and inspecting journal mode.
pub async fn check_db_health(pool: &DbPool) -> DbHealth {
    let start = Instant::now();

    let reachable = sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(pool)
        .await
        .is_ok();

    let latency_ms = start.elapsed().as_millis() as u64;

    let wal_mode = if reachable {
        sqlx::query_scalar::<_, String>("PRAGMA journal_mode")
            .fetch_one(pool)
            .await
            .map(|mode| mode.eq_ignore_ascii_case("wal"))
            .unwrap_or(false)
    } else {
        false
    };

    DbHealth {
        reachable,
        latency_ms,
        wal_mode,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::init_test_db;

    #[tokio::test]
    async fn check_healthy_db_reachable() {
        let pool = init_test_db().await.expect("init test db");
        let health = check_db_health(&pool).await;
        assert!(health.reachable);
        // In-memory SQLite doesn't report WAL mode via PRAGMA journal_mode.
    }

    #[tokio::test]
    async fn check_healthy_file_db_wal() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let db_path = dir.path().join("health_test.db");
        let pool = crate::storage::init_db(&db_path.to_string_lossy())
            .await
            .expect("init file db");
        let health = check_db_health(&pool).await;
        assert!(health.reachable);
        assert!(health.wal_mode);
        pool.close().await;
    }
}
