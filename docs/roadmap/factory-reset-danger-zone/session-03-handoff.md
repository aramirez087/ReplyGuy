# Session 03 Handoff -- Factory Reset Danger Zone

## Completed Work

1. **API client method** (`dashboard/src/lib/api.ts`):
   - Added `factoryReset(confirmation)` to `api.settings`. Uses the
     existing `request<T>()` helper which handles bearer/cookie auth,
     CSRF headers, and error throwing automatically.

2. **Auth store cleanup** (`dashboard/src/lib/stores/auth.ts`):
   - Added `clearSession()` export. Clears CSRF token and sets
     `authMode` to `'none'` without making an API call. Used after
     factory reset because the server has already deleted all sessions
     (calling `logout()` would fail with 401).

3. **Settings store cleanup** (`dashboard/src/lib/stores/settings.ts`):
   - Added `resetStores()` export. Nulls out `config`, `defaults`,
     `draft`; resets `loading` to `true`; clears all error and
     validation state. Prevents stale data from flashing if the user
     navigates to settings before completing re-onboarding.

4. **DangerZoneSection component** (`dashboard/src/routes/(app)/settings/DangerZoneSection.svelte`):
   - Self-contained Svelte 5 component (~170 lines).
   - Wraps `SettingsSection` with danger-themed overrides: red-tinted
     border and red icon background via scoped `:global()` selectors
     on a `.danger-zone` wrapper div.
   - Warning text explaining what factory reset does.
   - Two-column grid: "What gets deleted" vs "What is preserved".
   - Typed confirmation input with placeholder `RESET TUITBOT`.
   - Red "Factory Reset" button, disabled until exact phrase match.
   - Loading state with `Loader2` spinner icon during API call.
   - Error state below button in danger color.
   - On success: `clearSession()` + `resetStores()` + `disconnectWs()`
     + `goto('/onboarding')` with `window.location.href` fallback.

5. **Settings page integration** (`dashboard/src/routes/(app)/settings/+page.svelte`):
   - Imported `DangerZoneSection` and `AlertTriangle` icon.
   - Added `{ id: 'danger', label: 'Danger', icon: AlertTriangle }` to
     the `sections` nav array.
   - Rendered `<DangerZoneSection />` after `<LanAccessSection />`.
   - IntersectionObserver automatically picks up the new section for
     scroll-based nav highlighting.

6. **Frontend UX documentation** (`docs/roadmap/factory-reset-danger-zone/frontend-flow.md`):
   - User journey, confirmation UX, auth mode handling (bearer vs
     cookie), state cleanup sequence, and known limitations.

## Key Decisions

| Decision | Rationale |
|----------|-----------|
| `goto()` not `window.location.href` | Faster client-side nav; explicit store cleanup makes it equivalent to a hard reload. Fallback to `window.location.href` if `goto` throws. |
| `clearSession()` not `logout()` | Server already deleted all sessions; `api.auth.logout()` would fail with 401. Only clear local state. |
| Do not call `setAuthMode('bearer')` in `clearSession` | In web mode, the next boot in `+layout.svelte` re-detects auth. In Tauri mode, the api module's internal `authMode` remains `'bearer'` and the token survives. Setting bearer in web mode would break CSRF on retry. |
| `disconnectWs()` on success | Prevents infinite reconnect loop. After reset, WS auth fails (sessions cleared). |
| No nav item color override | The red `AlertTriangle` icon signals danger. Adding conditional active-state color for one nav item adds complexity without proportional UX benefit. |
| Wrap `SettingsSection` in `.danger-zone` div | Overrides scoped component styles (icon, border) without modifying the shared `SettingsSection` component. |
| Input not trimmed before comparison | Server does exact match on `"RESET TUITBOT"`. Trimming could mask accidental whitespace. |
| `:global(.spinning)` in DangerZoneSection | Re-declares the existing spinning keyframe for the `Loader2` icon. Avoids dependency on LanAccessSection's identical declaration being loaded. |

## CI Results

All gates pass:

```
cargo fmt --all && cargo fmt --all --check             # OK
RUSTFLAGS="-D warnings" cargo test --workspace         # 1,733 passed, 0 failed
cargo clippy --workspace -- -D warnings                # clean
cd dashboard && npm run check                          # 0 errors, 6 pre-existing warnings
cd dashboard && npm run build                          # success
```

## Open Issues

1. **Multi-tab stale state**: Other open tabs are not notified of the
   reset. They will see stale data until refresh. The layout boot check
   redirects to `/onboarding` on next navigation. A `BroadcastChannel`
   cross-tab notification is a potential future enhancement.

2. **Server subsystem hot-restart**: After re-onboarding, the user must
   manually start the runtime. The server does not auto-restart runtimes
   after factory reset. This matches the initial onboarding behavior.

## Exact Inputs for Session 4

Session 4 should focus on end-to-end manual testing and any polish:

### Manual Test Scenarios

1. **Bearer mode happy path**: Tauri dev -> Settings -> Danger Zone ->
   type `RESET TUITBOT` -> click reset -> verify redirect to onboarding
   -> complete re-onboarding -> verify app works normally.

2. **Cookie mode happy path**: Server with `--host 0.0.0.0` -> browser
   login -> Settings -> Danger Zone -> reset -> verify session cookie
   cleared -> onboarding shows claim step -> re-claim -> verify.

3. **Wrong confirmation**: lowercase, trailing whitespace, partial phrase
   -> button stays disabled.

4. **Error handling**: disconnect server -> attempt reset -> error shows
   -> reconnect -> retry succeeds.

5. **Idempotent**: Reset on already-reset instance -> succeeds (0 rows).

6. **Multi-tab (web mode)**: Reset from tab 1 -> tab 2 on next nav
   redirects to onboarding.

### Optional Future Work

- `BroadcastChannel` for cross-tab reset notification.
- Post-reset toast or transition animation before redirect.
- Runtime auto-start after re-onboarding (server-side change).
