# Structured Intent Prompt

Template: 1.0.0

Issue: 177

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Implement idempotent PR publication and bounded foreground waiting over the reviewed GitHub adapter.

## Required Outcome

durable intent idempotent readback cancellation and bounded canary is produced at an exact revision and independently reproducible.

## Scope

- Mode-bound publication intents, issue/PR/comment mutation, operation markers, exact linkage readback, `pr publish`, `pr watch`, check/review/mergeability updates, signal cancellation, and optional explicitly authorized merge policy.

## Authority

- Issue V3-14 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
