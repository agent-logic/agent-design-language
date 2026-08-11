# Structured Intent Prompt

Template: 1.0.0

Issue: 153

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Freeze the complete critical asset, account, owner, custodian, recovery, exclusion, and dependency inventory before any transfer begins.

## Required Outcome

redacted ownership and exclusion matrix is produced at an exact revision and independently reproducible.

## Scope

- Repositories, domains, brands, source and model IP, cloud and SaaS accounts, billing, credentials, recovery paths, data stores, deployment identities, contracts, and explicit exclusions.

## Authority

- Issue CORP-01 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
