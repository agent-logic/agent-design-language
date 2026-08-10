# Structured Intent Prompt

Template: 1.0.0

Issue: 181

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Freeze an exact distributed qualification contract before provisioning nodes or injecting faults.

## Required Outcome

topology scenario fault timing resource receipt and claim contract is produced at an exact revision and independently reproducible.

## Scope

- Topology, identities, ports, state roots, credentials, transport, AWS/Wuji placement, scenarios, timing, resource budgets, receipt schema, cleanup, and claim boundaries.

## Authority

- Issue DRT-01 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
