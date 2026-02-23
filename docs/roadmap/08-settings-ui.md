# 08 — Settings UI

> **Goal:** Build a visual settings editor that replaces manual `config.toml`
> editing. Users can configure their business profile, scoring engine, safety limits,
> schedule, LLM provider, and target accounts — all from the dashboard.

## Prerequisites

- Tasks 01-07 completed: server has `GET/PATCH /api/settings`, full dashboard with
  analytics, activity, approval, and calendar pages.

## Context

Tuitbot's configuration lives in `~/.tuitbot/config.toml` with a three-layer system:
defaults → TOML file → env vars. The `tuitbot settings` CLI command provides an
interactive terminal editor. The dashboard settings page is the visual equivalent.

The server already has (from task 02):
- `GET /api/settings` — returns current config as JSON
- `PATCH /api/settings` — merges partial updates into the TOML file

## What to build

### 1. Extend server endpoints

#### `settings.rs` (extend)

- `POST /api/settings/validate` — validate a config change without saving
  Body: same as PATCH. Returns `{"valid": true}` or `{"valid": false, "errors": [...]}`

- `GET /api/settings/defaults` — return the built-in defaults for all fields
  (so the UI can show "default: 60" next to the threshold input)

- `POST /api/settings/test-llm` — test the LLM provider connectivity
  Makes a simple generation request and returns success/failure + latency

- `POST /api/settings/test-x-api` — test X API connectivity
  Calls `get_me()` and returns the authenticated user info

### 2. Settings page (`src/routes/settings/+page.svelte`)

Organize settings into sections matching the config structure. Use a tabbed or
accordion layout:

```
┌──────────────────────────────────────────────────────────┐
│  Settings                                    [Save]       │
├──────────┬───────────────────────────────────────────────┤
│          │                                                 │
│ Sections │  Business Profile                              │
│          │  ─────────────────                              │
│ ● Business│                                                │
│ ○ Content│  Product Name                                   │
│ ○ Scoring│  [Docklet                          ]            │
│ ○ Limits │                                                 │
│ ○ Schedule│ Product Description                            │
│ ○ LLM    │  [Floating command strip for macOS — media     ]│
│ ○ X API  │  [controls, clipboard, AirDrop, timers...     ]│
│ ○ Storage│                                                 │
│          │  Product URL                                    │
│          │  [https://getdocklet.app           ]            │
│          │                                                 │
│          │  Target Audience                                │
│          │  [Mac power users, developers, and productivity]│
│          │                                                 │
│          │  Product Keywords            [+ Add]            │
│          │  [macos productivity] [×]                       │
│          │  [mac menu bar] [×]                             │
│          │  [mac clipboard manager] [×]                    │
│          │                                                 │
│          │  Industry Topics             [+ Add]            │
│          │  [Mac productivity tips] [×]                    │
│          │  [macOS power user workflows] [×]               │
│          │  ...                                            │
│          │                                                 │
└──────────┴───────────────────────────────────────────────┘
```

### 3. Settings sections

#### Business Profile
- `product_name` — text input (required)
- `product_description` — textarea (required)
- `product_url` — URL input (optional)
- `target_audience` — text input (required)
- `product_keywords` — tag input (required, at least 1)
- `competitor_keywords` — tag input (optional)
- `industry_topics` — tag input (required, at least 1)
- `brand_voice` — textarea (optional, with placeholder from config.example.toml)
- `reply_style` — textarea (optional)
- `content_style` — textarea (optional)

#### Content Persona
- `persona_opinions` — tag/list input (optional)
- `persona_experiences` — tag/list input (optional)
- `content_pillars` — tag/list input (optional)

#### Scoring Engine
- `threshold` — number input with slider (0-100, default: 60)
- Six signal max values — number inputs with sliders:
  - `keyword_relevance_max` (default: 25)
  - `follower_count_max` (default: 15)
  - `recency_max` (default: 10)
  - `engagement_rate_max` (default: 15)
  - `reply_count_max` (default: 15)
  - `content_type_max` (default: 10)
- Show "Total max: XX" computed from all signal maxes
- Explain each signal with a help tooltip

#### Safety Limits
- `max_replies_per_day` — number input (default: 5)
- `max_tweets_per_day` — number input (default: 6)
- `max_threads_per_week` — number input (default: 1)
- `min_action_delay_seconds` — number input (default: 45)
- `max_action_delay_seconds` — number input (default: 180)
- `max_replies_per_author_per_day` — number input (default: 1)
- `banned_phrases` — tag input
- `product_mention_ratio` — slider 0.0-1.0 (default: 0.2) with percentage display
- `approval_mode` — toggle switch

#### Schedule
- `timezone` — searchable dropdown of IANA timezone names
- `active_hours_start` / `active_hours_end` — hour pickers (0-23) with visual time bar
- `active_days` — day-of-week checkboxes
- `preferred_times` — time inputs with "auto" option
- `preferred_times_override` — per-day time inputs (collapsible)
- `thread_preferred_day` — day dropdown
- `thread_preferred_time` — time input

#### LLM Provider
- `provider` — select: OpenAI / Anthropic / Ollama
- `api_key` — password input with show/hide toggle
- `model` — text input (with suggestions based on provider)
- `base_url` — text input (optional, shown for Ollama)
- [Test Connection] button → calls `POST /api/settings/test-llm`

#### X API
- `client_id` — text input (read-only display, linking to developer portal)
- `client_secret` — password input (optional)
- `auth.mode` — select: manual / local_callback
- [Test Connection] button → calls `POST /api/settings/test-x-api`

#### Storage
- `db_path` — text input (with tilde expansion note)
- `retention_days` — number input (default: 90, 0 = forever)

### 4. Components to create

#### `SettingsSection.svelte`
- Wrapper component for each section with title, description
- Collapsible on mobile (though desktop-only, good for long pages)

#### `TagInput.svelte`
- For array fields (keywords, topics, phrases)
- Type to add, click X to remove, shows as chips
- Supports paste-to-add-multiple (comma-separated)

#### `SliderInput.svelte`
- Number input paired with a range slider
- Shows current value, default value, and min/max
- Optional help text tooltip

#### `TimeRangeBar.svelte`
- Visual 24-hour bar for active hours
- Draggable start/end handles
- Highlighted active zone, grayed inactive zone
- Supports wrapping ranges (e.g., 22:00 - 06:00)

#### `ConnectionTest.svelte`
- Button that triggers a test, shows spinner, then success/failure
- Success: green checkmark + latency (e.g., "Connected — 142ms")
- Failure: red X + error message

### 5. Form behavior

- **Auto-save vs manual save:** Use manual save with a sticky "Save" button.
  Show unsaved changes indicator (dot on the Save button).
- **Validation:** Validate on blur and on save. Show inline errors below fields.
  Call `POST /api/settings/validate` before saving.
- **Defaults:** Show "(default: X)" hint next to each field. If the value matches
  the default, show it in a lighter color.
- **Dangerous changes:** Warn before saving changes to X API credentials or LLM
  provider (these affect active automation).

### 6. Stores

Create `src/lib/stores/settings.ts`:

```typescript
export const settings = writable<Config | null>(null);
export const defaults = writable<Config | null>(null);
export const unsavedChanges = writable(false);

export async function loadSettings() { ... }
export async function saveSettings(partial: Partial<Config>) { ... }
export async function testLlm() { ... }
export async function testXApi() { ... }
```

## What NOT to build yet

- Import/export config files
- Config presets or templates
- Multi-account configuration
- OAuth re-authentication flow from the dashboard

## Acceptance criteria

- [ ] All config sections display with current values from the API
- [ ] Editing fields and saving persists changes to config.toml
- [ ] Validation catches invalid values (empty required fields, out-of-range numbers)
- [ ] Tag inputs work for array fields (add, remove, paste multiple)
- [ ] Sliders sync with number inputs
- [ ] Time range bar visualizes active hours correctly (including wrapping)
- [ ] LLM connection test works (success and failure states)
- [ ] X API connection test works
- [ ] Unsaved changes indicator shows when form is dirty
- [ ] Default values shown as hints
- [ ] Dangerous change warnings display before saving credential changes

## Reference files

- `crates/tuitbot-core/src/config/mod.rs` — full config structure with defaults
- `config.example.toml` — all fields with documentation
- `crates/tuitbot-server/src/routes/settings.rs` — settings API endpoints
- `crates/tuitbot-cli/src/` — existing CLI settings command for reference
