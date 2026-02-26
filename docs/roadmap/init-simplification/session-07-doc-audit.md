# Session 07 — Documentation Audit

## Before vs After Command Flow

### Before (old docs)

```
README.md CLI section:
  cargo install tuitbot-cli --locked
  tuitbot init          ← no explanation of what happens
  tuitbot auth
  tuitbot run           ← skips test and dry-run entirely

getting-started.md:
  tuitbot init          ← no mention of quickstart vs advanced
  tuitbot auth
  tuitbot test          ← no expected output shown
  tuitbot run | tick    ← "Choose an Execution Mode" with no dry-run path
  tuitbot health        ← DOES NOT EXIST as a CLI command

cli-reference.md:
  tuitbot init          ← no flags documented (--advanced, --force, --non-interactive missing)
  tuitbot auth          ← no flags documented (--mode missing)
  tuitbot health        ← DOES NOT EXIST
  tuitbot settings      ← missing enrich, voice, persona, targets subcommands
  tuitbot tick          ← missing --require-approval flag
  (no backup/restore commands documented)

configuration.md:
  References "features" section ← DOES NOT EXIST in Config struct
  References "scheduling" section ← actual section is "schedule"
  No mention of quickstart config or progressive enrichment
  No mention of tuitbot init generating config
```

### After (new docs)

```
README.md CLI section:
  cargo install tuitbot-cli --locked
  tuitbot init                         ← "5 quick questions → config ready"
  tuitbot auth                         ← "authenticate with X"
  tuitbot test                         ← "verify everything works"
  tuitbot tick --dry-run               ← "see the bot in action (no posts)"
  Explains quickstart, mentions --advanced, links to settings enrich

getting-started.md:
  "Hello World in Under 2 Minutes" section with 4 steps
  Each step shows exact command AND expected output
  Progressive Enrichment as explicit optional second phase
  Links to --advanced and --non-interactive as alternatives
  No reference to tuitbot health

cli-reference.md:
  All subcommands with all flags documented
  init: --force, --non-interactive, --advanced
  auth: --mode manual|local_callback
  settings: enrich, voice, persona, targets, business, schedule, limits
  tick: --dry-run, --loops, --ignore-schedule, --require-approval
  backup/restore commands included
  Global options documented (-c, -v, -q, --output)
  Environment variables and precedence documented
  Exit codes documented

configuration.md:
  Quickstart vs Advanced Config table (5 required fields)
  Progressive Enrichment section with 3 stages
  Correct section names matching Config struct
  Safety defaults table with actual values
  No stale "features" or "scheduling" references
```

## Removed Complexity Points

| # | What was removed/changed | Why |
|---|--------------------------|-----|
| 1 | `tuitbot health` references (getting-started, cli-reference) | Command does not exist in the CLI |
| 2 | Implicit "choose an execution mode" as first decision | Users now see dry-run first, then choose daemon vs scheduler |
| 3 | Init without explanation of what it asks | Quickstart (5 questions) is now front and center |
| 4 | No expected outputs in any doc | Every step now shows what users will see |
| 5 | `features` config section reference | Does not exist — removed |
| 6 | `scheduling` config section reference | Corrected to `schedule` |
| 7 | Missing CLI flags across all commands | All flags now documented from actual source code |
| 8 | No enrichment guidance | Progressive enrichment is now a documented second phase |
| 9 | No `backup`/`restore` in cli-reference | Both commands now documented |
| 10 | Advanced setup mixed with quickstart | Advanced is now explicitly a separate path (`--advanced` flag) |
| 11 | Getting-started ended at `tuitbot run` | Now ends at `tuitbot tick --dry-run` (safe first success) |
| 12 | Configuration doc had no "what's required" table | Quickstart vs Advanced table with 5 required fields |

## Remaining Doc Debt

| Item | Location | Priority | Notes |
|------|----------|----------|-------|
| `docs/troubleshooting.md` may reference `tuitbot health` | Not checked (not in scope) | Low | Grep confirmed no match in `docs/` |
| `docs/operations.md` may have stale setup instructions | Not rewritten | Medium | Should verify after this session |
| `docs/composer-mode.md` may not mention enrichment | Not checked | Low | Enrichment applies to both modes equally |
| `config.example.toml` (2 copies) may not match quickstart framing | Root + CLI crate | Medium | Template comments still describe all fields as equal; could benefit from quickstart/enrichment grouping |
| Desktop app onboarding wizard | Not in scope (GUI) | N/A | Desktop wizard is separate from CLI docs |
| MCP reference | Not in scope | N/A | Already complete and accurate |

## Files Changed

| File | Change |
|------|--------|
| `README.md` | Replaced CLI section with quickstart-first flow + expected outputs. Expanded Quick Commands with enrichment and backup. |
| `docs/getting-started.md` | Full rewrite. "Hello World in Under 2 Minutes" section. 4-step flow with expected outputs. Progressive enrichment as optional second phase. Removed `tuitbot health`. |
| `docs/cli-reference.md` | Full rewrite. All subcommands with all flags. Added init flags, auth --mode, settings enrich/voice/persona/targets, backup/restore, global options, env vars, exit codes. Removed `tuitbot health`. |
| `docs/configuration.md` | Full rewrite. Quickstart vs Advanced table. Progressive enrichment section. Fixed section names. Safety defaults with actual values. Removed stale references. |

## Verification

All CLI examples in docs match real commands verified against:
- `crates/tuitbot-cli/src/main.rs` (subcommand definitions)
- `crates/tuitbot-cli/src/commands/mod.rs` (argument structs)
- `crates/tuitbot-cli/src/commands/init/mod.rs` (init flow)
- `crates/tuitbot-cli/src/commands/init/display.rs` (printed output)
- `crates/tuitbot-cli/src/commands/settings/enrich.rs` (enrichment flow)
- `crates/tuitbot-cli/src/commands/tick/mod.rs` (tick args and output)
- `crates/tuitbot-cli/src/commands/test/mod.rs` (test checks and output)
