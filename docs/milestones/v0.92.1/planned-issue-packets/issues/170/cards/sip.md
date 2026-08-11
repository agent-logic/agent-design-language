# Structured Intent Prompt

Template: 1.0.0

Issue: 170

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Provide narrow, mockable effect boundaries without shell evaluation or credential leakage.

## Required Outcome

hostile argument cancellation redaction and adapter tests is produced at an exact revision and independently reproducible.

## Scope

- Git repository/branch/worktree/status/diff operations, bounded process execution for declared PVF commands, environment construction, credential resolution, timeout/cancellation, output caps, and structured observations.

## Authority

- Issue V3-09 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
