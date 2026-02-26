# UX Contract: `tuitbot init` Simplification

> **Status:** Locked — no TBDs remain.
> **Scope:** Defines the Quickstart vs Advanced boundary for `tuitbot init`.

---

## Problem Statement

`tuitbot init` currently runs an **8-step interactive wizard** collecting 22–27 prompts
(typical path: 25 — 23 wizard prompts + up to 4 post-config chaining confirmations).

This is the **only path** to a runnable config — there is no "fast lane."

First-time users who just want to evaluate the tool must answer questions about brand voice,
persona traits, scheduling preferences, and target accounts before seeing any output.
**13 of those prompts collect optional enrichment data** that already has usable defaults
in `defaults.rs` and `steps.rs`.

There is no early-exit path that produces a valid, runnable configuration.

---

## Prompt Inventory (Current Wizard)

| Step | Name | Prompts | Required for first run? |
|------|------|---------|------------------------|
| 1/8 | X API Credentials | 2–3 | 1 of 2–3 (Client ID) |
| 2/8 | Business Profile | 6 | 2 of 6 (product name, keywords) |
| 3/8 | Brand Voice & Style | 3 | 0 of 3 |
| 4/8 | LLM Provider | 2–4 | 2 of 2–4 (provider, API key) |
| 5/8 | Persona | 3 | 0 of 3 |
| 6/8 | Target Accounts | 1 | 0 of 1 |
| 7/8 | Approval Mode | 1 | 0 of 1 (default: true) |
| 8/8 | Active Hours Schedule | 4–5 | 0 of 4–5 (defaults: UTC 8–22 all days) |
| — | Post-config chaining | 4 | 0 of 4 |
| **Total** | | **22–27** (typ. 25) | **5 of 25** |

Only **5 prompts** are required to produce a config that passes `Config::validate()`.
The remaining 20 prompts collect enrichment data with functional defaults.

---

## User Archetypes

### Hello World

> "Let me evaluate this tool in under 2 minutes."

- Has API keys ready (copied from dev portal).
- Will abandon at >5 prompts.
- Wants to see scored tweets or a draft reply before investing more time.
- Path: `tuitbot init` (5 prompts) → `tuitbot auth` → `tuitbot tick --dry-run`.

### Operator

> "I'm setting this up for production — I'll invest time after I see value."

- Runs quickstart first, then enriches via `tuitbot init --advanced` or `tuitbot settings`.
- Cares about brand voice, persona, schedule, target accounts.
- Willing to spend 5–10 minutes on configuration after the tool has proven itself.

### Power User

> "Just give me the TOML."

- Edits `config.toml` directly.
- Uses `tuitbot init --non-interactive` to get the template, then hand-edits.
- May script setup with env vars.

---

## Quickstart Path (Default) — 5 Prompts Hard Cap

The default `tuitbot init` (no flags) runs the quickstart flow.

| # | Prompt | Type | Validation | Source |
|---|--------|------|------------|--------|
| 1 | Product name | Text input | Non-empty | `steps.rs` step 2 |
| 2 | Discovery keywords (comma-separated) | Text input | At least 1 keyword | `steps.rs` step 2 |
| 3 | LLM provider | Select list | openai / anthropic / ollama | `steps.rs` step 4 |
| 4 | API key (skipped for ollama) | Text input | Non-empty | `steps.rs` step 4 |
| 5 | X API Client ID | Text input | Non-empty | `steps.rs` step 1 |

**Order rationale:** Easy questions first (product name, keywords) build momentum.
Credential paste last (API key, Client ID) — highest friction, but user is already committed.
When ollama is selected, prompt 4 is skipped (4 prompts total).

### Auto-Derived Fields

| Field | Derivation |
|-------|------------|
| `industry_topics` | Copy of `product_keywords` (content loops handle empty gracefully; copying gives working generation) |
| `llm_model` | Provider default: `gpt-4o-mini` (openai), `claude-sonnet-4-6` (anthropic), `llama3.2` (ollama) — from `steps.rs:203-208` |
| `llm_base_url` | `http://localhost:11434/v1` for ollama, `None` otherwise |
| `product_description` | `""` (empty — generators handle gracefully) |
| `product_url` | `None` |
| `target_audience` | `""` (empty — generators handle gracefully) |
| `brand_voice` | `None` (uses framework defaults) |
| `reply_style` | `None` (uses framework defaults) |
| `content_style` | `None` (uses framework defaults) |
| `persona_opinions` | `[]` (empty — frameworks rotate without) |
| `persona_experiences` | `[]` (empty) |
| `content_pillars` | `[]` (empty) |
| `target_accounts` | `[]` (empty — discovery still works via keyword search) |
| `client_secret` | `None` (public client assumed) |
| `approval_mode` | `true` (safe default — nothing posts without review) |
| `timezone` | `"UTC"` |
| `active_hours_start` | `8` |
| `active_hours_end` | `22` |
| `active_days` | `["Mon","Tue","Wed","Thu","Fri","Sat","Sun"]` |

### Post-Write Behavior

No chaining prompts. After writing `config.toml`:

1. Print "Config saved to `~/.tuitbot/config.toml`"
2. Print single next step: `tuitbot auth`
3. Clean exit (return code 0)

No "Authenticate now?", "Validate now?", or "Start agent?" confirmations.

### Defaults Coverage

- **WizardResult fields:** 24 total
- **Quickstart collects:** 5 fields (product_name, product_keywords, llm_provider, llm_api_key, client_id)
- **Auto-derived:** 1 field (industry_topics ← product_keywords)
- **Defaulted:** 18 fields
- **Coverage:** 19 of 24 fields have values without user input = **79%**

When counting full `Config` fields (including nested scoring, safety, automation fields from `defaults.rs`):
~30 of ~35 total config fields have usable defaults = **~86%**.

---

## Advanced Path

`tuitbot init --advanced` runs the full 8-step wizard (unchanged from current behavior).

Also reachable post-setup via `tuitbot settings` for individual field editing.

The `--advanced` flag maps to the existing `InitArgs` pattern in `commands/mod.rs:42-51`.

---

## Non-Goals

1. **Web-based onboarding** — Dashboard scope; init is CLI-only.
2. **Auto-detect LLM provider from env vars** — Implicit behavior is hard to debug.
   Users should explicitly choose their provider.
3. **Config migration tooling** — Greenfield simplification; backward compat not required
   since init creates fresh configs.
4. **Ollama auto-discovery** — Would require network call in init path. Surface errors
   in `tuitbot test` instead.
5. **Guided auth inside init** — Separate complexity domain. OAuth PKCE flow has its
   own failure modes that shouldn't taint init UX.

---

## Acceptance Metrics

| Metric | Current | Target |
|--------|---------|--------|
| Quickstart prompt count | 25 | 5 |
| Time: init → config written | 3–5 min | < 60 sec |
| Time: install → first `tick --dry-run` output | 8–10 min | < 2 min |
| Defaults coverage (full config) | ~40% | ~86% |
| Post-init confirmation prompts | 4 | 0 |

---

## Hello World Definition

The complete "Hello World" experience:

```
tuitbot init          # 5 prompts → config.toml written
tuitbot auth          # OAuth 2.0 PKCE flow
tuitbot tick --dry-run # Scored tweets + optional draft reply
```

Full pipeline without mutations. **Under 2 minutes total.**

The user sees real output (scored tweets from their keywords) before investing
any additional configuration time. This is the evaluation gate — if the scored
tweets look relevant, the user is motivated to enrich their config via
`tuitbot init --advanced` or `tuitbot settings`.
