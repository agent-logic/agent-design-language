# Structured Intent Prompt

Template: 1.0.0

Issue: 165

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Implement the invocation-scoped dependency container and common I/O, configuration, error, cancellation, and observability services.

## Required Outcome

dependency boundaries cancellation streams and typed error contracts is produced at an exact revision and independently reproducible.

## Scope

- `App`, lazy sync/async initialization, streams, TTY and prompting, configuration precedence, credential references, cancellation token, tracing, redaction, operation IDs, OS signal handling, error-to-exit mapping, and test constructors.

## Authority

- Issue V3-04 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
