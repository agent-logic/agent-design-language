# Structured Planning Prompt

Template: 1.0.0

Issue: 170

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Implement narrow typed Git, child-process, and credential adapters using argv-only execution, scoped secrets, complete outcome distinctions, cancellation, truncation, and joined termination.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Implement the frozen Git/FileSystem/ProcessRunner/credential interfaces without shell strings or hidden topology inference.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement argv allowance enforcement and typed exit, stdout, stderr, timeout, cancellation, truncation, and spawn errors.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Resolve credentials only into the exact child/provider environment and redact URLs, arguments, diagnostics, and durable receipts.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Integrate root cancellation with bounded terminate/kill, handle wait, and stream drain on every supported platform.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Use strict fakes and negative fixtures for unexpected commands, shell metacharacters, ambiguous topology, secret leakage, timeout, cancellation, and truncation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Retain adapter receipts and stop on shell evaluation, surviving children, leaked secrets, or branch-name authority.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S7",
    "action": "Terminate and wait every child, drain streams, clear scoped credentials, remove adapter scratch files, and prove no process or secret survives.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- Issue V3-09 owns only its declared repository paths and named external operation/evidence boundary.
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

.csdlc/prepared/issues/170/design.md

Digest: 141a5a833417f5b199f689d653b8b26651201d467ec572f90493d57533bfa6b2

## Diagram

.csdlc/prepared/issues/170/diagram.mmd

Digest: 7ec968f8c5a843ede251db4cb0320a1087854e04fac0080f6c7762049fadb598

## Stop Conditions

- Any adapter invokes a shell, logs secrets, accepts ambiguous topology as authority, or cannot terminate and join a child process.
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
