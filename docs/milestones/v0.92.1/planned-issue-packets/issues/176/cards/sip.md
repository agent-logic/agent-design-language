# Structured Intent Prompt

Template: 1.0.0

Issue: 176

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Establish one typed, mockable GitHub boundary and complete read-only issue, PR, check, review, mergeability, and repository observation.

## Required Outcome

pagination ambiguity rate limit retry and redaction fixtures is produced at an exact revision and independently reproducible.

## Scope

- Octocrab client construction, Rustls, authentication, repository and authenticated human-reviewer identity observation, REST/GraphQL endpoint wrappers, pagination, rate-limit and retry classification, response normalization, fake transport registry, and `pr status`.

## Authority

- Issue V3-13 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
