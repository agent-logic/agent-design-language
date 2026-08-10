# Structured Intent Prompt

Template: 1.0.0

Issue: 189

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Prepare the exact release candidate, rollback rehearsal, and operator ceremony after integrated review passes.

## Required Outcome

release checklist rollback rehearsal and exact artifact inventory is produced at an exact revision and independently reproducible.

## Scope

- Artifact inventory, version and revision pinning, release checklist, company authority, change window, rollback triggers and commands, rehearsal, communications, and abort conditions.

## Authority

- Issue INT-02 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
