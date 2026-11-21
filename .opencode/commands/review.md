---
description: Independently verify a requirement, plan or change
agent: verifier
subagent: true
---

Verify: $ARGUMENTS

Use the verifier workflow. Resolve the requirement ID (`R###`) and plan ID (`P###`) from
the arguments, or find the most recent ones. Review the diff, run the available checks,
test every Gherkin acceptance scenario, and write `docs/reviews/<requirement-id>-verification.md`.
Return a severity-ordered summary with a clear pass/fail verdict.
