# Verification: R003

## Summary

**PASS — R003 is implemented faithfully against the approved P003 and is ready to close.** All 13 Gherkin scenarios in `R003-terminal-theme.md` (the brief says 14, but the document contains 13 — see spec-review Finding 3) have passing evidence from the component suite and an independent headless-Chromium computed-style run. The explicit five-value `statusLine` mapping and `copyStatus` flag resolve spec-review suggestions #1/#2 exactly as recorded in the P003 change log. Every configured check is green and the R002 suite is byte-for-byte unchanged. There are no blocking findings; the items below are non-blocking suggestions only.

Note on the fixed point: the brief supplied `9db725e`, which does not resolve in this repo. The real parent of implementation commit `e04c25e` is `9db7254` ("docs: approve R003/P003 and refine spec-review gate"). All review commands used `git diff 9db7254...HEAD` and `git log 9db7254..HEAD --oneline` (single commit: `e04c25e`).

## Standards

Standards axis (fresh context, diff `9db7254...HEAD`; only documented source `AGENTS.md` plus the Fowler smell baseline). **No hard violations** of any documented repo standard. AGENTS.md artifact naming and workflow gates are respected (`R003-verification-browser-smoke.md` matches the `<id>-verification-<stage>.md` pattern, with R002 as precedent; spec review and approved plan predate implementation; the Context7 skip is documented in the approved plan). The only DoD gap was the missing verifier report and unchecked plan task 8, both closed by this document.

Judgement calls (Fowler baseline — never hard):
1. **Repeated Switches / Data Clumps** — the `status`/`copyStatus` cascade is expressed twice: `statusLine()` (`App.tsx:8-22`) and the `statusClass` ternary (`App.tsx:94-101`); the two fields are also reset and passed around together (`App.tsx:27,48,95`). Consider one descriptor (label + class) from a shared lookup.
2. **Mysterious Name** — `const text = statusLine(status, copyStatus)` (`App.tsx:95`) is a generic name for a status-line string; `statusText` would reveal intent.
3. **Duplicated Code** — raw RGB literals repeat the palette tokens in glows/shadows: `styles.css:17,27,50,123,128,133`. Changing `--phosphor`/`--error` would leave these stale.
4. **Duplicated Code** — submit boilerplate (`fireEvent.change` → `fireEvent.click` → `findByRole('link')`) recurs in three theme tests (`theme.test.tsx:69-92`); a `submitValidUrl()` helper beside `renderApp()` would remove it.
5. **Speculative Generality (minor)** — `transition: none !important` and `.result a { clip-path: none }` (`styles.css:196-210`) exceed the two animations; not a standard breach.

Axis summary: 5 findings (0 hard violations, 5 judgement calls); worst within axis = Repeated Switches/Data Clumps in `App.tsx:8-22,94-101`.

## Spec

Spec axis (fresh context; requirement `docs/requirements/R003-terminal-theme.md`, plan `docs/plans/P003-terminal-theme.md`, prior spec review, browser smoke doc). **No blocking findings.** The explicit `statusLine` (`App.tsx:8-22`) matches P003 step 4 for all five values, including the non-clipboard `error`/`idle` → `READY` fallback (`App.tsx:21`) required by the change log. `validation.ts` and `App.test.tsx` are untouched, so R002 strings, labels and the error contract hold.

Findings, with the requirement line quoted:

- **REQ-013 / `R003:21-22`** — "Decoration: any element that is purely visual (scanlines, cursor, prompt prefix, header bracket, boot overlay) is `aria-hidden="true"` and not focusable." Scanlines/boot/prompt are hidden (`App.tsx:105,106,114`), but the blinking cursor is `.crt-title::after` (`styles.css:60-63`), a pseudo-element that cannot carry `aria-hidden`, and the header brackets are literal `<h1>` text (`App.tsx:110`). Two listed decorations are not explicitly hidden. Non-blocking.
- **REQ-010 / `R003:190`** — "any boot overlay is gone within 500 ms." The overlay is never removed from the DOM; it fades to `visibility:hidden` at 500 ms (`styles.css:41,164-173`). "Gone" is visual only (usability is fine: `pointer-events:none`, `styles.css:38`). Minor deviation from P003 step 2's wording "removed after 500 ms"; observable behaviour matches the requirement. Non-blocking.
- **`R003:13-14`** out of scope: "changes to validation rules, messages, copy behaviour, accessibility of the form itself" — the diff adds `button:hover:not(:disabled)` styling (`styles.css:106-109`) and recolours the R002 focus outline blue→amber (`styles.css:153-156`). Both are presentational, remain visible, and are within the theme's remit; low severity.
- **REQ-006 / REQ-007 interplay** — `copyStatus` is set on copy (`App.tsx:75,81`) and reset only on submission (`App.tsx:48`); the 2 s timer reverts only `copied` (`App.tsx:79`). After a successful copy the status stays `COPY OK` indefinitely while the button label reverts, so `LINK READY` (`R003:65`) is not re-shown for that result. The spec does not mandate a revert. Non-blocking.

Axis summary: 4 findings (0 blocking, 4 suggestions); worst within axis = REQ-013 pseudo-element/bracket `aria-hidden` gap (`styles.css:60-63`, `App.tsx:110`).

### Plan conformance (P003)

Steps 1–7 are implemented as specified: palette/monospace/panel/scanlines/glow/cursor/boot/typing/reduced-motion CSS; explicit status line with `copyStatus`; `>` prompt; unchanged R002 strings; theme tests and R002 regression; checks; browser smoke evidence. Traceability rows REQ-001…REQ-014 each map to a real change and test. No Rust, Docker, compose or AGENTS.md changes — as required (`P003:68`). Deviations: boot overlay uses `visibility:hidden` rather than DOM removal (see REQ-010 above), and the typing animation uses `clip-path` only rather than "clip-path/width" (`P003:24-26`) — both behaviourally equivalent and non-blocking. Change log suggestions #1 and #2 are demonstrably resolved. Plan task 8 is completed by this report.

## Acceptance criteria

| Scenario | Status | Evidence |
|---|---|---|
| Terminal palette and monospace typography | pass | Headless Chromium: body bg `rgb(5, 8, 5)`, colour `rgb(51, 255, 102)`, font `ui-monospace, SFMono-Regular, Menlo, Consolas, monospace` (`styles.css:2-8,11-17`); source assertion `theme.test.tsx:112-120`; smoke doc:8-9. |
| CRT panel with header and blinking cursor | pass | Headless Chromium: `.page` width/max-width `640px` (40rem, `box-sizing: border-box`), title `[ URL SHORTENER ]`, `::after` content `_` with `animation-name: blink` (`styles.css:44-63`, `App.tsx:110`); smoke doc:12. |
| Scanlines and glow are decorative only | pass | Headless Chromium: `.scanlines` `aria-hidden=true`, `pointer-events: none`, no tabbable decoration among `.scanlines/.boot/.prompt` (`App.tsx:105,114`, `styles.css:20-30`); `theme.test.tsx:41-43`; smoke doc:10. |
| In-flight status is amber | pass | Headless Chromium with delayed API: status `SHORTENING...`, computed colour `rgb(255, 176, 0)`, class `status status-busy` (`App.tsx:9-10,96-98`, `styles.css:121-124`); `theme.test.tsx:50-66`. |
| Idle status | pass | Headless Chromium: initial status `READY` (`App.tsx:21,26`); `theme.test.tsx:39-47`; smoke doc:17. |
| Result reveal keeps text in the DOM | pass | Headless Chromium: status `LINK READY`, link `http://localhost:8080/…` present in DOM with `animation-name: typing` immediately after response (`App.tsx:60-61,136-140`, `styles.css:144-147,175-182`); `theme.test.tsx:68-76`; smoke doc:18-19. |
| Copy success status | pass | Headless Chromium: status `COPY OK` and button label `Copied!` (`App.tsx:74-75,142`); `theme.test.tsx:78-86`; R002 test `App.test.tsx:154-173` still passes. |
| Copy failure status | pass | Headless Chromium: status `CLIPBOARD DENIED` computed colour `rgb(255, 95, 86)` and alert exactly `Copy failed — select the link manually.` (`App.tsx:80-91,99-100`, `styles.css:126-129`, `validation.ts:5`); `theme.test.tsx:88-99`; smoke doc:20-22. |
| Input prompt prefix | pass | Headless Chromium: `>` with `aria-hidden=true` immediately before the URL input (`App.tsx:113-116`); `theme.test.tsx:41`; smoke doc:13. |
| Boot sequence does not block input | pass | Headless Chromium: while boot `visibility: visible`, boot `pointer-events: none` and input `disabled: false`; typing and submitting succeeded during boot; boot `visibility: hidden` after 700 ms (`styles.css:32-42,164-173`); smoke doc:11. |
| Reduced motion disables decoration | pass | Headless Chromium with `prefers-reduced-motion: reduce`: cursor `animation-name: none`, `.boot` `display: none`, input/button/status all `visibility: visible` (`styles.css:196-211`); `theme.test.tsx:130-142`. |
| All R002 behaviour still passes | pass | `npm test` → `src/App.test.tsx (15 tests)` passed; `git diff 9db7254...HEAD --name-only` confirms `App.test.tsx` and `validation.ts` are unchanged. |
| Error messages use the error colour | pass | Headless Chromium: `example.com` → alert `URL must use http or https` computed colour `rgb(255, 95, 86)`, status `READY` (`validation.ts:2,12-13`, `App.tsx:131-134`, `styles.css:131-134`); `theme.test.tsx:101-110`. |

## Checks

- `npm ci` (frontend/): added 185 packages. Note: `npm audit` reported 3 vulnerabilities (1 moderate, 2 critical) in the dependency tree; no dependency was added or changed by R003, so informational only.
- `npm test` (frontend/): 23 passed (2 files: `theme.test.tsx` 8, `App.test.tsx` 15).
- `npm run build` (frontend/): `tsc --noEmit && vite build` succeeded (dist emitted).
- `cargo fmt --all -- --check` (root): exit 0.
- `cargo clippy --all-targets -- -D warnings` (root): exit 0.
- `cargo test --all-targets` (root): 17 passed (lib 7 + api 10), 0 failed.
- Independent browser corroboration: no in-app browser or browse extension was connected to the verifier session, so I drove `/usr/bin/chromium` headless via CDP against the live Docker stack (`/health` → HTTP 200). All computed-style and interaction results are recorded in the acceptance-criteria table above and match the recorded smoke doc (`docs/reviews/R003-verification-browser-smoke.md`).

## Findings

1. **[suggestion]** Status/copy cascade duplicated between the label mapping and class mapping; bundle into one descriptor — `frontend/src/App.tsx:8-22,94-101`
2. **[suggestion]** Generic local name `text` for the status-line string; rename to `statusText` — `frontend/src/App.tsx:95`
3. **[suggestion]** Raw RGB literals duplicate palette custom properties in glow/shadow declarations — `frontend/src/styles.css:17,27,50,123,128,133`
4. **[suggestion]** Repeated submit boilerplate across three theme tests; extract a helper — `frontend/src/theme.test.tsx:69-92`
5. **[suggestion]** Blinking cursor is a `::after` pseudo-element and header brackets are literal `<h1>` text, so two decorations listed as `aria-hidden` in `R003:21-22` are not explicitly hidden — `frontend/src/styles.css:60-63`, `frontend/src/App.tsx:110`
6. **[suggestion]** Boot overlay fades to `visibility:hidden` instead of being removed from the DOM; "gone" is visual only and this differs from P003 step 2's wording — `frontend/src/styles.css:41,164-173`
7. **[suggestion]** Copy success status is not reverted with the 2 s `copied` timer, so `COPY OK` persists and `LINK READY` is not re-shown for that result — `frontend/src/App.tsx:48,75,79`
8. **[suggestion]** Recorded browser smoke omits computed checks for amber, `COPY OK`, error red and reduced-motion visibility; this verification's headless run supplies them — `docs/reviews/R003-verification-browser-smoke.md:6-29`
