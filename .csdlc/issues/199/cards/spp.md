# Structured Planning Prompt

Template: 1.0.0

Issue: 199

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After #202 is reviewed, merged, ancestral, and followed by a fresh preparation resync and typed validation, bind and implement one crash-resumable coordinator for learner catch-up, standard OpenRaft joint/final changes, exact Runtime authority parity, pending removal exclusion, and governed rejoin; prove it, independently review it, and publish a ready unmerged PR while preserving already-merged #200 as an out-of-scope consumer boundary.

## Plan

Revision 7

## Steps

[
  {
    "id": "S1",
    "action": "Verify merged #202 exact ancestry, current governed factory APIs, coarse Membership plus sealed discriminator contracts, and clean typed readiness; freeze the local and external generation saga before product edits.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement MembershipTransitionCoordinator and narrow authority, membership, and governed Polis integration; invoke only #202 governed ports, persist exact external operation and generation receipts, drive standard OpenRaft learner, joint, and final changes, and publish local parity only after re-observation.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Prove add, remove, rejoin, coarse-kind and discriminator denial, learner catch-up, joint and final quorum, stable ids, exclusion, leader change, exact retry, before and after #202 call and observation crash windows, rollback, corruption, capacity, and path safety against real secure OpenRaft nodes.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Resolve fresh exact-head review, publish a ready PR closing #199, shepherd only required hosted CI, and typed-finish the exact green head.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "in_progress"
  }
]

## Invariants

- No caller, leader, learner, removed voter, stale node, route hint, local log, or local snapshot can choose or restore voting authority
- Every transition preserves stable existing Raft ids and binds exact old, joint, and final configuration digests
- A learner never votes before exact committed-prefix or canonical-snapshot catch-up
- A pending removal target cannot authorize new operations even before final uniform membership publishes
- No #199 local membership-parity view becomes authoritative before final Raft membership, exact observed #202 receipt, local authority parity, and checkpoint reconciliation; #202 independently owned views may publish earlier while #199 remains fail closed

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

Digest: 2f586106b23edf6709f77f9065e5df5afca1dbfcfdf10700154d81d5e743fdb9

## Diagram

.csdlc/prepared/issues/199/diagram.mmd

Digest: b193376447764581c50ca5906de1c770a3bfa4d3aabac113444595588c349eda

## Stop Conditions

- Serial stop: #202 is not externally reviewed, merged, and ancestral to the preparation branch
- After #202 merges, #199 has not yet been resynced onto the resulting origin/main and passed typed csdlc-validate and csdlc-doctor
- The coordinator would reimplement OpenRaft voting or accept caller-selected voter authority
- Stable Raft ids, exact joint/final observation, or learner catch-up cannot be established through the merged APIs
- Pending removal cannot deny new authority before final publication without mutating #200-owned stores
- Crash reconciliation cannot keep incomplete parity fail closed
- Implementation expands into #200, kernel continuity, Guardian/API/WSS, models, AWS, or live qualification
- Any focused proof or independent review has an unresolved actionable finding

## Handoff

Proceed only after doctor readiness.
