---
name: Requirements (EARS + BDD)
description: Write testable requirements using EARS statements and Gherkin Given/When/Then acceptance criteria. Use when authoring or refining a feature specification.
---

# Requirements authoring

Use this skill to write `docs/requirements/R###-<slug>.md`. Start from
`docs/requirements/_template.md`.

## EARS statements

EARS (Easy Approach to Requirements Syntax) makes each requirement a single, testable
sentence with a stable `REQ-###` ID. Use the type that fits:

| Type | Pattern | Example |
|---|---|---|
| Ubiquitous | The `<system>` shall `<behaviour>`. | The CLI shall accept one optional positional argument named `name`. |
| Event-driven | When `<event>`, the `<system>` shall `<response>`. | When invoked with a name, the CLI shall print `Hello, <name>!` to stdout. |
| State-driven | While `<state>`, the `<system>` shall `<behaviour>`. | While no name is provided, the CLI shall print `Hello, World!`. |
| Unwanted | If `<unwanted condition>`, then the `<system>` shall `<response>`. | If more than one argument is provided, then the CLI shall print a usage error to stderr and exit with code 2. |
| Optional | Where `<feature>` is included, the `<system>` shall `<behaviour>`. | Where colour output is enabled, the CLI shall use a green greeting. |

Rules:
- One behaviour per statement. Split compound requirements.
- State observable behaviour, not implementation details.
- Include exit codes, streams and exact strings when they are part of the contract.
- Number statements sequentially as `REQ-001`, `REQ-002`, … within a requirement document.
- Never redefine an existing ID without marking the change in the document.

## Acceptance criteria

Write Gherkin scenarios that cover every `REQ-###`:

```gherkin
Scenario: Custom name
  Given the CLI is invoked with the argument "Ada"
  When it runs
  Then stdout is "Hello, Ada!" and the exit code is 0
```

- Use `Scenario Outline` + `Examples` for repeated data-driven cases.
- Add scenarios for unhappy paths, empty values and boundaries.
- Reference the covered IDs in a comment or a `Covers:` line, for example `Covers: REQ-002, REQ-004`.
- A scenario must be verifiable without reading the implementation.

## Checklist before finishing

- [ ] Every requirement has an ID and exactly one behaviour.
- [ ] Every `REQ-###` is covered by at least one scenario.
- [ ] Error cases and exit codes are specified.
- [ ] No ambiguity (words like "fast", "friendly", "appropriate" are replaced with concrete values).
- [ ] The document is ready for the user to approve.
