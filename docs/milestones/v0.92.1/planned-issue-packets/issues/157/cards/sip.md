# Structured Intent Prompt

Template: 1.0.0

Issue: 157

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Transfer administrative control of repositories, domains, brands, and external vendors to Agent Logic while preserving availability and rollback.

## Required Outcome

administrative control manifests and live readback is produced at an exact revision and independently reproducible.

## Scope

- Seven approved repository copies and source authority, GitHub organization settings, domains and registrars, brand accounts, package and webhook identities, vendor ownership, redirects, and legacy-public-repository disposition.

## Authority

- Issue CORP-05 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
