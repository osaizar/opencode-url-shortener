# Project instructions

This repository is a reusable OpenCode development kit. These rules apply to every
session in this project.

## Workflow gates

- Do **not** write or edit implementation code until an approved requirement document
  (`docs/requirements/R###-*.md`) and an approved plan (`docs/plans/P###-*.md`) exist.
- Requirements come first: EARS statements with stable `REQ-###` IDs, plus Gherkin
  `Given/When/Then` acceptance criteria linked to those IDs.
- A plan must trace every `REQ-###` it addresses to a concrete change and a test.
- Independent review of the requirement and plan (`/spec-review`) happens before
  implementation; independent verification of the implementation (`/review`) happens
  again before completion.
- Never skip the gates because a change looks small. If the user explicitly asks to skip
  one, confirm before proceeding.

## Artifacts

| Artifact | Location | Naming |
|---|---|---|
| Requirements | `docs/requirements/` | `R###-<slug>.md` |
| Implementation plans | `docs/plans/` | `P###-<slug>.md` |
| Architecture decisions | `docs/decisions/` | `ADR###-<slug>.md` |
| Spec reviews | `docs/reviews/` | `<id>-spec-review.md` |
| Verification reports | `docs/reviews/` | `<id>-verification.md` (archived runs: `<id>-verification-<stage>.md`) |
| Templates | `docs/<area>/_template.md` | edit only when the format changes |

Pick the next number by scanning the directory; never reuse a number.

## Definition of done

- The pre-implementation spec review report has no blocking findings.
- Every Gherkin acceptance scenario for the requirement passes, with evidence recorded
  in the verification report.
- The verifier report has no blocking findings.
- Tests/checks succeed, or the report states that no automated checks are configured.
- The implementation matches the approved plan; deviations are documented in the plan
  or an ADR.

## Documentation lookups

Use Context7 when working with third-party libraries. Resolve the library ID first, then
query docs with a specific question that mentions the version when it matters. Record
resolved library IDs in the plan when the design depends on non-obvious APIs. Do not guess
library APIs from training data.

Run `npx skills find <keyword>` to discover more skills; installed skills live in
`.agents/skills/` and are tracked in `skills-lock.json`. Local skills in
`.opencode/skills/` take precedence on ID collisions. `grill-me` delegates to `grilling`;
`code-review` uses two nested sub-agents, enabled by `experimental.subagent_depth: 2`.
Check for updates with `npx skills check` and apply them with `npx skills update -p -y`.

## Verification commands

Rust backend (run from the repository root, with `$HOME/.cargo/env` sourced):

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --all-targets`

Docker smoke: `docker compose up --build -d`, then check `GET /health`, `POST /api/v1/urls`
and the returned redirect; `docker compose down && docker compose up -d` verifies that
mappings survive container recreation via the named volume.

Frontend (run from `frontend/`):

- `npm ci`
- `npm test`
- `npm run build` (runs `tsc --noEmit && vite build`)

## Safety

- Never commit secrets. Use `{env:NAME}` substitution for credentials in configuration.
- Ask before destructive shell operations; `git push *` requires approval.
- The `verifier` agent must never edit code; it may write only to `docs/reviews/**`.
