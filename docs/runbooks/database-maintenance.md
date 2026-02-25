# Database Maintenance

## WAL Checkpoint

SQLite WAL (Write-Ahead Log) mode is used for concurrent read/write performance. The WAL file grows over time and is checkpointed automatically, but you can force a checkpoint:

```bash
sqlite3 ~/.tuitbot/tuitbot.db "PRAGMA wal_checkpoint(TRUNCATE);"
```

This writes all WAL data back to the main database file and truncates the WAL.

## Cleanup and Retention

Tuitbot has a configurable retention period (default: 90 days). Old records are cleaned up automatically, but you can verify:

```bash
# Check retention setting in config:
grep retention ~/.tuitbot/config.toml

# View database size:
ls -lh ~/.tuitbot/tuitbot.db
ls -lh ~/.tuitbot/tuitbot.db-wal
```

## VACUUM

Over time, deleted records leave empty space in the database file. Run VACUUM to reclaim space:

```bash
# Stop the server first:
systemctl stop tuitbot

# VACUUM:
sqlite3 ~/.tuitbot/tuitbot.db "VACUUM;"

# Restart:
systemctl start tuitbot
```

**Note:** VACUUM requires exclusive access to the database. Always stop Tuitbot first.

## Integrity Check

```bash
sqlite3 ~/.tuitbot/tuitbot.db "PRAGMA integrity_check;"
# Should return: ok
```

If integrity check fails, restore from backup:

```bash
tuitbot restore ~/.tuitbot/backups/tuitbot_LATEST.db
```

## Database Health via API

```bash
curl -H "Authorization: Bearer $(cat ~/.tuitbot/api_token)" \
  http://localhost:3001/api/health/detailed | jq '.checks.database'
```

Returns:
- `reachable`: can the server query the database
- `latency_ms`: query latency
- `wal_mode`: whether WAL journal mode is active (should be `true`)
