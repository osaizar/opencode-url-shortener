# Verification: R001

## Summary

**PASS.** The implementation in commit `cfda989` (diff `2c6510d...HEAD`, 19 files) satisfies R001: all 12 Gherkin scenarios were exercised with concrete evidence, all three documented Rust checks pass (17 tests), and the Docker smoke test confirms persistence across container recreation via the named volume. No blockers were found. The findings below are suggestions: one genuine spec-conformance edge case (the 2048-character limit measures bytes rather than characters), one plan-traceability gap (no automated process-restart test exists), and several minor hygiene/plan-alignment items. The implementation matches the approved plan functionally; the only behavioural deviation is where trailing-slash trimming lives (handler rather than `Config`), which is covered by a test but not reflected in the plan text.

## Standards

Standards sources reviewed: `AGENTS.md` (workflow gates, artifact naming, definition of done, verification commands, safety), `README.md` (workflow and P001/P002 scope), and the pinned-version/Context7 decisions in `P001`. There is no `CODING_STANDARDS.md` or `CONTRIBUTING.md`. No secrets or R002-only code (catch-all route, `ServeDir`, `tower-http`) were introduced; `axum = "0.7"` and `rand = "0.9"` match `P001:60`, and the Context7 library IDs are recorded. No tooling-enforced issues (fmt/clippy are clean).

- **Artifact naming — documented-standard breach (minor).** `docs/reviews/R001-verification-preimplementation.md` does not follow the `<id>-verification.md` convention at `AGENTS.md:27`. This was a deliberate preservation of the old pre-implementation report so the canonical `R001-verification.md` could hold this post-implementation result; the archive naming should be documented if kept. — `docs/reviews/R001-verification-preimplementation.md:1`
- **Workflow gate — process (minor).** P001 task 14 is still `[ ]`; the verifier may write only to `docs/reviews/**`, so the implementer must tick it after including this report. — `docs/plans/P001-url-shortener-backend.md:145`
- **Stale doc (minor, pre-existing).** `README.md:128-129` still says "no stack is chosen yet… 'no automated checks configured'", contradicted by the commands now in `AGENTS.md:55-65`. — `README.md:128-129`
- **Duplicated Code (judgement call).** Router assembly is identical in `src/main.rs:39-43` and `tests/api.rs:43-47`; SQLite bootstrap (`format!("sqlite://…")` → `SqliteConnectOptions` → `SqlitePoolOptions` → `migrate!`) is duplicated in `src/main.rs:24-31` and `tests/api.rs:23-35`. An extracted `build_router`/pool helper would remove both; test-side duplication is a common and acceptable trade-off. — `src/main.rs:24-43`, `tests/api.rs:23-54`
- **Speculative Generality / Middle Man (judgement call).** `Store::pool()` has no callers (`grep` confirms); remove it unless needed later. `SequenceGenerator`'s counter fallback (`src/code.rs:38-40`) and unused derives on response types (`src/handlers.rs:28,34`) are also speculative, though harmless. — `src/store.rs:15-17`, `src/code.rs:19-42`, `src/handlers.rs:28-37`

Axis summary: 3 minor documented/process items + 3 judgement-call smells; 0 hard code violations. Worst within axis: the artifact-naming breach (`AGENTS.md:27`).

## Spec

Spec source: `docs/requirements/R001-url-shortener.md` (Approved, with change log at `R001:297-303`). All REQ-001…REQ-020 are implemented; no scope creep was found.

- **Mis-implemented boundary — bytes vs characters.** > REQ-008 (`R001:90-91`): "If the submitted `url` exceeds 2048 **characters**…" and error text (`R001:29`): "…of at most 2048 **characters**". `src/validation.rs:6` uses `input.len()`, which is UTF-8 **byte** length in Rust. A URL of 2048 characters containing multibyte UTF-8 is wrongly rejected with `invalid_url`. Both boundary tests use ASCII only (`src/validation.rs:43-48`, `tests/api.rs:182-184`), so the ASCII 2048/2049 cases pass and mask this. `input.chars().count() > 2048` would match the requirement. Does not fail any Gherkin scenario. — `src/validation.rs:6`
- Everything else conforms: exact status codes/error codes/messages (`src/error.rs:20-52`), `201` with exactly `code` and `short_url` (`src/handlers.rs:50-56`), 10-attempt cap (`src/handlers.rs:14,46-60`), 302 with verbatim `Location` preserving query+fragment (`src/handlers.rs:67-69`; `Url::parse` used only for validation), distinct codes for duplicate URLs (no unique index at `migrations/0001_create_urls.sql:1-4`), defaults and overrides (`src/config.rs:10-31`), trailing-slash trimming at use (`src/handlers.rs:49`), and named-volume persistence (`docker-compose.yml:10-11,18-19`).

Axis summary: 1 finding (a suggestion, not blocking an acceptance scenario). Worst within axis: the byte-vs-character length check (`src/validation.rs:6`).

## Acceptance criteria

| Scenario | Status | Evidence |
|---|---|---|
| Shorten a valid URL and use the link (`R001:179`, REQ-001/002/004/006/010/014/015/016/019) | pass | Live binary, all three env vars unset, from an empty temp cwd: `POST /api/v1/urls {"url":"https://example.com/path?q=1#frag"}` → `201` `{"code":"XgViW7o","short_url":"http://localhost:8080/XgViW7o"}` (exactly 2 fields; code matched `^[0-9A-Za-z]{7}$`); `./urls.sqlite3` created (20480 bytes) in cwd; after restarting the binary `GET /XgViW7o` → `302` `location: https://example.com/path?q=1#frag`. Also `cargo test`: `creates_a_short_url_and_redirects_to_the_exact_target` and `config::tests::defaults_are_used_when_variables_are_unset` ok. |
| Repeated submission of the same URL gets a new code (`R001:192`, REQ-002/005) | pass | `cargo test --all-targets`: `repeated_submission_gets_a_distinct_code ... ok` — two `201`s with distinct codes, both redirecting to `https://example.com/same`. No unique index on `target_url` (`migrations/0001_create_urls.sql:1-4`). |
| Generated code collision is retried (`R001:199`, REQ-003) | pass | `cargo test`: `collision_retries_with_the_next_code ... ok` — generator seeded `aaaaaaa` (pre-inserted), `ccccccc`; created mapping used `ccccccc` and `aaaaaaa` still mapped to its original target. Retry loop capped at `MAX_CODE_ATTEMPTS = 10` (`src/handlers.rs:14,46-60`). |
| Mappings survive a process restart (`R001:206`, REQ-004) | pass | Live process: binary on port 9091 with `DATABASE_PATH=/tmp/opencode/restart/urls.sqlite3` created code `1dYQMUJ`; process killed (health unreachable), same binary restarted with same DB path; `GET /1dYQMUJ` → `302` `location: https://example.com/persist`. (No automated restart test exists — see Findings.) |
| Mappings survive container recreation (`R001:212`, REQ-016/018) | pass | Docker smoke: `POST` → code `priTjn9`, `GET /priTjn9` → `302` `location: https://example.com/path?q=1#frag-docker`; `docker compose down && docker compose up -d` (no `-v`); same GET again → identical `302` and `Location`. `docker compose exec -T url-shortener ls -l /data/urls.sqlite3` → file present; named volume `url-shortener_url-data` exists. |
| Invalid target URL is rejected (outline, 4 examples: `ftp://example.com`, `example.com`, `https://`, `""`) (`R001:220`, REQ-007/020) | pass | `cargo test`: `invalid_urls_are_rejected_with_the_exact_error ... ok` asserts `400` and the exact `invalid_url` body for all four inputs; `validation::tests::rejects_unsupported_schemes_and_missing_hosts ... ok`. |
| URL longer than 2048 characters is rejected (`R001:234`, REQ-008/020) | pass | `cargo test`: `overlong_url_is_rejected ... ok` (2049-character ASCII URL → `400`, `error: invalid_url`) and `validation::tests::enforces_the_2048_character_boundary ... ok` (2048 accepted, 2049 rejected). Exact message is the single `InvalidUrl` body in `src/error.rs:23-29`, asserted verbatim by the invalid-URL test. See byte-vs-character finding. |
| Malformed creation request is rejected (outline, 3 examples: `not-json`, `{}`, `{"url": 123}`) (`R001:242`, REQ-009/020) | pass | `cargo test`: `malformed_requests_are_rejected ... ok` — all three → `400` with exact `invalid_request` body (`JsonRejection` mapping at `src/handlers.rs:43`). |
| Unknown short code (`R001:255`, REQ-011/020) | pass | `cargo test`: `unknown_code_returns_not_found ... ok` (`404`, exact `not_found` body). Live binary also returned `404` with `content-type: application/json` and the exact body for an unmapped code. |
| Health check (`R001:263`, REQ-012) | pass | `cargo test`: `health_returns_ok ... ok`; live Docker `GET /health` → `200` `{"status":"ok"}`. |
| Internal failure returns a structured error (`R001:270`, REQ-013/020) | pass | `cargo test`: `internal_failure_returns_the_structured_error ... ok` — `DROP TABLE urls` at runtime → `500` with exact `internal_error` body. |
| Environment variables override defaults (`R001:278`, REQ-014/017) | pass | Live binary with `PORT=9090`, `PUBLIC_BASE_URL=https://sho.rt/`, `DATABASE_PATH=/tmp/custom/urls.sqlite3`: health on 9090 → `200`; `POST` → `201` `{"code":"ozv5wA9","short_url":"https://sho.rt/ozv5wA9"}` (trailing slash trimmed); DB file created at `/tmp/custom/urls.sqlite3` (20480 bytes). Also `config::tests::environment_variables_override_defaults ... ok` and `trailing_slash_in_public_base_url_is_trimmed ... ok`. |

## Checks

- `cargo fmt --all -- --check` (with `$HOME/.cargo/env` sourced): exit 0, no diffs. Toolchain `rustc 1.100.0`, `cargo 1.100.0`.
- `cargo clippy --all-targets -- -D warnings`: exit 0 ("Finished `dev` profile").
- `cargo test --all-targets`: exit 0 — 7 unit tests + 10 integration tests = **17 passed, 0 failed** (list of test names recorded in Acceptance criteria above).
- Docker smoke (`docker compose up --build -d` state already running; verified `GET /health`, `POST /api/v1/urls`, `302` exact `Location` with query+fragment, then `docker compose down && docker compose up -d`, re-check redirect, `docker compose exec … ls -l /data/urls.sqlite3`, `docker volume ls`): all steps passed; stack left running and healthy.
- Live-binary checks (default/env-override/process-restart): described in the Acceptance criteria rows; temp artifacts under `/tmp/opencode/` and `/tmp/custom/`, no repo files written.

## Plan alignment

All P001 tasks 1–13 are marked `[x]` and each REQ-001…REQ-020 traceability row (`P001:149-170`) maps to a real change and an exercised test or the Docker smoke step, with two deviations:

- `REQ-004` is traced to a "Mappings survive a process restart" test (`P001:154`), but `tests/api.rs` contains no restart test. The verifier verified it manually with a real process kill/restart; an automated equivalent is missing. This repeats spec-review finding #2 (`docs/reviews/R001-spec-review.md:105-109`).
- `P001:42-44` places trailing-slash trimming in `Config::from_env()`; the implementation keeps the raw env value in `Config` (`src/config.rs:28`) and trims at use (`src/handlers.rs:49`). REQ-017's observable behaviour is correct and tested, but the plan text is not updated (change log `P001:174-178` says "configuration now trims trailing slashes").

## Findings

1. **[suggestion]** The 2048-character limit is measured in bytes (`input.len()`), not characters; a 2048-character URL containing multibyte UTF-8 is wrongly rejected. Use `input.chars().count()`. — `src/validation.rs:6`
2. **[suggestion]** Plan traceability promises a process-restart test for REQ-004 but none exists in the suite; add an in-process test that rebuilds the pool/router against the same temp DB file. — `docs/plans/P001-url-shortener-backend.md:154`, `tests/api.rs:1-260`
3. **[suggestion]** Trailing-slash trimming lives in the handler, not `Config::from_env()` as the approved plan states; either move it into config or record the deviation in the plan/ADR to keep traceability accurate. — `src/handlers.rs:49`, `src/config.rs:28`, `docs/plans/P001-url-shortener-backend.md:42-44`
4. **[suggestion]** Preserving the old report as `R001-verification-preimplementation.md` deviates from the `<id>-verification.md` naming rule; document the archive convention in `AGENTS.md` if the suffix is to stay. — `AGENTS.md:27`, `docs/reviews/R001-verification-preimplementation.md:1`
5. **[suggestion]** P001 task 14 remains unchecked; the implementer should tick it after including this report (the verifier may not edit the plan). — `docs/plans/P001-url-shortener-backend.md:145`
6. **[suggestion]** `README.md` still claims no automated checks are configured; update now that `AGENTS.md` documents the cargo commands. — `README.md:128-129`
7. **[suggestion]** `Store::pool()` has no callers; remove it. — `src/store.rs:15-17`
8. **[suggestion]** Router assembly and SQLite bootstrap are duplicated between production and test setup; consider extracting shared helpers. — `src/main.rs:24-43`, `tests/api.rs:23-54`

**Verdict: PASS — R001 is satisfied. No blocking findings.**
