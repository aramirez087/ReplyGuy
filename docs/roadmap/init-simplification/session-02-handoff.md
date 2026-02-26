# Session 02 Handoff — Config Architecture for Quickstart/Advanced Boundary

## Decision: Keep `BusinessProfile` flat

Splitting into sub-structs would change TOML layout (`[business]` → `[business]` + `[enrichment]`), cascade into 19+ consumer sites, settings commands, env var overrides, and render functions. The existing type system (`Option<String>` vs `String`, `Vec<String>` defaults) already communicates the boundary. Instead, we added accessor methods and doc comments.

## What Changed

### 1. `BusinessProfile` methods (config/mod.rs)

Three new methods in `impl BusinessProfile`:

- **`quickstart(product_name, product_keywords) -> Self`** — Constructor setting only required fields. Copies keywords into `industry_topics`.
- **`effective_industry_topics() -> &[String]`** — Returns `industry_topics` if non-empty, falls back to `product_keywords`. The key derivation rule.
- **`is_enriched() -> bool`** — True if any enrichment field is set (brand_voice, reply_style, content_style, persona_*, content_pillars). For Session 05 progressive enrichment hints.

Struct doc comment updated to delineate quickstart / optional context / enrichment tiers.

### 2. `target_audience` empty-string fix (content/generator.rs)

Three methods embedded `target_audience` directly into format strings. Now wrapped in a conditional — if empty, the audience clause is omitted entirely. Same pattern as `voice_section`.

Affected methods:
- `generate_reply_with_archetype()`
- `generate_tweet_with_format()`
- `generate_thread_with_structure()`

### 3. Runtime consumers → `effective_industry_topics()`

All runtime code paths that read `industry_topics` now call the accessor instead of the raw field. This ensures quickstart configs (empty `industry_topics`) automatically derive topics from `product_keywords`.

**Changed sites:**
- `cli/commands/tick.rs` — content and thread loop construction
- `cli/commands/run.rs` — content and thread loop construction
- `core/context/engagement.rs` — keyword collection for scoring
- `server/routes/strategy.rs` — API response (inputs endpoint)
- `cli/commands/test.rs` — diagnostic display
- `mcp/server/workflow.rs` — default topic fallback for generate_tweet/generate_thread
- `mcp/tools/scoring.rs` — keyword collection
- `mcp/tools/workflow/composite/find_opportunities.rs` — keyword collection
- `mcp/tools/workflow/composite/thread_plan.rs` — relevance check

**NOT changed** (intentionally show/edit raw configured value):
- `settings/show.rs`, `settings/set.rs`, `settings/interactive.rs`
- `settings/render.rs`, `init/render.rs`
- `init/display.rs`, `init/steps.rs`
- Test fixtures

### 4. `config.example.toml` restructured

Business section now has visual groupings:
- **Quickstart (required):** `product_name`, `product_keywords`
- **Optional context:** `product_description`, `product_url`, `target_audience`, `competitor_keywords`, `industry_topics` (with note: "Defaults to product_keywords if omitted")
- **Enrichment:** `brand_voice`, `reply_style`, `content_style`, `persona_opinions`, `persona_experiences`, `content_pillars`

### 5. `render_config_toml()` quickstart-aware

Empty `industry_topics`, `target_audience`, and `product_description` are rendered as comments so TOML stays valid while signalling these fields are optional.

### 6. Validation rules (unchanged)

`Config::validate()` requires:
- `business.product_name` non-empty
- `business.product_keywords` or `business.competitor_keywords` non-empty
- Valid `llm.provider` + `api_key` for openai/anthropic

No changes needed — quickstart already satisfies these.

## Tests Added

**config/mod.rs:**
- `quickstart_minimal_config_validates` — quickstart fields pass `validate()`
- `quickstart_industry_topics_derived_from_keywords` — accessor returns keywords when topics empty
- `explicit_industry_topics_override_derived` — explicit topics take precedence
- `is_enriched_false_for_quickstart` — quickstart constructor produces unenriched profile
- `is_enriched_true_with_brand_voice` — enrichment field flips the flag

**init/tests.rs:**
- `render_quickstart_minimal_is_valid_toml` — roundtrip quickstart-minimal WizardResult
- `render_quickstart_omits_empty_fields` — empty fields rendered as comments

## Follow-ups

- **File split:** `config/mod.rs` is ~1720 lines (exceeds 500-line CLAUDE.md limit). Deferred to a dedicated session.
- **Session 03 inputs:** CLI init rebuild needs a quickstart variant of `WizardResult` that only collects 5 fields.
- **Session 05 inputs:** `is_enriched()` is ready for progressive enrichment hints in the dashboard/CLI.
