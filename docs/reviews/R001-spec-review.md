# Spec review: R001

## Summary

**PASS.** R001 (`docs/requirements/R001-url-shortener.md`, Approved at line 295) and P001
(`docs/plans/P001-url-shortener-backend.md`, Approved at line 172) are ready for
implementation. Every REQ-001…REQ-020 has exactly one observable behaviour with an
appropriate EARS type, uses exact strings/status codes/boundaries, is covered by at least
one Gherkin scenario, and is traced in P001 to a concrete change and a named test. The
plan is concrete and file-by-file, considers alternatives, links ADR001 for the
consequential stack decision, records resolved library IDs/versions, mitigates each risk,
and orders small tasks. No implementation exists (no `Cargo.toml`, `src/`, `tests/`,
`migrations/`, `Dockerfile` or `docker-compose.yml`), so this is correctly the
pre-implementation gate; no blocking findings were raised. The items under **Findings**
are non-blocking suggestions to tighten precision and testability.

## Requirement review

- **Purpose / Scope (`R001:3-14`).** In scope and out of scope are explicit; the out-of-scope
  list (analytics, aliases, expiry, auth, rate limiting, management, QR, bulk import) is
  concrete and none of it reappears in the plan.
- **Terminology and error table (`R001:16-28`).** Base62 alphabet/case-sensitivity and the
  exact error shape `{"error": "<code>", "message": "<exact text>"}` with
  `Content-Type: application/json` are fixed. All four conditions give exact status,
  `error` and `message` — no ambiguous words ("fast", "friendly", "appropriate") appear.
- **Requirements (`R001:30-170`).** REQ-001…REQ-020 are numbered sequentially with no
  redefinition or duplicate. Each is one behaviour and the EARS types fit: ubiquitous
  (e.g. `REQ-001` line 34, `REQ-004` line 55, `REQ-020` line 167), event-driven
  (`REQ-002` line 41, `REQ-010` line 98, `REQ-017` line 146), state-driven (`REQ-005`
  line 62, `REQ-014` line 126, `REQ-016` line 139), unwanted (`REQ-003` line 48,
  `REQ-007` line 77, `REQ-013` line 119). Exact contracts are given: 7-character base62
  (line 44), status/error codes (lines 80, 87, 94, 108, 122), `201` with exactly `code`
  and `short_url` (line 71-73), `302` + `Location` (line 100-101), `200` + `{"status":
  "ok"}` (line 114-115), and the three defaults (lines 128, 134, 141-142).
- **Acceptance criteria (`R001:174-281`).** Twelve scenarios/outline sets cover happy path,
  error cases, empty values and boundaries, and are stated as observable HTTP/DB/Docker
  outcomes (no implementation knowledge needed). Coverage is proven by the explicit
  `Covers:` lines: line 186 (REQ-001/002/004/006/010/014/015/016/019), line 193
  (REQ-002/005), line 200 (REQ-003), line 206 (REQ-004), line 214 (REQ-016/018),
  line 228 (REQ-007/020), line 236 (REQ-008/020), line 249 (REQ-009/020), line 257
  (REQ-011/020), line 264 (REQ-012), line 272 (REQ-013/020), line 280 (REQ-014/017).
  Every REQ-001…REQ-020 appears in at least one of these lines; the empty value `""`
  is covered at line 227 and boundary 2049 at line 232.
- **Assumptions / Status (`R001:283-295`).** Assumptions are explicit (local Docker only,
  no target fetching, single instance, case-sensitive codes, exact messages) and the
  document is Approved (line 295).

## Plan review

- **Summary / Approach (`P001:3-60`).** The design is concrete (axum layers, serde types,
  `validate_target`, `CodeGenerator` seam, `Store`, `ApiError`, `Config`, multi-stage
  Docker). It explicitly avoids scope creep: "No catch-all and no handler for `/` or
  `/assets/*`; those are reserved for R002" (lines 19-21) and `ServeDir` is "not used by
  P001, noted for P002" (line 56).
- **Library APIs (`P001:48-60`).** Resolved Context7 IDs are recorded for axum, sqlx, url,
  rand and tower-http, with pinned version decisions (axum 0.7.x, rand 0.9.x at line 58),
  satisfying the "record non-obvious APIs" rule.
- **Alternatives + ADR (`P001:62-71`, `P001:12`, `174-178`).** Six alternatives are
  considered with reasons; the consequential stack decision has an Accepted ADR001 linked
  from the plan.
- **Changes (`P001:73-97`).** File-by-file responsibilities are listed; no behaviour beyond
  R001 is introduced (the 10-attempt cap at lines 36-37 is the one judgement call — see
  Findings).
- **Test strategy (`P001:99-115`).** Unit tests, one-integration-test-per-scenario, exact
  check commands (`cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D
  warnings`, `cargo test --all-targets`, lines 110-111) and a Docker smoke test are given;
  browser tests are correctly deferred to P002 (line 115).
- **Risks / Tasks (`P001:117-143`).** Each risk has a mitigation; tasks 1-14 are ordered and
  small enough to execute individually.
- **Traceability (`P001:145-168`).** Every REQ appears with a concrete change and a test,
  e.g. `REQ-003` line 151 (collision + retry / "Generated code collision is retried"),
  `REQ-018` line 166 (named volume / "Mappings survive container recreation"). Total rows:
  20, matching REQ-001…REQ-020.

## Cross-check

- **No uncovered requirement.** All 20 REQs have both a Gherkin `Covers:` line (R001, quoted
  above) and a traceability row with change + test (P001:149-168).
- **No scenario without a verification path.** The error scenarios, config overrides,
  collision retry and 500 fault injection are all named in the plan's integration-test list
  (P001:104-109) and the container-recreation scenario in the Docker smoke step
  (P001:112-114). The process-restart scenario is traced at P001:152 but not re-stated in
  the Test strategy (see Finding 2).
- **No contradictions or conflicting IDs** between the two documents. One minor tension:
  REQ-003 says generate "until an unused code is found" (`R001:50-51`) while the plan caps
  retries at 10 and returns `internal_error` on exhaustion (`P001:36-37`, `123`); the cap is
  a sensible defence and exhaustion maps to REQ-013, but the requirement's unbounded wording
  is not reconciled in writing (Finding 3).
- **No out-of-scope task.** All plan tasks serve R001 behaviours; R002-only pieces are
  explicitly excluded (P001:19-21, 56, 115).

## Code-level axes

Standards: not applicable — pre-implementation gate.
Spec: not applicable — pre-implementation gate.

## Findings

1. **[suggestion]** `REQ-016`'s "when running in the container" condition (`R001:141-142`)
   has no observable definition, and no scenario leaves `DATABASE_PATH` unset inside a
   container: the Docker scenario asserts `/data/urls.sqlite3` (`R001:213`) while Compose
   sets `DATABASE_PATH` explicitly (`P001:43-44`, `80-81`), so the unset-default branch is
   never exercised. Define the container-detection signal and cover it with a unit test. —
   `docs/requirements/R001-url-shortener.md:137-142`
2. **[suggestion]** The "Mappings survive a process restart" scenario (`R001:202-206`) is
   traced to a test (`P001:152`) but is not explicitly listed in the Test strategy's
   integration-test enumeration (`P001:104-109`). Add an explicit restart test description
   (e.g. rebuild the pool/router against the same temp DB file). —
   `docs/plans/P001-url-shortener-backend.md:104-109`
3. **[suggestion]** Reconcile REQ-003's unbounded "until an unused code is found"
   (`R001:50-51`) with the plan's 10-attempt cap returning `internal_error` (`P001:36-37`,
   `123`); note that exhaustion is an unexpected internal failure covered by REQ-013, or
   reword REQ-003 as bounded. — `docs/requirements/R001-url-shortener.md:46-51`
4. **[suggestion]** Boundary 2048 (accepted) is only in unit tests (`P001:101-102`), not in
   any Gherkin scenario; the Gherkin set tests only 2049 rejection (`R001:230-236`). Add a
   2048-character accepted scenario for a complete boundary. —
   `docs/requirements/R001-url-shortener.md:230-236`
5. **[suggestion]** Trailing slash in `PUBLIC_BASE_URL` is unspecified: REQ-006 "followed by
   `/` and the code" (`R001:71-73`) plus REQ-017 "exact value as the prefix" (`R001:148-149`)
   would yield `https://sho.rt//CODE` if a trailing slash is supplied. Specify trim/no-trim
   behaviour. — `docs/requirements/R001-url-shortener.md:71-73`
6. **[suggestion]** `REQ-020` traceability reads "All four error scenarios" (`P001:168`),
   but five scenarios assert its exact error body (`R001:228`, `236`, `249`, `257`, `272`).
   Correct the count. — `docs/plans/P001-url-shortener-backend.md:168`
7. **[suggestion]** "Environment variables override defaults" asserts `PORT` and
   `DATABASE_PATH` overrides (`R001:275-279`) for which no REQ exists, and its `Covers:
   REQ-014` tag (`R001:280`) is inaccurate — REQ-014 is a "while unset" default, correctly
   covered by the first scenario (`R001:186`). Add REQs for the overrides or clarify them as
   deployment/configuration detail. — `docs/requirements/R001-url-shortener.md:274-280`
8. **[suggestion]** `REQ-016` packs two default paths into one statement
   (`/data/urls.sqlite3` vs `./urls.sqlite3`, `R001:141-142`); consider splitting into
   separate REQs to keep strictly to one behaviour per statement. —
   `docs/requirements/R001-url-shortener.md:137-142`
