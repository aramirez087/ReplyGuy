# Getting Started

## Prerequisites

- X Developer App credentials
- One LLM provider:
  - OpenAI
  - Anthropic
  - Ollama
- Rust 1.75+ (only needed for source builds)

## Install

### Recommended (crates.io)

```bash
cargo install tuitbot-cli --locked
```

### Prebuilt binary (no Rust toolchain)

```bash
curl -fsSL https://raw.githubusercontent.com/aramirez087/TuitBot/main/scripts/install.sh | bash
```

### From source (contributors)

```bash
cargo install --path crates/tuitbot-cli --locked
```

### Windows

Download the Windows asset from GitHub Releases and add `tuitbot.exe` to `PATH`.

## First-Time Setup

1. Create an app in the X Developer Portal.
2. Configure callback URI exactly as `http://127.0.0.1:8080/callback`.
3. Run initialization:

```bash
tuitbot init
```

4. Authenticate and validate:

```bash
tuitbot auth
tuitbot test
```

## Choose an Execution Mode

### Mode A: Daemon

```bash
tuitbot run
```

Use this mode on a VPS or always-on host.

### Mode B: External scheduler

```bash
tuitbot tick --output json
```

Use this mode with cron/systemd/launchd/OpenClaw.

## Health Check

```bash
tuitbot health
tuitbot stats --output json
```
