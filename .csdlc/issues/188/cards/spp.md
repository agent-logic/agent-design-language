# Structured Planning Prompt

Template: 1.0.0

Issue: 188

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

At exact terminal CORP-08, V3-16, and DRT-07 revisions, independently recompute each lane, preserve lane separation, route every finding, and issue a bounded release recommendation only after all blockers close.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Resolve terminal CORP-08, V3-16, and DRT-07 merge revisions and receipts; verify each is ancestral to the integrated review revision before reading lane evidence.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Build an exact lane-by-lane artifact and validator inventory with expected digests, producer commands, proof roles, external authorities, and explicit rejected or unavailable items.",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Independently rerun or recompute every required corporate, v3, and Runtime quality gate at its exact revision without using one lane's success as evidence for another.",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Publish findings first with severity, evidence, owning issue, required remediation, and residual risk; do not implement undisclosed fixes in the integration worktree.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Verify terminal dispositions and exact fix revisions for every P1/P2 finding, rerun affected gates, and retain unresolved P3 or accepted-risk authority explicitly.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Run the issue validator over ancestry, artifact digests, lane independence, finding denominator, and dispositions; obtain a separate exact-head review and emit only the bounded recommendation supported by the evidence.",
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

- Issue INT-01 owns only its declared repository paths and named external operation/evidence boundary.
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

.csdlc/prepared/issues/188/design.md

Digest: 98d941cdeeecd563a0c55b2ce49e92c2a2e350be4c0e45ea3b0771be0fd585f6

## Diagram

.csdlc/prepared/issues/188/diagram.mmd

Digest: 8ab4e1853b69a70ad74b0398410686199252e4d5cf64da140d68815079e43c4b

## Stop Conditions

- A lane is nonterminal
- Evidence cannot be reproduced
- A blocking finding remains unresolved
- Review independence cannot be established
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
