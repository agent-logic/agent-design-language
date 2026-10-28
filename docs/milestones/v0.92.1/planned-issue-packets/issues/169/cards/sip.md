# Structured Intent Prompt

Template: 1.0.0

Issue: 169

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make state mutation crash-consistent with one explicit commit point and recoverable projections.

## Required Outcome

fault injection at write sync rename and recovery boundaries is produced at an exact revision and independently reproducible.

## Scope

- Advisory locking, compare-and-swap generation/digest checks, intent records, temporary writes, fsync policy, atomic `state.json` replacement, projection regeneration, recovery classification, fault injection, and concurrent writer behavior.

## Authority

- Issue V3-08 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
