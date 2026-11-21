# R003 — Retro terminal theme

## Purpose

Give the existing URL-shortener UI (R002) a retro terminal aesthetic — phosphor green on
near-black, CRT scanlines, blinking cursor, boot-up and typing effects, and
terminal-style status feedback — without changing any R002 behaviour or API contract.

## Scope

- In scope: colour palette, typography, CRT framing and glow, decorative animations,
  prompt prefix, status line, and accessibility guards for the decoration.
- Out of scope: light-mode toggle, new endpoints, new UI features, changes to validation
  rules, messages, copy behaviour, accessibility of the form itself (already in R002).

### Terminology and fixed values

- Phosphor green `#33ff66`, background `#050805`, amber `#ffb000`, error red `#ff5f56`.
- Status line values (exact): `READY`, `SHORTENING...`, `LINK READY`, `COPY OK`,
  `CLIPBOARD DENIED`.
- Decoration: any element that is purely visual (scanlines, cursor, prompt prefix,
  header bracket, boot overlay) is `aria-hidden="true"` and not focusable.

## Requirements

### REQ-001 — Terminal palette

**Type:** ubiquitous

The frontend shall render text in `#33ff66` on a `#050805` background using a monospace
font stack.

### REQ-002 — CRT panel

**Type:** ubiquitous

The frontend shall render the interface in a centred panel at most 40rem wide with a
bracketed header and a blinking cursor.

### REQ-003 — Scanlines and glow

**Type:** ubiquitous

The frontend shall overlay scanlines and a phosphor glow that neither intercept pointer
events nor appear in the accessibility tree.

### REQ-004 — In-flight status

**Type:** state-driven

While a submission is in flight, the frontend shall display the status `SHORTENING...` in
`#ffb000`.

### REQ-005 — Idle status

**Type:** state-driven

While no submission has succeeded and none is in flight, the frontend shall display the
status `READY`.

### REQ-006 — Result reveal

**Type:** event-driven

When a short URL has been created, the frontend shall display the status `LINK READY` and
reveal the link with a typing effect while keeping the link text present in the DOM from
the moment it is received.

### REQ-007 — Copy success status

**Type:** event-driven

When a clipboard write succeeds, the frontend shall display the status `COPY OK`.

### REQ-008 — Copy failure status

**Type:** unwanted

If a clipboard write fails, then the frontend shall display the status
`CLIPBOARD DENIED` in `#ff5f56` and still show the exact R002 fallback message.

### REQ-009 — Prompt prefix

**Type:** ubiquitous

The frontend shall display a `>` prompt character immediately before the URL input.

### REQ-010 — Boot sequence

**Type:** event-driven

When the page loads, the frontend shall play a boot-up sequence that completes within
500 ms and shall leave the form usable immediately.

### REQ-011 — Reduced motion

**Type:** state-driven

While the user prefers reduced motion, the frontend shall disable all decorative
animation while keeping all text and controls fully visible.

### REQ-012 — R002 behaviour preserved

**Type:** ubiquitous

The frontend shall keep every R002 requirement, exact message, error contract and the
`Shorten` / `Copied!` button labels unchanged.

### REQ-013 — Decorative accessibility

**Type:** ubiquitous

The frontend shall mark decorative layers with `aria-hidden="true"` and keep them out of
the keyboard tab order.

### REQ-014 — Error colour

**Type:** ubiquitous

The frontend shall render validation and API error messages in `#ff5f56`.

## Acceptance criteria

```gherkin
Scenario: Terminal palette and monospace typography
  Given the page at / is open
  Then the computed body background is "rgb(5, 8, 5)"
  And the computed body text colour is "rgb(51, 255, 102)"
  And the computed font family contains "monospace"
  # Covers: REQ-001

Scenario: CRT panel with header and blinking cursor
  Given the page at / is open
  Then a centred panel is rendered with width at most 40rem
  And the panel header text is bracketed with "[" and "]"
  And a cursor element has a CSS animation that blinks
  # Covers: REQ-002

Scenario: Scanlines and glow are decorative only
  Given the page at / is open
  Then a scanline overlay element exists with aria-hidden="true"
  And its computed pointer-events value is "none"
  And it is not reachable by pressing Tab
  # Covers: REQ-003, REQ-013

Scenario: In-flight status is amber
  Given the API response for a valid submission is delayed
  When the user clicks "Shorten"
  Then the status line shows exactly "SHORTENING..."
  And its computed colour is "rgb(255, 176, 0)"
  # Covers: REQ-004

Scenario: Idle status
  Given the page at / has loaded and no submission has been made
  Then the status line shows exactly "READY"
  # Covers: REQ-005

Scenario: Result reveal keeps text in the DOM
  Given the API responds with 201 and a short_url
  When the user submits a valid URL
  Then the status line shows exactly "LINK READY"
  And the result link text is present in the DOM immediately
  And the link is revealed with a CSS typing animation
  # Covers: REQ-006

Scenario: Copy success status
  Given a result link is displayed and clipboard writes are permitted
  When the user clicks "Copy"
  Then the status line shows exactly "COPY OK"
  And the button label temporarily becomes "Copied!"
  # Covers: REQ-007, REQ-012

Scenario: Copy failure status
  Given a result link is displayed and clipboard writes are rejected
  When the user clicks "Copy"
  Then the status line shows exactly "CLIPBOARD DENIED" in "rgb(255, 95, 86)"
  And an alert shows exactly "Copy failed — select the link manually."
  # Covers: REQ-008, REQ-012

Scenario: Input prompt prefix
  Given the page at / is open
  Then a ">" character is displayed immediately before the URL input
  And that character is aria-hidden="true"
  # Covers: REQ-009, REQ-013

Scenario: Boot sequence does not block input
  Given the page at / is loading
  When the user focuses the URL input during the boot sequence
  Then the input accepts text immediately
  And any boot overlay is gone within 500 ms
  # Covers: REQ-010

Scenario: Reduced motion disables decoration
  Given the user prefers reduced motion
  When the page at / is open
  Then no CSS animation is active on the cursor, boot overlay or result link
  And all text and controls remain visible
  # Covers: REQ-011

Scenario: All R002 behaviour still passes
  Given the R002 acceptance suite is run unchanged
  Then every R002 scenario still passes
  # Covers: REQ-012

Scenario: Error messages use the error colour
  Given the user submits "example.com"
  When the inline error appears
  Then its computed colour is "rgb(255, 95, 86)"
  And its text is exactly "URL must use http or https"
  # Covers: REQ-014
```

## Assumptions

- "Monospace font stack" means a CSS `font-family` list ending in the generic
  `monospace` keyword; no web font is downloaded.
- CSS animations only decorate; no text is inserted or removed on a timer for visual
  effect. This keeps R002's tests and assistive technology working.
- The status line is separate from the R002 `role="alert"` error element and the button
  labels.
- `prefers-reduced-motion: reduce` is the only user preference handled.

## Status

Approved.
