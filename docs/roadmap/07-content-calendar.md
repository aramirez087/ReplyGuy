# 07 — Content Calendar & Manual Composition

> **Goal:** Build a content calendar showing scheduled and posted content on a
> timeline, plus a manual tweet/thread composer. This is the Typefully-competing
> feature — but backed by autonomous content generation.

## Prerequisites

- Tasks 01-06 completed: full API, approval queue UI working.

## Context

Currently tuitbot generates and posts content autonomously based on configured
intervals and preferred times. Users have no visual way to see what's scheduled,
compose manual tweets, or mix autonomous and manual content.

The content calendar adds:
1. A visual timeline of past + scheduled content
2. A tweet/thread composer for manual posts
3. Slot-based scheduling (picking specific times)

Data sources:
- `original_tweets` table — posted tweets with timestamps
- `threads` + `thread_tweets` tables — posted threads
- `approval_queue` table — pending items (upcoming content)
- `schedule` config section — preferred times, active hours, timezone

## What to build

### 1. New storage queries (in tuitbot-core)

Add to `tuitbot-core/src/storage/`:

- `tweets::get_tweets_in_range(pool, from, to)` — tweets within a date range
- `threads::get_threads_in_range(pool, from, to)` — threads within a date range
- `replies::get_replies_in_range(pool, from, to)` — replies within a date range

These enable calendar-style querying by date range.

### 2. New server endpoints

#### `content.rs` (extend)

- `GET /api/content/calendar?from=2026-02-01&to=2026-02-28` — all content items
  (tweets, threads, replies) in the date range, merged into a unified timeline:
  ```json
  [
    {"id": 1, "type": "tweet", "content": "...", "posted_at": "...", "status": "posted", "performance_score": 12.5},
    {"id": 2, "type": "reply", "content": "...", "target_author": "@...", "posted_at": "...", "status": "posted"},
    {"id": 3, "type": "tweet", "content": "...", "scheduled_for": "...", "status": "pending"},
    ...
  ]
  ```

- `GET /api/content/schedule` — the configured posting schedule
  ```json
  {
    "timezone": "America/Chicago",
    "active_hours": {"start": 8, "end": 22},
    "preferred_times": ["09:15", "12:30", "17:00"],
    "preferred_times_override": {"Sat": ["11:00"], "Sun": []},
    "thread_day": "Tue",
    "thread_time": "10:00"
  }
  ```

- `POST /api/content/compose` — create a manual tweet or thread
  ```json
  {
    "type": "tweet",
    "content": "My tweet text...",
    "scheduled_for": "2026-02-24T09:15:00-06:00"  // optional, null = queue for next slot
  }
  ```
  If approval mode is on, routes to approval queue. Otherwise, schedules or posts immediately.

### 3. Manual content storage (new migration)

Add a new migration for manually composed content:

```sql
CREATE TABLE IF NOT EXISTS scheduled_content (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content_type TEXT NOT NULL,  -- 'tweet' or 'thread'
    content TEXT NOT NULL,       -- JSON: string for tweet, array for thread
    scheduled_for TEXT,          -- ISO8601 timestamp, NULL = next available slot
    status TEXT NOT NULL DEFAULT 'scheduled',  -- scheduled, posted, cancelled
    posted_tweet_id TEXT,        -- filled after posting
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

Add corresponding CRUD operations in a new `tuitbot-core/src/storage/scheduled_content.rs`.

### 4. Content calendar page (`src/routes/content/+page.svelte`)

Layout:

```
┌──────────────────────────────────────────────────────────┐
│  Content Calendar                      [+ Compose ▾]      │
│  February 2026 · America/Chicago                          │
│  [< prev]  [Today]  [next >]   View: [Week] [Month]      │
├──────────────────────────────────────────────────────────┤
│                                                            │
│  Week view (default):                                     │
│                                                            │
│  Mon 17    Tue 18    Wed 19    Thu 20    Fri 21           │
│  ────────  ────────  ────────  ────────  ────────         │
│  09:15     09:15     09:15     09:15     09:15            │
│  [tweet]   [tweet]   [reply]   [tweet]   [tweet]          │
│                                                            │
│  12:30     12:30              12:30     12:30              │
│  [reply]            10:00     [tweet]                      │
│                     [thread]                               │
│  17:00     17:00     17:00              17:00              │
│  [tweet]   [reply]   [reply]            [reply]           │
│                                                            │
│  Slot legend: ■ posted  □ scheduled  ○ available          │
│                                                            │
└──────────────────────────────────────────────────────────┘
```

### 5. Components to create

#### `CalendarWeekView.svelte`
- 7-column grid showing days of the week
- Time slots from the schedule config shown as rows
- Content items placed in their time slots
- Color coding: posted (solid), scheduled (outline), available (dot)
- Click on an available slot to open the composer with that time pre-filled

#### `CalendarMonthView.svelte`
- Traditional month grid
- Each day shows count of posts + small dots for content types
- Click a day to drill into that day's timeline

#### `ContentItem.svelte` (in calendar context)
- Small card/pill showing: content type icon, text preview (first ~40 chars)
- Click to expand: full text, performance metrics (if posted), edit/cancel (if scheduled)
- Color by type: blue for tweets, purple for threads, green for replies

#### `ComposeModal.svelte`
- Modal dialog for writing a tweet or thread
- Tab toggle: Tweet / Thread
- **Tweet mode:**
  - Textarea with character counter (280)
  - Optional scheduled time picker (defaults to next available slot)
  - Preview showing how it will look
- **Thread mode:**
  - Multiple textarea fields (one per tweet in the thread)
  - Add/remove/reorder tweets
  - Character counter per tweet
  - Numbering preview (1/N, 2/N, ...)
- Submit: calls `POST /api/content/compose`
- If approval mode: show notice "This will be added to the approval queue"

#### `TimePicker.svelte`
- Shows the configured preferred times as quick-select buttons
- Also allows custom time input
- Highlights the next available slot

### 6. Stores

Create `src/lib/stores/calendar.ts`:

```typescript
export const calendarItems = writable<CalendarItem[]>([]);
export const schedule = writable<ScheduleConfig | null>(null);

export async function loadCalendar(from: string, to: string) { ... }
export async function loadSchedule() { ... }
export async function composeContent(data: ComposeRequest) { ... }
```

### 7. Integration with automation

The content loop and thread loop in tuitbot-core should check the `scheduled_content`
table for manually scheduled items that are due. This requires a small addition to
the content loop:

- Before generating autonomous content, check if there's a scheduled item due in
  the current window
- If so, post it instead of generating new content
- Mark it as `posted` with the tweet ID

This change goes in `tuitbot-core/src/automation/content_loop.rs`.

## What NOT to build yet

- Drag-and-drop rescheduling (future enhancement)
- AI content suggestions / "generate tweet about X" button (future)
- Cross-posting to other platforms
- Recurring content schedules

## Acceptance criteria

- [ ] Calendar shows posted content from the database in the correct time slots
- [ ] Week view and month view both work with navigation
- [ ] Compose modal creates tweets with character counting
- [ ] Compose modal creates threads with multi-tweet editing
- [ ] Scheduled content appears in the calendar at the right time
- [ ] Clicking an available slot opens composer with time pre-filled
- [ ] Posted content shows performance metrics
- [ ] Scheduled content can be edited or cancelled
- [ ] Timezone is correctly displayed from config
- [ ] Empty state: available slots shown as dots, encouraging content creation

## Reference files

- `crates/tuitbot-core/src/storage/tweets.rs` — tweet storage
- `crates/tuitbot-core/src/storage/threads.rs` — thread storage
- `crates/tuitbot-core/src/storage/replies.rs` — reply storage
- `crates/tuitbot-core/src/automation/content_loop.rs` — content posting logic
- `crates/tuitbot-core/src/automation/schedule.rs` — schedule gating, preferred times
- `crates/tuitbot-core/src/config/mod.rs` — schedule config structure
- `config.example.toml` — `[schedule]` section with preferred_times
