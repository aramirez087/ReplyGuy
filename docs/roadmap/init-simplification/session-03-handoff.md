# Session 03 Handoff — CLI Init Quickstart/Advanced Split

## What Changed

Rebuilt `tuitbot init` so the default path is a **5-prompt quickstart** (~60 seconds) while the full 8-step wizard is preserved behind `--advanced`.

### CLI Examples

```bash
tuitbot init                # quickstart: 5 prompts → config.toml
tuitbot init --advanced     # full 8-step wizard (25 prompts + chaining)
tuitbot init --non-interactive  # copy template config (unchanged)
```

### Prompt Count Comparison

| Mode | Prompts | Post-config chaining |
|------|---------|---------------------|
| Before (all users) | 25 | auth → test → run (4 more) |
| Quickstart (default) | 5 | None — prints "Next: tuitbot auth" |
| Advanced (`--advanced`) | 25 | auth → test → run (unchanged) |

### Quickstart Prompts (in order)

1. **Product name** — easy start, builds momentum
2. **Discovery keywords** — CSV, at least 1 required
3. **LLM provider** — select: openai / anthropic / ollama
4. **API key** — skipped for ollama
5. **X API Client ID** — highest friction last

### Key Defaults Applied by Quickstart

- `approval_mode = true` (safe default — all posts queued for review)
- `timezone = "UTC"`, active hours 8–22, all 7 days
- No brand voice, reply style, or content style
- No persona (opinions, experiences, pillars)
- No target accounts
- `industry_topics` left empty → `effective_industry_topics()` derives from `product_keywords`
- LLM model defaults: openai → `gpt-4o-mini`, anthropic → `claude-sonnet-4-6`, ollama → `llama3.2`

## Files Changed

| File | Change |
|------|--------|
| `crates/tuitbot-cli/src/commands/mod.rs` | Added `advanced: bool` field to `InitArgs` |
| `crates/tuitbot-cli/src/main.rs` | Pass `args.advanced` to `init::execute()` |
| `crates/tuitbot-cli/src/commands/init/mod.rs` | Added `run_quickstart()`, renamed `run_wizard()` → `run_advanced_wizard()`, routing logic |
| `crates/tuitbot-cli/src/commands/init/steps.rs` | Added `step_quickstart()` (5-prompt collector) |
| `crates/tuitbot-cli/src/commands/init/display.rs` | Added `print_quickstart_banner()` and `print_quickstart_summary()` |
| `crates/tuitbot-cli/src/commands/init/tests.rs` | Added 3 new tests: quickstart renders, defaults verification, advanced regression |

### Files NOT Changed

- `wizard.rs` — `WizardResult` struct unchanged
- `render.rs` — already quickstart-aware from Session 02
- `prompts.rs` — only used by advanced wizard and `upgrade` command
- `helpers.rs` — reused as-is

## Remaining Gaps

- **Session 04: Hello-world flow** — post-init guided first action
- **Session 05: Progressive enrichment** — prompt users to fill in optional fields over time
- **Session 06: Testing and quality gates**
- **Session 07: Docs and positioning**
- **Session 08: Release readiness**
