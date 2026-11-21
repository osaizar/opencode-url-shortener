# Spec review: R003

## Summary

**PASS — ready for implementation.** R003 (`docs/requirements/R003-terminal-theme.md`,
Approved at line 225) and P003 (`docs/plans/P003-terminal-theme.md`, Approved at line 119)
pass the pre-implementation gate. This is correctly the pre-implementation gate: HEAD is
`f63fdb5` and `git diff f63fdb5...HEAD --name-only` is empty, so no R003 implementation
exists; the R001/R002 code in the tree is expected and was not mistaken for R003 work. Every
REQ-001…REQ-014 is covered by at least one Gherkin scenario and traced in P003 to a concrete
change and a named test. The requirement fixes exact colours, status strings and boundaries,
and the plan is concrete and file-by-file, considers alternatives, records that no new
library/ADR is needed, mitigates each risk and orders small tasks. No blocking findings were
raised; the items under **Findings** are non-blocking suggestions to tighten status-mapping
concreteness and the verification of computed-style assertions.

## Requirement review

- **Purpose / Scope / Terminology (`R003:3-22`).** In scope (palette, typography, CRT framing,
  decorative animations, prompt, status line, accessibility guards) is explicit at
  `R003:11-12`; out of scope (light-mode toggle, new endpoints, new UI features, validation
  changes, message/copy behaviour, form accessibility) at `R003:13-14`. Fixed values are
  given where they form the contract: colours `#33ff66`/`#050805`/`#ffb000`/`#ff5f56`
  (`R003:18`), the five exact status strings `READY`, `SHORTENING...`, `LINK READY`, `COPY OK`,
  `CLIPBOARD DENIED` (`R003:19-20`), and the definition of decoration as `aria-hidden="true"`
  and non-focusable (`R003:21-22`). No ambiguous words ("fast", "friendly", "appropriate")
  were found; the two inherently visual requirements are still bounded — the boot sequence
  "completes within 500 ms" and leaves the form usable (`R003:92-93`), and the reveal keeps
  text in the DOM (`R003:65-67`).
- **Requirements (`R003:26-120`).** REQ-001…REQ-014 are numbered sequentially with no
  redefinition or duplicate ID. EARS types fit: ubiquitous (REQ-001 `R003:26-31`, REQ-002
  `R003:33-38`, REQ-003 `R003:40-45`, REQ-009 `R003:82-86`, REQ-012 `R003:102-107`, REQ-013
  `R003:109-114`, REQ-014 `R003:116-120`), state-driven (REQ-004 `R003:47-52`, REQ-005
  `R003:54-59`, REQ-011 `R003:95-100`), event-driven (REQ-006 `R003:61-67`, REQ-007
  `R003:69-73`, REQ-010 `R003:88-93`), and unwanted (REQ-008 `R003:75-80`). Each has one
  primary observable behaviour; several bundle more than one assertion (see Finding 4), but
  every assertion is independently stated in a scenario, so none is untestable.
- **Acceptance criteria (`R003:124-211`).** There are 13 scenarios (the brief says 14 — see
  Finding 3), and they cover all 14 REQs, proven by the `Covers:` lines: REQ-001 line 130,
  REQ-002 line 137, REQ-003 + REQ-013 line 144, REQ-004 line 151, REQ-005 line 156, REQ-006
  line 164, REQ-007 + REQ-012 line 171, REQ-008 + REQ-012 line 178, REQ-009 + REQ-013 line
  184, REQ-010 line 191, REQ-011 line 198, REQ-012 line 203, REQ-014 line 210. Coverage
  includes happy paths (palette, panel, reveal, copy success), error cases (copy denial
  `R003:173-178`, validation error `R003:205-210`), a preference state (reduced motion
  `R003:193-198`) and boundaries (panel `<= 40rem` `R003:134`, overlay gone `within 500 ms`
  `R003:190`). "Empty values" is not applicable to a purely visual theme; the equivalent
  boundary checks are present. Scenarios are phrased as observable DOM/CSS/preference
  outcomes and need no implementation knowledge.
- **Assumptions / Status (`R003:213-225`).** Assumptions are explicit: monospace stack with
  no web font (`R003:215-216`), no timer-inserted text (`R003:217-218`), status line separate
  from the R002 `role="alert"` element and button labels (`R003:219-220`), and only
  `prefers-reduced-motion` handled (`R003:221`). Document is Approved (`R003:225`).

## Plan review

- **Summary / Approach (`P003:3-40`).** Concrete CSS/React design: palette custom properties,
  monospace stack and centred `max-width: 40rem` panel (`P003:15-18`); scanline overlay via
  `repeating-linear-gradient` with `pointer-events: none` and `aria-hidden` (`P003:18-19`);
  cursor/boot keyframes with the overlay removed after 500 ms and never covering pointer
  input (`P003:20-23`); typing reveal via `clip-path`/`width` with no DOM churn (`P003:24-26`);
  a `<p class="status">` driven by `statusText` with the five exact values (`P003:27-30`);
  `>` prompt span (`P003:31-32`); one reduced-motion media block (`P003:33-35`); and
  decorative `aria-hidden` nodes (`P003:36-38`). It correctly records that no third-party
  library is introduced, so no Context7 lookup is needed (`P003:40`).
- **Alternatives / ADR (`P003:11`, `P003:42-49`).** Four alternatives are considered with
  reasons (Canvas/WebGL, JS per-character typing, Tailwind/UI kit, theme toggle). No ADR is
  needed because the change is pure presentation with no consequential architectural decision
  and no new library, consistent with `P003:11`.
- **Changes (`P003:51-62`).** File-by-file responsibilities are listed (`styles.css`,
  `App.tsx`, tests, verification report) and it explicitly states "R002 behaviour and exact
  strings unchanged" (`P003:55-56`) and that no Rust/Docker/compose/AGENTS changes are needed
  (`P003:62`). No behaviour beyond R003 is introduced.
- **Test strategy (`P003:64-75`).** Component tests cover status values, prompt, aria-hidden
  decoration and CSS assertions; the unchanged R002 suite runs in the same `npm test`;
  browser smoke verifies visuals, the 320px layout and the 500 ms boot overlay with screenshot
  evidence; checks are `npm ci && npm test` and `npm run build` (`P003:74-75`). Each Gherkin
  scenario maps to a named test through the traceability table. Caveat on computed styles noted
  in Finding 2.
- **Risks / Tasks (`P003:77-96`).** Five risks each have a mitigation. Eight tasks are ordered
  and small enough to execute one at a time, ending with the verifier hand-off.
- **Traceability (`P003:98-115`).** Fourteen rows map REQ-001…REQ-014 one-to-one to a concrete
  change and a named test; no requirement is left without a change or a test.

## Cross-check

- **Coverage.** No requirement lacks a scenario or a plan change/test: the 13 `# Covers:`
  lines (R003:130,137,144,151,156,164,171,178,184,191,198,203,210) cover all 14 REQs, and the
  14 traceability rows (P003:102-115) each pair a change with a test. REQ-013 is legitimately
  covered by two shared scenarios rather than a dedicated one.
- **Exact-string consistency.** Contract strings in R003 match R002 and the current code:
  `Copy failed — select the link manually.` (R003:177 vs `validation.ts:5`),
  `URL must use http or https` (R003:209 vs `validation.ts:2`), and `Copied!` (R003:170 vs
  `App.tsx:103`). The status string `SHORTENING...` (three ASCII dots, R003:51, P003:28) is a
  new, separate status-line value and does not conflict with the R002 in-flight button label
  `Shortening…` (ellipsis, `App.tsx:89`); the assumption at R003:219-220 keeps them separate.
- **Contradictions / duplicates.** No contradictory statements between R003 and P003 and no
  duplicated REQ IDs were found. The "no behaviour change" claim (P003:8, R003:7) is
  consistent with adding the in-scope status line and with out-of-scope items left untouched.
- **Scope.** No plan task implements out-of-scope behaviour: there is no light-mode toggle,
  endpoint, validation change or message/copy change. P003 task 4 (`P003:92`) actively guards
  against changing R002 strings.
- **State-machine note.** The existing `status` has four values
  `'idle' | 'submitting' | 'success' | 'error'` (`App.tsx:5`), while five status strings are
  required; `COPY OK` comes from `copied` (`App.tsx:12,55`) and clipboard denial is folded
  into `error` + `success` with no dedicated flag (`App.tsx:60-70`). Therefore "derived from
  the existing state machine" (`P003:27-30`) alone does not define the status line for the
  generic validation/API/network `error` state (see Finding 1).

## Code-level axes

Standards: not applicable — pre-implementation gate.
Spec: not applicable — pre-implementation gate.

## Findings

1. **[suggestion]** Approach step 4 says `statusText` is "derived from the existing state
   machine" (`P003:27-30`), but the existing `status` has only four values including `error`
   (`App.tsx:5`), the five status strings leave the generic validation/API/network error state
   unmapped, and clipboard denial shares the `error`/`success` state with no dedicated flag
   (`App.tsx:60-70`). State explicitly what the status line shows on a non-clipboard error
   (REQ-005 implies `READY` at `R003:58-59`) and how clipboard denial is detected, so REQ-005
   and REQ-008 are unambiguous at implementation time. — `docs/plans/P003-terminal-theme.md:27`
2. **[suggestion]** Several Gherkin assertions are computed-style or browser observations —
   palette colours (`R003:127-129`), blinking animation (`R003:136`), `pointer-events: none`
   (`R003:142`), amber (`R003:150`), error red (`R003:176`, `R003:208`) and "no CSS animation
   is active" (`R003:196`) — which jsdom cannot derive from a stylesheet. The plan hedges with
   "where jsdom supports them" and source-level "presence of keyframes/reduced-motion rules"
   (`P003:66-68`). Add explicit browser-level computed-style assertions, or name the browser
   smoke (`P003:71-73`) as the authoritative verification, mapped to those scenarios so they
   are verified rather than only source-checked. — `docs/plans/P003-terminal-theme.md:66`
3. **[suggestion]** The brief states 14 Gherkin scenarios, but R003 contains 13
   (`R003:125-210`); all 14 REQs remain covered because REQ-013 is shared across two scenarios
   (`R003:144`, `R003:184`). Informational only — no action needed unless a dedicated REQ-013
   scenario is desired. — `docs/requirements/R003-terminal-theme.md:144`
4. **[suggestion]** A few REQs bundle multiple observable assertions (REQ-002
   panel + header + cursor `R003:37-38`; REQ-003 scanlines + glow + pointer-events + a11y
   `R003:44-45`; REQ-008 status + colour + fallback `R003:79-80`; REQ-012 the entire R002
   contract `R003:106-107`), which is looser than strict EARS "one observable behaviour".
   Each assertion is independently asserted in a scenario, so this is testable and
   non-blocking; consider splitting if strict EARS conformance is required. —
   `docs/requirements/R003-terminal-theme.md:37`
