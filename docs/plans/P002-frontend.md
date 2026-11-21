# P002 — URL shortener frontend

## Summary

Implement the browser UI specified in `R002-frontend.md`: a React + TypeScript single
page served by the existing axum binary at `/`, which submits URLs to the R001 API,
shows the short link, copies it, and displays validation/API/network errors. The SPA is
built in a Node stage of the same Docker image, so there is still only one container.

**Requirement:** `R002-frontend.md` (Approved)  
**Depends on:** `P001-url-shortener-backend.md` (this plan modifies files P001 creates)  
**Related decisions:** `ADR002-frontend-stack.md`

## Approach

1. **SPA (`frontend/`).** Vite + React + TypeScript. `index.html` holds a single
   `<div id="root">`; React renders a form (visible `<label>`, input, `Shorten` button),
   an inline error region, and the result link plus `Copy` button.
2. **State machine in `App.tsx`.** `idle | submitting | success | error` drives the
   button label/disabled state (`Shortening…` while submitting) and clears previous
   result/error at the start of each submission (REQ-004, REQ-011, REQ-018).
3. **Client validation (`validation.ts`).** Three exact checks in order: empty →
   `Enter a URL`; missing `http://`/`https://` prefix → `URL must use http or https`;
   length > 2048 → `URL must be at most 2048 characters`. Invalid input never calls
   the API.
4. **API wrapper (`api.ts`).** `POST /api/v1/urls` with JSON body using relative URL
   (same origin). On a non-2xx response, parse `message` and surface it; if `fetch`
   itself throws, surface `Could not reach the server. Please try again.`
5. **Error rendering.** All errors render in one element with `role="alert"` so tests
   and assistive tech can find them reliably.
6. **Copy (`App.tsx`).** `navigator.clipboard.writeText(short_url)`; success sets label
   `Copied!` and a 2000 ms `setTimeout` restores `Copy`. On rejection/unavailability,
   focus and select the result link text and show the fallback message.
7. **Serving (Rust side).** `GET /` returns `index.html` read from disk at startup into
   `AppState` (avoids service-type friction with `ServeFile`); `GET /assets/*` uses
   `tower_http::services::ServeDir` mounted with `Router::nest_service("/assets", ...)`
   (context7-verified). No `fallback_service`, so unknown `/{code}` paths still hit the
   R001 redirect handler and return `404` JSON.
8. **Build.** Vite `base: '/'`, `build.assetsDir: 'assets'`, `outDir: 'dist'` ⇒ emitted
   JS/CSS paths are `/assets/...`. Dockerfile gains a `node:...-bookworm` stage that
   runs `npm ci && npm run build`; the Rust runtime stage copies `dist` to `/app/dist`.

### Library IDs and versions resolved via Context7

| Purpose | Library ID | API confirmed / version |
|---|---|---|
| React UI + hooks | `/websites/react_dev` | controlled input with `useState`, `onSubmit` + `event.preventDefault()`, `disabled` submit during pending; pin React **18.3.1** |
| Build + asset paths | `/vitejs/vite` | `build.assetsDir` prefixes emitted JS/CSS; `--base` / `base` rewrites asset URLs; pin Vite **6.4.x** |
| Test runner + mocks | `/vitest-dev/vitest` | `vi.stubGlobal`/`vi.fn`, `vi.useFakeTimers()` + `vi.advanceTimersByTime(2000)`; pin Vitest **3.2.x** |
| Component tests | `/testing-library/react-testing-library` | `render`, `screen.getByRole/getByLabelText`, `fireEvent`, `findByRole('alert')`, `waitFor`; pin `@testing-library/react` **16.3.x** |
| Rust static serving | `/websites/rs_tower-http` (from P001) | `ServeDir::new(...)`, `Router::nest_service` (verified in P001 research) |

TypeScript **5.9.x**, `@vitejs/plugin-react` **4.x**, `jsdom`, `@testing-library/jest-dom`
and `@testing-library/user-event` are supporting dev-dependencies; exact versions are
pinned by `package-lock.json` after `npm ci`.

### Spec interpretation (confirmed by the user)

`R002` REQ-001's scenario says `GET /` HTML "contains a form, an input with an
associated visible label, and a button labelled Shorten". A client-rendered SPA returns
an almost-empty `index.html`; the form exists in the **rendered DOM**, not the raw HTTP
body. The user confirmed this scenario applies to the DOM after React mounts, verified
by an RTL test plus the browser smoke check. The HTTP-level check remains `200` and the
asset-path checks. This is an interpretation, not a behaviour change — recorded here per
`AGENTS.md`'s deviation rule.

Frontend versions listed above were also approved by the user.

## Alternatives considered

| Alternative | Why rejected |
|---|---|
| Vanilla HTML/CSS/JS embedded in Rust | Rejected during spec interview in favour of React + Vite. |
| Server-rendered HTML (Askama/Tera) | Would change the approved frontend architecture and blur the API/UI boundary. |
| Separate frontend container / nginx | Violates REQ-016 (single image and host). |
| Client-side router with catch-all | Violates REQ-017 (short-code paths must stay with the backend). |
| Tailwind / component library | Rejected during spec interview; plain CSS keeps the build small. |

## Changes

- `frontend/package.json`, `package-lock.json` — scripts `dev`, `build`, `test`, `preview`;
  pinned deps listed above.
- `frontend/tsconfig.json`, `frontend/tsconfig.node.json` — TypeScript config.
- `frontend/vite.config.ts` — React plugin, `base: '/'`, `build.assetsDir: 'assets'`,
  Vitest config (`environment: 'jsdom'`, setup file).
- `frontend/index.html` — root element and Vite entry script.
- `frontend/src/main.tsx` — `createRoot` mount.
- `frontend/src/App.tsx` — form, state machine, result link, copy button.
- `frontend/src/validation.ts` — three client rules with exact messages.
- `frontend/src/api.ts` — `shortenUrl` wrapper with API/network error distinction.
- `frontend/src/styles.css` — plain CSS: visible focus styles, 320px responsive layout.
- `frontend/src/App.test.tsx`, `frontend/src/setupTests.ts` — Vitest/RTL tests and
  fetch/clipboard/timer mocking helpers.
- `src/main.rs` (from P001) — extend `AppState` with `index_html: String`; add
  `GET /` handler returning `Html`; mount `ServeDir` at `/assets`; new config value
  `ASSET_DIR` (default `frontend/dist` locally, `/app/dist` in the image).
- `Dockerfile` (from P001) — add Node build stage, copy `frontend/dist` to `/app/dist`,
  set `ASSET_DIR=/app/dist`.
- `.dockerignore` — exclude `frontend/node_modules`, `frontend/dist`, `target/`.
- `AGENTS.md` — add frontend check commands (`npm ci && npm test`, `npm run build`,
  `npx tsc --noEmit` in `frontend/`).
- `docs/reviews/R002-verification.md` — written by the `verifier`.

## Test strategy

- **Component tests (`npm test`, Vitest + RTL):** form renders with label/`Shorten`;
  empty/scheme/length validation shows exact messages and calls no fetch; successful
  201 renders the `short_url` link; delayed fetch shows disabled button + `Shortening…`;
  API error shows the server `message` in `role="alert"`; fetch rejection shows the
  network message; previous result/error cleared on new submit; clipboard success
  changes label to `Copied!` then back after 2000 ms (fake timers); clipboard rejection
  selects link text and shows fallback; keyboard focus styles and Enter-to-submit; two
  successive successes leave only the latest link.
- **Serving tests (HTTP, in Docker smoke):** `GET /` → 200 and serves the SPA entry;
  `GET /assets/<file>` → 200 with correct content type; no assets outside `/assets/`;
  `GET /aB3xY7z` unknown code → R001's 404 JSON, not HTML.
- **Browser smoke (recorded evidence):** open the running stack at `http://localhost:8080`,
  submit a URL, click the result link, exercise copy, and observe the 320px layout.
- **Checks:** `npm ci && npm test`, `npm run build`, `npx tsc --noEmit` in `frontend/`,
  plus P001's `cargo fmt`/`clippy`/`test` to confirm the Rust changes still pass.
- Not in P002: backend behaviour tests (P001).

## Risks

| Risk | Mitigation |
|---|---|
| Raw `index.html` lacks the form, conflicting with REQ-001's literal wording | Documented interpretation above; verify rendered DOM in RTL + browser; amend spec first if literal raw HTML is required. |
| Version incompatibility across React/Vite/Vitest/RTL | Pin all versions; `npm ci` from lockfile; re-query Context7 before any upgrade. |
| Clipboard API absent in jsdom and some browser contexts | Mock `navigator.clipboard` in tests; implement the select-text fallback for production. |
| `ServeDir`/`nest_service` type constraints or path mistakes | Use the Context7-verified `nest_service("/assets", ServeDir::new(...))`; assert asset paths in the Docker smoke. |
| SPA catch-all accidentally shadows `/{code}` | Do not add `fallback_service`; keep the explicit `404` JSON scenario as a regression test. |
| `Copied!` timer leaks across tests / unmount | Use fake timers and clear timers on unmount in the component. |

## Tasks

- [x] 1. Scaffold `frontend/` with pinned package.json, tsconfig and Vite config.
- [x] 2. Implement `index.html`, `main.tsx`, `styles.css`.
- [x] 3. Implement `validation.ts` and `api.ts`.
- [x] 4. Implement `App.tsx` state machine, form, result and copy behaviour.
- [x] 5. Add Vitest/RTL tests covering all R002 scenarios.
- [x] 6. Extend Rust serving with `index.html` handler and `/assets` ServeDir.
- [x] 7. Add Node build stage to Dockerfile; set `ASSET_DIR`; update `.dockerignore`.
- [x] 8. Update `AGENTS.md` with frontend check commands.
- [x] 9. Run `npm test`, `npm run build`, `npx tsc --noEmit`, and P001's cargo checks.
- [x] 10. Run Docker smoke incl. asset paths and unknown-code 404.
- [x] 11. Record the browser smoke test evidence in `docs/reviews/R002-verification-browser-smoke.md`.
- [x] 12. Launch the `verifier` with R002 + P002 and include its report — PASS, see `docs/reviews/R002-verification.md`.

## Traceability

| Requirement | Change | Test |
|---|---|---|
| REQ-001 | `frontend/src/App.tsx`, `src/main.rs` `/` handler | RTL "form renders" + HTTP `GET /` 200 |
| REQ-002 | `vite.config.ts` `assetsDir`, Rust `nest_service("/assets", ...)` | "Static assets are served under the assets prefix" |
| REQ-003 | `src/api.ts` `shortenUrl` | "Submit a valid URL and display the result" (assert fetch body) |
| REQ-004 | `App.tsx` submitting state | "In-flight button state" |
| REQ-005 | `validation.ts` empty rule | "Empty input is rejected locally" |
| REQ-006 | `validation.ts` scheme rule | "Invalid scheme is rejected locally" (outline) |
| REQ-007 | `validation.ts` length rule | "Overlong input is rejected locally" |
| REQ-008 | `App.tsx` result link | "Submit a valid URL and display the result" |
| REQ-009 | `api.ts` error message + `App.tsx` alert | "API error message is displayed" |
| REQ-010 | `api.ts` fetch failure mapping | "Network failure is displayed" |
| REQ-011 | `App.tsx` submission start clears state | "Previous result and error are cleared" |
| REQ-012 | `App.tsx` copy success + timer | "Copy succeeds and gives feedback" |
| REQ-013 | `App.tsx` copy fallback + select | "Copy fallback when clipboard is unavailable" |
| REQ-014 | `styles.css` focus styles, native form semantics | "Keyboard operation and focus visibility" |
| REQ-015 | `styles.css` responsive layout | "Narrow viewport layout" + browser smoke |
| REQ-016 | `Dockerfile` Node → Rust stages | `docker compose up --build`, single host/container |
| REQ-017 | Rust router (no fallback service) | "Short-code paths are not captured by the SPA" |
| REQ-018 | `App.tsx` result state replacement | "Only the latest result is shown" |

## Implementation notes

- A single `tsconfig.json` is used with `tsc --noEmit && vite build`; the separate
  `tsconfig.node.json` sketched in the plan was unnecessary.
- `npx tsc --noEmit` is part of `npm run build` rather than a separate command.
- Browser smoke evidence is recorded in `docs/reviews/R002-verification-browser-smoke.md`.
- `index.html` is read at startup and captured in a closure on the `/` route rather than
  stored on `AppState` as Approach 7 sketched; behaviourally equivalent.
- The browser smoke file was renamed after verification to match the `AGENTS.md`
  artifact naming table.

## Status

Approved.
