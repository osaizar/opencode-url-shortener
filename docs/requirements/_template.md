# R### — <feature name>

> Requirement template. Copy to `R###-<slug>.md`. Pick the next number by scanning this directory.

## Purpose

<One or two sentences describing the behaviour to be delivered.>

## Scope

- In scope: <...>
- Out of scope: <...>

## Requirements

### REQ-001 — <short title>

**Type:** ubiquitous / event-driven / state-driven / unwanted / optional

The <system> shall <one observable behaviour>.

### REQ-002 — <short title>

**Type:** event-driven

When <event>, the <system> shall <response>.

<!-- Add one block per EARS statement. Use sequential REQ-### IDs. -->

## Acceptance criteria

```gherkin
Scenario: <descriptive name>
  Given <precondition>
  When <action>
  Then <observable outcome>
  # Covers: REQ-001
```

<!-- Add a scenario for every REQ-###, including error and boundary cases. -->

## Assumptions

- <Assumption made during clarification.>

## Status

Draft — awaiting approval.
