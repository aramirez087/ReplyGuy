# 09 — Target Accounts UI

> **Goal:** Build a target account management page showing monitored accounts,
> relationship progress, interaction history, and controls for adding/removing targets.

## Prerequisites

- Tasks 01-08 completed: full dashboard with analytics, activity, approval, calendar,
  and settings pages.

## Context

Tuitbot's target monitoring loop watches specific X accounts and engages with their
tweets. This builds relationships with key people in the user's space. The CLI has
no dedicated UI for this — targets are just a list in config.toml.

The dashboard brings visibility to:
- Which accounts are being monitored
- How many interactions have happened with each
- Recent tweets from targets and whether tuitbot replied
- Follow status and warmup progress
- Controls to add/remove targets

Data sources:
- `target_accounts` table — target state (username, user_id, follow status, added date)
- `target_tweets` table — tweets fetched from target accounts
- `author_interactions` table — per-author reply counts
- `replies_sent` table — replies to target tweets (filtered by target author)
- Config `[targets]` section — target list, auto-follow, warmup settings

## What to build

### 1. Extend server endpoints

#### `targets.rs` (extend from tasks 01-02)

- `GET /api/targets` — list all target accounts with enriched data:
  ```json
  [
    {
      "username": "pmarca",
      "user_id": "...",
      "followed": true,
      "follow_date": "2026-02-10T...",
      "warmup_complete": true,
      "total_interactions": 12,
      "interactions_today": 1,
      "last_interaction_at": "2026-02-23T...",
      "added_at": "2026-02-05T..."
    }
  ]
  ```

- `GET /api/targets/:username/timeline` — recent tweets from this target + our
  replies to them:
  ```json
  [
    {
      "tweet_id": "...",
      "text": "...",
      "posted_at": "...",
      "our_reply": {  // null if we didn't reply
        "content": "...",
        "replied_at": "...",
        "performance": { "likes": 3, "replies": 1 }
      },
      "score": 72
    }
  ]
  ```

- `GET /api/targets/:username/stats` — interaction stats for a specific target:
  ```json
  {
    "total_replies": 12,
    "avg_score": 74.5,
    "best_reply": { "content": "...", "performance_score": 28.5 },
    "first_interaction": "2026-02-06T...",
    "interaction_frequency": "every 2.3 days"
  }
  ```

Add the necessary join queries to `tuitbot-core/src/storage/target_accounts.rs`
and `tuitbot-core/src/storage/replies.rs`. Keep all SQL in core.

### 2. Target accounts page (`src/routes/targets/+page.svelte`)

Layout:

```
┌──────────────────────────────────────────────────────────┐
│  Target Accounts                         [+ Add Target]   │
│  6 accounts monitored · 3 replies today                   │
├──────────────────────────────────────────────────────────┤
│                                                            │
│  ┌──────────────────────────────────────────────────────┐ │
│  │  @pmarca                              12 interactions │ │
│  │  ✓ Following · Warmup complete                        │ │
│  │  Last interaction: 2 days ago                         │ │
│  │  ████████████░░░░ 1/3 today                          │ │
│  │                                         [View] [Remove]│ │
│  └──────────────────────────────────────────────────────┘ │
│                                                            │
│  ┌──────────────────────────────────────────────────────┐ │
│  │  @naval                               8 interactions  │ │
│  │  ✓ Following · Warmup complete                        │ │
│  │  Last interaction: 5 hours ago                        │ │
│  │  ██████████████████ 2/3 today                        │ │
│  │                                         [View] [Remove]│ │
│  └──────────────────────────────────────────────────────┘ │
│                                                            │
│  ┌──────────────────────────────────────────────────────┐ │
│  │  @newaccount                          0 interactions  │ │
│  │  ✓ Following · Warmup: 2 days remaining              │ │
│  │  Added 1 day ago                                      │ │
│  │                                         [View] [Remove]│ │
│  └──────────────────────────────────────────────────────┘ │
│                                                            │
└──────────────────────────────────────────────────────────┘
```

### 3. Target detail view (`src/routes/targets/[username]/+page.svelte`)

Layout:

```
┌──────────────────────────────────────────────────────────┐
│  ← Back to Targets                                        │
│                                                            │
│  @pmarca                                                  │
│  ✓ Following · 12 total interactions · avg score: 74.5    │
│  First interaction: Feb 6 · Frequency: every 2.3 days     │
├──────────────────────────────────────────────────────────┤
│                                                            │
│  Interaction Timeline                                     │
│  ─────────────────────                                    │
│                                                            │
│  Feb 23 — Their tweet:                                    │
│  "The best products solve problems people didn't know..."  │
│  Score: 72                                                 │
│  → Our reply: "This resonates — we see this pattern..."   │
│    ♥ 3  💬 1  👁 450                                       │
│                                                            │
│  Feb 21 — Their tweet:                                    │
│  "Every great company starts with a contrarian truth..."   │
│  Score: 58 (below threshold, skipped)                     │
│                                                            │
│  Feb 19 — Their tweet:                                    │
│  "Software is eating the world, but taste is eating..."    │
│  Score: 81                                                 │
│  → Our reply: "The taste gap is real. I've noticed..."    │
│    ♥ 7  💬 2  👁 1200                                      │
│                                                            │
└──────────────────────────────────────────────────────────┘
```

### 4. Components to create

#### `TargetCard.svelte`
- Shows: username, follow status, warmup progress, total interactions, daily usage bar
- Actions: View (navigate to detail), Remove (with confirmation)
- Visual indicators: warmup progress bar, daily limit usage

#### `AddTargetModal.svelte`
- Text input for username (without @)
- On submit: `POST /api/targets` → validates the username exists on X, adds to monitoring
- Show error if username not found
- Optionally show a preview of the account (follower count, bio) before confirming

#### `InteractionTimeline.svelte`
- Chronological list of target's tweets + our replies
- Each item shows: tweet text, score, our reply (if any), reply performance
- Distinguishes: replied (green), skipped-below-threshold (gray), skipped-rate-limited (yellow)

#### `WarmupProgress.svelte`
- Small visual showing days remaining in follow warmup
- Progress bar: `[███░░░] 1/3 days`
- Complete state: checkmark

#### `DailyLimitBar.svelte`
- Mini progress bar showing target reply usage today (e.g., 1/3)
- Same color coding as rate limit bars in activity feed

### 5. Stores

Create `src/lib/stores/targets.ts`:

```typescript
export const targets = writable<TargetAccount[]>([]);

export async function loadTargets() { ... }
export async function addTarget(username: string) { ... }
export async function removeTarget(username: string) { ... }
export async function loadTargetTimeline(username: string) { ... }
export async function loadTargetStats(username: string) { ... }
```

### 6. Real-time updates

- When a `TargetReply` WebSocket event arrives, update the relevant target's
  interaction count and last_interaction_at
- Increment daily usage counter in real-time

## What NOT to build yet

- Target account suggestions (based on followers/following)
- Target grouping/categories
- Direct messaging targets
- Target import from CSV/list

## Acceptance criteria

- [ ] Target list shows all monitored accounts with enriched data
- [ ] Add target modal validates username and adds to monitoring
- [ ] Remove target works with confirmation dialog
- [ ] Target detail page shows interaction timeline
- [ ] Timeline distinguishes replied vs skipped tweets with reasons
- [ ] Reply performance metrics shown (likes, replies, impressions)
- [ ] Follow status and warmup progress displayed correctly
- [ ] Daily limit usage bar updates correctly
- [ ] Empty state: "No target accounts yet — add accounts to build relationships"

## Reference files

- `crates/tuitbot-core/src/storage/target_accounts.rs` — target account queries
- `crates/tuitbot-core/src/storage/author_interactions.rs` — per-author interaction counts
- `crates/tuitbot-core/src/storage/replies.rs` — reply queries
- `crates/tuitbot-core/src/automation/target_loop.rs` — target monitoring logic
- `crates/tuitbot-server/src/routes/targets.rs` — target API endpoints
- `config.example.toml` — `[targets]` section
