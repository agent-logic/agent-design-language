# Structured Intent Prompt

Template: 1.0.0

Issue: 166

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Resolve repository and issue context deterministically and import v2 records without granting v3 mutation authority.

## Required Outcome

deterministic discovery unsupported-field reporting and no-write proof is produced at an exact revision and independently reproducible.

## Scope

- Root discovery, canonical repository identity, remote resolution, branch/worktree observation, issue selection precedence, symlink-safe paths, v2 record/card parsing, unsupported-field reporting, and normalized read-only projections.

## Authority

- Issue V3-05 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
