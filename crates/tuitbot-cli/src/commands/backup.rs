//! Implementation of the `tuitbot backup` command.

use std::path::PathBuf;

use tuitbot_core::startup::data_dir;
use tuitbot_core::storage;

use super::BackupArgs;

/// Execute the `tuitbot backup` command.
pub async fn execute(args: BackupArgs) -> anyhow::Result<()> {
    let data = data_dir();
    let db_path = data.join("tuitbot.db");

    if args.list {
        return list_backups(&data);
    }

    if let Some(keep) = args.prune {
        return prune_backups(&data, keep);
    }

    // Create a backup.
    let backup_dir = args
        .output_dir
        .clone()
        .map(PathBuf::from)
        .unwrap_or_else(|| data.join("backups"));

    if !db_path.exists() {
        anyhow::bail!(
            "Database not found at {}. Run `tuitbot init` first.",
            db_path.display()
        );
    }

    // Open a read-only pool to the existing DB for VACUUM INTO.
    let pool = storage::init_db(&db_path.to_string_lossy()).await?;

    eprintln!("Creating backup...");
    let result = storage::backup::create_backup(&pool, &backup_dir).await?;
    pool.close().await;

    eprintln!("Backup created successfully:");
    eprintln!("  Path: {}", result.path.display());
    eprintln!("  Size: {} bytes", result.size_bytes);
    eprintln!("  Duration: {}ms", result.duration_ms);

    Ok(())
}

fn list_backups(data_dir: &std::path::Path) -> anyhow::Result<()> {
    let backup_dir = data_dir.join("backups");
    let backups = storage::backup::list_backups(&backup_dir);

    if backups.is_empty() {
        eprintln!("No backups found in {}", backup_dir.display());
        return Ok(());
    }

    eprintln!("Backups in {}:", backup_dir.display());
    for backup in &backups {
        let ts = backup.timestamp.as_deref().unwrap_or("unknown");
        let size_kb = backup.size_bytes / 1024;
        eprintln!("  {} ({ts}) — {size_kb} KB", backup.path.display());
    }
    eprintln!("\nTotal: {} backup(s)", backups.len());

    Ok(())
}

fn prune_backups(data_dir: &std::path::Path, keep: usize) -> anyhow::Result<()> {
    let backup_dir = data_dir.join("backups");
    let deleted = storage::backup::prune_backups(&backup_dir, keep)?;

    if deleted == 0 {
        eprintln!("Nothing to prune (at most {keep} backups exist).");
    } else {
        eprintln!("Pruned {deleted} old backup(s), kept {keep} most recent.");
    }

    Ok(())
}
