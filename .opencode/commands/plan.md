---
description: Write an implementation plan traced to an approved requirement
agent: plan
---

Plan the implementation of: $ARGUMENTS

1. Load the requirement named in $ARGUMENTS from `docs/requirements/`. If it does not
   exist or is not approved, stop and tell me.
2. Load the `implementation-planning` skill and read `docs/plans/_template.md`.
3. If the feature involves a third-party library, load the `context7-docs` skill and
   resolve its documentation before deciding on APIs. Record the library ID in the plan.
4. Scan `docs/plans/` and pick the next `P###` number.
5. Write `docs/plans/P###-<slug>.md` with: approach, alternatives considered, file-level
   changes, test strategy, risks, a task checklist, and a traceability table mapping every
   `REQ-###` to a change and a test.
6. If a decision is consequential (affects architecture or is hard to reverse), write an
   ADR using `docs/decisions/_template.md` and link it from the plan.
7. Do not write implementation code. Present the plan for my approval.
