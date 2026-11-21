# P001 — URL shortener backend

## Summary

Implement the v1 URL-shortener backend specified in `R001-url-shortener.md`: an axum
HTTP service that accepts a URL, stores a random base62 code → URL mapping in SQLite,
redirects short codes with `302`, exposes a health endpoint, returns structured JSON
errors, reads configuration from the environment, and ships as a single Docker image
with a persistent volume.

**Requirement:** `R001-url-shortener.md` (Approved)  
**Related decisions:** `ADR001-backend-stack.md`

## Approach

Layers, from outside in:

1. **HTTP layer (axum).** Router with `POST /api/v1/urls`, `GET /:code` (or `/{code}`
   depending on pinned axum version — see Risks), `GET /health`. No catch-all and no
   handler for `/` or `/assets/*`; those are reserved for R002 and must not swallow
   short-code paths.
2. **Request/response types.** `CreateUrlRequest { url: String }`, `CreateUrlResponse
   { code, short_url }`, `ErrorBody { error, message }` with serde. Handlers use
   `Result<Json<CreateUrlRequest>, JsonRejection>` so malformed bodies can be mapped to
   the exact `invalid_request` contract instead of axum's default rejection text.
3. **Validation.** `validate_target(&str)` checks length ≤ 2048, `Url::parse`, scheme in
   `{http, https}` and `host_str().is_some()`. The original string is stored verbatim —
   no normalisation — so query strings and fragments survive the redirect unchanged.
4. **Code generation.** A `CodeGenerator` trait with a production `Base62Generator`
   (`rand::distr::Alphanumeric::sample_string(&mut rand::rng(), 7)`) and a deterministic
   `SequenceGenerator` for collision tests. This seam makes REQ-003 testable without
   controlling global randomness.
5. **Persistence.** `Store` wraps a `SqlitePool` and exposes `insert(code, url) -> bool`
   (false on primary-key collision) and `find(code) -> Option<String>`. Schema created by
   `sqlx::migrate!("./migrations")` at startup. Runtime-checked `sqlx::query` calls avoid
   a compile-time `DATABASE_URL`. On collision, the handler retries generation up to
   **10** attempts, then returns `internal_error`.
6. **Errors.** `ApiError` enum maps each contract case to its exact status, `error` code
   and `message`; `impl IntoResponse` renders `Json(ErrorBody)`. Any DB failure maps to
   `internal_error` (500). A global fallback handler (or per-handler mapping) covers
   unexpected panics/errors.
7. **Configuration & runtime.** `Config::from_env()` with defaults `PORT=8080`,
   `PUBLIC_BASE_URL=http://localhost:8080` (trailing slashes trimmed before appending
   `/<code>`) and `DATABASE_PATH=./urls.sqlite3`. Compose sets
   `DATABASE_PATH=/data/urls.sqlite3` explicitly for the container. `#[tokio::main]` +
   `axum::serve`.
8. **Packaging.** Multi-stage Dockerfile (Rust builder → slim runtime); Compose maps the
   host port and mounts a named volume at `/data`.

### Library APIs resolved via Context7 (recorded before committing to APIs)

| Purpose | Library ID | API confirmed |
|---|---|---|
| Routing, extractors, state, responses | `/websites/rs_axum_axum` (versions 0.7.9, 0.8.4 available) | `Router::with_state` + `State` extractor (State before body extractor); `Json` must be the last extractor; response tuples `(StatusCode, Json<_>)`; `Result<Json<T>, JsonRejection>` and custom rejection extraction for 400 mapping; `nest_service` / `route_service` for `ServeDir` (reserved for R002) |
| SQLite pool, migrations, queries | `/websites/rs_sqlx_sqlx` | `SqliteConnectOptions::from_str(...)` + `SqlitePool::connect_with`; `sqlx::migrate!("./migrations").run(&pool)`; runtime-checked `sqlx::query(...).bind(...)` |
| URL validation | `/websites/rs_url` | `Url::parse`, `scheme()`, `host_str()`; `host_str()` is `None` for `data:`/`mailto:` etc. |
| Code generation | `/websites/rs_rand_rand` (versions 0.10.3, 0.9.5, 0.8.8) | `Alphanumeric.sample_string(&mut rand::rng(), 7)` with `rand::distr::{Alphanumeric, SampleString}` (rand 0.9+ API) |
| Static file serving (R002 only) | `/websites/rs_tower-http` | `ServeDir::new(...).fallback(ServeFile::new(...))`; not used by P001, noted for P002 |

Version decision (approved): **axum 0.7.x** and **rand 0.9.x** pinned in `Cargo.toml`.
The APIs above are valid in both 0.7/0.8 and 0.9/0.10 respectively, but route syntax
differs (Risks).

## Alternatives considered

| Alternative | Why rejected |
|---|---|
| `rusqlite` (synchronous SQLite) | Would need `spawn_blocking`/connection-pool glue inside async handlers; SQLx fits the Tokio stack directly. |
| PostgreSQL | Requires a separate server; overkill for a single-instance v1 with modest write volume. |
| In-memory `HashMap` | Cannot satisfy REQ-004 (persistence across restarts). |
| actix-web / rocket | Chosen axum in the approved spec; no benefit to revisiting. |
| Sequential/hash-based codes | Rejected in the approved spec (enumeration / forced dedupe). |
| Compile-time `query!` macros | Require `DATABASE_URL` at build time; runtime-checked queries keep local builds and Docker simple. |

## Changes

- `Cargo.toml`, `Cargo.lock` — new Rust binary crate; dependencies: axum, tokio, sqlx
  (sqlite, runtime-tokio, migrate), serde, serde_json, url, rand, tower-http (only if a
  placeholder static route is needed — prefer leaving it to P002); dev-deps: tower
  (`util` for `oneshot`), tempfile or a temp-dir helper.
- `src/main.rs` — config, pool, migrations, router assembly, `axum::serve`.
- `src/config.rs` — `Config::from_env()` and defaults; helper for "running in container"
  detection (Compose sets `DATABASE_PATH` explicitly, so detection is a fallback only).
- `src/error.rs` — `ApiError` → exact status/`error`/`message`; `impl IntoResponse`.
- `src/validation.rs` — `validate_target`: length, parse, scheme, host.
- `src/code.rs` — `CodeGenerator` trait, `Base62Generator`, `SequenceGenerator` (cfg(test)
  or public for integration tests).
- `src/store.rs` — `Store` with `insert`, `find`; SQL and collision detection.
- `src/handlers.rs` — `AppState`, `create_url` (validation → generate/insert retry → 201
  response), `redirect` (302 + `Location`), `health` (200 JSON).
- `migrations/0001_create_urls.sql` — `urls(code TEXT PRIMARY KEY, target_url TEXT NOT NULL)`.
- `tests/api.rs` — in-process integration tests for the Gherkin scenarios using a
  temporary SQLite file, migrations applied at startup.
- `Dockerfile` — multi-stage `rust:...-bookworm` build → `debian:bookworm-slim` runtime.
- `docker-compose.yml` — build, `ports`, `environment`, named volume at `/data`.
- `.dockerignore`, `.gitignore` — exclude `target/`, local sqlite files, secrets.
- `AGENTS.md` — replace "no automated checks configured" with the exact check commands
  (see Test strategy), per the kit's instruction to document commands once a stack exists.
- `docs/reviews/R001-verification.md` — written by the `verifier`, not by implementation.

## Test strategy

- **Unit tests** (`cargo test --lib`): validation boundaries (2048/2049, http/https vs
  ftp, missing host, empty, `data:` URL), `Base62Generator` output length and alphabet,
  `SequenceGenerator` collision sequence, config defaults and overrides.
- **Integration tests** (`tests/api.rs`, in-process router + temp SQLite): one test per
  Gherkin scenario — create/redirect with query+fragment, duplicate URLs get distinct
  codes, unknown code → 404 with exact body, malformed bodies → 400 `invalid_request`,
  invalid URLs → 400 `invalid_url`, overlong URL → 400, health → 200, env overrides for
  port/base URL/DB path, collision retry using `SequenceGenerator`, and internal failure
  via `DROP TABLE urls` fault injection → 500 `internal_error`.
- **Checks:** `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`,
  `cargo test --all-targets`. These become the documented commands in `AGENTS.md`.
- **Docker smoke:** `docker compose up --build -d`; curl `GET /health`, `POST /api/v1/urls`,
  the returned `GET /{code}`, then `docker compose down && docker compose up -d` and
  re-check the redirect to prove volume persistence.
- **Not in P001:** browser/UI tests — those belong to P002 (R002).

## Risks

| Risk | Mitigation |
|---|---|
| axum version drift in route syntax (`:code` in 0.7 vs `{code}` in 0.8) | Pin the exact axum version in `Cargo.toml`; if upgrading, re-query Context7 before changing route patterns. |
| `DROP TABLE` fault injection might be undone by startup migrations | Apply migrations once at startup only; the test drops the table *after* startup and issues a request against the live pool. |
| Collision retry loop could spin indefinitely | Cap at 10 attempts; return `internal_error` on exhaustion (probability with 62^7 is negligible). |
| SQLite write contention under concurrent requests | Short transactions and a single writer with WAL journal mode if needed; acceptable for v1 scale. |
| Volume data lost if `docker compose down -v` is used | Document that `-v` removes the named volume; normal `down`/`up` preserves it. |
| `Location` header with raw query/fragment must not be normalised | Store and emit the original input string; assert exact equality in tests. |

## Tasks

- [x] 1. Install/verify Rust toolchain (or decide Docker-only builds); record in verification report. — Done: rustup stable 1.100.0 installed locally.
- [x] 2. Scaffold `Cargo.toml` with pinned dependencies and `src/main.rs` skeleton.
- [x] 3. Implement `Config::from_env` with defaults and env overrides.
- [x] 4. Add migration and `Store` with `insert`/`find`.
- [x] 5. Implement `validate_target` and its unit tests.
- [x] 6. Implement `CodeGenerator`, `Base62Generator`, `SequenceGenerator`.
- [x] 7. Implement `ApiError` and exact error bodies.
- [x] 8. Implement the three handlers and router; wire `AppState`.
- [x] 9. Add integration tests for all R001 scenarios, including collision and 500 fault injection.
- [x] 10. Add Dockerfile, compose file, ignore files.
- [x] 11. Update `AGENTS.md` with the exact verification commands.
- [x] 12. Run fmt/clippy/tests; fix failures.
- [x] 13. Run Docker smoke test incl. container-recreation persistence.
- [ ] 14. Launch the `verifier` with R001 + P001 and include its report.

## Traceability

| Requirement | Change | Test |
|---|---|---|
| REQ-001 | `src/handlers.rs` `create_url`; `src/main.rs` POST route | "Shorten a valid URL and use the link" / `tests/api.rs::create_endpoint` |
| REQ-002 | `src/code.rs` `Base62Generator` | `src/code.rs` unit test + "Shorten a valid URL..." |
| REQ-003 | `src/store.rs` `insert` collision + retry in `create_url`; `SequenceGenerator` | "Generated code collision is retried" |
| REQ-004 | `migrations/0001_create_urls.sql`, `src/store.rs`, `DATABASE_PATH` config | "Mappings survive a process restart" |
| REQ-005 | `create_url` always generates a new code; no unique index on `target_url` | "Repeated submission of the same URL gets a new code" |
| REQ-006 | `src/handlers.rs` `CreateUrlResponse` with `201` | "Shorten a valid URL and use the link" |
| REQ-007 | `src/validation.rs` scheme + host checks | "Invalid target URL is rejected" (outline) |
| REQ-008 | `src/validation.rs` length check | "URL longer than 2048 characters is rejected" |
| REQ-009 | `JsonRejection` → `invalid_request` in `src/handlers.rs`/`src/error.rs` | "Malformed creation request is rejected" (outline) |
| REQ-010 | `src/handlers.rs` `redirect` with `302` + `Location` | "Shorten a valid URL and use the link" |
| REQ-011 | `src/error.rs` `not_found` | "Unknown short code" |
| REQ-012 | `src/handlers.rs` `health` | "Health check" |
| REQ-013 | `src/error.rs` `internal_error` + DB error mapping | "Internal failure returns a structured error" |
| REQ-014 | `src/config.rs` `PORT` default | "Shorten a valid URL..." (listens on 8080) |
| REQ-015 | `src/config.rs` `PUBLIC_BASE_URL` default | "Shorten a valid URL..." (`short_url` prefix) |
| REQ-016 | `src/config.rs` `DATABASE_PATH` default | "Shorten a valid URL..." (local) + "Mappings survive container recreation" (container) |
| REQ-017 | `src/config.rs` `PUBLIC_BASE_URL` override | "Environment variables override defaults" |
| REQ-018 | `docker-compose.yml` named volume at `/data` | "Mappings survive container recreation" |
| REQ-019 | `src/store.rs` stores raw string; `redirect` emits it | "Shorten a valid URL and use the link" (query + fragment) |
| REQ-020 | `src/error.rs` `ApiError` → `ErrorBody` | All five error scenarios |

## Change log

- After `docs/reviews/R001-spec-review.md` (non-blocking suggestions #1, #3, #5):
  configuration now trims trailing slashes from `PUBLIC_BASE_URL`, the local
  `DATABASE_PATH` default is the only unset default (Compose sets the container path
  explicitly), REQ-003's 10-attempt cap is the implementation contract, and REQ-020's
  traceability now says five error scenarios. No `REQ-###` was redefined.

## Status

Approved.

---

## ADR001 — Backend stack and persistence

See `docs/decisions/ADR001-backend-stack.md`.
