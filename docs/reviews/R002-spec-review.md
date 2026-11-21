# Spec review: R002

## Summary

**PASS.** R002 (`docs/requirements/R002-frontend.md`, Approved at line 282) and P002
(`docs/plans/P002-frontend.md`, Approved at line 174) are ready for implementation. Every
REQ-001…REQ-018 states one observable behaviour with an appropriate EARS type, fixes exact
strings/status codes/boundaries, is covered by at least one Gherkin `Covers:` line, and is
traced in P002 to a concrete change and a named test. The plan is concrete and file-by-file,
considers alternatives, links the Accepted ADR002 for the consequential stack decision,
records Context7-resolved library IDs/versions, mitigates each risk, and orders small tasks.
No implementation exists (no `package.json`, `src/`, `Cargo.toml`, `Dockerfile`, etc.), so
this is correctly the pre-implementation gate; no blocking findings were raised. The items
under **Findings** are non-blocking suggestions to tighten scenario verifiability and
coverage precision.

## Requirement review

- **Purpose / Scope / Terminology (`R002:3-23`).** In scope and out of scope are explicit
  (`R002:11-16`); fixed button labels, exact validation messages and the `role="alert"`
  contract are given (`R002:20-23`), with API error text referring to R001's table
  (`R002:23`). No ambiguous words ("fast", "friendly", "appropriate"); exact strings,
  status `201`, 2048 chars, 320px and 2 seconds are specified where they form the contract.
- **Requirements (`R002:27-150`).** REQ-001…REQ-018 are numbered sequentially with no
  redefinition or duplicate. Each is one behaviour and the EARS types fit: ubiquitous
  (e.g. REQ-001 `R002:27-32`, REQ-016 `R002:131-136`, REQ-017 `R002:138-143`),
  event-driven (REQ-003 `R002:41-46`, REQ-008 `R002:76-81`, REQ-012 `R002:104-109`),
  state-driven (REQ-004 `R002:48-53`, REQ-018 `R002:145-150`), and unwanted
  (REQ-005 `R002:55-60`, REQ-010 `R002:90-95`, REQ-013 `R002:111-116`).
- **Acceptance criteria (`R002:152-266`).** Sixteen scenarios/outline sets cover the happy
  path, every error path, empty input and the 2049 boundary, stated as observable DOM /
  HTTP / clipboard outcomes (no implementation knowledge needed). Coverage is proven by the
  `Covers:` lines: REQ-001 line 160, REQ-002/REQ-016 line 167, REQ-003/REQ-008 line 174,
  REQ-004 line 181, REQ-005 line 188, REQ-006 line 199, REQ-007 line 206, REQ-009 line 212,
  REQ-010 line 218, REQ-011 line 224, REQ-012 line 232, REQ-013 line 239, REQ-014 line 246,
  REQ-015 line 253, REQ-017 line 259, REQ-018 line 265. All 18 REQs appear.
- **Assumptions / Status (`R002:268-282`).** Assumptions are explicit (button labels, result
  `href`, evergreen browsers, the three validation rules only, `npm test` as the automated
  check); the document is Approved (`R002:282`).

## Plan review

- **Summary / Approach (`P002:3-41`).** Concrete design: Vite/React SPA, `App.tsx` state
  machine, `validation.ts`, `api.ts`, Rust serving of `index.html` plus `ServeDir` at
  `/assets`, and a Node stage in the Docker build. No `fallback_service`, so `/{code}` stays
  with the backend (`P002:37-38`), consistent with P001's reservation of `/` and `/assets/*`
  (`P001:19-21`).
- **Library IDs and versions (`P002:43-55`).** Context7 IDs and pinned versions are recorded
  (React 18.3.1, Vite 6.4.x, Vitest 3.2.x, @testing-library/react 16.3.x, tower-http),
  satisfying the "record non-obvious APIs" rule.
- **Spec interpretation (`P002:57-67`).** REQ-001's "HTML contains a form" is documented as
  the rendered DOM, user-confirmed and recorded per `AGENTS.md`'s deviation rule; also noted
  in `ADR002:32-33`.
- **Alternatives + ADR (`P002:69-77`, `P002:12`).** Five alternatives are considered with
  reasons; the consequential stack decision has an Accepted ADR002 linked from the plan.
- **Changes (`P002:79-102`).** File-by-file responsibilities are listed; no behaviour beyond
  R002 is introduced (no routing, history, dashboards, stats, auth, aliases, dark mode or
  extra endpoints).
- **Test strategy (`P002:104-121`).** Each scenario maps to a component test, the Docker
  smoke or the browser smoke, with exact check commands (`npm ci && npm test`,
  `npm run build`, `npx tsc --noEmit`, plus P001's cargo checks).
- **Risks / Tasks (`P002:123-147`).** Every risk has a mitigation; tasks 1-12 are ordered and
  small enough to execute individually.
- **Traceability (`P002:149-170`).** All 18 REQs appear with a concrete change and a named
  test (e.g. REQ-004 line 156, REQ-016 line 168, REQ-018 line 170).

## Cross-check

- **No uncovered requirement.** All 18 REQs have both a Gherkin `Covers:` line (R002, quoted
  above) and a traceability row with change + test (`P002:153-170`).
- **No scenario without a verification path.** Every Gherkin scenario is named in the plan's
  component/Docker/browser test strategy (`P002:104-121`), with two assertions covered only
  weakly — see Findings 1-3.
- **No contradictions or conflicting IDs** between the two documents. P001 explicitly
  reserves `/` and `/assets/*` for R002 (`P001:19-21`). The one spec interpretation is
  documented (`P002:57-67`, `ADR002:32-33`). P002 reading `index.html` from disk
  (`P002:34-35`) differs from P001's noted `ServeDir::fallback(ServeFile)` (`P001:56`), but
  the choice is explained and is not a behavioural conflict.
- **No out-of-scope task.** All plan tasks serve R002 behaviours; the excluded items
  (`R002:15-16`) do not appear in the plan.

## Code-level axes

Standards: not applicable — pre-implementation gate.
Spec: not applicable — pre-implementation gate.

## Findings

1. **[suggestion]** The keyboard scenario's Given is only "the page at / is open"
   (`R002:242`), yet its Then requires tabbing through "both buttons" (`R002:243`) — the Copy
   button does not exist until a result link is displayed (REQ-008/REQ-012, `R002:80-81`).
   Add a precondition that produces a result so the Copy button is present and the assertion
   is verifiable. — `docs/requirements/R002-frontend.md:241-246`
2. **[suggestion]** REQ-016's "no separate frontend container / single image" behaviour
   (`R002:135-136`) is tagged onto the asset-path scenario (`R002:167`), but that scenario
   only checks served paths and never asserts single-container delivery. Its test is likewise
   just "docker compose up --build, single host/container" (`P002:168`). Add an observable
   scenario/assertion (e.g. one running container built from the single image) so the
   behaviour is verifiable rather than assumed. — `docs/requirements/R002-frontend.md:162-167`
3. **[suggestion]** "No horizontal scrollbar" (`R002:129`, `R002:251`) has no automated check
   (jsdom does no layout); it relies only on the browser smoke's "observe the 320px layout"
   (`P002:117-118`). Record an explicit assertion in the smoke evidence (e.g.
   `document.documentElement.scrollWidth <= 320`) to make the outcome verifiable. —
   `docs/plans/P002-frontend.md:117-118`
4. **[suggestion]** Only the 2049 rejection boundary is in Gherkin (`R002:203-206`); the
   2048-character accepted boundary is untested at scenario level. Add a 2048 accepted
   scenario to complete the boundary, as was suggested for R001. —
   `docs/requirements/R002-frontend.md:201-206`
5. **[suggestion]** "If the input is empty" (REQ-005, `R002:59`) does not define whether a
   whitespace-only value counts as empty, and the scenario uses a bare "empty input"
   (`R002:185`). Specify trim/no-trim so the validation message and request suppression are
   testable for that case. — `docs/requirements/R002-frontend.md:55-60`
6. **[suggestion]** REQ-001's scenario says "the HTML contains a form" (`R002:159`) while the
   plan interprets it as the rendered DOM (`P002:57-67`). The interpretation is documented and
   user-confirmed, but rewording the scenario to "the rendered DOM contains..." would remove
   the tension at its source. — `docs/requirements/R002-frontend.md:159`
7. **[suggestion]** "No asset is served from a path outside /assets/" (`R002:166`) is a
   negative assertion that cannot be verified exhaustively. Define it concretely (e.g. every
   `script`/`link` reference in the built HTML starts with `/assets/`) so the check is
   deterministic. — `docs/requirements/R002-frontend.md:166`
