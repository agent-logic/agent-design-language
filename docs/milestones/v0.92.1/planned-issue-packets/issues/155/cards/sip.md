# Structured Intent Prompt

Template: 1.0.0

Issue: 155

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Resolve provenance, licensing, trademark, model, media, contributor, and third-party dispositions for every critical asset.

## Required Outcome

contributor third-party OSS model media and brand disposition report is produced at an exact revision and independently reproducible.

## Scope

- Git provenance, contributor rights, dependencies and licenses, model and dataset terms, generated media, trademarks, domains, podcast and publication assets, and unresolved third-party claims.

## Authority

- Issue CORP-03 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
