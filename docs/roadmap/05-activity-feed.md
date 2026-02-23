# 05 — Activity Feed

> **Goal:** Build a real-time activity feed that shows everything tuitbot is doing:
> discovered tweets, scored conversations, sent replies, posted content, and errors.
> This is the "window into the brain" of the autonomous agent.

## Prerequisites

- Tasks 01-04 completed: server with WebSocket, Tauri app with dashboard page working.

## Context

Tuitbot's automation runtime performs actions continuously — discovering tweets,
scoring them, replying, posting content, measuring engagement. Currently, the only
visibility into this is CLI logs. The activity feed brings this to the dashboard
as a real-time stream.

Data sources:
- **`action_log` table** — historical actions with timestamps, types, and metadata
- **WebSocket events** — real-time `ActionPerformed` events from the server
- **`discovered_tweets` table** — tweets found and scored by discovery loop
- **`rate_limits` table** — daily action counts

## What to build

### 1. Extend server endpoints

#### `activity.rs` (extend from task 01)

- `GET /api/activity?limit=50&offset=0&type=all` — paginated action log
  - Filter by type: `all`, `reply`, `tweet`, `thread`, `follow`, `mention_reply`
  - Return: `[{id, action_type, target, content_preview, score, timestamp, metadata}]`

- `GET /api/activity/discovered?limit=20` — recently discovered tweets with their scores
  - Return: `[{tweet_id, author, text_preview, score, score_breakdown, replied, discovered_at}]`

- `GET /api/activity/rate-limits` — current daily rate limit usage
  - Return: `{replies: {used: 3, max: 5}, tweets: {used: 1, max: 6}, threads: {used: 0, max: 1}}`

Add any missing queries to `tuitbot-core` storage modules. The server crate only
calls into core — no SQL in the server.

### 2. WebSocket event enrichment

Ensure the automation runtime (from task 02's broadcast channel integration) publishes
events for:

- `TweetDiscovered` — when a tweet is found and scored (include score)
- `ActionPerformed` — when a reply/tweet/thread is sent (include content preview)
- `ActionSkipped` — when a tweet is scored below threshold or rate-limited (include reason)
- `ErrorOccurred` — when a loop encounters an error

### 3. Activity page (`src/routes/activity/+page.svelte`)

Layout:

```
┌─────────────────────────────────────────────────────┐
│  Rate Limit Bar                                       │
│  Replies: ███░░ 3/5   Tweets: █░░░░ 1/6   Threads: 0/1 │
├─────────────────────────────────────────────────────┤
│  [Filter: All ▾]  [Discovery] [Replies] [Content]     │
├─────────────────────────────────────────────────────┤
│                                                       │
│  ● 2m ago — Replied to @johndoe                      │
│    "Great point about productivity! I've found..."    │
│    Score: 72 · Archetype: AgreeAndExpand              │
│                                                       │
│  ○ 5m ago — Discovered tweet (score: 45, skipped)    │
│    @janedoe: "Anyone know a good clipboard manager?"  │
│    Below threshold (60) · Keyword match: 15/25        │
│                                                       │
│  ● 12m ago — Posted tweet                            │
│    "5 macOS keyboard shortcuts most people miss..."   │
│    Format: ListTweet                                  │
│                                                       │
│  ● 15m ago — Replied to @devuser                     │
│    "That's a great approach! One thing I'd add..."    │
│    Score: 81 · Archetype: ShareExperience             │
│                                                       │
│  [Load more...]                                       │
│                                                       │
└─────────────────────────────────────────────────────┘
```

### 4. Components to create

#### `RateLimitBar.svelte`
- Visual progress bars for daily reply/tweet/thread limits
- Color coding: green (<60%), yellow (60-80%), red (>80%)
- Updates via WebSocket or periodic polling

#### `ActivityItem.svelte`
- Props: `event` (the action/discovery event)
- Shows: icon by type, relative timestamp, author/target, content preview, score/metadata
- Different styling for: actions taken (solid dot), skipped (outline dot), errors (red)
- Expandable: click to see full content, score breakdown, tweet link

#### `ActivityFilter.svelte`
- Tab/chip filter: All, Discovery, Replies, Content, Errors
- Active filter highlights and filters the feed

#### `ScoreBreakdown.svelte` (for expanded items)
- Shows the 6 scoring signals and their values
- Small horizontal stacked bar or list:
  `Keyword: 20/25 | Followers: 12/15 | Recency: 8/10 | Engagement: 10/15 | Replies: 13/15 | Type: 10/10`

### 5. Stores

Create `src/lib/stores/activity.ts`:

- `activityFeed` — combined list of historical + real-time events
- On page load: fetch last 50 from API
- On WebSocket event: prepend to the list (cap at 200 items)
- Support filtering by type
- Support pagination (load more)

### 6. Real-time behavior

- New events appear at the top with a subtle slide-in animation
- If the user has scrolled down, show a "New events" pill at the top instead of
  auto-scrolling (don't disrupt reading)
- Auto-scroll only when the user is at the top of the feed

## What NOT to build yet

- Click-through to tweet on X (nice-to-have, add later)
- Bulk actions on discovered tweets
- Manual scoring of tweets

## Acceptance criteria

- [ ] Activity page shows rate limit bars with current daily usage
- [ ] Historical actions load from the API on page visit
- [ ] Real-time events appear via WebSocket without page refresh
- [ ] Type filters work (All, Discovery, Replies, Content, Errors)
- [ ] Each item shows appropriate metadata (score, archetype, format, timestamp)
- [ ] Expanded view shows score breakdown for discovery/reply items
- [ ] "Load more" pagination works for history
- [ ] Empty state handled (no activity yet → friendly message)
- [ ] Scrolling behavior: new events don't disrupt reading

## Reference files

- `crates/tuitbot-core/src/storage/action_log.rs` — action log queries
- `crates/tuitbot-core/src/storage/rate_limits.rs` — rate limit queries
- `crates/tuitbot-core/src/scoring/` — scoring signal breakdown
- `crates/tuitbot-server/src/routes/activity.rs` — API endpoints
- `crates/tuitbot-server/src/ws.rs` — WebSocket event types
- `dashboard/src/lib/stores/websocket.ts` — WebSocket store from task 03
