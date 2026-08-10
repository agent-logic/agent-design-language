# Structured Intent Prompt

Template: 1.0.0

Issue: 172

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Deliver semantic card operations and a specific read-only doctor without making rendered Markdown authoritative.

## Required Outcome

semantic edits deterministic rendering and readiness diagnostics is produced at an exact revision and independently reproducible.

## Scope

- `card show/edit/render`, `doctor`, capability-matrix-driven command availability, schema-aware repair planning, projection drift, stranded-state detection, finding taxonomy, next-valid-operation derivation, and human/JSON presentation.

## Authority

- Issue V3-10B owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
