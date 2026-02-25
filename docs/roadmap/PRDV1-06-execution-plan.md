# PRDV1-06 Execution Plan: Bilingual + Brand Voice + QA Gates

## Objective

Ship a production-safe quality gate so every generated artifact (reply, tweet, thread draft) gets a structured QA report, hard flags block approval/publish by default, and reviewers can override with explicit audited notes.

## Scope Boundaries

In scope:
- Structured quality policy config (`brand_voice_profile`, `language_policy`, `link_policy`, `glossary_terms`).
- Deterministic QA evaluator pipeline in `tuitbot-core`.
- QA persistence on draft + approval artifacts.
- Hard-flag enforcement and override workflow.
- Dashboard surfaces for QA report and override actions.
- Tests and docs.

Out of scope:
- New X tools and posting capabilities (PRDV1-04).
- Agency/multi-account tenancy (PRDV1-09).
- Major redesign of existing composer UX.

## Current Baseline (from code)

- Existing freeform voice fields are already present: `business.brand_voice`, `business.reply_style`, `business.content_style`.
- Existing safety checks exist for banned phrases + semantic dedupe.
- `approval_queue` already stores governance metadata (`reason`, `detected_risks`, review info, edit history).
- Drafts currently live in `scheduled_content` (`status = draft`) and are published by enqueueing approval rows.
- Approve/reject endpoints currently do not enforce QA/hard-flag gating.

## Design Decisions

1. Additive, backward-compatible config only.
2. Persist QA as JSON text columns in existing tables (no cross-table join required for UI/API).
3. Keep one QA schema for both drafts and approval items.
4. Distinguish hard vs soft flags explicitly and gate only on hard flags.
5. Re-evaluate QA on every edit to prevent stale overrides.
6. Preserve current freeform brand fields as fallback prompt context.

## Data Model Plan

### 1) Config schema additions (`crates/tuitbot-core/src/config/mod.rs`)

Add top-level fields to `Config`:
- `brand_voice_profile: BrandVoiceProfileConfig`
- `language_policy: LanguagePolicyConfig`
- `link_policy: LinkPolicyConfig`
- `glossary_terms: Vec<GlossaryTermConfig>`

Add validation rules:
- `language_policy.supported_languages` must include at least `en` and/or `es`.
- `language_policy.default_reply_language` must be in supported set.
- `link_policy.allowlist` and `denylist` cannot overlap.
- UTM required params must be non-empty and valid keys.
- Length constraints must be positive and within safe max.

Add env overrides for critical fields:
- `TUITBOT_LANGUAGE_POLICY__DEFAULT_REPLY_LANGUAGE`
- `TUITBOT_LANGUAGE_POLICY__MODE`
- `TUITBOT_BRAND_VOICE_PROFILE__FORBIDDEN_WORDS`
- `TUITBOT_LINK_POLICY__ALLOWLIST`
- `TUITBOT_LINK_POLICY__DENYLIST`

### 2) QA report schema (`crates/tuitbot-core/src/safety/mod.rs`)

Introduce shared structs:
- `QaReport`
- `QaFlag` (`code`, `severity`, `message`, `evidence`, `suggestion`)
- `QaScoreSummary` (`overall`, `language`, `brand`, `compliance`)
- `QaLanguages` (`source`, `output`, `policy_target`)

Severity mapping:
- Hard: language mismatch (policy), forbidden words/phrases, denied domain, missing required UTM, disallowed claims.
- Soft: style drift, low glossary confidence, near-length warning, high similarity warning.

### 3) DB migrations

Create one new migration, additive only:
- `migrations/20260226xxxxxx_qa_gates.sql`

`approval_queue` new columns:
- `qa_report TEXT DEFAULT '{}'`
- `qa_hard_flags TEXT DEFAULT '[]'`
- `qa_soft_flags TEXT DEFAULT '[]'`
- `qa_recommendations TEXT DEFAULT '[]'`
- `qa_score REAL DEFAULT 0`
- `qa_requires_override INTEGER DEFAULT 0`
- `qa_override_by TEXT DEFAULT NULL`
- `qa_override_note TEXT DEFAULT NULL`
- `qa_override_at TEXT DEFAULT NULL`

`scheduled_content` new columns:
- `qa_report TEXT DEFAULT '{}'`
- `qa_hard_flags TEXT DEFAULT '[]'`
- `qa_soft_flags TEXT DEFAULT '[]'`
- `qa_recommendations TEXT DEFAULT '[]'`
- `qa_score REAL DEFAULT 0`

## QA Evaluator Pipeline

Primary file: `crates/tuitbot-core/src/safety/mod.rs` (plus `dedup.rs` reuse).

Evaluation steps:
1. Language detect source text (`en`/`es` minimum).
2. Language detect generated text.
3. Resolve target language from `language_policy`.
4. Enforce language match policy.
5. Glossary preservation check (term unchanged or approved alias).
6. Forbidden words/phrases check (legacy + profile).
7. Claims check (rule-based patterns from profile).
8. Semantic dedupe check (existing checker; configurable threshold).
9. Link extraction and policy checks (allow/deny + UTM).
10. Length/emoji policy checks.
11. Build score + recommendations.

Implementation detail:
- Add `safety::qa` submodule for parser/check functions and keep `SafetyGuard` API stable.

## Integration Plan by Flow

### 1) Generation-time QA hooks

Core:
- `crates/tuitbot-core/src/content/generator.rs`
  - Add helper to produce prompt constraints from structured config.
  - Preserve legacy brand fields for compatibility.

Server API:
- `crates/tuitbot-server/src/routes/discovery.rs`
  - `compose_reply` returns `qa_report` with content.
  - `queue_reply` stores QA artifact and sets pending/approved based on hard flags.
- `crates/tuitbot-server/src/routes/content.rs`
  - `compose`/`create_draft` compute and store QA report immediately.
  - `publish_draft` blocks if hard flags unless override path used.
- `crates/tuitbot-server/src/routes/assist.rs` (optional but recommended)
  - Return `qa_report` in assist responses so compose UI can preview risk early.

Automation runtime:
- `crates/tuitbot-core/src/automation/adapters.rs`
  - Evaluate QA before queueing approval actions in approval mode.

### 2) Approval enforcement

Storage:
- `crates/tuitbot-core/src/storage/approval_queue/mod.rs`
- `crates/tuitbot-core/src/storage/approval_queue/queries.rs`
  - Include QA columns in select tuple, serde serialization.
  - Add update methods for QA and override metadata.

Routes:
- `crates/tuitbot-server/src/routes/approval.rs`
  - On approve: reject with `400`/`409` if `qa_requires_override = true` and no override.
  - Add endpoint:
    - `POST /api/approval/{id}/override`
    - body: `{ actor: string, note: string }` (note required, non-empty).
  - Re-run QA on edit and clear previous override metadata when content changes.
  - `approve-all` skips blocked items and returns `blocked_ids`.

### 3) Dashboard/API contract updates

API client:
- `dashboard/src/lib/api.ts`
  - Extend `ApprovalItem` and draft/compose response types with QA fields.
  - Add `approval.override(id, actor, note)` client method.

Approval UI:
- `dashboard/src/lib/components/ApprovalCard.svelte`
- `dashboard/src/lib/stores/approval.ts`
- `dashboard/src/routes/(app)/approval/+page.svelte`
  - Show score, hard/soft flag groups, recommendations.
  - Show explicit "Override Hard Flags" action with required note modal.
  - Disable Approve button while hard flags unresolved.

Settings UI:
- `dashboard/src/routes/(app)/settings/+page.svelte`
- new section component (recommended): `QualityPolicySection.svelte`
  - Manage language policy, brand profile constraints, link policy, glossary.

Compose surfaces:
- `dashboard/src/routes/(app)/discovery/+page.svelte`
- `dashboard/src/lib/components/ComposeModal.svelte`
  - Show QA preview from server response before queue/submit.

## Execution Slices (recommended PR order)

1. Slice A: Config + types + validation
   - New config structs, serde defaults, validation, docs update for config example.
2. Slice B: QA evaluator core
   - `QaReport` models + evaluator + unit tests (language, glossary, links, hard/soft mapping).
3. Slice C: DB + storage
   - Migration + storage queries/serializers + storage tests.
4. Slice D: Server enforcement + override API
   - Route logic, gating, override endpoint, edit re-evaluation.
5. Slice E: Dashboard integration
   - API types, stores, approval card, settings policy section, compose preview.
6. Slice F: Hardening
   - Batch approve behavior, edge cases, docs and verification sweep.

## Test Plan

Core tests:
- `cargo test -p tuitbot-core safety`
- Add:
  - language detection (`en`/`es`) and same-language default behavior.
  - glossary preservation.
  - link allow/deny + UTM enforcement.
  - hard vs soft flag classification.

Content tests:
- `cargo test -p tuitbot-core content`
- Add:
  - prompt policy embedding.
  - fallback behavior when new config absent.

Server tests:
- `cargo test -p tuitbot-server content`
- Add:
  - approval blocked on hard flags.
  - override requires non-empty note.
  - edit clears stale override and re-runs QA.
  - `approve-all` returns blocked IDs correctly.

Dashboard checks:
- `cd dashboard && npm run check`

## Risk Register

Risk: false positives on short language samples.
- Mitigation: confidence threshold + fallback to source language + soft flag downgrade when low confidence.

Risk: policy regressions from strict link rules.
- Mitigation: default permissive allowlist + only denylist hard-blocks by default.

Risk: stale QA after manual edits.
- Mitigation: mandatory re-evaluation on any content edit.

Risk: friction in approval throughput.
- Mitigation: one-click override with explicit audit note and clear visual reason.

## Acceptance Mapping (Task 6)

- Every draft has QA payload.
  - Enforced via QA columns on `scheduled_content` at draft creation/update.
- Hard flags block publishing until override/edit.
  - Enforced in approval/publish endpoints with explicit override endpoint.
- Reply language follows configured policy.
  - Evaluator checks source/output vs policy target.
- Glossary terms remain unmodified.
  - Glossary preservation checker produces hard/soft flags and recommendations.
