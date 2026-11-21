---
description: Implement an approved plan and verify the result
agent: build
---

Implement: $ARGUMENTS

1. Load the approved plan named in $ARGUMENTS from `docs/plans/` and its referenced
   requirement from `docs/requirements/`. If either is missing or unapproved, stop.
2. Load the `implement-spec` skill and use its task-graph discipline. Adapt it to this kit:
   the plan's task checklist is the ticket list, and there is no issue tracker, PR flow or
   worktree-per-ticket step here. Implement exactly what the plan specifies.
3. If you use a third-party library, load the `context7-docs` skill and query its current
   documentation before writing API calls. Do not guess APIs.
4. Follow `AGENTS.md`. Write or update tests for every acceptance scenario.
5. Run the available checks and fix failures before finishing.
6. Finally, launch the `verifier` subagent with the requirement and plan IDs. Include its
   report in your result. Do not claim completion if any acceptance scenario fails or the
   report has blocking findings.
