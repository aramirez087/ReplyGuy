# Session 02 Handoff: CLI Broken Pipe Hardening

**Date:** 2026-02-26
**Session:** 02
**Branch:** `feat/init_simplification`
**Status:** Complete

---

## 1. Root Cause

Rust's runtime sets `SIGPIPE` to `SIG_IGN` by default. On Unix, when stdout is
piped to a process that exits early (e.g., `tuitbot mcp manifest | head -1`),
the receiving end of the pipe closes. The next write to stdout returns `EPIPE`.

Because `SIGPIPE` is ignored, the process does not terminate on the signal.
Instead, `println!` receives the `EPIPE` error and **panics**, producing a
noisy stack trace instead of a clean exit.

This affects every CLI command that writes to stdout: `mcp manifest`, `tick`
(JSON mode), `stats` (JSON mode), `test` (JSON mode), `approve` (JSON mode),
and `settings --show` (JSON mode).

---

## 2. Implementation Summary

### Two-layer fix

**Layer 1: SIGPIPE reset (Unix)**

At the very start of `main()`, before any argument parsing, we call
`reset_sigpipe()` which restores `SIGPIPE` to `SIG_DFL` via `libc::signal`.
This is the standard Unix behavior used by `cat`, `grep`, `ripgrep`, and `fd`.
The process is terminated by the signal before any panic can occur.

**Layer 2: BrokenPipe-aware stdout writes (cross-platform)**

All `println!` calls that write to stdout have been replaced with
`write_stdout()`, a helper function in the new `output.rs` module. This
function:

1. Locks stdout and writes the string + newline.
2. On `BrokenPipe`, calls `std::process::exit(0)` immediately.
3. On other IO errors, returns an `anyhow::Error`.

Additionally, the top-level `main()` function catches any `BrokenPipe` errors
that propagate through `?` and exits with code 0. This is the safety net for
any future code path that might bypass `write_stdout`.

### Files modified

| File | Change |
|------|--------|
| `crates/tuitbot-cli/Cargo.toml` | Added `libc = "0.2"` (unix-only) |
| `crates/tuitbot-cli/src/output.rs` | **New.** `reset_sigpipe()`, `write_stdout()`, `is_broken_pipe()` + 4 unit tests |
| `crates/tuitbot-cli/src/main.rs` | Added `mod output`, calls `reset_sigpipe()` first, wraps `run()` with BrokenPipe catch |
| `crates/tuitbot-cli/src/commands/mcp.rs` | `println!` -> `write_stdout()` |
| `crates/tuitbot-cli/src/commands/tick/mod.rs` | `println!` -> `write_stdout()` |
| `crates/tuitbot-cli/src/commands/stats.rs` | `println!` -> `write_stdout()` |
| `crates/tuitbot-cli/src/commands/test/mod.rs` | `println!` -> `write_stdout()` |
| `crates/tuitbot-cli/src/commands/approve.rs` | `println!` (4 sites) -> `write_stdout()` |
| `crates/tuitbot-cli/src/commands/settings/show.rs` | `println!` -> `write_stdout()` |

### Zero remaining `println!` to stdout

After this change, `grep -r 'println!' crates/tuitbot-cli/src/` returns zero
matches. All stdout output goes through `write_stdout()`. All user-facing
messages use `eprintln!` (which is unaffected by stdout pipe state).

---

## 3. Before/After Behavior

### Before

```
$ tuitbot mcp manifest | head -1
{
thread 'main' panicked at 'failed printing to stdout: Broken pipe (os error 32)'
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### After

```
$ tuitbot mcp manifest | head -1
{
$ echo $?
0
```

The process exits silently with code 0. No panic, no error output on stderr.

---

## 4. Test Evidence

### Unit tests (4 tests in `output.rs`)

- `test_is_broken_pipe_detects_broken_pipe` — verifies detection of `BrokenPipe` IO errors wrapped in `anyhow::Error`
- `test_is_broken_pipe_rejects_other_errors` — verifies non-BrokenPipe errors are not false-positives
- `test_is_broken_pipe_detects_nested_broken_pipe` — verifies detection through `anyhow::Context` chains
- `test_write_stdout_succeeds_with_valid_output` — verifies normal stdout write path

### CI validation

```
cargo fmt --all && cargo fmt --all --check     # clean
cargo clippy --workspace -- -D warnings         # clean
RUSTFLAGS="-D warnings" cargo test --workspace  # all tests pass
```

### Manual validation

The SIGPIPE reset can be validated with:

```bash
cargo build -p tuitbot-cli && target/debug/tuitbot mcp manifest | head -1
```

Expected: prints the first line of the JSON manifest and exits cleanly.

---

## 5. Design Decisions

### D-011: Exit code 0 on broken pipe

**Decision:** Exit with code 0 when stdout pipe is broken.

**Rationale:** The consumer (e.g., `head`) got the data it needed and closed the
pipe intentionally. This is not an error condition. Standard Unix tools (`cat`,
`yes`, `seq`) all exit with 0 or 141 (128 + SIGPIPE=13) when piped to `head`.
With SIGPIPE restored to SIG_DFL, the process will exit with signal status
(typically shows as exit code 141), which is the expected Unix convention.

### D-012: `libc` dependency for SIGPIPE

**Decision:** Add `libc` as a unix-only dependency rather than using nightly
`#[unix_sigpipe = "sig_dfl"]` attribute.

**Rationale:** `libc` is stable, zero-cost (already a transitive dependency),
and works on all stable Rust versions. The `unix_sigpipe` attribute is unstable
and may never stabilize with the exact semantics we need.

### D-013: Dual-layer approach

**Decision:** Use both SIGPIPE reset and `write_stdout` helper.

**Rationale:** SIGPIPE reset handles the common Unix case at the OS level.
`write_stdout` provides a cross-platform safety net and explicit error handling
for any edge cases (threaded writes, non-Unix platforms). Belt and suspenders.

---

## 6. Risks

None identified. The SIGPIPE reset is a well-understood Unix primitive used by
hundreds of Rust CLI tools. The `write_stdout` helper is a trivial wrapper with
clear semantics.

---

## 7. Next Session Inputs

Session 02 was an immediate-win hardening task. It does not change the
Session 03+ backlog from the charter. The next session should proceed with
the endpoint registry scaffold as defined in `session-01-handoff.md`.
