# 06 — Approval Queue UI

> **Goal:** Build a visual approval queue that replaces `tuitbot approve` CLI.
> Users can review AI-generated content, edit before approving, and batch-process
> items. This is the human-in-the-loop control center.

## Prerequisites

- Tasks 01-05 completed: server has approval CRUD endpoints + WebSocket, Tauri app
  has working activity feed.

## Context

When `approval_mode = true` in config, tuitbot queues all generated content instead
of posting it directly. Currently, users review items via `tuitbot approve --list`
and `tuitbot approve --approve <id>` in the CLI. The dashboard replaces this with
a visual review experience.

The approval queue table stores:
- `id`, `action_type` (reply/tweet/thread), `status` (pending/approved/rejected)
- `target_tweet_id`, `target_author` (for replies)
- `content` (the generated text)
- `topic`, `archetype` (metadata about generation)
- `score` (tweet score for replies)
- `created_at`, `updated_at`

Server endpoints from task 02:
- `GET /api/approval?status=pending`
- `POST /api/approval/:id/approve`
- `POST /api/approval/:id/reject`
- `POST /api/approval/approve-all`

## What to build

### 1. Extend server endpoints

#### New/modified endpoints:

- `PATCH /api/approval/:id` — edit content before approving
  Body: `{"content": "edited text..."}`
  Updates the content field without changing status.

- `GET /api/approval?status=pending&status=approved&status=rejected` — support
  multiple status filters (default: pending only)

- `GET /api/approval/stats` — counts by status
  Return: `{"pending": 5, "approved": 23, "rejected": 8}`

Add the content-edit query to `tuitbot-core/src/storage/approval_queue.rs` if it
doesn't exist.

### 2. Approval page (`src/routes/approval/+page.svelte`)

Layout:

```
┌─────────────────────────────────────────────────────────┐
│  Approval Queue                          [Approve All ▾] │
│  5 pending · 23 approved · 8 rejected                    │
├──────────┬──────────────────────────────────────────────┤
│ Filters  │                                                │
│          │  ┌──────────────────────────────────────────┐ │
│ ○ All    │  │  Reply to @johndoe                 72pts │ │
│ ● Pending│  │  ─────────────────────────────────────── │ │
│ ○ Approved│ │  Original: "Anyone know a good clipboard │ │
│ ○ Rejected│ │  manager for Mac?"                       │ │
│          │  │                                          │ │
│ ──────── │  │  Generated reply:                        │ │
│ Type     │  │  ┌────────────────────────────────────┐  │ │
│ □ Replies│  │  │ I've been using the built-in       │  │ │
│ □ Tweets │  │  │ clipboard history (Cmd+Shift+V)    │  │ │
│ □ Threads│  │  │ and it's surprisingly capable...   │  │ │
│          │  │  └────────────────────────────────────┘  │ │
│          │  │                                          │ │
│          │  │  Archetype: ShareExperience              │ │
│          │  │  Topic: Mac productivity                 │ │
│          │  │  Queued: 5 minutes ago                   │ │
│          │  │                                          │ │
│          │  │  [Edit] [Approve ✓] [Reject ✗]          │ │
│          │  └──────────────────────────────────────────┘ │
│          │                                                │
│          │  ┌──────────────────────────────────────────┐ │
│          │  │  Original tweet · Score: 68              │ │
│          │  │  ... (next item)                         │ │
│          │  └──────────────────────────────────────────┘ │
└──────────┴──────────────────────────────────────────────┘
```

### 3. Components to create

#### `ApprovalCard.svelte`
- Shows the full context: original tweet (for replies), generated content, metadata
- For replies: show original tweet text, author, score, archetype
- For tweets: show the generated tweet, format type, topic
- For threads: show all tweets in the thread, structure type
- Action buttons: Edit, Approve, Reject
- Status badge: pending (yellow), approved (green), rejected (red)

#### `ApprovalEditor.svelte`
- Inline editing of the generated content (textarea that replaces the display text)
- Character counter (280 limit for tweets, per-tweet for threads)
- Save/Cancel buttons
- On save: `PATCH /api/approval/:id` then refresh

#### `ApprovalActions.svelte`
- Approve button: sends `POST /api/approval/:id/approve`, item slides out
- Reject button: sends `POST /api/approval/:id/reject`, item slides out
- Keyboard shortcuts: `a` = approve, `r` = reject, `e` = edit, `j/k` = navigate
- Toast notification on success

#### `BulkActions.svelte`
- "Approve All" dropdown in the header
- Confirmation dialog: "Approve all 5 pending items?"
- Select multiple items for batch approve/reject

#### `ApprovalStats.svelte`
- Small stat line at the top showing counts by status
- Links to filter by status

### 4. Stores

Create `src/lib/stores/approval.ts`:

```typescript
export const approvalItems = writable<ApprovalItem[]>([]);
export const approvalStats = writable<{ pending: number; approved: number; rejected: number }>();

export async function loadApproval(status = 'pending') { ... }
export async function approveItem(id: number) { ... }
export async function rejectItem(id: number) { ... }
export async function editItem(id: number, content: string) { ... }
export async function approveAll() { ... }
```

### 5. Real-time updates

- When a new `ApprovalQueued` WebSocket event arrives:
  - Increment the pending counter
  - If currently viewing pending items, prepend the new item with animation
  - Show a subtle notification badge on the "Approval" sidebar link

- Add a badge/count to the sidebar Approval link showing pending count

### 6. Keyboard navigation

- `j` / `k` or arrow keys to navigate between cards
- `a` to approve focused card
- `r` to reject focused card
- `e` to enter edit mode on focused card
- `Escape` to cancel edit

## What NOT to build yet

- Regeneration (asking the LLM to generate a new version) — future enhancement
- Scheduling approved items for specific times
- Thread-specific editing (reordering tweets within a thread)

## Acceptance criteria

- [ ] Pending items display with full context (original tweet for replies, metadata)
- [ ] Approve/reject work and items move to correct status
- [ ] Edit mode allows modifying content before approving
- [ ] Character counter shows tweet length
- [ ] Bulk approve-all works with confirmation dialog
- [ ] Status filters work (pending/approved/rejected/all)
- [ ] Type filters work (replies/tweets/threads)
- [ ] Real-time: new queued items appear without refresh
- [ ] Sidebar badge shows pending count
- [ ] Keyboard shortcuts work for navigation and actions
- [ ] Empty state: "No pending items — you're all caught up!"

## Reference files

- `crates/tuitbot-core/src/storage/approval_queue.rs` — approval queue queries
- `crates/tuitbot-server/src/routes/approval.rs` — approval API endpoints
- `crates/tuitbot-server/src/ws.rs` — `ApprovalQueued` event
- `dashboard/src/lib/stores/websocket.ts` — WebSocket store
- `crates/tuitbot-cli/src/` — look at the existing CLI approve command for reference
  on what data is displayed
