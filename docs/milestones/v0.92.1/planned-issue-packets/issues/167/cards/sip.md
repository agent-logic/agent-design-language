# Structured Intent Prompt

Template: 1.0.0

Issue: 167

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Define the versioned v3 aggregate and deterministically render all six lifecycle cards and declared evidence projections.

## Required Outcome

canonical bytes round trips schemas and deterministic six-card projection is produced at an exact revision and independently reproducible.

## Scope

- `state.json`, embedded typed audit events and state-size guard, schema evolution, closed enums, canonical serialization, card AST values, SIP-STP-SPP-VPP-SRP-SOR rendering, per-phase field optionality and placeholders, digest rules, projection manifests, and drift detection.

## Authority

- Issue V3-06 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
