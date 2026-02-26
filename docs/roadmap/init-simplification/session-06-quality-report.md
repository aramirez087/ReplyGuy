# Session 06 — Quality Report

## Test Matrix

| Test File | Module | Before S06 | After S06 | Delta |
|-----------|--------|------------|-----------|-------|
| `tuitbot-cli` `init::tests` | init wizard helpers, TOML rendering, enrichment invariant | 21 | 27 | +6 |
| `tuitbot-cli` `test::tests` | auth evaluation, LLM connectivity, business profile, LLM config, database | 11 | 23 | +12 |
| `tuitbot-cli` `tick::tests` | LoopFilter, compute_enrichment_tip, TickOutput serialization | 0 | 15 | +15 |
| `tuitbot-cli` `settings::tests` | settings formatting, display helpers | 17 | 17 | 0 |
| `tuitbot-core` `enrichment_tests` | ProfileCompleteness, EnrichmentStage | 14 | 14 | 0 |
| **Total CLI tests** | | **49** | **82** | **+33** |
| **Total core tests** | | 718 | 718 | 0 |

## Coverage by Command

### `tuitbot init`
- **Covered:** TOML rendering roundtrips, helper functions, quickstart→enrichment invariant (6 new tests ensure quickstart configs always trigger enrichment hints).
- **Not covered:** Interactive prompt flows (requires TTY mock), file I/O edge cases (permissions, disk full).

### `tuitbot test`
- **Covered:** Auth evaluation (all branches), business profile validation (4 new), LLM config validation (6 new), database check (2 new), next-step guidance.
- **Not covered:** `check_llm_connectivity` (requires network — tested only via provider creation mock).

### `tuitbot tick`
- **Covered:** LoopFilter construction from CLI args (4 tests), enrichment tip computation (5 tests), JSON serialization contracts (6 tests).
- **Not covered:** Full `execute()` flow (requires RuntimeDeps initialization with real DB and X API client). Process lock acquisition. PostExecutor drain timeout.

### `tuitbot settings enrich`
- **Covered:** Via `settings::tests` (17 existing) + enrichment stage logic in `tuitbot-core` (14 existing).
- **Not covered:** Interactive dialoguer prompts.

## Regression Guards

| Test | Prevents |
|------|----------|
| `quickstart_config_not_enriched` | Quickstart accidentally setting enrichment fields |
| `quickstart_config_all_enrichment_stages_incomplete` | ProfileCompleteness miscounting quickstart as enriched |
| `quickstart_config_next_incomplete_is_voice` | Wrong first-stage recommendation after quickstart |
| `quickstart_config_one_line_summary_all_dashes` | Status display regression for unenriched profiles |
| `advanced_config_fully_enriched` | Enrichment detection breaking for fully-configured profiles |
| `enrichment_tip_empty_config_suggests_voice` | compute_enrichment_tip suggesting wrong stage |
| `enrichment_tip_contains_settings_enrich` | Tip text missing the actionable command |
| `tick_output_json_omits_null_enrichment_tip` | JSON bloat from null enrichment_tip field |
| `loop_filter_default_all_enabled` | --loops flag regression disabling loops by default |
| `check_business_profile_*` | False positives/negatives in preflight validation |
| `check_llm_config_*` | Provider validation accepting invalid or rejecting valid configs |
| `check_database_*` | Database check crashing on missing files or wrong messages |

## Remaining Risk

| Risk | Severity | Mitigation |
|------|----------|------------|
| Interactive prompt flows untested | Medium | Manual testing; dialoguer mocking is complex and fragile |
| `tick execute()` requires full runtime | Medium | Covered by smoke test script + dry-run integration |
| File I/O edge cases (permissions, disk full) | Low | OS-level; not worth mocking |
| Auth token chaining (init → auth → test) | Medium | Covered by smoke test script end-to-end flow |
| Process lock contention | Low | Simple `fs2` call; tested by OS guarantees |
| LLM network connectivity | Low | Provider creation tested; actual connectivity requires network |

## Smoke Test

The `scripts/smoke-test-setup.sh` script validates the setup architecture end-to-end:

```bash
chmod +x scripts/smoke-test-setup.sh
./scripts/smoke-test-setup.sh target/debug/tuitbot
```

Tests performed:
1. `tuitbot --help` exits 0 and mentions "init"
2. `tuitbot init --help` exits 0 and mentions `--force`
3. `tuitbot init --non-interactive` writes config to temp dir
4. Written `config.toml` exists
5. `tuitbot settings --show` reads config successfully
6. `tuitbot test` runs without crashing (failures expected: no auth)

All tests are offline — no API keys or auth tokens required.

## Recommended CI Gates

Add these as a `setup-conformance` step in `.github/workflows/ci.yml`:

```yaml
- name: Setup conformance gates
  run: |
    cargo test -p tuitbot-core -- enrichment_tests
    cargo test -p tuitbot-cli -- init::tests
    cargo test -p tuitbot-cli -- test::tests
    cargo test -p tuitbot-cli -- tick::tests
    cargo test -p tuitbot-cli -- settings::tests
```

This ensures all setup-related tests run as named conformance gates, making failures easy to triage. The existing `cargo test --workspace` already runs them all, but named gates provide better signal in CI dashboards.

## Structural Change

`tick.rs` (768 lines, exceeding 500-line limit) was converted to `tick/mod.rs` + `tick/tests.rs` following the existing pattern used by `init/`, `test/`, and `settings/`. No logic changes — only module reorganization and test addition.
