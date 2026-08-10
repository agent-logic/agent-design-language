# Structured Planning Prompt

Template: 1.0.0

Issue: 199

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After #191 and #201 merge, bind and implement one crash-resumable coordinator for learner catch-up, standard OpenRaft joint/final changes, exact Runtime authority parity, pending removal exclusion, and governed rejoin; prove it, independently review it, and publish a ready unmerged PR before releasing #200.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "After #191, #201, and #202 merge ancestrally, bind #199 and freeze the exact old-cut parity, stable Raft-id registry, non-voting enrollment, learner catch-up, durable joint/final observation, shared pending exclusion, reconcile-before-visible publication, and restart contracts.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement only MembershipTransitionCoordinator plus the narrow existing authority and PolisRuntime integration needed to consume #201 tokens and publish exact membership parity.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Prove add, remove, rejoin, learner catch-up, joint/final quorum, stable ids, exclusion, leader change, exact retry, phase crash windows, rollback, corruption, capacity, and path safety against real secure OpenRaft nodes.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Resolve fresh exact-head review, publish a ready PR closing #199, shepherd hosted CI, and wait for operator review and merge authorization.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- No caller, leader, learner, removed voter, stale node, route hint, local log, or local snapshot can choose or restore voting authority
- Every transition preserves stable existing Raft ids and binds exact old, joint, and final configuration digests
- A learner never votes before exact committed-prefix or canonical-snapshot catch-up
- A pending removal target cannot authorize new operations even before final uniform membership publishes
- No public membership or route view becomes authoritative before final Raft and concrete authority parity plus checkpoint reconciliation

## Risks

- OpenRaft membership API semantics could be wrapped incorrectly or duplicate joint consensus
- A remapped Raft id or incomplete joint-config check could authorize the wrong voter
- Pending removal could leave a stale voter able to endorse while the transition is in flight
- A learner could be promoted from lagging or divergent local history
- Crash or leader change between OpenRaft and concrete publication could expose split authority
- Scope could drift into #200 concrete authority-store mutations

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/199/design.md

Digest: f29aa160c2543e6c30e147c1e5bd95617794ed2c97c0eb38dd7cb4ce058b9542

## Diagram

.csdlc/prepared/issues/199/diagram.mmd

Digest: a2d18fc4bae10a7a66894ffdec22ebf446147f6d7dae414215eae0c4cda1cb47

## Stop Conditions

- PR #197, #201, or #202 is not externally reviewed, merged, and ancestral
- The coordinator would reimplement OpenRaft voting or accept caller-selected voter authority
- Stable Raft ids, exact joint/final observation, or learner catch-up cannot be established through the merged APIs
- Pending removal cannot deny new authority before final publication without mutating #200-owned stores
- Crash reconciliation cannot keep incomplete parity fail closed
- Implementation expands into #200, kernel continuity, Guardian/API/WSS, models, AWS, or live qualification
- Any focused proof or independent review has an unresolved actionable finding

## Handoff

Proceed only after doctor readiness.
