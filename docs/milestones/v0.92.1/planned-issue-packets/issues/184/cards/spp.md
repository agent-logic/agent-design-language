# Structured Planning Prompt

Template: 1.0.0

Issue: 184

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Using only the Agent Logic business AWS account, prove hybrid Wuji and two-private-AZ quorum continuity, stale fencing, partition healing, halt behavior, independent snapshots, and provider-verified cleanup.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Verify the permanent agent-logic-admin profile resolves to the approved Agent Logic business account; stop before provisioning on any identity, billing, quota, or authority mismatch.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Provision two private AWS voters in distinct availability zones with explicit no-public-ingress networking, approved trust chains, distinct credentials and state roots, independently materialized snapshots, and bounded billing tags; launch the Wuji voter separately.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Establish authenticated private transport and prove three-voter commit, then isolate Wuji and prove AWS-only quorum continuity while stale Wuji mutation is fenced.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Remove one AWS voter to lose quorum and prove mutation halts; restore connectivity and verify convergence of term, commit index, state digest, fence, and Observatory ownership before resuming traffic.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Collect provider API readback for placement, networking, snapshots, resources, cost tags, and all termination operations; clean every resource after each success or failure phase.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Run the issue validator and single-command hybrid proof, fail on public endpoints, self-signed production certificates, shared snapshots, or incomplete cleanup, then obtain independent exact-head review.",
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

- Issue DRT-04 owns only its declared repository paths and named external operation/evidence boundary.
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
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/184/design.md

Digest: 0b5cd46c62b64eff7e8278e3d42dad97343193483ef183a896398351c48834f5

## Diagram

.csdlc/prepared/issues/184/diagram.mmd

Digest: 6a047f33214b501d811c7afe7eaf21f35531fc49308d6371bae30977753ecd09

## Stop Conditions

- AWS identity is wrong
- A public endpoint or self-signed production certificate appears
- Snapshots share materialization history
- Resources cannot be enumerated and removed
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
