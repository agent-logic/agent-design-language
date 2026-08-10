# Structured Planning Prompt

Template: 1.0.0

Issue: 166

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Resolve canonical repository/issue identity and import v2 records through a strictly read-only, loss-reporting compatibility boundary that cannot confer v3 mutation authority.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Define explicit repository, remote, worktree, issue, and argument precedence and one canonical qualified identity.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement discovery with canonicalization and fail-closed symlink, path-escape, ambiguous-remote, ambiguous-issue, and current-directory fixtures.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Parse representative v2 records into normalized import types while preserving every known field and reporting record/field identity for unknowns.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Enforce BlockedUnsupportedFields and prove no imported record reaches mutation until every field has a reviewed disposition.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Use write-denying filesystem/state fakes to prove import cannot alter v2 or v3 state or infer authority.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Retain compatibility/parity reports and stop on silent drops, ambient cwd authority, or any write attempt.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S7",
    "action": "Remove import scratch projections and prove both v2 and v3 stores are byte-identical to their pre-import snapshots.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Issue V3-05 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.
- No unsupported completion, legal, production, or release claim
- No mutation outside exact owned paths

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/166/design.md

Digest: 5cc2256ec9f54c0374a12cde4dc92b7e3e3f6cf034989c6ef0f49e710bab21db

## Diagram

.csdlc/prepared/issues/166/diagram.mmd

Digest: 21c67220b6f9691373ed52b86a4b5d5847979ba06d4d53e240b97796cc94bc6b

## Stop Conditions

- Context depends on process-global current directory, unsupported fields are dropped silently, or importer execution can mutate either generation.
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
