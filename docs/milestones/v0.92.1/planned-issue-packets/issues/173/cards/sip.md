# Structured Intent Prompt

Template: 1.0.0

Issue: 173

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Implement the pure governed model for validation manifests, classification, resource profiles, dependencies, and lane selection.

## Required Outcome

lane classification budgets and manifest determinism is produced at an exact revision and independently reproducible.

## Scope

- `validate plan`, lane manifest schema, PVF classification, proof roles, determinism and live/deferred posture, resource profiles, budgets, parallel-group DAG rules, and planning results.

## Authority

- Issue V3-11A owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
