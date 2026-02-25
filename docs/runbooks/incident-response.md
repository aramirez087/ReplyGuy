# Incident Response

General triage procedure for Tuitbot issues.

## Step 1: Check Health

```bash
# Liveness probe (no auth):
curl http://localhost:3001/api/health

# Deep health (requires auth):
curl -H "Authorization: Bearer $(cat ~/.tuitbot/api_token)" \
  http://localhost:3001/api/health/detailed
```

The detailed endpoint returns:
- `status`: `healthy`, `degraded`, or `unhealthy`
- `checks.database`: reachable, latency, WAL mode
- `checks.runtime`: running, task count
- `checks.circuit_breaker`: state, error count, cooldown

## Step 2: Check Logs

```bash
# If running via systemd:
journalctl -u tuitbot -n 100 --no-pager

# If running in tmux/screen:
# Check the terminal output or log file

# If running as server:
journalctl -u tuitbot-server -n 100 --no-pager
```

Look for:
- `error` or `warn` level entries
- `AuthExpired` messages -> see [Auth Expiry](auth-expiry.md)
- `Circuit breaker OPENED` -> see [Rate Limit Storms](rate-limit-storms.md)
- `Database` errors -> see [Database Maintenance](database-maintenance.md)

## Step 3: Identify Subsystem

| Symptom | Subsystem | Next step |
|---------|-----------|-----------|
| `AuthExpired` / 401 errors | Auth | [auth-expiry.md](auth-expiry.md) |
| 429/403 errors, breaker open | X API rate limits | [rate-limit-storms.md](rate-limit-storms.md) |
| Database unreachable | Storage | [database-maintenance.md](database-maintenance.md) |
| No posts going out | Posting queue | Check runtime status, approval queue |

## Step 4: Validate Fix

```bash
tuitbot test
```

This checks authentication, X API connectivity, and LLM provider.
