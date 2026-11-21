---
name: Implementation planning
description: Turn an approved EARS + BDD requirement into a traceable implementation plan with approach, alternatives, test strategy and risks. Use when writing docs/plans/P### documents.
---

# Implementation planning

Read the approved requirement in `docs/requirements/R###-*.md` and start from
`docs/plans/_template.md`. Write the plan to `docs/plans/P###-<slug>.md`.

## Sections

1. **Summary** — one paragraph: what will be built and for which requirement.
2. **Approach** — the design in concrete terms: components, boundaries, data flow.
   Keep it stack-neutral until a stack has been chosen.
3. **Alternatives considered** — at least one credible alternative and why it was
   rejected. If the decision is consequential, create an ADR from
   `docs/decisions/_template.md` and link it here.
4. **Changes** — file-by-file list of what will be created or modified. Do not invent
   file names for an unchosen stack; describe responsibilities instead.
5. **Test strategy** — how each Gherkin scenario will be verified, and which checks will
   be run.
6. **Risks** — what could go wrong and the mitigation.
7. **Tasks** — an ordered checklist small enough to execute one at a time.
8. **Traceability** — a table mapping every `REQ-###` to the change that implements it
   and the test that proves it.

## Library APIs

If the design depends on a third-party library, load the `context7-docs` skill, resolve
the library ID and query its docs before committing to an API. Record the library ID and
the version (if relevant) in the plan. Never rely on remembered APIs.

## Rules

- Do not write implementation code.
- Do not introduce new behaviour beyond the approved requirement; put it in a follow-up note.
- Every `REQ-###` in the requirement must appear in the traceability table.
- Present the plan for approval before implementation starts.
