# Auth Expiry

## Symptoms

- `AuthExpired` errors in logs
- `Token refresh failed: authentication expired` message
- Runtime shuts down with auth error
- 401 responses from X API

## Diagnosis

```bash
# Check if tokens exist and are readable:
ls -la ~/.tuitbot/tokens.json

# Validate configuration and connectivity:
tuitbot test
```

If `tuitbot test` shows "Authentication expired", the refresh token has been revoked or expired.

## Resolution

```bash
# Re-authenticate with X API:
tuitbot auth

# This will:
# 1. Open a browser for OAuth 2.0 PKCE flow
# 2. Save new tokens to ~/.tuitbot/tokens.json
# 3. Display the authenticated username

# Verify:
tuitbot test

# Restart the daemon:
systemctl restart tuitbot
# or restart your tmux session
```

## Prevention

- The token refresh loop runs every 60 seconds and refreshes tokens 5 minutes before expiry
- If the X API revokes your refresh token (e.g., app permissions changed), you must re-authenticate manually
- Monitor for `Token refresh attempt failed` warnings in logs
