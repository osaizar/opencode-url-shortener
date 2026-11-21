# P003 — Retro terminal theme

## Summary

Restyle the existing R002 single page as a retro terminal: phosphor green on near-black,
monospace type, CRT panel with bracketed header, blinking cursor, scanline overlay,
boot-up sequence, typing-effect result reveal, `>` prompt and a status line. The change is
purely presentational — no R002 behaviour, message, endpoint or Docker change.

**Requirement:** `R003-terminal-theme.md`  
**Related decisions:** none (no architectural decision; pure CSS/React presentation)

## Approach

1. **Theme in CSS (`frontend/src/styles.css`).** Set the palette with CSS custom
   properties (`--phosphor: #33ff66`, `--bg: #050805`, `--amber: #ffb000`,
   `--error: #ff5f56`), a monospace `font-family` stack ending in `monospace`, and a
   centred panel with `max-width: 40rem`. Scanlines use a `repeating-linear-gradient`
   overlay with `pointer-events: none` and `aria-hidden="true"` in markup.
2. **Blinking cursor and boot overlay.** CSS `@keyframes` on a `::after` cursor and a
   boot overlay that animates opacity/scale and is removed after 500 ms. The form is
   never disabled or covered for pointer input; the overlay is `aria-hidden` and
   `pointer-events: none`.
3. **Typing-effect reveal without DOM churn.** Animate `clip-path`/`width` on the result
   container so the full link text is in the DOM immediately — no per-character JS
   insertion. This preserves R002's tests and screen-reader behaviour.
4. **Status line in `App.tsx`.** A dedicated `<p class="status">` with an explicit mapping,
   not an implicit inference from the R002 `status` state:
   - `status === 'submitting'` → `SHORTENING...`
   - a separate `copyStatus` flag (`'none' | 'ok' | 'denied'`, reset on every submission)
     → `COPY OK` or `CLIPBOARD DENIED`
   - `status === 'success'` and `copyStatus === 'none'` → `LINK READY`
   - `status === 'error'` (including non-clipboard API/validation errors) or `idle` →
     `READY`
   This keeps the clipboard path independent of the generic error state and preserves
   the exact R002 alert text.
5. **Prompt prefix.** A `>` span rendered immediately before the input with
   `aria-hidden="true"`.
6. **Reduced motion.** A single `@media (prefers-reduced-motion: reduce)` block sets all
   animations to `none` and makes the boot overlay immediately invisible. All content
   remains visible.
7. **Accessibility.** Decorative nodes carry `aria-hidden="true"` and are non-focusable
   (spans/divs with no `tabindex`); the form, labels, buttons and error alert stay exactly
   as R002 specifies.

No third-party library is introduced, so no Context7 lookups are needed for this plan.

## Alternatives considered

| Alternative | Why rejected |
|---|---|
| Canvas/WebGL CRT shader | Heavy, hurts accessibility and testability for a cosmetic effect. |
| JS per-character typing | Would withhold text from the DOM, breaking R002 tests and assistive tech. |
| Tailwind or a UI kit | R002 chose plain CSS; adding a build-time dependency for a theme is unnecessary. |
| Light/dark theme toggle | Explicitly out of scope in R002/R003. |

## Changes

- `frontend/src/styles.css` — palette variables, monospace stack, CRT panel, scanlines,
  glow, cursor/boot/typing keyframes, status colours, reduced-motion block.
- `frontend/src/App.tsx` — status line, prompt prefix, boot overlay, decorative
  `aria-hidden` nodes; R002 behaviour and exact strings unchanged.
- `frontend/src/App.test.tsx` (or a new `frontend/src/theme.test.tsx`) — tests for the
  status values, aria-hidden decoration, palette/animation CSS assertions and that the
  R002 suite still passes unchanged.
- `docs/reviews/R003-verification.md` — written by the `verifier` after implementation.

No Rust, Docker, compose or AGENTS.md changes are needed.

## Test strategy

- **Component tests (`npm test`):** assert each status value at the right state, the
  `>` prompt, `aria-hidden` decoration, error colour assertions via computed styles
  where jsdom supports them, and the presence of keyframes/reduced-motion rules.
- **Regression:** run the unchanged R002 suite in the same `npm test` execution; every
  R002 scenario must still pass.
- **Browser smoke (authoritative for computed styles):** jsdom cannot compute CSS, so
  the browser check is the evidence for palette colours, `pointer-events`, animations
  and reduced-motion behaviour. Open the running container, read computed styles, verify
  the visual treatment and the 320px layout, check the boot overlay is gone within 500 ms,
  and capture a screenshot for evidence in
  `docs/reviews/R003-verification-browser-smoke.md`. Component tests assert class names
  and the presence of CSS rules only.
- **Checks:** `npm ci && npm test`, `npm run build`; backend checks remain green but no
  backend files change.

## Risks

| Risk | Mitigation |
|---|---|
| CSS animations hide text from tests or screen readers | Animate only visual properties (`clip-path`, opacity); keep text in the DOM from first render. |
| `prefers-reduced-motion` ignored | Single media query disabling all keyframes; assert its presence in tests and in the browser. |
| Scanline overlay blocks clicks | `pointer-events: none` and `aria-hidden`; assert in tests. |
| Contrast/readability of green-on-black | Use the specified `#33ff66` on `#050805`, amber and red only for state, verify in browser screenshot. |
| Boot overlay delays interaction | Never disable the form; `pointer-events: none`; 500 ms maximum via CSS animation. |

## Tasks

- [ ] 1. Add palette variables and monospace/CRT panel styles to `styles.css`.
- [ ] 2. Add scanline/glow overlay, cursor and boot/typing keyframes with reduced-motion rules.
- [ ] 3. Add status line and prompt prefix to `App.tsx`, wiring the five exact status values.
- [ ] 4. Keep the R002 alert/fallback text and button labels unchanged.
- [ ] 5. Add theme tests and confirm the unchanged R002 suite still passes.
- [ ] 6. Run `npm test` and `npm run build`; check the 320px layout in a browser.
- [ ] 7. Record browser smoke evidence.
- [ ] 8. Launch the `verifier` with R003 + P003 and include its report.

## Traceability

| Requirement | Change | Test |
|---|---|---|
| REQ-001 | `styles.css` palette + monospace stack | "Terminal palette and monospace typography" |
| REQ-002 | `styles.css` panel/header/cursor keyframes | "CRT panel with header and blinking cursor" |
| REQ-003 | `styles.css` scanline overlay; `App.tsx` `aria-hidden` | "Scanlines and glow are decorative only" |
| REQ-004 | `App.tsx` `SHORTENING...` status + amber CSS | "In-flight status is amber" |
| REQ-005 | `App.tsx` initial `READY` status | "Idle status" |
| REQ-006 | `App.tsx` `LINK READY` + CSS typing animation | "Result reveal keeps text in the DOM" |
| REQ-007 | `App.tsx` `COPY OK` on clipboard success | "Copy success status" |
| REQ-008 | `App.tsx` `CLIPBOARD DENIED` + unchanged R002 alert | "Copy failure status" |
| REQ-009 | `App.tsx` `>` prompt span | "Input prompt prefix" |
| REQ-010 | `styles.css` boot animation; `App.tsx` overlay | "Boot sequence does not block input" |
| REQ-011 | `styles.css` reduced-motion block | "Reduced motion disables decoration" |
| REQ-012 | No behaviour changes in `App.tsx`/`api.ts`/`validation.ts` | "All R002 behaviour still passes" |
| REQ-013 | `aria-hidden` on decorative nodes | "Scanlines and glow are decorative only" + "Input prompt prefix" |
| REQ-014 | `styles.css` error colour | "Error messages use the error colour" |

## Change log

- After `docs/reviews/R003-spec-review.md` (non-blocking suggestions #1 and #2): the
  status-line mapping is now explicit, with a dedicated `copyStatus` flag and a defined
  `READY` fallback for non-clipboard errors; browser smoke is named as the authoritative
  verification for computed styles that jsdom cannot evaluate. Suggestions #3 (scenario
  count) and #4 (bundled assertions) require no action.

## Status

Approved.
