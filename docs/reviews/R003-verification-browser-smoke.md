# Browser smoke test: R003

Date: 2026-11-21. Environment: Docker Compose stack on `http://localhost:8080`,
in-app Chromium plus headless Chromium at 320px.

## Computed styles (authoritative for CSS assertions)

- `body` background: `rgb(5, 8, 5)`; colour: `rgb(51, 255, 102)`.
- Font family: `ui-monospace, SFMono-Regular, Menlo, Consolas, monospace`.
- Scanline overlay: `pointer-events: none`, `aria-hidden="true"`.
- Boot overlay: `aria-hidden="true"`; `visibility: hidden` after the 500 ms animation.
- CRT panel width: **640px** (40rem with `box-sizing: border-box`).
- Prompt prefix: `>` with `aria-hidden="true"`.

## Interaction

- Initial status line: `READY`.
- After a valid submission: status `LINK READY`, result link text present in the DOM,
  computed `animation-name: typing` on the link.
- After a clipboard-denied Copy: status `CLIPBOARD DENIED` with computed colour
  `rgb(255, 95, 86)` and an alert reading exactly
  `Copy failed — select the link manually.` — R002 behaviour unchanged.
- Screenshot: `/tmp/opencode-browser-0nfqsN/0/screenshot.jpeg`.

## Narrow viewport

Headless Chromium at `320x800` with a 2 s virtual-time budget: form fits within the
viewport, no horizontal overflow, bracketed title on one line. Screenshot:
`/tmp/opencode/r003-320.png`.

## Verdict

All observed behaviour matches R003; no blocking findings from the smoke test.
