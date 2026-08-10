# Structured Intent Prompt

Template: 1.0.0

Issue: 156

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Establish company-controlled billing, administration, MFA, recovery, vault, break-glass, and custody for every critical service.

## Required Outcome

redacted custody and recovery verification is produced at an exact revision and independently reproducible.

## Scope

- Company identities, billing profiles, secure MFA, recovery channels, vault custody, break-glass procedure, least privilege, role separation, and founder-dependency removal.

## Authority

- Issue CORP-04 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
