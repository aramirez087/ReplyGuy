# Rate Limit Storms

## Symptoms

- `Circuit breaker OPENED` in logs
- `health/detailed` shows `circuit_breaker.state: "open"`
- No mutations (tweets, replies) going out
- Sustained 429 (Too Many Requests) or 403 (Forbidden) from X API

## Diagnosis

```bash
# Check circuit breaker status:
curl -H "Authorization: Bearer $(cat ~/.tuitbot/api_token)" \
  http://localhost:3001/api/health/detailed | jq '.checks.circuit_breaker'

# Expected output when tripped:
# {
#   "state": "open",
#   "error_count": 5,
#   "cooldown_remaining_seconds": 420
# }
```

## Resolution

### Wait for Cooldown (Recommended)

The circuit breaker automatically transitions to `half_open` after the cooldown period (default: 600 seconds / 10 minutes). It then allows a single probe mutation:
- If it succeeds, the breaker resets to `closed`
- If it fails, the breaker re-opens for another cooldown period

### Tune Configuration

If storms are frequent, adjust `config.toml`:

```toml
[circuit_breaker]
# Increase threshold to tolerate more transient errors
error_threshold = 8

# Widen the sliding window
window_seconds = 600

# Longer cooldown to let X API rate limits fully reset
cooldown_seconds = 900
```

Then restart:

```bash
systemctl restart tuitbot
```

### Check X API Tier

Rate limit storms may indicate you're exceeding your API tier's limits:

```bash
tuitbot test
# Check the detected tier (Free, Basic, Pro)
```

Tier limits:
- **Free**: Very limited search, no tweet lookup
- **Basic**: 100 tweets/month, limited search
- **Pro**: Full search, higher rate limits

## Prevention

- Use appropriate `min_action_delay_seconds` and `max_action_delay_seconds` for your tier
- Monitor `activity/rate-limits` endpoint for usage patterns
- Consider upgrading your X API tier if limits are consistently hit
