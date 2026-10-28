# Structured Intent Prompt

Template: 1.0.0

Issue: 168

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Encode lifecycle transitions and authorization predicates as a pure, exhaustive, side-effect-free state machine.

## Required Outcome

exhaustive transition capability and recovery graph tests is produced at an exact revision and independently reproducible.

## Scope

- Phases, transition commands, preconditions, topology ownership, design/readiness/review/publication/terminal predicates, capability-derived field authorization, recovery reachability, idempotent outcomes, and stable domain errors.

## Authority

- Issue V3-07 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
