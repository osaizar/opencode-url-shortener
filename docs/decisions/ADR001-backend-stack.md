# ADR001 — Backend stack and persistence

## Status

Accepted

## Context

The approved requirement R001 needs an HTTP service with durable storage, structured
errors and a single-container deployment. No stack existed in the base kit; the spec
interview settled on Rust, axum and SQLite. Rust and Docker availability differ in this
environment (Docker present, Rust toolchain to be installed locally via rustup).

## Decision

Use Rust with axum 0.7.x (Tokio), SQLx over SQLite with embedded migrations, `rand` 0.9.x
Alphanumeric distribution for 7-character base62 codes, and a multi-stage Docker image
with a named volume for the database.

## Consequences

### Positive

- One async runtime and one binary; no separate database service.
- Durable mappings with zero external infrastructure.
- Testable code generation through the `CodeGenerator` seam.

### Negative

- Rust toolchain must be installed before the `cargo` checks can run locally.
- SQLx runtime-checked queries lose compile-time SQL verification.
- Single-instance SQLite does not suit future multi-replica scaling without rework.

## Alternatives

| Alternative | Reason not chosen |
|---|---|
| rusqlite | Blocking I/O in async handlers; extra glue. |
| PostgreSQL | Extra service and operational cost for v1. |
| In-memory store | Fails durability requirement. |

## Related

- Requirement: `R001-url-shortener.md`
- Plan: `P001-url-shortener-backend.md`
