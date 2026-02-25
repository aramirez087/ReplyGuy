# Tuitbot Runbooks

Step-by-step guides for common operational scenarios.

| Runbook | When to use |
|---------|-------------|
| [Incident Response](incident-response.md) | General triage: check health, identify subsystem |
| [Auth Expiry](auth-expiry.md) | `AuthExpired` errors, token refresh failures |
| [Rate Limit Storms](rate-limit-storms.md) | Circuit breaker trips, sustained 429/403 errors |
| [Backup & Restore](backup-restore.md) | Scheduled backups, manual recovery |
| [Database Maintenance](database-maintenance.md) | WAL checkpoint, cleanup, VACUUM |
