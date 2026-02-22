# Contributing

## Local development

```bash
cargo check --workspace
cargo test --workspace
cargo fmt --all
cargo clippy --workspace -- -D warnings
```

## Contribution scope

- bug fixes
- tests
- documentation
- performance and reliability improvements

## PR standards

- include rationale and impact
- include tests for behavior changes
- keep commits focused
- follow Conventional Commit style when possible

## Documentation updates

Any behavior, config, or CLI change should update docs in the same PR.
