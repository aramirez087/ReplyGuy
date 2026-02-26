# Session 08 — Scraper Risk Guardrails

## Threat Model

The scraper backend carries **elevated risk** of X account restrictions because
it uses unofficial means to interact with the platform. Mutations (posting,
liking, following) are the highest-risk operations.

## Default Posture: Deny Mutations

| Category | Default Behavior |
|----------|-----------------|
| Read tools (public data) | Stub — returns "not yet implemented" |
| Read tools (auth-gated) | Blocked — returns `NotConfigured` error |
| Write tools (post, reply, quote, delete, thread) | **Blocked** |
| Engage tools (like, follow, retweet, bookmark) | **Blocked** |
| Media upload | **Blocked** |

## Override

Set in `config.toml`:

```toml
[x_api]
provider_backend = "scraper"
scraper_allow_mutations = true
```

## Guard Implementation

### Workflow Profile

`scraper_mutation_guard(state, start)` in `x_actions/mod.rs`:
- Checks `parse_backend(config.x_api.provider_backend) == Scraper`
- Checks `!config.x_api.scraper_allow_mutations`
- Returns `ScraperMutationBlocked` error response if both true

Wired into: all 5 write functions, all 8 engage functions, upload_media.

### API Profile

`scraper_mutations_blocked()` on `ApiMcpServer`:
- Same logic as workflow guard
- Applied to: all 5 write, all 8 engage, upload_media methods

## Logging

Server init logs a `tracing::warn!` when scraper backend is selected,
including whether mutations are enabled.

## Future Work (Session 09+)

- Conformance tests proving mutation gating across both profiles
- Rate limiting specific to scraper to reduce detection risk
- Automatic fallback from scraper to X API on restriction detection
