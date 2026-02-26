# Session 05 Handoff — Progressive Profile Enrichment

## What Changed

After quickstart, enrichment fields (brand_voice, reply_style, content_style, persona_opinions, persona_experiences, content_pillars, target_accounts, competitor_keywords) are all empty. Content works but is generic. This session adds a guided `tuitbot settings enrich` workflow and surfaces completeness hints at existing touch points.

### Three Enrichment Stages

| Stage | Fields | Complete When | Impact |
|-------|--------|---------------|--------|
| **Voice** | brand_voice, reply_style, content_style | ANY is non-empty | Shapes every LLM output |
| **Persona** | persona_opinions, persona_experiences, content_pillars | ANY is non-empty | Makes content authentic |
| **Targeting** | targets.accounts, business.competitor_keywords | EITHER has entries | Focuses discovery |

### Changes Summary

1. **Core types** (`tuitbot-core/config/mod.rs`) — `EnrichmentStage` enum, `ProfileCompleteness` struct, `Config::profile_completeness()` method, plus 14 unit tests.
2. **Guided flow** (`settings/enrich.rs`) — New module with `run_enrichment()` that walks through incomplete stages, reusing existing `edit_category_voice/persona/targets` editors.
3. **Settings routing** (`settings/mod.rs`) — `"enrich" | "enrichment" | "profile"` category shortcut.
4. **Interactive menu** (`settings/interactive.rs`) — "Enrich Profile" added as first menu item.
5. **Test command hint** (`test/mod.rs`) — After all checks pass, prints profile completeness and next-stage tip.
6. **Tick dry-run tip** (`tick.rs`) — Generic tip replaced with context-aware enrichment tip; `enrichment_tip` field added to `TickOutput` (backward-compatible JSON via `skip_serializing_if`).

## Files Changed

| File | Change |
|------|--------|
| `crates/tuitbot-core/src/config/mod.rs` | +EnrichmentStage, ProfileCompleteness, profile_completeness(), 14 tests |
| `crates/tuitbot-cli/src/commands/settings/enrich.rs` | New: guided enrichment flow |
| `crates/tuitbot-cli/src/commands/settings/mod.rs` | +mod enrich, +category routing |
| `crates/tuitbot-cli/src/commands/settings/interactive.rs` | +Enrich Profile menu item |
| `crates/tuitbot-cli/src/commands/test/mod.rs` | +print_enrichment_hint() |
| `crates/tuitbot-cli/src/commands/tick.rs` | +enrichment_tip field, compute_enrichment_tip() |

## Expected UX

### `tuitbot test` (all checks pass, no enrichment)

```
All checks passed.

Profile: Voice --  Persona --  Targeting --
Tip: Run `tuitbot settings enrich` to configure voice (shapes every LLM-generated reply and tweet)

Ready! Try: tuitbot tick --dry-run
```

### `tuitbot settings enrich` (nothing configured)

```
Profile Enrichment
──────────────────

  Voice        --   shapes every LLM-generated reply and tweet
  Persona      --   makes content authentic with opinions and experiences
  Targeting    --   focuses discovery on specific accounts and competitors

Each stage improves content quality. Press Enter to skip any stage.

? Configure Voice? (shapes every LLM-generated reply and tweet) Yes
[interactive voice editor]

? Configure Persona? (makes content authentic with opinions and experiences) No

? Configure Targeting? (focuses discovery on specific accounts and competitors) Yes
[interactive targeting editor]

Updated status:
  Voice        OK   shapes every LLM-generated reply and tweet
  Persona      --   makes content authentic with opinions and experiences
  Targeting    OK   focuses discovery on specific accounts and competitors
```

### `tuitbot settings enrich` (fully enriched)

```
Profile Enrichment
──────────────────

  Voice        OK   shapes every LLM-generated reply and tweet
  Persona      OK   makes content authentic with opinions and experiences
  Targeting    OK   focuses discovery on specific accounts and competitors

All enrichment stages are complete. Nice work!
Use `tuitbot settings voice`, `persona`, or `targets` to fine-tune.
```

### `tuitbot tick --dry-run` (voice missing)

```
Result: success

Tip: Run `tuitbot settings enrich` to configure voice — shapes every LLM-generated reply and tweet
```

## Verification

```bash
cargo fmt --all && cargo fmt --all --check     # clean
RUSTFLAGS="-D warnings" cargo test --workspace  # all pass
cargo clippy --workspace -- -D warnings         # clean
cargo test -p tuitbot-core -- enrichment_tests   # 14 enrichment tests pass
```

## What's NOT Changed

- `init/` — No changes to the quickstart wizard
- `prompts.rs`, `wizard.rs`, `main.rs`, `deps.rs`, `upgrade.rs` — Untouched
- JSON output schema — `enrichment_tip` is optional, backward-compatible

## Next Steps

- Session 06 could add richer enrichment prompts (example suggestions, style previews)
- Consider auto-detecting partial enrichment from existing config on `tuitbot upgrade`
- Dashboard could surface enrichment progress as a setup checklist
