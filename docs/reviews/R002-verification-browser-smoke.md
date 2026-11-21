# Browser smoke test: R002

Date: 2026-11-21. Environment: Docker Compose stack on `http://localhost:8080`,
single container serving the Vite-built SPA and the axum API.

## Steps and observed results

1. Opened `http://localhost:8080/` in the in-app browser and waited for the page.
   - Rendered DOM contained heading "URL shortener", a labelled textbox "Long URL"
     and a submit button "Shorten". No console errors.
2. Filled `https://example.com/browser-smoke` and clicked "Shorten".
   - A clickable link appeared with text and `href` `http://localhost:8080/3x91apG`,
     plus a "Copy" button. No console errors.
3. Clicked "Copy" with clipboard access denied by the browser.
   - An element with `role="alert"` displayed exactly
     `Copy failed — select the link manually.` and the link text was selected.
   - This exercised the REQ-013 fallback in a real browser.
4. Filled `example.com` (unsupported scheme) and clicked "Shorten".
   - An element with `role="alert"` displayed exactly `URL must use http or https`;
     no request was sent (matching the component test).
   - Screenshot: `/tmp/opencode-browser-yr322N/0/screenshot.jpeg`.

## HTTP-level checks (curl)

- `GET /` → `200 text/html`, referencing `/assets/index-DoQql1Kf.js` and
  `/assets/index-Dzt1pCV9.css`.
- Both asset URLs → `200` with `text/javascript` and `text/css` respectively.
- `GET /aB3xY7z` (unknown code) → `404` with R001's JSON error, not the SPA HTML.
- `POST /api/v1/urls` → `201` JSON; `GET /{code}` → `302` with exact
  `Location: https://example.com/frontend-test`.

## Verdict

The rendered-DOM interpretation of R001's form scenario is confirmed. All observed
behaviour matches R002; no blocking findings from the smoke test.
