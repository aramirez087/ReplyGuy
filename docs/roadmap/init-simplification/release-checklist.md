# Init Simplification — Release Checklist

## Pre-Release Gates

All gates must pass before merge to `main`.

### 1. CI Checklist (mandatory)

```bash
cargo fmt --all && cargo fmt --all --check
RUSTFLAGS="-D warnings" cargo test --workspace
cargo clippy --workspace -- -D warnings
```

| Gate | Expected | Status |
|------|----------|--------|
| `cargo fmt --all --check` | Exit 0 | |
| `cargo test --workspace` (1,178 tests) | 0 failures | |
| `cargo clippy --workspace -- -D warnings` | 0 warnings | |

### 2. Conformance Gates (targeted)

```bash
cargo test -p tuitbot-core -- enrichment_tests      # 14 tests
cargo test -p tuitbot-cli -- init::tests             # 27 tests
cargo test -p tuitbot-cli -- test::tests             # 23 tests
cargo test -p tuitbot-cli -- tick::tests             # 15 tests
cargo test -p tuitbot-cli -- settings::tests         # 17 tests
```

| Gate | Count | Status |
|------|-------|--------|
| `enrichment_tests` | 14 | |
| `init::tests` | 27 | |
| `test::tests` | 23 | |
| `tick::tests` | 15 | |
| `settings::tests` | 17 | |
| **Total** | **96** | |

### 3. Smoke Test (end-to-end)

```bash
cargo build
chmod +x scripts/smoke-test-setup.sh
./scripts/smoke-test-setup.sh target/debug/tuitbot
```

| Test | Status |
|------|--------|
| `--help` exits 0, mentions "init" | |
| `init --help` exits 0, mentions `--force` | |
| `init --non-interactive` writes config | |
| `config.toml` exists at expected path | |
| `settings --show` reads config | |
| `test` runs without crash | |

### 4. Packaging Validation

```bash
release-plz update --config release-plz.toml --allow-dirty
cargo package --workspace --allow-dirty
```

| Gate | Status |
|------|--------|
| `release-plz update` | |
| `cargo package --workspace` | |

### 5. Documentation Audit

| Check | Status |
|-------|--------|
| No references to `tuitbot health` in any doc | |
| All CLI examples match real commands | |
| `tuitbot test` output shows all 8 checks | |
| Config section names match Config struct | |
| Quickstart flow: init → auth → test → tick --dry-run | |
| Progressive enrichment documented as optional second phase | |

## Release Steps

1. **Merge branch** `feat/init_simplification` → `main`
2. **Verify CI passes** on `main` (GitHub Actions)
3. **Let `release-plz`** create the release PR with version bumps
4. **Review release PR** — confirm changelog entries cover:
   - 5-question quickstart wizard (was 25+ prompts)
   - `tuitbot settings enrich` guided profile enrichment
   - `tick.rs` module split (structural, no logic change)
   - 33 new setup tests (96 total conformance tests)
   - Smoke test script
5. **Merge release PR** → triggers crates.io publish + binary builds
6. **Verify** published crate installs: `cargo install tuitbot-cli --locked`
7. **Follow quickstart** in docs with a clean `~/.tuitbot/` to validate user path

## Rollback Strategy

### Scenario 1: CI/Test Failure After Merge

**Symptoms:** Tests fail on `main` after merge.

**Action:**
```bash
git revert <merge-commit-sha>
git push origin main
```

**Impact:** Low — all changes are additive (new tests, new module structure, new docs). Revert removes the new code without affecting existing functionality.

### Scenario 2: Quickstart Wizard Produces Invalid Config

**Symptoms:** User reports `tuitbot init` generates config that fails `tuitbot test`.

**Diagnosis:**
```bash
tuitbot init --non-interactive   # does template path work?
tuitbot test                      # which check fails?
```

**Action:**
- If rendering bug: fix in `init/render.rs`, add regression test
- If validation regression: fix in `core/config/mod.rs`
- If urgent: users can `tuitbot init --non-interactive` + manual edit as workaround

**Impact:** Medium — workaround exists (`--non-interactive`).

### Scenario 3: Enrichment Tip Causes Tick Failure

**Symptoms:** `tuitbot tick` crashes or produces wrong JSON due to `enrichment_tip` field.

**Diagnosis:**
```bash
tuitbot tick --output json --dry-run 2>/dev/null | jq .
```

**Action:**
- `enrichment_tip` uses `skip_serializing_if = "Option::is_none"` — null case is handled
- If crash: revert `compute_enrichment_tip` function (pure function, no side effects)

**Impact:** Low — enrichment tip is display-only, never affects loop execution.

### Scenario 4: Module Split Breaks Imports

**Symptoms:** Downstream code that imports from `commands::tick` fails to compile.

**Action:** `tick` is a private module — no external consumers. Internal imports use `super::` and `crate::` which are unaffected by the file move. This scenario is not possible.

**Impact:** None.

### General Rollback

All changes in this initiative are **additive and backward-compatible**:
- No config schema changes (enrichment fields existed before, just undocumented)
- No CLI flag removals (only additions: `--advanced`, `settings enrich`)
- No database migrations
- No API changes

A full revert of the branch returns to the pre-initiative state with zero data loss.
