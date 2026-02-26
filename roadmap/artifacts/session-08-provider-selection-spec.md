# Session 08 — Provider Selection Specification

## Overview

Introduces a `provider_backend` configuration field that selects between the
official X API (`x_api`, default) and a scraper-based backend (`scraper`).
The scraper lane is **opt-in** and carries elevated risk of account restrictions.

## Config Fields

| Field | Type | Default | Location |
|-------|------|---------|----------|
| `provider_backend` | String | `""` (→ `x_api`) | `[x_api]` in config.toml |
| `scraper_allow_mutations` | bool | `false` | `[x_api]` in config.toml |

## ProviderBackend Enum

```rust
pub enum ProviderBackend { XApi, Scraper }
```

Parsed from config string via `parse_backend()` — empty or unknown strings
default to `XApi` for backwards compatibility.

## ScraperReadProvider

Stub implementation of `SocialReadProvider`:

- **Public-data methods** (10): Return `ProviderError::Other` — "not yet implemented"
- **Auth-gated methods** (4): Return `ProviderError::NotConfigured` — instructs
  user to switch to `x_api` backend

Auth-gated: `get_user_mentions`, `get_home_timeline`, `get_me`, `get_bookmarks`.

## Mutation Gating

All mutation tools (write/engage/media) check `scraper_mutation_guard()` before
executing. When `provider_backend = "scraper"` and `scraper_allow_mutations = false`:

- Workflow profile: `scraper_mutation_guard()` returns `ScraperMutationBlocked` error
- API profile: `scraper_mutations_blocked()` method returns same error

## Error Code

`ScraperMutationBlocked` — not retryable, not transient. Added to `X_WRITE_ERR`,
`X_ENGAGE_ERR`, and media upload error groups.

## Telemetry

`telemetry::record()` accepts optional `provider_backend` parameter, encoded as
`{"provider_backend":"..."}` in the metadata JSON field.

## Capabilities

`get_capabilities` output includes a `provider` section with:
- `backend`, `mutations_available`, `risk_level`, `data_confidence`
- `unsupported_methods`, `note`
