# ADR002 — Frontend stack

## Status

Accepted

## Context

R002 requires a minimal browser UI for the URL shortener. The spec interview chose a
React + TypeScript SPA built with Vite and served by the same axum binary and Docker
image as the API (no separate frontend container), with plain CSS and Vitest + React
Testing Library for checks.

## Decision

Use React 18.3.x + TypeScript 5.9.x + Vite 6.4.x, built in a Node stage of the existing
Dockerfile. Emit assets under `/assets/` (`base: '/'`, `assetsDir: 'assets'`) and serve
`index.html` at `/` from axum via `tower_http::services::ServeDir` mounted with
`nest_service`. Tests use Vitest 3.2.x, jsdom and @testing-library/react 16.3.x.

## Consequences

### Positive

- One container and one origin; no CORS or separate deployment pipeline.
- Familiar component testing with deterministic fetch/clipboard/timer mocks.
- Asset path rule from R002 is enforceable with Vite's `assetsDir`.

### Negative

- Adds a Node toolchain stage and `node_modules` to the build.
- Raw `index.html` is a shell, so R002's "HTML contains a form" scenario must be read
  as the rendered DOM (documented in P002).
- Version drift across four JS packages requires pinned versions and a lockfile.

## Alternatives

| Alternative | Reason not chosen |
|---|---|
| Vanilla JS embedded in Rust | Rejected in the spec interview. |
| Server-rendered templates | Would change the approved architecture. |
| Separate nginx/frontend container | Violates R002 REQ-016. |

## Related

- Requirement: `R002-frontend.md`
- Plan: `P002-frontend.md`
