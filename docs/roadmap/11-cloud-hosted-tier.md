# 11 — Cloud Hosted Tier (Plausible Model)

> **Goal:** Add a paid cloud-hosted option alongside the existing desktop app.
> Users who don't want to self-host pay $19-29/mo for a managed instance with
> always-on automation. This is the Plausible model: open-source core, paid cloud,
> self-hostable.

## Prerequisites

- Tasks 01-10 completed: desktop app is shipping, users are paying for it, you have
  product-market fit signal. Do NOT start this task until you have paying desktop
  customers confirming the product works.

## Context

The desktop app (Phase 1) validated the product. Now we're adding a cloud tier
where you run the infrastructure and users get:

- Always-on automation (no laptop-sleeping problem)
- Zero setup (no API key management for X — you provide the OAuth app)
- Web dashboard (same UI, no Tauri/install needed)
- Managed updates (always on latest version)

Users who want control keep self-hosting. Everyone else pays monthly.

The key architectural shift: `tuitbot-server` goes from single-user localhost
to multi-tenant cloud deployment. The core crate stays untouched.

## What to build

### 1. Multi-tenant architecture

The simplest multi-tenant model for tuitbot: **one SQLite database per user**.
No shared tables, no row-level isolation complexity. Each user gets their own
`{user_id}.db` file and their own config. This matches the desktop model exactly —
each "tenant" is identical to a self-hosted instance.

Create `crates/tuitbot-server/src/tenant.rs`:

```rust
pub struct TenantManager {
    data_dir: PathBuf,  // e.g., /data/tenants/
    tenants: DashMap<UserId, Arc<TenantState>>,
}

pub struct TenantState {
    pub db: DbPool,
    pub config: TuitbotConfig,
    pub runtime_handle: Option<RuntimeHandle>,
    pub event_tx: broadcast::Sender<WsEvent>,
}
```

- On user login: load or create their tenant state
- Each tenant has its own DB pool, config, and automation runtime
- Idle tenants (no WebSocket connections, no dashboard open) keep the runtime
  running but drop the DB pool after timeout (reopen on next request)

### 2. User authentication

Replace the local file token auth with real user accounts.

#### Option A: Self-built (simpler, fewer dependencies)
- `users` table in a shared management database (separate from per-tenant DBs)
- Email + password with argon2 hashing
- Session tokens (JWT or opaque tokens in Redis/SQLite)
- Email verification on signup
- Password reset flow

#### Option B: Auth provider (faster, more features)
- Use Clerk, Auth0, or Supabase Auth
- OAuth login (Google, GitHub) — lower friction for developers
- Handles email verification, password reset, MFA out of the box

**Recommendation:** Start with Option A (email + password + GitHub OAuth via a
small custom implementation). Avoid vendor lock-in early. Add more providers later.

Create `crates/tuitbot-server/src/auth/` with:
- `mod.rs` — middleware that extracts user from session token
- `password.rs` — argon2 hash/verify
- `session.rs` — session create/validate/revoke
- `oauth.rs` — GitHub OAuth flow (optional but high-value)

New routes:
- `POST /auth/signup` — create account (email, password)
- `POST /auth/login` — authenticate, return session token
- `POST /auth/logout` — revoke session
- `GET /auth/me` — current user info + subscription status
- `POST /auth/forgot-password` — send reset email
- `POST /auth/reset-password` — reset with token

### 3. Subscription billing (Stripe)

Integrate Stripe for subscription management.

#### Pricing tiers:
| Tier | Price | Limits |
|------|-------|--------|
| **Starter** | $19/mo | 5 replies/day, 3 tweets/day, 1 thread/week, 3 targets |
| **Growth** | $29/mo | 15 replies/day, 10 tweets/day, 3 threads/week, 10 targets |
| **Pro** | $49/mo | 30 replies/day, 20 tweets/day, 7 threads/week, unlimited targets |

Create `crates/tuitbot-server/src/billing.rs`:

- Stripe Checkout for new subscriptions
- Stripe Customer Portal for managing subscriptions
- Webhook handler (`POST /webhooks/stripe`) for:
  - `checkout.session.completed` — activate subscription
  - `invoice.paid` — extend subscription
  - `invoice.payment_failed` — grace period warning
  - `customer.subscription.deleted` — deactivate, stop automation

New routes:
- `POST /api/billing/checkout` — create Stripe Checkout session
- `GET /api/billing/portal` — redirect to Stripe Customer Portal
- `GET /api/billing/status` — current plan, usage, renewal date

Enforce tier limits in the automation runtime: override `max_replies_per_day` etc.
based on the user's plan. The tenant config is the source of truth — billing just
sets the ceilings.

### 4. X API OAuth (managed app)

For cloud users, you provide the X API OAuth app so they don't need developer
portal access. This is the biggest UX win over self-hosting.

- Register a single X API OAuth 2.0 app under your developer account
- Cloud users authenticate via OAuth flow (you store their refresh tokens)
- Each user's tokens are stored encrypted in their tenant DB
- Token refresh loop runs per-tenant (already exists in core)

Create `crates/tuitbot-server/src/x_oauth.rs`:

- `GET /api/x/connect` — start OAuth flow, redirect to X authorization URL
- `GET /api/x/callback` — handle OAuth callback, store tokens
- `DELETE /api/x/disconnect` — revoke tokens, disconnect account

**Security considerations:**
- Encrypt refresh tokens at rest (use `aes-gcm` with a server-level key)
- The server-level encryption key lives in environment variables, not in code
- Users can revoke access at any time from their X settings
- Rate limit the OAuth flow to prevent abuse

### 5. LLM provider handling

Two options for cloud users:

**Option A: BYOK (Bring Your Own Key)** — users enter their OpenAI/Anthropic key
in settings, stored encrypted per-tenant. You pay zero LLM costs.

**Option B: Managed LLM** — you proxy LLM calls through your own API key, cost
absorbed into subscription price. Simpler for users, but you eat the cost.

**Recommendation:** Start with BYOK. Add managed LLM as an add-on later ($5-10/mo
extra). This keeps margins healthy and avoids subsidizing heavy LLM users.

### 6. Dashboard as web app

The Svelte dashboard already works as a standalone web app (SvelteKit with static
adapter). For cloud hosting:

- Switch to `@sveltejs/adapter-node` for server-side rendering (better SEO for
  marketing pages, faster initial load)
- Add marketing pages: `/` (landing), `/pricing`, `/login`, `/signup`
- The authenticated dashboard lives at `/dashboard/*` (same routes as desktop)
- Add a layout guard that redirects unauthenticated users to `/login`

The desktop app (Tauri) continues using the static adapter. You now have two build
targets:

```bash
# Desktop (static SPA, served by Tauri)
cd dashboard && npm run build:desktop

# Cloud (Node.js server, SSR for marketing pages)
cd dashboard && npm run build:cloud
```

### 7. Automation runtime management

Each cloud user gets their own automation runtime instance. The tenant manager
handles lifecycle:

- **On subscription activation:** Create tenant state, start runtime
- **On dashboard visit:** Ensure runtime is running, connect WebSocket
- **On subscription cancellation:** Stop runtime, keep data for 30 days (grace period)
- **On account deletion:** Stop runtime, schedule data deletion

Resource management:
- Each runtime is lightweight (6 tokio tasks + a DB pool)
- A single server can handle ~500-1000 tenants depending on hardware
- Monitor per-tenant memory and CPU usage
- Add health checks: if a tenant's runtime crashes, auto-restart it

### 8. Docker deployment

Create `Dockerfile` and `docker-compose.yml` at the repo root:

```dockerfile
# Multi-stage build
FROM node:20-slim AS frontend
WORKDIR /app/dashboard
COPY dashboard/package*.json ./
RUN npm ci
COPY dashboard/ ./
RUN npm run build:cloud

FROM rust:1.75-slim AS backend
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY migrations/ migrations/
RUN cargo build --release -p tuitbot-server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=backend /app/target/release/tuitbot-server /usr/local/bin/
COPY --from=frontend /app/dashboard/build /srv/dashboard
ENV TUITBOT_DASHBOARD_DIR=/srv/dashboard
ENV TUITBOT_DATA_DIR=/data
VOLUME /data
EXPOSE 3001
CMD ["tuitbot-server", "--cloud"]
```

```yaml
# docker-compose.yml
services:
  tuitbot:
    build: .
    ports: ["3001:3001"]
    volumes: ["tuitbot_data:/data"]
    environment:
      - STRIPE_SECRET_KEY=${STRIPE_SECRET_KEY}
      - STRIPE_WEBHOOK_SECRET=${STRIPE_WEBHOOK_SECRET}
      - X_API_CLIENT_ID=${X_API_CLIENT_ID}
      - X_API_CLIENT_SECRET=${X_API_CLIENT_SECRET}
      - ENCRYPTION_KEY=${ENCRYPTION_KEY}
      - SMTP_URL=${SMTP_URL}
    restart: unless-stopped

volumes:
  tuitbot_data:
```

This same Docker image works for:
- **Your cloud hosting** — deploy to fly.io, Railway, or a VPS
- **Self-hosters** — `docker pull tuitbot/tuitbot && docker compose up`

### 9. Cloud deployment

Recommended: **Fly.io** or **Railway** for initial deployment.

- Single machine to start (scale later)
- Persistent volume for `/data` (tenant databases)
- Automatic TLS via the platform
- Deploy via `fly deploy` or `railway up`

Domain: `app.tuitbot.dev` (or similar)

### 10. Email transactional

You'll need to send emails for:
- Account verification
- Password reset
- Payment receipts (Stripe handles these)
- Weekly performance summaries (optional, high-value feature)
- Approval queue reminders ("You have 5 items waiting")

Use Resend, Postmark, or AWS SES. Create a small email module:
- `crates/tuitbot-server/src/email.rs`
- Template-based (plain text first, HTML later)
- Queue emails via a background tokio task

### 11. Marketing site

The landing page at `/` should sell the product. Key pages:

- **`/`** — hero, features, social proof, CTA
- **`/pricing`** — tier comparison table, FAQ
- **`/login`** and **`/signup`** — auth forms
- **`/docs`** — self-hosting guide, API reference

These are SvelteKit pages with SSR enabled for SEO. The authenticated dashboard
at `/dashboard/*` is SPA (client-side only).

## Migration path for existing desktop users

Desktop app users who want to switch to cloud:

1. Export: `tuitbot export --format json` (new CLI command) — dumps their config +
   historical data
2. Import: upload the JSON in cloud dashboard settings
3. Their SQLite data becomes the tenant DB, config maps to cloud settings
4. Disconnect local runtime, cloud takes over

## What NOT to build yet

- Team/agency features (multiple X accounts per subscription)
- API access for third-party integrations
- White-label / reseller program
- Mobile app
- Advanced analytics (compared to competitors, industry benchmarks)

## Acceptance criteria

- [ ] Users can sign up with email + password
- [ ] Stripe checkout creates a subscription and activates the account
- [ ] X API OAuth flow connects the user's X account
- [ ] Automation runtime starts and runs 24/7 for paying users
- [ ] Dashboard works in the browser at `app.tuitbot.dev` (no Tauri needed)
- [ ] Per-user data isolation (each user has their own DB)
- [ ] Subscription cancellation stops automation and shows grace period notice
- [ ] Tier limits enforce reply/tweet/thread maximums
- [ ] Docker image builds and runs with `docker compose up`
- [ ] Self-hosters can deploy with the same Docker image
- [ ] Desktop app continues to work unchanged for self-host users

## Reference files

- `crates/tuitbot-server/src/` — everything from tasks 01-02
- `crates/tuitbot-core/src/automation/mod.rs` — runtime lifecycle
- `crates/tuitbot-core/src/config/mod.rs` — config structure
- `dashboard/` — Svelte frontend from tasks 03-09
- `docs/roadmap/10-packaging-and-distribution.md` — desktop packaging (stays as-is)
