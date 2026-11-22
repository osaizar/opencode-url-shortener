# URL Shortener — OpenCode framework demo

A small but complete URL shortener: a Rust/axum backend, a React single-page UI, and a
persistent SQLite database, all shipped as **one Docker image**.

This repository is also a **demo of the OpenCode development framework** described at
<https://github.com/osaizar/opencode>. It shows that framework's requirements-first
workflow in practice: EARS + BDD requirement documents, traceable implementation plans,
architecture decisions, two independent review gates, skill-driven authoring and
Context7 library lookups. Every feature was specified, planned, reviewed, implemented and
verified through that workflow, and the resulting artifacts are kept under `docs/`.

## What the app does

- `POST /api/v1/urls` with `{"url": "https://..."}` → `201` with a 7-character base62
  `code` and a ready-to-use `short_url`.
- `GET /{code}` → `302` to the original URL; unknown codes return a structured `404`.
- `GET /health` → `200 {"status": "ok"}`.
- Consistent JSON errors (`invalid_url`, `invalid_request`, `not_found`, `internal_error`).
- A retro-terminal UI for pasting a URL and copying the short link.
- Configuration through `PORT`, `PUBLIC_BASE_URL`, `DATABASE_PATH` and `ASSET_DIR`.
- Mappings survive restarts: Compose mounts a named volume at `/data`.

## Stack

| Layer | Technology |
|---|---|
| Backend | Rust 1.100, axum 0.7, SQLx + SQLite, `rand` 0.9, `url` 2 |
| Frontend | React 18.3, TypeScript 5.9, Vite 6.4, Vitest 3.2 + React Testing Library |
| Delivery | Multi-stage Dockerfile (Node → Rust → slim runtime), Docker Compose |
| Tooling | OpenCode config, custom agents, slash commands, skills, Context7 MCP |

## Quick start

Prerequisites: Docker + Compose, Rust via `rustup`, Node.js ≥ 22.20.

```bash
docker compose up --build -d
curl http://localhost:8080/health
# open http://localhost:8080 in a browser
```

`docker compose down` stops it; the named volume keeps the database. Use
`docker compose down -v` only if you want to delete saved links.

For local development without Docker:

```bash
cd frontend && npm ci && npm run build && cd ..
source "$HOME/.cargo/env" && cargo run
```

The backend reads the built SPA from `frontend/dist` by default; set `ASSET_DIR` to point
elsewhere. To change the API port or public base URL, set `PORT` and `PUBLIC_BASE_URL`.

## Repository layout

```
src/                     Rust backend (handlers, validation, store, code generation)
frontend/src/            React UI and component tests
migrations/              SQLite schema
tests/api.rs             Backend integration tests
Dockerfile               Three-stage build: frontend assets, Rust binary, runtime image
docker-compose.yml       Port mapping, env vars and the persistent data volume
opencode.jsonc           OpenCode config: Context7 MCP, plan-agent override, permissions
AGENTS.md                Persistent rules, artifact locations, verification commands
.opencode/agents/        spec-reviewer (pre-implementation), verifier (post-implementation)
.opencode/commands/      /spec, /plan, /spec-review, /implement, /review
.opencode/skills/        Local skills: EARS+BDD authoring, planning, Context7 docs
.agents/skills/          Third-party skills vendored via skills-lock.json
docs/                    Templates plus R###/P###/ADR### artifacts and review reports
```

## OpenCode workflow

```
/spec  →  /plan  →  /spec-review  →  /implement  →  /review  →  done
```

1. **`/spec "<feature>"`** — the `plan` agent uses `grilling` to clarify the feature,
   then `requirements-earns-bdd` to write `docs/requirements/R###-<slug>.md`: EARS
   statements with stable `REQ-###` IDs and Gherkin `Given/When/Then` scenarios.
2. **Refine** — challenge completeness and feasibility, then approve the spec.
3. **`/plan R###`** — writes `docs/plans/P###-<slug>.md` with approach, alternatives,
   file-level changes, test strategy, risks, ordered tasks and `REQ-###` traceability.
   Consequential decisions get an ADR in `docs/decisions/`.
4. **`/spec-review R### P###`** — the `spec-reviewer` subagent checks document quality,
   coverage, testability and traceability, and writes
   `docs/reviews/<id>-spec-review.md`.
5. **`/implement P###`** — the `build` agent writes code and tests, using Context7 for
   library APIs, then launches the `verifier`.
6. **Final `/review`** — the `verifier` reviews the diff, runs the checks, verifies every
   Gherkin scenario and writes `docs/reviews/<id>-verification.md`. Done when every
   scenario passes and there are no blocking findings.

`AGENTS.md` enforces the gates: no implementation code until the requirement and plan are
approved, and independent verification both before and after implementation.

## Artifacts in this repo

| Requirement | Plan | Decision | Verification |
|---|---|---|---|
| `R001-url-shortener.md` (backend API) | `P001` | `ADR001` backend stack | `R001-spec-review.md`, `R001-verification.md` |
| `R002-frontend.md` (SPA) | `P002` | `ADR002` frontend stack | `R002-spec-review.md`, `R002-verification.md` |
| `R003-terminal-theme.md` (UI theme) | `P003` | — | `R003-spec-review.md`, `R003-verification.md` |

Browser smoke evidence is filed alongside the verification reports as
`docs/reviews/R00#-verification-browser-smoke.md`.

## Verification commands

From the repository root:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

From `frontend/`:

```bash
npm ci
npm test
npm run build        # runs tsc --noEmit && vite build
```

The same commands are documented in `AGENTS.md` and are run by the `verifier`.

## OpenCode configuration notes

- `opencode.jsonc` keeps `default_agent: "build"` and overrides only the built-in `plan`
  agent so it may author `docs/**` but not implementation code. `git push *` requires
  approval, and `experimental.subagent_depth: 2` lets the verifier run its two review axes.
- Context7 is configured as a remote MCP server with OAuth; sign in via `/mcps`. Library
  IDs used by the implementation are recorded in the plans.
- Local skills (`requirements-earns-bdd`, `implementation-planning`, `context7-docs`) win
  on ID collisions with installed skills (`grill-me`, `grilling`, `implement-spec`,
  `code-review`, `find-skills`). Check with `npx skills check`; update with
  `npx skills update -p -y`.

An optional local plugin could run the verifier on `session.idle` for any build session;
it is described in the framework repository but intentionally not included here, because
injecting a prompt would otherwise re-trigger itself.

---

[ ![AI:X EXECUTE](https://raw.githubusercontent.com/NamtaoProductions/0rwx.com/main/static/AI-X-light.svg#gh-light-mode-only) ![AI:X EXECUTE](https://raw.githubusercontent.com/NamtaoProductions/0rwx.com/main/static/AI-X-dark.svg#gh-dark-mode-only)](https://0rwx.com)

**This repository follows [X (EXECUTE)](https://0rwx.com) in the [0rwx framework](https://0rwx.com)** — AI has execute permissions (vibe coding): I vibe code, "I curate".
