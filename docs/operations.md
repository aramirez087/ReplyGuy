# Operations

## Deployment patterns

### Long-running daemon

- Run under `systemd`, `tmux`, or equivalent supervisor.
- Set restart policy to `on-failure`.

### Tick-based scheduling

- Use cron/systemd timer/launchd/OpenClaw.
- Run every 15-30 minutes.
- Pipe JSON output to logs.

## Logging

- Use JSON output where supported.
- Capture stdout/stderr to centralized logging.
- Track failed loops and repeated skips.

## Backup and recovery

- Snapshot SQLite DB regularly.
- Back up config and token material securely.
- Validate restore procedure before incident.

## Upgrades

- Upgrade binary.
- Run `tuitbot test`.
- Validate auth and rate-limit state.
