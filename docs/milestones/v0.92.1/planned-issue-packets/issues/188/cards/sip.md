# Structured Intent Prompt

Template: 1.0.0

Issue: 188

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Run one independent integrated review over terminal corporate, C-SDLC v3, and Runtime qualification evidence and remediate every blocker.

## Required Outcome

findings dispositions residual risks and release recommendation is produced at an exact revision and independently reproducible.

## Scope

- Exact terminal lane revisions, proof inventories, cross-lane assumptions, release gates, findings, dispositions, residual risks, and remediation verification.

## Authority

- Issue INT-01 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
