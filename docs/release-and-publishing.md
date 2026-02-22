# Release and Publishing

## CI release model

Workflow: `.github/workflows/release.yml`

1. Every push to `main` runs release jobs.
2. `release-plz release-pr` maintains/updates release PRs.
3. Merging release PR publishes crates and creates release tags.

## Tags

- `tuitbot-core-vX.Y.Z`
- `tuitbot-mcp-vX.Y.Z`
- `tuitbot-cli-vX.Y.Z`

## GitHub releases and binary assets

Only CLI releases produce GitHub release assets:

- linux x86_64
- macOS Intel
- macOS Apple Silicon
- windows x86_64
- `SHA256SUMS`

## Required repository secrets

- `CARGO_REGISTRY_TOKEN`: crates.io API token for publish.
- `RELEASE_PLZ_TOKEN` (optional): PAT for broader automation trigger behavior.

## First-time crates.io requirements

The crates.io account bound to `CARGO_REGISTRY_TOKEN` must have a verified email.

Profile settings:

- <https://crates.io/settings/profile>

## Manual verification

```bash
release-plz update --config release-plz.toml --allow-dirty
cargo check --workspace
```
