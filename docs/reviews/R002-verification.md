# Verification: R002

## Summary

**Verdict: PASS — no blocking findings.** All 18 EARS requirements in `R002-frontend.md`
are implemented, and all 16 Gherkin scenarios pass with concrete evidence (15 Vitest/RTL
tests, HTTP checks against the running Docker stack, and a headless-Chromium 320px layout
measurement). All documented checks pass: `npm ci`, `npm test` (15/15), `npm run build`,
`cargo fmt --check`, `cargo clippy -D warnings`, `cargo test --all-targets` (17/17). The
implementation matches the approved plan P002; the only deviations are the two already
recorded in P002's "Implementation notes" plus one minor undocumented difference noted
below. The confirmed interpretation that REQ-001's "HTML contains a form" applies to the
rendered DOM (not the raw `index.html`) was applied throughout. Remaining findings are
non-blocking suggestions (one documentation-naming convention breach, a few design smells).

## Standards

Standards sources: `AGENTS.md` (the only documented coding-standard file; no
`CODING_STANDARDS.md`/`CONTRIBUTING.md` exist), `docs/decisions/ADR002-frontend-stack.md`,
the pinned-version table in P002, plus the Fowler smell baseline. Pinned versions are honoured
(React 18.3.1, Vite 6.4.5, TS 5.9.3, Vitest 3.2.4, RTL 16.3.3, tower-http 0.6/`fs`). No
secrets committed; `ASSET_DIR` uses env substitution (`AGENTS.md`, `src/config.rs:32`,
`Dockerfile:25`). `AGENTS.md:67-71` records the new check commands. Tooling-enforced items
(fmt/clippy/tsc/build) were skipped as instructed.

- **Documented-standard breach (hard, low impact)** — `docs/reviews/R002-verification-browser-smoke.md:1`
  does not follow the `AGENTS.md` Artifacts naming table (`AGENTS.md:26-27`), which allows
  only `<id>-verification.md` or `<id>-verification-<stage>.md` under `docs/reviews/`.
  Rename to `R002-verification-browser-smoke.md` or fold into this report.
- **Speculative Generality (judgement call)** — `frontend/src/App.tsx:5,9,73`: `Status` has
  four variants, but only `submitting = status === 'submitting'` is ever read; `idle`,
  `success` and `error` are written and never consumed. A single `submitting` boolean would
  carry the same behaviour.
- **Unreachable fallback / Speculative Generality (judgement call)** —
  `frontend/src/App.tsx:44`: `cause instanceof Error ? cause.message : NETWORK_MESSAGE` —
  every throw from `shortenUrl` is an `ApiRequestError` (`frontend/src/api.ts:24,37`), so the
  fallback branch is dead.
- **Duplicated Code (judgement call)** — `frontend/src/App.tsx:18-20` and `:56-58` repeat
  the identical `if (timerRef.current !== null) window.clearTimeout(timerRef.current)`;
  extract a helper.
- **Divergent Change (judgement call)** — `frontend/src/validation.ts:4-5`:
  `NETWORK_MESSAGE` and `COPY_FALLBACK_MESSAGE` are network/copy concerns, not validation
  rules, yet are imported by `api.ts:1` and `App.tsx:3`.
- **Primitive Obsession (judgement call)** — `src/main.rs:41,43`: `asset_dir` is joined with
  `format!` strings although `std::path::Path` is already imported (`:1`) and used (`:20`).
- **Duplicated Code in tests (judgement call, tolerable)** — `frontend/src/App.test.tsx`:
  `fireEvent.change(input, …); fireEvent.click(button)` and the literal `'aB3xY7z'` recur
  across ~8 tests; a `submit(value)` helper would reduce it.

One-axis summary (Standards): 7 findings (1 documented-standard breach, 6 judgement-call
smells); worst within axis = the artifact-naming convention breach.

## Spec

Spec source: `docs/requirements/R002-frontend.md` (Approved); cross-checked against
`docs/plans/P002-frontend.md` and R001's error table (R001:29-32). No missing requirements,
no scope creep of concern, and no incorrect implementations were found.

- **"The frontend shall render a URL-shortening form at `GET /` …" (REQ-001, R002:31-32)** —
  satisfied per the user-confirmed, P002-recorded rendered-DOM interpretation: RTL test
  `App.test.tsx:48-53` and the 320px browser measurement both find the `<form>`, the label
  "Long URL" and the `Shorten` button in the mounted DOM; the raw `index.html` is
  intentionally near-empty. Not a finding.
- **"…send `POST /api/v1/urls` with the JSON body `{"url": "<input value>"}`" (REQ-003,
  R002:45-46)** — benign deviation: `App.tsx:40` sends `url.trim()`, so values with leading
  or trailing whitespace differ from the literal input value. Validation trims identically
  (`validation.ts:8`) and no scenario covers whitespace; consistent with the approved design.
- **"…display `…` inline near the input" (REQ-005/006/007, R002:59,66-67,73-74)** — exact
  messages are correct (`validation.ts:1-3`); the single `role="alert"` element sits just
  after the form (`App.tsx:92-96`) rather than inside it, which matches P002 §"Error
  rendering" and is not a defect.
- Exact strings, in-flight state, error roles, copy feedback/fallback, `/assets/` paths,
  single-image delivery and the untouched `/{code}` route were all verified correct;
  see the acceptance table for file:line evidence.

Plan conformance: matches P002. Deviations recorded in P002 "Implementation notes" (single
`tsconfig.json`; `tsc --noEmit` folded into `npm run build`). One **undocumented minor
deviation**: P002 Approach 7 says `index.html` is read "at startup into `AppState`"; the
implementation reads it at startup into a local `String` captured by the `/` handler closure
(`src/main.rs:41-52`) rather than storing it on `AppState`. Behaviour is equivalent.

One-axis summary (Spec): 0 blocking findings; 2 benign notes (trimmed body; inline-error
placement per approved design); conformance **pass**.

## Acceptance criteria

| Scenario | Status | Evidence |
|---|---|---|
| Page renders an accessible form | pass | `npm test` test `App.test.tsx:48-53` (label + submit button) passed; `GET /` → `200 text/html`; headless Chromium at 320px: `formPresent=true`, `labelText="Long URL"`, `submitText="Shorten"`; `docs/reviews/R002-verification-browser-smoke.md:8-10`. DOM interpretation per P002:57-65. |
| Static assets are served under the assets prefix | pass | `GET /` body references only `/assets/index-DoQql1Kf.js` and `/assets/index-Dzt1pCV9.css`; both → `200` with `text/javascript` / `text/css`; root-path copy of the JS → `404 application/json` (not served as an asset); `vite.config.ts:6-10` (`base:'/'`, `assetsDir:'assets'`); built `frontend/dist/index.html`. |
| Submit a valid URL and display the result | pass | `npm test` test `App.test.tsx:55-70` asserts `fetch('/api/v1/urls', { method:'POST', body:'{"url":"https://example.com/path?q=1"}' })` and link text/`href` = `short_url`; passed. HTTP: `POST /api/v1/urls` → `201` JSON, `GET /{code}` → `302` to the exact target. |
| In-flight button state | pass | `npm test` test `App.test.tsx:72-94` (`fetch` promise held open) asserts disabled + `Shortening…`, then enabled + `Shorten` after resolution; passed. `App.tsx:38,86-90`. |
| Empty input is rejected locally | pass | `npm test` test `App.test.tsx:96-101` asserts `role="alert"` "Enter a URL" and `fetch` not called; passed. `validation.ts:9-11`. |
| Invalid scheme is rejected locally (outline: `example.com`, `ftp://example.com`) | pass | `npm test` parameterized test `App.test.tsx:103-109` covers both examples, asserts exact message and no `fetch`; passed. `validation.ts:12-14`. Browser smoke `R002-verification-browser-smoke.md:18-20` confirms `example.com` in a real browser. |
| Overlong input is rejected locally | pass | `npm test` test `App.test.tsx:111-119` uses a 2049-character value starting with `https://example.com/`, asserts exact message and no `fetch`; passed. `validation.ts:15-17`. |
| API error message is displayed | pass | `npm test` test `App.test.tsx:121-129` returns `400 { error:'invalid_url', message:'URL must be a valid http or https URL of at most 2048 characters' }` and asserts exact text in `role="alert"`; passed. `api.ts:27-37`, `App.tsx:92-96`. |
| Network failure is displayed | pass | `npm test` test `App.test.tsx:131-139` rejects `fetch` and asserts exact "Could not reach the server. Please try again." in `role="alert"`; passed. `api.ts:23-25`. |
| Previous result and error are cleared | pass | `npm test` test `App.test.tsx:141-152` shows a result then triggers a failing resubmit; asserts prior link removed and new alert present; passed. `App.tsx:27-29`. |
| Copy succeeds and gives feedback | pass | `npm test` test `App.test.tsx:154-173` (fake timers) asserts `writeText(short_url)`, label `Copied!`, then `Copy` after 2000 ms; passed. `App.tsx:54-59`. |
| Copy fallback when clipboard is unavailable | pass | `npm test` test `App.test.tsx:175-187` rejects `writeText`, asserts link text selected and exact `Copy failed — select the link manually.` message; passed. `App.tsx:60-70`. Real-browser denial recorded in `R002-verification-browser-smoke.md:14-17`. |
| Keyboard operation and focus visibility | pass | `npm test` test `App.test.tsx:189-202` uses `userEvent.tab()` to focus input then `Shorten`, asserts `:focus-visible` present in `styles.css`, and that submitting the form produces the link; passed. `styles.css:37-40`; native `<form>/<button>`. |
| Narrow viewport layout | pass | Headless Chromium at `innerWidth=320`: `clientWidth=320`, `scrollWidth=320`, `hasHorizontalScroll=false`; input and submit `getBoundingClientRect` = left 16 / right 304 (inside viewport), form and label present. `npm test` test `App.test.tsx:204-207` asserts the `@media (max-width: 360px)` and `overflow-wrap: anywhere` rules; `styles.css:26-29,51,58-62`. |
| Short-code paths are not captured by the SPA | pass | `GET /aB3xY7z` → `404 application/json` `{"error":"not_found","message":"Short code not found"}` (R001's JSON, not HTML). No `fallback_service`; `/:code` route retained at `src/main.rs:45-57`. |
| Only the latest result is shown | pass | `npm test` test `App.test.tsx:209-227` performs two 201 successes and asserts only the second `short_url` is present and the first is absent; passed. Single `result` state replaced at `App.tsx:41`. |

## Checks

- `cd frontend && npm ci`: success (lockfile installed; `npm audit` advisory printed but not a check failure).
- `cd frontend && npm test`: **15 passed** (1 file, `src/App.test.tsx`), 0 failed.
- `cd frontend && npm run build` (`tsc --noEmit && vite build`): success; emitted `dist/index.html` plus `/assets/index-DoQql1Kf.js` and `/assets/index-Dzt1pCV9.css`.
- `cargo fmt --all -- --check` (repo root, `$HOME/.cargo/env` sourced): success (no diff).
- `cargo clippy --all-targets -- -D warnings`: success (Finished, no warnings).
- `cargo test --all-targets`: **17 passed** (7 lib + 0 bin + 10 integration), 0 failed.
- Docker smoke (curl against running `url-shortener-url-shortener-1`, healthy): `GET /` 200 text/html; both `/assets/*` 200 with correct content types; asset copied to root path → 404 JSON; `GET /aB3xY7z` → 404 R001 JSON; `POST /api/v1/urls` → 201 JSON; `GET /nB9KCC1` → 302 `Location: https://example.com/verify-r002`.
- Headless Chromium (`/usr/bin/chromium --headless=new`, DevTools `Emulation.setDeviceMetricsOverride` width 320): no horizontal scroll; input and submit button visible.
- `docs/reviews/R002-verification-browser-smoke.md`: recorded real-browser form, result link, clipboard-denied fallback and local validation message (reviewed, not re-executed).

## Findings

1. **[suggestion — resolved after the review]** The browser-smoke evidence file was renamed to `docs/reviews/R002-verification-browser-smoke.md` to match the `AGENTS.md` naming table; references in P002 and this report were updated.
2. **[suggestion]** REQ-003 asks for `{"url": "<input value>"}` but the trimmed value is sent; harmless (no whitespace scenario) but a literal reading differs — `frontend/src/App.tsx:40`.
3. **[suggestion]** `Status` is four-valued but only the `submitting` branch is ever read; a boolean would suffice (Speculative Generality) — `frontend/src/App.tsx:5,9,73`.
4. **[suggestion]** Unreachable `instanceof Error` fallback because all failures are `ApiRequestError` — `frontend/src/App.tsx:44`, `frontend/src/api.ts:24,37`.
5. **[suggestion]** Duplicate timer-clear block (Judgement: Duplicated Code) — `frontend/src/App.tsx:18-20` and `:56-58`.
6. **[suggestion]** Network/copy message constants live in `validation.ts` (Divergent Change) — `frontend/src/validation.ts:4-5`.
7. **[suggestion]** Raw string path joins despite `Path` import (Primitive Obsession) — `src/main.rs:41,43`.
8. **[suggestion]** Undocumented plan deviation: `index.html` is captured in a closure rather than stored on `AppState` as P002 Approach 7 states (behaviourally equivalent) — `src/main.rs:41-52`, `docs/plans/P002-frontend.md:34-38`.

No blockers. Verdict: **PASS**.
