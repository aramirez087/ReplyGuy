# Session 01 Handoff: Resolved Decisions & Implementation Backlog

> **Prerequisite:** [ux-contract.md](./ux-contract.md) — locked UX contract.
> **Status:** All decisions resolved. Zero open questions.

---

## Resolved Decisions

### 1. Quickstart hard cap = 5 prompts

The quickstart path collects exactly 5 values: product name, discovery keywords,
LLM provider, API key (skipped for ollama), and X API Client ID.

No additional prompts may be added to the quickstart path. New features route
to `--advanced` or `tuitbot settings`.

### 2. `industry_topics` = auto-derive from `product_keywords`

Content generation loops (`content/mod.rs`) skip gracefully on empty topics,
but copying `product_keywords` into `industry_topics` gives working generation
from day one — the topic scorer and framework rotation produce varied output
even with narrow seed topics.

### 3. LLM model = default per provider, don't ask

Defaults already exist in `steps.rs:203-208`:
- `openai` → `gpt-4o-mini`
- `anthropic` → `claude-sonnet-4-6`
- `ollama` → `llama3.2`

These are sensible starting points. Users who want a different model can change
it via `tuitbot settings` or edit `config.toml` directly.

### 4. Advanced mode = `--advanced` CLI flag

Maps to the existing `InitArgs` pattern in `commands/mod.rs:42-51`. Not an
in-wizard prompt ("Would you like the quick or advanced setup?") — that would
add a prompt before the user has context to choose.

### 5. `target_audience` in quickstart = skip, default to `""`

The content generator handles empty `target_audience` gracefully — it omits
audience-specific framing rather than producing broken output. Users who need
audience targeting enrich via settings.

### 6. Post-init chaining = remove from quickstart

The current wizard chains 4 confirmation prompts after config write:
"Write config?" → "Authenticate now?" → "Validate now?" → "Start agent?"

This breaks the 5-prompt cap and couples auth/test failure UX to init.
Quickstart prints "Config saved" + next step (`tuitbot auth`) and exits.

The `--advanced` path preserves chaining for operators who want the guided flow.

### 7. Hello World = `tuitbot tick --dry-run`

Shows scored tweets from the user's keywords — full pipeline, no mutations.
This is the evaluation gate: user sees real output before investing in enrichment.

---

## Risks & Mitigations

### 1. Sparse config → generic content

**Risk:** Minimal quickstart config produces generic replies/tweets that don't
feel authentic, leading users to dismiss the tool.

**Mitigation:** Framework rotation in `content/frameworks.rs` provides variety.
Progressive enrichment hints ("Your config is missing brand voice — run
`tuitbot settings` to add it") surface after first run. Weekly strategy reports
recommend specific enrichments based on metrics.

### 2. Auto-derived topics too narrow

**Risk:** Copying `product_keywords` into `industry_topics` gives a narrow topic
set, causing repetitive content.

**Mitigation:** The topic scorer and framework rotation generate varied output even
with narrow seeds. Strategy reports flag "topic concentration" and suggest
diversification. Users can expand via `tuitbot settings`.

### 3. Chaining removal breaks muscle memory

**Risk:** Existing users expect `init` to flow into `auth → test → run`.

**Mitigation:** `--advanced` preserves the full chaining flow. Since quickstart
is the *new default* for new users, there's no existing muscle memory to break.
Existing users who re-init likely use `--advanced` or `--non-interactive` anyway.

### 4. Five-prompt cap leaves no room

**Risk:** Future features need data during init, but the 5-prompt cap blocks them.

**Mitigation:** New features route to `--advanced` and `tuitbot settings`. The cap
is intentional — it forces feature designers to provide sensible defaults rather
than front-loading questions onto new users.

### 5. Ollama non-default port

**Risk:** Users running Ollama on a non-standard port get silent failures at
runtime with no hint during init.

**Mitigation:** Default URL (`http://localhost:11434/v1`) covers the standard case.
`tuitbot test` validates LLM connectivity and surfaces clear error messages.
Advanced users who change the port know to update their config.

---

## Session 02 Implementation Backlog

All tasks are dependency-sequenced. Independent tasks (no "Depends On") can be
parallelized. Tasks with dependencies must wait for their prerequisites.

| ID | Task | File(s) | Depends On |
|----|------|---------|-----------|
| S02-1 | Add `advanced: bool` field to `InitArgs` | `crates/tuitbot-cli/src/commands/mod.rs` | — |
| S02-2 | Verify `Config::validate()` passes with quickstart-minimal config | `crates/tuitbot-core/src/config/mod.rs` | — |
| S02-3 | Add `QuickstartResult` struct + `into_wizard_result()` converter | `crates/tuitbot-cli/src/commands/init/wizard.rs` | — |
| S02-4 | Implement `step_quickstart()` — 5 prompts, returns `QuickstartResult` | `crates/tuitbot-cli/src/commands/init/steps.rs` | S02-3 |
| S02-5 | Update `render_config_toml()` to handle quickstart defaults + comments | `crates/tuitbot-cli/src/commands/init/render.rs` | S02-3 |
| S02-6 | Add quickstart banner, compact summary, and next-steps output | `crates/tuitbot-cli/src/commands/init/display.rs` | S02-3 |
| S02-7 | Wire mode dispatch in `execute()`: quickstart (default) vs advanced | `crates/tuitbot-cli/src/commands/init/mod.rs` | S02-1, S02-4, S02-6 |
| S02-8 | Add unit tests for quickstart path (prompt count, validation, output) | `crates/tuitbot-cli/src/commands/init/tests.rs` | S02-3, S02-5 |
| S02-9 | Update `config.example.toml` comments to reference quickstart defaults | `config.example.toml` | S02-5 |

### Dependency Graph

```
S02-1 ─────────────────────────────────┐
S02-2 (independent)                    │
S02-3 ──┬──────────┬──────────┐        │
        │          │          │        │
      S02-4      S02-5     S02-6      │
        │          │          │        │
        │          ├── S02-8  │        │
        │          │          │        │
        └──────────┼──────────┴── S02-7
                   │
                 S02-9
```

### Task Details

**S02-1: Add `advanced` flag to `InitArgs`**
Add `#[arg(long)] pub advanced: bool` to `InitArgs` in `commands/mod.rs:42-51`.
Update `execute()` call signature to accept the new flag.

**S02-2: Verify `Config::validate()` with minimal config**
Write a test that constructs a config with only quickstart fields populated
(product_name, product_keywords, llm_provider, llm_api_key, client_id) plus
defaults for everything else. Confirm `Config::validate()` returns `Ok(())`.
Location: `config/mod.rs` — validate is around line 666.

**S02-3: Add `QuickstartResult` struct**
New struct with 5 fields matching the quickstart prompts. Include
`into_wizard_result()` that converts to `WizardResult` by applying defaults
(copy keywords → topics, set provider default model, etc.).

**S02-4: Implement `step_quickstart()`**
Single function collecting all 5 prompts in sequence. Returns `QuickstartResult`.
Uses same validation patterns as existing steps (non-empty, at-least-one for CSV).

**S02-5: Update `render_config_toml()`**
Ensure quickstart-defaulted fields get helpful TOML comments like
`# Auto-defaulted — customize with: tuitbot settings`.

**S02-6: Quickstart display functions**
- `print_quickstart_banner()` — shorter than current 8-step banner
- `print_quickstart_summary()` — compact, shows only collected values
- `print_quickstart_next_steps()` — single line: "Next: tuitbot auth"

**S02-7: Wire mode dispatch**
In `execute()`, check `args.advanced`. If true, run existing `run_wizard()`.
If false (default), run new `run_quickstart()` which calls `step_quickstart()`,
renders config, prints summary, exits.

**S02-8: Unit tests**
- Quickstart result converts to valid wizard result
- Rendered TOML from quickstart result is parseable
- industry_topics == product_keywords after auto-derive
- Provider defaults map correctly

**S02-9: Update example config**
Add comments noting which fields are auto-defaulted in quickstart mode and
how to customize them.
