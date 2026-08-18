# Structured Planning Prompt

Template: 1.0.0

Issue: 296

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Design and independently approve the narrow typed contract, bind #296, implement the recovery-only semantic operation, prove lifecycle/CAS/path/atomicity/history invariants, obtain a fresh exact-head review, publish ready, and stop before merge.

## Plan

Revision 5

## Steps

[
  {
    "id": "S1",
    "action": "Define and approve the recovery-only authored-design refresh contract and fail-closed artifact boundary.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement atomic binding refresh, approval invalidation, append-only provenance, and authority guards.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Add focused fresh-review, history, negative, and pre-bind regression proof.",
    "acceptance_ids": [
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Validate, independently review, publish ready, and keep #294 blocked until terminal ancestry.",
    "acceptance_ids": [
      "AC-9"
    ],
    "status": "completed"
  }
]

## Invariants

- Only current implemented review recovery authorizes refresh
- Both design-bearing cards update atomically or neither changes
- Stale design approval is always invalidated
- Phase, topology, execution evidence, transitions, and prior audit entries are preserved
- Fresh canonical design approval precedes any new implementation review authority

## Risks

- Over-broad authorization could permit later-phase truth rewrite
- Partial artifact reads could desynchronize SPP and VPP
- Approval invalidation could fail to block review assignment
- Audit provenance could under-specify the state change

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/296/design.md

Digest: e4d4e8f5b9385cb4f7d7f286ddae8365f93245628d01308017bd76bd70222d77

## Diagram

.csdlc/prepared/issues/296/diagram.mmd

Digest: 3ae668e6528b89970f972b1b895ae3676b0561c48de46bfbcc9acc9b28e9e654

## Stop Conditions

- Issue #294, #291, #292, or unrelated root state would be mutated
- Typed lifecycle reports stale or conflicting topology
- Exclusive csdlc-v2 file ownership is lost
- Focused validation, exact-head review, publication, or required CI fails

## Handoff

Proceed only after doctor readiness.
