# R002 — URL shortener frontend

## Purpose

Deliver a minimal browser UI for the URL shortener: a single page where a user pastes a
long URL, receives a short link, copies it, and sees clear errors. The backend contract
is defined in `R001-url-shortener.md`.

## Scope

- In scope: a single-page UI at `/`, form submission against `POST /api/v1/urls`, result
  display, copy-to-clipboard, inline validation and API/network error display, basic
  accessibility and responsiveness, and building/serving the SPA from the same Docker
  image as the backend.
- Out of scope: browser-local history, dashboards, click statistics, authentication,
  client-side routing, dark mode, custom aliases, and any backend endpoint beyond R001.

### Terminology and fixed values

- Submit button default label: `Shorten`; in-flight label: `Shortening…`.
- Copy button default label: `Copy`; success label: `Copied!` (2 seconds).
- Client validation messages are exact strings and are shown inline near the input.
- API errors use the `message` from `R001`'s error table, displayed with `role="alert"`.

## Requirements

### REQ-001 — Single-page form

**Type:** ubiquitous

The frontend shall render a URL-shortening form at `GET /` with a visible `<label>` for
the URL input and a submit button labelled `Shorten`.

### REQ-002 — Static assets

**Type:** ubiquitous

The frontend shall load its JavaScript, CSS and other static assets exclusively from
paths under `/assets/`.

### REQ-003 — Submit to the API

**Type:** event-driven

When the form is submitted with a URL that passes client validation, the frontend shall
send `POST /api/v1/urls` with the JSON body `{"url": "<input value>"}`.

### REQ-004 — In-flight state

**Type:** state-driven

While a submission request is in flight, the frontend shall disable the submit button and
display the label `Shortening…`.

### REQ-005 — Empty input

**Type:** unwanted

If the input is empty when submitted, then the frontend shall display `Enter a URL`
inline and shall not send a request.

### REQ-006 — Unsupported scheme

**Type:** unwanted

If the input does not start with `http://` or `https://`, then the frontend shall display
`URL must use http or https` inline and shall not send a request.

### REQ-007 — Overlong input

**Type:** unwanted

If the input exceeds 2048 characters, then the frontend shall display
`URL must be at most 2048 characters` inline and shall not send a request.

### REQ-008 — Success display

**Type:** event-driven

When the API responds with status `201`, the frontend shall display the returned
`short_url` as a clickable link whose text is the full `short_url`.

### REQ-009 — API error display

**Type:** event-driven

When the API responds with an error status, the frontend shall display the response's
`message` inline in an element with `role="alert"`.

### REQ-010 — Network failure

**Type:** unwanted

If the submission request fails before an API response is received, then the frontend
shall display `Could not reach the server. Please try again.` inline with `role="alert"`.

### REQ-011 — Clear previous state

**Type:** event-driven

When a new submission starts, the frontend shall remove any previously displayed result
and error message.

### REQ-012 — Copy feedback

**Type:** event-driven

When the Copy button is activated and the clipboard write succeeds, the frontend shall
change the button label to `Copied!` for 2 seconds and then restore `Copy`.

### REQ-013 — Copy fallback

**Type:** unwanted

If the clipboard write is unavailable or rejected, then the frontend shall select the
result link's text and display `Copy failed — select the link manually.` inline.

### REQ-014 — Keyboard accessibility

**Type:** ubiquitous

The frontend shall make the input and both buttons operable by keyboard with visible
focus styles.

### REQ-015 — Narrow viewport

**Type:** ubiquitous

The frontend shall render without horizontal scrolling at a viewport width of 320px.

### REQ-016 — Single-image delivery

**Type:** ubiquitous

The Docker image shall contain the compiled frontend served by the same application and
host as the API, with no separate frontend container.

### REQ-017 — Redirect paths untouched

**Type:** ubiquitous

The frontend shall not handle `GET /{code}` paths, leaving them to the backend redirect
handler defined in R001.

### REQ-018 — Latest result only

**Type:** state-driven

While multiple submissions have succeeded, the frontend shall display only the most
recent `short_url`.

## Acceptance criteria

```gherkin
Scenario: Page renders an accessible form
  Given the service is running
  When GET / is requested
  Then the status is 200
  And the HTML contains a form, an input with an associated visible label, and a button labelled "Shorten"
  # Covers: REQ-001

Scenario: Static assets are served under the assets prefix
  Given the service is running
  When a frontend asset path under /assets/ is requested
  Then the status is 200 and the content type matches the asset type
  And no asset is served from a path outside /assets/
  # Covers: REQ-002, REQ-016

Scenario: Submit a valid URL and display the result
  Given the page at / is open
  When the user enters "https://example.com/path?q=1" and clicks "Shorten"
  Then POST /api/v1/urls is called with {"url": "https://example.com/path?q=1"}
  And a clickable link is displayed whose text is the returned "short_url"
  # Covers: REQ-003, REQ-008

Scenario: In-flight button state
  Given the API response for a valid submission is delayed
  When the user clicks "Shorten"
  Then the submit button is disabled and its label is "Shortening…"
  And after the response arrives the button is enabled again with label "Shorten"
  # Covers: REQ-004

Scenario: Empty input is rejected locally
  Given the page at / is open
  When the user clicks "Shorten" with an empty input
  Then "Enter a URL" is displayed inline
  And no request is sent to /api/v1/urls
  # Covers: REQ-005

Scenario Outline: Invalid scheme is rejected locally
  Given the page at / is open
  When the user enters <input> and clicks "Shorten"
  Then "URL must use http or https" is displayed inline
  And no request is sent to /api/v1/urls
  Examples:
    | input |
    | example.com |
    | ftp://example.com |
  # Covers: REQ-006

Scenario: Overlong input is rejected locally
  Given the page at / is open
  When the user enters a 2049-character value starting with "https://example.com/" and clicks "Shorten"
  Then "URL must be at most 2048 characters" is displayed inline
  And no request is sent to /api/v1/urls
  # Covers: REQ-007

Scenario: API error message is displayed
  Given the API is configured to return 400 with message "URL must be a valid http or https URL of at most 2048 characters"
  When the user submits a syntactically valid form
  Then an element with role="alert" displays exactly "URL must be a valid http or https URL of at most 2048 characters"
  # Covers: REQ-009

Scenario: Network failure is displayed
  Given the API server is unreachable from the page
  When the user submits a valid URL
  Then an element with role="alert" displays exactly "Could not reach the server. Please try again."
  # Covers: REQ-010

Scenario: Previous result and error are cleared
  Given a result link and an error message are both currently displayed
  When the user starts a new valid submission
  Then the previous result link and the previous error message are no longer displayed
  # Covers: REQ-011

Scenario: Copy succeeds and gives feedback
  Given a result link is displayed and clipboard writes are permitted
  When the user clicks "Copy"
  Then the clipboard receives the displayed "short_url"
  And the button label becomes "Copied!"
  And after 2 seconds the label returns to "Copy"
  # Covers: REQ-012

Scenario: Copy fallback when clipboard is unavailable
  Given a result link is displayed and clipboard writes are rejected
  When the user clicks "Copy"
  Then the result link text is selected
  And "Copy failed — select the link manually." is displayed inline
  # Covers: REQ-013

Scenario: Keyboard operation and focus visibility
  Given the page at / is open
  When the user tabs through the input and both buttons
  Then each focused control has a visible focus style
  And pressing Enter in the input submits the form
  # Covers: REQ-014

Scenario: Narrow viewport layout
  Given a browser viewport 320px wide
  When the page at / is open
  Then the document has no horizontal scrollbar
  And the input and buttons are visible
  # Covers: REQ-015

Scenario: Short-code paths are not captured by the SPA
  Given the service is running with the frontend deployed
  When GET /aB3xY7z is requested for an unknown code
  Then the response is the 404 JSON error from R001, not the frontend HTML
  # Covers: REQ-017

Scenario: Only the latest result is shown
  Given two URLs have been shortened successfully in the same session
  When the second submission completes
  Then only the second "short_url" is displayed as a result link
  # Covers: REQ-018
```

## Assumptions

- The submit button's default label is `Shorten` and the copy button's is `Copy`; both
  are part of the UI contract so the in-flight/success labels can be asserted.
- The result link's `href` is the `short_url` returned by the API; no URL rewriting is
  performed by the frontend.
- Modern evergreen browsers are supported; no legacy browser polyfills are included.
- Client validation intentionally mirrors only the three rules in REQ-005..REQ-007;
  server responses remain authoritative for all other cases.
- `npm test` in `frontend/` is the automated check command; a browser smoke test is
  recorded as additional evidence.

## Status

Approved.
