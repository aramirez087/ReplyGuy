# Backup & Restore

## Creating Backups

### Manual Backup

```bash
# Create a backup to the default location (~/.tuitbot/backups/):
tuitbot backup

# Create a backup to a custom directory:
tuitbot backup --output-dir /path/to/backups

# List existing backups:
tuitbot backup --list

# Keep only the 5 most recent backups:
tuitbot backup --prune 5
```

### Scheduled Backups (cron)

Add to your crontab (`crontab -e`):

```cron
# Daily backup at 2 AM, keep 7 days:
0 2 * * * /usr/local/bin/tuitbot backup && /usr/local/bin/tuitbot backup --prune 7
```

For systemd timer:

```ini
# ~/.config/systemd/user/tuitbot-backup.timer
[Unit]
Description=Tuitbot daily backup

[Timer]
OnCalendar=*-*-* 02:00:00
Persistent=true

[Install]
WantedBy=timers.target
```

```ini
# ~/.config/systemd/user/tuitbot-backup.service
[Unit]
Description=Tuitbot backup

[Service]
Type=oneshot
ExecStart=/usr/local/bin/tuitbot backup
ExecStartPost=/usr/local/bin/tuitbot backup --prune 7
```

## Restoring from Backup

### Validate First

```bash
# Check if a backup is valid without restoring:
tuitbot restore ./backup.db --validate-only
```

### Restore

```bash
# Stop the running server/daemon first:
systemctl stop tuitbot

# Restore (interactive confirmation):
tuitbot restore ~/.tuitbot/backups/tuitbot_20240115_020000.db

# Restore without confirmation:
tuitbot restore ~/.tuitbot/backups/tuitbot_20240115_020000.db --force

# Restart:
systemctl start tuitbot
```

The restore process:
1. Validates the backup file (tables, integrity check)
2. Creates a safety backup of the current database
3. Atomically replaces the database file
4. Cleans up WAL/SHM files

## Pre-Migration Backups

Every time `init_db()` runs (server startup, `tuitbot run`), a pre-migration backup is automatically created if the database already exists. These are stored in `~/.tuitbot/backups/` with a `pre_migration_` prefix. Only the 3 most recent are kept.
