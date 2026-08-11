# Structured Intent Prompt

Template: 1.0.0

Issue: 163

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Record the measured operator decision required by architecture Decision 11 before transaction storage implementation begins.

## Required Outcome

operator decision record is produced at an exact revision and independently reproducible.

## Scope

- Per-platform atomic commit primitives, filesystem durability semantics, supported-platform matrix, Windows mutation or fail-closed read-only posture, evidence, and rollback implications.

## Authority

- Issue V3-D11 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
