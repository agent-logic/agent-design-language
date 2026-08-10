# Structured Planning Prompt

Template: 1.0.0

Issue: 163

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Convert V3-02's measured platform-commit recommendation into a separate authorized Decision 11 record that fixes the mutation/read-only posture for every supported platform before storage work.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Verify terminal V3-02 evidence and recompute every cited platform measurement and artifact digest.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Compare candidate commit primitives against atomic replacement, durability, recovery, and filesystem-capability requirements for every supported platform.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Choose and document the Windows posture as equivalently proven mutation or stable fail-closed read-only operation.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Record the authorized Decision 11, exact evidence references, approved matrix, and explicit non-authority of the prior recommendation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Prove V3-08 remains blocked without this terminal record and stop on ambiguous or unmeasured semantics.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Remove decision-analysis scratch files, retain the authorized matrix and cited measurement digests, and verify V3-08 remains blocked until terminal handoff.",
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

- Issue V3-D11 owns only its declared repository paths and named external operation/evidence boundary.
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

.csdlc/prepared/issues/163/design.md

Digest: 79e453f4d8954bd04fdc1030dd62a46c4efe8245e90c41430cdf562146482382

## Diagram

.csdlc/prepared/issues/163/diagram.mmd

Digest: 0598f78fe61173e7eff49f118b25237b9a44620b0c972494a6873cd2ca098d22

## Stop Conditions

- A supported platform lacks measured semantics
- Windows posture is ambiguous
- The decision is not issued by authorized operator review
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
