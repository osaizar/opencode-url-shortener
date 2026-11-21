---
description: Independently review an approved requirement and plan before implementation
agent: spec-reviewer
subagent: true
---

Review the requirement and plan for: $ARGUMENTS

Use the spec-reviewer workflow. Resolve the requirement ID (`R###`) and plan ID (`P###`)
from the arguments, or find the most recent approved ones. This is the
**pre-implementation** gate: check document quality, `REQ-###` coverage by Gherkin
scenarios, testability, plan traceability and contradictions. Write
`docs/reviews/<requirement-id>-spec-review.md` and return a severity-ordered summary with
a clear PASS/FAIL verdict. A pre-existing codebase is normal; stop and direct the caller
to `/review` only if this requirement is already implemented.
