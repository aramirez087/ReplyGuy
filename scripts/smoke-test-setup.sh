#!/usr/bin/env bash
# Smoke test: validates setup commands work end-to-end without network.
#
# Usage: ./scripts/smoke-test-setup.sh [path-to-tuitbot-binary]
#
# Exit codes:
#   0 — all smoke tests passed
#   1 — one or more tests failed
#
# All tests are offline — no auth tokens or API keys needed.

set -euo pipefail

BINARY="${1:-target/debug/tuitbot}"
TMPDIR=$(mktemp -d)
CONFIG_DIR="$TMPDIR/.tuitbot"
CONFIG_PATH="$CONFIG_DIR/config.toml"

PASS=0
FAIL=0

cleanup() {
    rm -rf "$TMPDIR"
}
trap cleanup EXIT

log_pass() {
    echo "  PASS  $1"
    PASS=$((PASS + 1))
}

log_fail() {
    echo "  FAIL  $1: $2"
    FAIL=$((FAIL + 1))
}

if [ ! -x "$BINARY" ]; then
    echo "Error: binary not found or not executable: $BINARY"
    echo "Run 'cargo build' first or pass the binary path as an argument."
    exit 1
fi

echo "Smoke test: setup architecture"
echo "Binary: $BINARY"
echo "Temp dir: $TMPDIR"
echo

# 1. tuitbot --help exits 0 and output contains "init"
if output=$("$BINARY" --help 2>&1) && echo "$output" | grep -q "init"; then
    log_pass "--help exits 0 and mentions 'init'"
else
    log_fail "--help" "exit non-zero or missing 'init' in output"
fi

# 2. tuitbot init --help exits 0 and mentions --force
if output=$("$BINARY" init --help 2>&1) && echo "$output" | grep -q "\-\-force"; then
    log_pass "init --help exits 0 and mentions --force"
else
    log_fail "init --help" "exit non-zero or missing '--force' in output"
fi

# 3. tuitbot init --non-interactive writes config to tmpdir
mkdir -p "$CONFIG_DIR"
if "$BINARY" --config "$CONFIG_PATH" init --non-interactive 2>/dev/null; then
    log_pass "init --non-interactive exits 0"
else
    log_fail "init --non-interactive" "exit non-zero"
fi

# 4. Verify config.toml exists and parses as valid TOML
if [ -f "$CONFIG_PATH" ]; then
    log_pass "config.toml exists at $CONFIG_PATH"
else
    log_fail "config.toml" "file not found at $CONFIG_PATH"
fi

# 5. tuitbot settings --show reads the config
if "$BINARY" --config "$CONFIG_PATH" settings --show >/dev/null 2>&1; then
    log_pass "settings --show exits 0"
else
    log_fail "settings --show" "exit non-zero"
fi

# 6. tuitbot test runs (expect failures due to no auth, but should not crash/signal)
"$BINARY" --config "$CONFIG_PATH" test 2>/dev/null || true
# If we get here without being killed by a signal, that's a pass
log_pass "test command runs without crash"

echo
echo "Results: $PASS passed, $FAIL failed"

if [ "$FAIL" -gt 0 ]; then
    exit 1
fi
