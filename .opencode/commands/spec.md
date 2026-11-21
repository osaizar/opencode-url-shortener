---
description: Clarify a feature and write EARS + BDD requirements
agent: plan
---

Create or refine a requirement for: $ARGUMENTS

1. Load the `grill-me` skill. It points to the `grilling` skill, which is installed — load
   `grilling` and use it to interview me until the feature is unambiguous. Ask about
   inputs, outputs, error cases, edge cases and constraints. Do not write anything until
   the answers are clear.
2. Load the `requirements-earns-bdd` skill and read `docs/requirements/_template.md`.
3. Scan `docs/requirements/` and pick the next `R###` number.
4. Write `docs/requirements/R###-<slug>.md` with:
   - a short purpose statement and scope;
   - EARS statements (ubiquitous, event-driven, state-driven, unwanted, optional as
     applicable), each with a stable `REQ-###` ID;
   - Gherkin `Given/When/Then` acceptance scenarios, each referencing the `REQ-###` IDs
     it covers.
5. Do not write implementation code. Present the spec for my approval and list any
   assumptions you made.
