# Structured Intent Prompt

Template: 1.0.0

Issue: 159

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Move Terraform state, CI/CD, deployment identity, rollback, and operational runbook authority to company-controlled systems.

## Required Outcome

company-identity plan deployment rollback and operations proof is produced at an exact revision and independently reproducible.

## Scope

- Remote state and locks, OIDC and deployment roles, GitHub environments, secrets by name, workflow permissions, release and rollback commands, monitoring escalation, and operator runbooks.

## Authority

- Issue CORP-07 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
