# Structured Intent Prompt

Template: 1.0.0

Issue: 171

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Deliver issue initialization, observation, and topology-bound execution context over the kernel and transaction store.

## Required Outcome

branch-worktree ownership and bind recovery journeys is produced at an exact revision and independently reproducible.

## Scope

- `issue init/show/status`, `bind`, repository and issue selection, topology collision checks, typed request/result schemas, and human/JSON presentation.

## Authority

- Issue V3-10A owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
