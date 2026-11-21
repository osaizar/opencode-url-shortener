# R001 — URL shortener (v1)

## Purpose

Deliver a minimal HTTP service that turns a long URL into a short link and redirects
visitors from that short link back to the original URL. Mappings must survive restarts.

## Scope

- In scope: shortening a URL, redirecting a short code, a health endpoint, structured
  errors, configuration via environment variables, and a Docker deployment with a
  persistent SQLite file.
- Out of scope: click analytics, custom aliases, expiry dates, authentication, rate
  limiting, link management (list/update/delete), QR codes, bulk import.

### Terminology and fixed values

- **Base62 alphabet:** digits `0-9`, uppercase `A-Z`, lowercase `a-z`. Codes are
  case-sensitive.
- **Error response shape:** `{"error": "<code>", "message": "<exact text>"}` with
  `Content-Type: application/json`.
- **Public base URL:** trailing slashes are trimmed before appending `/\<code\>`; for
  example `https://sho.rt/` produces `https://sho.rt/\<code\>`.
- **Container database path:** Docker Compose sets `DATABASE_PATH=/data/urls.sqlite3`
  explicitly (REQ-018); the unset default in REQ-016 applies outside the container.

| Condition | Status | `error` | `message` |
|---|---|---|---|
| URL missing scheme/host, unsupported scheme, or over 2048 chars | `400` | `invalid_url` | `URL must be a valid http or https URL of at most 2048 characters` |
| Body not a JSON object with a `url` string | `400` | `invalid_request` | `Request body must be a JSON object with a url string` |
| No mapping for the requested code | `404` | `not_found` | `Short code not found` |
| Unexpected internal failure | `500` | `internal_error` | `Internal server error` |

## Requirements

### REQ-001 — Creation endpoint

**Type:** ubiquitous

The URL shortener shall expose `POST /api/v1/urls` accepting a JSON object with a `url`
field.

### REQ-002 — Code generation

**Type:** event-driven

When a valid URL is submitted, the URL shortener shall assign a newly generated
7-character base62 code.

### REQ-003 — Collision handling

**Type:** unwanted

If a generated code already exists, then the URL shortener shall generate another code,
retrying at most 10 times before treating the submission as an internal failure.

### REQ-004 — Persistent mappings

**Type:** ubiquitous

The URL shortener shall store each code-to-URL mapping in SQLite so that it remains
available after a process restart.

### REQ-005 — Duplicate submissions

**Type:** state-driven

While a target URL has already been shortened, the URL shortener shall assign a distinct
new code to each further accepted submission of that same URL.

### REQ-006 — Creation response

**Type:** event-driven

When a URL is accepted, the URL shortener shall respond with status `201` and a JSON body
containing exactly the fields `code` and `short_url`, where `short_url` is the configured
base URL followed by `/` and the code.

### REQ-007 — Invalid URL scheme or host

**Type:** unwanted

If the submitted `url` does not use `http` or `https` or does not have a host, then the
URL shortener shall respond with status `400` and error code `invalid_url`.

### REQ-008 — URL length limit

**Type:** unwanted

If the submitted `url` exceeds 2048 characters, then the URL shortener shall respond with
status `400` and error code `invalid_url`.

### REQ-009 — Malformed request body

**Type:** unwanted

If the request body is not a JSON object containing a `url` string, then the URL shortener
shall respond with status `400` and error code `invalid_request`.

### REQ-010 — Redirect

**Type:** event-driven

When `GET /{code}` is requested for an existing code, the URL shortener shall respond with
status `302` and a `Location` header equal to the stored URL.

### REQ-011 — Unknown short code

**Type:** unwanted

If `GET /{code}` is requested for a code with no stored mapping, then the URL shortener
shall respond with status `404` and error code `not_found`.

### REQ-012 — Health check

**Type:** event-driven

When `GET /health` is requested, the URL shortener shall respond with status `200` and the
JSON body `{"status": "ok"}`.

### REQ-013 — Internal failure

**Type:** unwanted

If an unexpected internal failure occurs while handling a request, then the URL shortener
shall respond with status `500` and error code `internal_error`.

### REQ-014 — Default port

**Type:** state-driven

While `PORT` is unset, the URL shortener shall listen on port `8080`.

### REQ-015 — Default public base URL

**Type:** state-driven

While `PUBLIC_BASE_URL` is unset, the URL shortener shall use `http://localhost:8080` as
the base of `short_url`.

### REQ-016 — Default database path

**Type:** state-driven

While `DATABASE_PATH` is unset, the URL shortener shall use `./urls.sqlite3` as the
SQLite file path.

### REQ-017 — Configured public base URL

**Type:** event-driven

When `PUBLIC_BASE_URL` is set, the URL shortener shall use its value with trailing
slashes trimmed as the prefix of `short_url`.

### REQ-018 — Container data volume

**Type:** ubiquitous

The Docker Compose setup shall keep the SQLite file on a named volume so that mappings
survive container recreation.

### REQ-019 — Exact target URL

**Type:** ubiquitous

The URL shortener shall redirect to the submitted URL exactly as stored, preserving any
query string and fragment.

### REQ-020 — Structured errors

**Type:** ubiquitous

The URL shortener shall return every error as JSON with the fields `error` and `message`,
using the codes and exact messages in the error table above.

## Acceptance criteria

```gherkin
Scenario: Shorten a valid URL and use the link
  Given the service is running with no environment variables set and an empty database
  When POST /api/v1/urls is called with body {"url": "https://example.com/path?q=1#frag"}
  Then the status is 201
  And the response body has exactly the fields "code" and "short_url"
  And "code" matches ^[0-9A-Za-z]{7}$
  And "short_url" is "http://localhost:8080/<code>"
  And a SQLite file exists at "./urls.sqlite3" containing the mapping
  When GET /<code> is requested
  Then the status is 302
  And the Location header is exactly "https://example.com/path?q=1#frag"
  # Covers: REQ-001, REQ-002, REQ-004, REQ-006, REQ-010, REQ-014, REQ-015, REQ-016, REQ-019

Scenario: Repeated submission of the same URL gets a new code
  Given a mapping already exists for "https://example.com/same"
  When POST /api/v1/urls is called twice more with that same URL
  Then both responses have status 201 with distinct codes
  And all three codes redirect to "https://example.com/same"
  # Covers: REQ-002, REQ-005

Scenario: Generated code collision is retried
  Given the code generator is seeded to return an existing code first and an unused code second
  When POST /api/v1/urls is called with "https://example.com/new"
  Then the created mapping uses the second code
  And the existing mapping is unchanged
  # Covers: REQ-003

Scenario: Mappings survive a process restart
  Given a short link was created for "https://example.com/persist"
  When the service process is stopped and started again with the same database
  Then GET /<code> still returns 302 with Location "https://example.com/persist"
  # Covers: REQ-004

Scenario: Mappings survive container recreation
  Given the Docker Compose stack is running with its named volume
  And a short link was created for "https://example.com/docker"
  When the containers are recreated without removing the volume
  Then GET /<code> still returns 302 with Location "https://example.com/docker"
  And the database file in the container is at "/data/urls.sqlite3"
  # Covers: REQ-016, REQ-018

Scenario Outline: Invalid target URL is rejected
  Given the service is running
  When POST /api/v1/urls is called with body <body>
  Then the status is 400
  And the response body is exactly
    {"error": "invalid_url", "message": "URL must be a valid http or https URL of at most 2048 characters"}
  Examples:
    | body |
    | {"url": "ftp://example.com"} |
    | {"url": "example.com"} |
    | {"url": "https://"} |
    | {"url": ""} |
  # Covers: REQ-007, REQ-020

Scenario: URL longer than 2048 characters is rejected
  Given the service is running
  When POST /api/v1/urls is called with a "url" value of 2049 characters starting with "https://example.com/"
  Then the status is 400
  And the response body is exactly
    {"error": "invalid_url", "message": "URL must be a valid http or https URL of at most 2048 characters"}
  # Covers: REQ-008, REQ-020

Scenario Outline: Malformed creation request is rejected
  Given the service is running
  When POST /api/v1/urls is called with body <body>
  Then the status is 400
  And the response body is exactly
    {"error": "invalid_request", "message": "Request body must be a JSON object with a url string"}
  Examples:
    | body |
    | not-json |
    | {} |
    | {"url": 123} |
  # Covers: REQ-009, REQ-020

Scenario: Unknown short code
  Given the service is running
  When GET /doesnotexist is requested
  Then the status is 404
  And the response body is exactly
    {"error": "not_found", "message": "Short code not found"}
  # Covers: REQ-011, REQ-020

Scenario: Health check
  Given the service is running
  When GET /health is requested
  Then the status is 200
  And the response body is exactly {"status": "ok"}
  # Covers: REQ-012

Scenario: Internal failure returns a structured error
  Given the service is running and the mappings table has been dropped at runtime
  When POST /api/v1/urls is called with "https://example.com/fail"
  Then the status is 500
  And the response body is exactly
    {"error": "internal_error", "message": "Internal server error"}
  # Covers: REQ-013, REQ-020

Scenario: Environment variables override defaults
  Given the service is started with PORT=9090, PUBLIC_BASE_URL="https://sho.rt/", DATABASE_PATH="/tmp/custom/urls.sqlite3"
  When POST /api/v1/urls is called on port 9090 with "https://example.com/config"
  Then the status is 201
  And "short_url" is "https://sho.rt/<code>"
  And a SQLite file exists at "/tmp/custom/urls.sqlite3"
  # Covers: REQ-014, REQ-017
```

## Assumptions

- Docker and Docker Compose v2 are available locally; no cloud deployment in v1.
- The service never fetches target URLs, so private-network targets are not a security
  concern for v1.
- A single service instance is sufficient; SQLite concurrency needs are modest.
- Codes are case-sensitive; URL validation happens only on the `url` field, not on the
  host's reachability.
- Error messages are exact strings as listed in the error table.

## Change log

- After `docs/reviews/R001-spec-review.md` (non-blocking suggestions #1, #3, #5):
  REQ-003 now states the 10-attempt retry cap used by P001; REQ-016 now defines only the
  observable local default (the container path is set explicitly by Compose, REQ-018);
  REQ-017 and the override scenario now specify trimming trailing slashes from
  `PUBLIC_BASE_URL`. No `REQ-###` ID was redefined or removed.

## Status

Approved.
