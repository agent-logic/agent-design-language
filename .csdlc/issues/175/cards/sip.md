# Structured Intent Prompt

Template: 1.0.0

Issue: 175

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Implement independent exact-revision review assignment, result recording, staleness, finding disposition, and publication authorization.

## Required Outcome

staleness independence finding lifecycle linkage and bypass negatives is produced at an exact revision and independently reproducible.

## Scope

- `review assign/record/recover/status`, structurally bound reviewer principals, independence enforcement and policy-only limitation handling, exact scope/revision identity, findings and dispositions, non-substantive change proof, typed recovery provenance and invalidation, mode-bound publication intent, and fail-closed review guard.

## Authority

- Issue V3-12 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
