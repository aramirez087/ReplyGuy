# 03 — Tauri + Svelte Dashboard Scaffold

> **Goal:** Set up the Tauri desktop app with a Svelte frontend, wire it to
> `tuitbot-server`, and build the app shell (sidebar navigation, layout, API client).
> After this task, you have a running desktop app with an empty dashboard.

## Prerequisites

- Task 01 + 02 completed: `tuitbot-server` has full REST + WebSocket API with auth.

## Context

We're building a native desktop app using Tauri v2. The architecture:

```
Tauri app (native shell)
  └─ spawns tuitbot-server as a sidecar process on launch
  └─ loads Svelte SPA from embedded assets
  └─ Svelte connects to http://localhost:3001 for API + WebSocket
```

Tauri gives us: system tray, native menus, auto-update, OS-level notifications,
and a ~5MB binary (vs Electron's ~150MB).

## What to build

### 1. Initialize the Svelte + Tauri project

In the repo root, create the `dashboard/` directory:

```bash
npm create svelte@latest dashboard    # choose: Skeleton, TypeScript, ESLint, Prettier
cd dashboard
npm install
npm install -D @tauri-apps/cli@next
npx tauri init
```

Choose these Tauri options:
- App name: `Tuitbot`
- Window title: `Tuitbot`
- Dev server URL: `http://localhost:5173`
- Frontend build command: `npm run build`
- Frontend dev command: `npm run dev`

### 2. Configure Tauri

Edit `dashboard/src-tauri/tauri.conf.json`:

- `identifier`: `com.tuitbot.app`
- `windows[0].title`: `Tuitbot`
- `windows[0].width`: 1200
- `windows[0].height`: 800
- `windows[0].minWidth`: 900
- `windows[0].minHeight`: 600
- `build.beforeDevCommand`: `npm run dev`
- `build.beforeBuildCommand`: `npm run build`
- `build.frontendDist`: `../build`

### 3. Sidecar setup

The Tauri Rust code (`dashboard/src-tauri/src/main.rs`) should:

- On app start: spawn `tuitbot-server` as a child process (use `Command::new` or
  Tauri's sidecar feature)
- Read the API token from `~/.tuitbot/api_token`
- Pass the token to the frontend via Tauri's `invoke` / `window.__TAURI__` API
- On app quit: send SIGTERM to the server process for graceful shutdown
- Handle the case where the server is already running (port in use) gracefully

### 4. Install frontend dependencies

```bash
cd dashboard
npm install -D tailwindcss @tailwindcss/vite
npm install @tauri-apps/api@next
npm install lucide-svelte           # icons
```

Set up TailwindCSS v4 with the Vite plugin.

### 5. App shell layout

Create the main layout with sidebar navigation:

```
┌──────────────┬──────────────────────────────────┐
│              │                                    │
│  [logo]      │   Page content                    │
│              │                                    │
│  Dashboard   │                                    │
│  Activity    │                                    │
│  Approval    │                                    │
│  Content     │                                    │
│  Targets     │                                    │
│  Settings    │                                    │
│              │                                    │
│              │                                    │
│  ──────────  │                                    │
│  [status]    │                                    │
│  Running ●   │                                    │
└──────────────┴──────────────────────────────────┘
```

Files:
- `src/routes/+layout.svelte` — sidebar + main content area
- `src/lib/components/Sidebar.svelte` — navigation links, runtime status indicator
- `src/routes/+page.svelte` — dashboard home (placeholder for task 04)
- `src/routes/activity/+page.svelte` — placeholder
- `src/routes/approval/+page.svelte` — placeholder
- `src/routes/content/+page.svelte` — placeholder
- `src/routes/targets/+page.svelte` — placeholder
- `src/routes/settings/+page.svelte` — placeholder

### 6. API client

Create `src/lib/api.ts`:

```typescript
const BASE_URL = 'http://localhost:3001';
let token: string = '';

export function setToken(t: string) { token = t; }

async function request<T>(path: string, options?: RequestInit): Promise<T> {
  const res = await fetch(`${BASE_URL}${path}`, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${token}`,
      ...options?.headers,
    },
  });
  if (!res.ok) {
    const body = await res.json().catch(() => ({ error: res.statusText }));
    throw new Error(body.error || res.statusText);
  }
  return res.json();
}

export const api = {
  health: () => request<{ status: string; version: string }>('/api/health'),
  // ... expand in later tasks
};
```

### 7. WebSocket store

Create `src/lib/stores/websocket.ts`:

```typescript
import { writable } from 'svelte/store';

export const events = writable<WsEvent[]>([]);
export const connected = writable(false);

export function connectWs(token: string) {
  const ws = new WebSocket(`ws://localhost:3001/api/ws?token=${token}`);
  ws.onopen = () => connected.set(true);
  ws.onclose = () => { connected.set(false); /* reconnect logic */ };
  ws.onmessage = (e) => {
    const event = JSON.parse(e.data);
    events.update(list => [event, ...list].slice(0, 200));
  };
}
```

### 8. SvelteKit config

Configure `svelte.config.js` for SPA mode (no SSR — Tauri serves static files):

```javascript
import adapter from '@sveltejs/adapter-static';

export default {
  kit: {
    adapter: adapter({ fallback: 'index.html' }),
  },
};
```

## Design guidelines

- **Color scheme:** Dark mode by default (match developer tools aesthetic). Use a
  neutral dark gray base (#0f1117) with a subtle sidebar (#161b22). Accent: blue (#58a6ff).
- **Typography:** System font stack (`-apple-system, BlinkMacSystemFont, ...`). 14px base.
- **Components:** Functional and clean. No excessive decoration. Data density over whitespace.
- **Responsive:** Not needed (desktop-only), but sidebar should be collapsible.

## What NOT to build yet

- Actual page content for any route (tasks 04-09)
- Charts or data visualizations (task 04)
- Any API calls beyond health check (later tasks)

## Acceptance criteria

- [ ] `cd dashboard && npm run dev` starts the Svelte dev server
- [ ] `cd dashboard && npm run tauri dev` opens a native window with the sidebar layout
- [ ] Clicking sidebar links navigates between placeholder pages
- [ ] The API client successfully calls `/api/health` and displays the result somewhere
- [ ] WebSocket connects and shows connection status in the sidebar
- [ ] Sidebar shows runtime status (dot indicator: green = running, gray = stopped)
- [ ] TailwindCSS works with dark mode styles
- [ ] `npm run build` produces a static build in `dashboard/build/`

## Reference files

- `crates/tuitbot-server/src/main.rs` — server entry point, port config
- `crates/tuitbot-server/src/auth.rs` — where the API token file is stored
- `crates/tuitbot-server/src/ws.rs` — WebSocket event types
