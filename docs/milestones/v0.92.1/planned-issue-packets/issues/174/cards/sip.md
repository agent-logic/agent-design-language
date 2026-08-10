# Structured Intent Prompt

Template: 1.0.0

Issue: 174

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Execute approved PVF plans with bounded structured concurrency, OS child control, cancellation, and tamper-evident evidence.

## Required Outcome

execution receipts tamper detection cancellation and recovery is produced at an exact revision and independently reproducible.

## Scope

- `validate run/status`, bounded scheduler, process adapter integration, parallel groups, timeouts, root cancellation, child termination/drain, output caps, evidence digests, result projection, and interruption recovery.

## Authority

- Issue V3-11B owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
