# Session 04 Handoff — True "Hello World" First-Success Flow

## What Changed

After `tuitbot init`, users now see a clear 3-step path to their first successful bot action. The flow `init → auth → test → tick --dry-run` is guided at every step, with actionable failure messages and next-step hints.

### New Command Sequence

```bash
tuitbot init                # writes config, prints 3-step getting-started guide
tuitbot auth                # OAuth flow (unchanged)
tuitbot test                # now includes LLM connectivity check + next-step hint
tuitbot tick --dry-run      # now prints dry-run banner + enrichment tip on success
```

### Changes Summary

1. **Post-init guidance** — `tuitbot init` (quickstart) now prints a numbered 3-step guide instead of the terse "Next: tuitbot auth".
2. **LLM connectivity check** — `tuitbot test` creates the LLM provider and calls `health_check()` to verify the API key and network access work.
3. **Next-step hint** — When all checks pass, `tuitbot test` prints `Ready! Try: tuitbot tick --dry-run`.
4. **Dry-run framing** — `tuitbot tick --dry-run` prints a banner ("No posts will be made") and a success tip ("Customize your bot's voice...").
5. **Module split** — `commands/test.rs` (536 lines) converted to `commands/test/mod.rs` + `commands/test/tests.rs` following the `init/` and `settings/` patterns.

## Expected Outputs

### After `tuitbot init` (quickstart)

```
Wrote ~/.tuitbot/config.toml

Get started:
  1. tuitbot auth           — connect your X account
  2. tuitbot test           — verify everything works
  3. tuitbot tick --dry-run — see the bot in action (no posts)
```

### After `tuitbot test` (all pass)

```
Configuration:      OK (loaded from ~/.tuitbot/config.toml)
Business profile:   OK (product_name: "MyApp", 3 keywords, 3 topics)
X API token:        OK (token valid, expires in 1h 58m)
X API refresh:      OK (refresh token present)
X API scopes:       OK (all required scopes granted)
LLM provider:       OK (openai, model: gpt-4o-mini)
LLM connectivity:   OK (openai: reachable)
Database:           OK (will be created at ~/.tuitbot/tuitbot.db)

All checks passed.
Ready! Try: tuitbot tick --dry-run
```

### After `tuitbot tick --dry-run` (success)

```
Dry run: showing what the bot would do. No posts will be made.

tuitbot tick  tier=basic  schedule=active  dry_run=true  ...

  analytics    OK     followers=1000, replies_measured=15
  discovery    OK     found=12, qualifying=3, replied=0, skipped=3
  content      OK     topic='rust', chars=240
  ...

Result: success

Tip: Customize your bot's voice with `tuitbot init --advanced` or `tuitbot settings`
```

## Failure Matrix

| Failure | Where | Message | Fix |
|---------|-------|---------|-----|
| No config | `tuitbot test` | "Failed to load configuration" | Run `tuitbot init` |
| Empty product_name | `tuitbot test` | "Business profile: FAIL (product_name is empty)" | Edit config.toml or `tuitbot init --force` |
| No auth tokens | `tuitbot test` | "X API auth: FAIL (no tokens found, run `tuitbot auth` first)" | Run `tuitbot auth` |
| Expired token | `tuitbot test` | "X API token: FAIL (token expired, run `tuitbot auth`)" | Run `tuitbot auth` |
| No LLM API key | `tuitbot test` | "LLM provider: FAIL (api_key required for openai)" | Edit config.toml `[llm]` section |
| LLM not configured | `tuitbot test` | "LLM connectivity: FAIL (provider not configured)" | Set `[llm] provider` in config.toml |
| LLM unreachable | `tuitbot test` | "LLM connectivity: FAIL (openai: connection refused)" | Check API key, network, service status |

## Files Changed

| File | Change |
|------|--------|
| `commands/init/display.rs` | Added `print_quickstart_next_steps()` |
| `commands/init/mod.rs` | Replaced "Next: tuitbot auth" with `print_quickstart_next_steps()` call |
| `commands/test.rs` → `commands/test/mod.rs` | Module split, added `check_llm_connectivity()`, `next_step_guidance()`, wired into `run_checks()` and `execute()` |
| `commands/test/tests.rs` (new) | Moved 7 existing tests + 4 new tests (LLM connectivity, next-step guidance) |
| `commands/tick.rs` | Added dry-run banner and success enrichment tip |
| `docs/roadmap/init-simplification/session-04-handoff.md` (new) | This file |

### Files NOT Changed

- `wizard.rs`, `render.rs`, `prompts.rs`, `steps.rs` — quickstart logic unchanged
- `main.rs`, `commands/mod.rs` — no interface changes
- `deps.rs` — runtime deps unchanged
- Anything in `tuitbot-core` — all changes are CLI-only

## New Tests

- `check_llm_connectivity_not_configured` — empty provider → FAIL with "not configured"
- `check_llm_connectivity_bad_provider_returns_fail` — unknown provider → FAIL
- `next_step_guidance_all_pass` — all OK → prints "tuitbot tick --dry-run" hint
- `next_step_guidance_any_fail` — any FAIL → no hint

## Remaining Gaps

- **Session 05: Progressive enrichment** — prompt users to fill in optional fields over time
- **Session 06: Testing and quality gates**
- **Session 07: Docs and positioning**
- **Session 08: Release readiness**
