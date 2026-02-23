# 04 — Analytics Dashboard

> **Goal:** Build the main dashboard page with follower growth charts, engagement
> metrics, top-performing topics, and key stats. This is the first real screen users
> see — it needs to feel polished and data-rich.

## Prerequisites

- Tasks 01-03 completed: server API running, Tauri + Svelte scaffold with sidebar navigation.

## Context

Tuitbot already collects rich analytics data in SQLite:

- **`follower_snapshots`** — daily follower counts (via `storage::analytics`)
- **`reply_performance`** — likes, replies, impressions, performance score per reply
- **`tweet_performance`** — same metrics for original tweets
- **`content_scores`** — running averages per topic + format (epsilon-greedy)
- **`rate_limits`** — daily action counts by type
- **`action_log`** — full event history with timestamps

The dashboard page visualizes this data. The server endpoints from task 01 already
expose most of it via `GET /api/analytics/*`.

## What to build

### 1. Extend server endpoints if needed

Review existing analytics endpoints from task 01. You may need to add or adjust:

- `GET /api/analytics/followers?days=30` — should return `[{date, count}]` array
- `GET /api/analytics/summary` — new endpoint that returns a combined overview:
  ```json
  {
    "followers": { "current": 1234, "change_7d": +56, "change_30d": +203 },
    "actions_today": { "replies": 3, "tweets": 2, "threads": 0 },
    "engagement": {
      "avg_reply_score": 12.5,
      "avg_tweet_score": 8.3,
      "total_replies_sent": 156,
      "total_tweets_posted": 89
    },
    "top_topics": [
      { "topic": "Mac productivity", "avg_score": 15.2, "count": 23 },
      ...
    ]
  }
  ```

Add any missing queries to `tuitbot-core/src/storage/analytics.rs` first, then
expose them via the server route. Never put query logic in the server crate.

### 2. Dashboard page (`src/routes/+page.svelte`)

Layout (top to bottom):

```
┌──────────────────────────────────────────────────────┐
│  Stat Cards (4 across)                                │
│  [Followers ↑56]  [Replies Today]  [Tweets Today]  [Engagement] │
├──────────────────────────────────────────────────────┤
│                                                        │
│  Follower Growth Chart (30 days)                      │
│  ────────────────────────────────────                 │
│                                                        │
├──────────────────────┬───────────────────────────────┤
│                      │                                 │
│  Top Topics          │   Recent Performance           │
│  (bar chart or       │   (table: last 10 replies/     │
│   ranked list)       │    tweets with scores)         │
│                      │                                 │
└──────────────────────┴───────────────────────────────┘
```

### 3. Components to create

#### `StatCard.svelte`
- Props: `label`, `value`, `change` (optional, with +/- coloring), `icon`
- Compact card with large number, small label, change indicator

#### `FollowerChart.svelte`
- Line chart showing follower count over time
- X-axis: dates, Y-axis: follower count
- Use a lightweight chart library: **Chart.js** (`svelte-chartjs`) or **LayerCake**
  (Svelte-native). Prefer LayerCake for Svelte idiomatic approach, but Chart.js is
  fine for speed.
- Include period selector: 7d / 30d / 90d

#### `TopTopics.svelte`
- Ranked list or horizontal bar chart of top-performing topics
- Show: topic name, average performance score, post count
- Data from `GET /api/analytics/topics`

#### `RecentPerformance.svelte`
- Table showing the last 10-20 replies and tweets with:
  - Content preview (truncated)
  - Type (reply/tweet/thread)
  - Likes, replies, impressions
  - Performance score
  - Posted timestamp (relative: "2h ago")

### 4. Svelte stores

Create `src/lib/stores/analytics.ts`:

```typescript
import { writable, derived } from 'svelte/store';
import { api } from '$lib/api';

export const summary = writable<AnalyticsSummary | null>(null);
export const followers = writable<FollowerSnapshot[]>([]);

export async function loadDashboard() {
  const [s, f] = await Promise.all([
    api.analytics.summary(),
    api.analytics.followers({ days: 30 }),
  ]);
  summary.set(s);
  followers.set(f);
}
```

### 5. Auto-refresh

- Poll `summary` every 60 seconds (or use WebSocket events to trigger refresh)
- Follower chart updates when a `FollowerUpdate` WebSocket event arrives

## Design notes

- Stat cards: dark background (#1c2128), white text, subtle border. Change indicator:
  green for positive, red for negative.
- Chart: use the blue accent (#58a6ff) for the line/bars. Minimal grid lines.
  Dark chart background matching the page.
- Table: zebra striping with very subtle alternating rows. No heavy borders.
- Keep the page fast: no loading spinners if data is cached. Show skeleton placeholders
  on first load only.

## What NOT to build yet

- Drill-down into individual tweets/replies (could be a future enhancement)
- Export/download functionality
- Comparison views (week-over-week)

## Acceptance criteria

- [ ] Dashboard loads and shows 4 stat cards with real data from the API
- [ ] Follower chart renders with 30-day data and period selector works
- [ ] Top topics section shows ranked topics with scores
- [ ] Recent performance table shows last 10+ items with metrics
- [ ] Page auto-refreshes summary stats periodically
- [ ] Empty state handled gracefully (new user with no data → friendly message, not broken UI)
- [ ] Dark mode styling consistent with app shell from task 03

## Reference files

- `crates/tuitbot-core/src/storage/analytics.rs` — all analytics queries
- `crates/tuitbot-server/src/routes/analytics.rs` — API endpoints
- `dashboard/src/routes/+page.svelte` — placeholder from task 03
- `dashboard/src/lib/api.ts` — API client from task 03
