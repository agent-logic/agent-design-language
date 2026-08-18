# Structured Planning Prompt

Template: 1.0.0

Issue: 297

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Independently approve the crash/restart protocol, bind #297, implement typed classification/recovery/cleanup with immutable receipts and inode authority, prove all failure boundaries and regressions, obtain a fresh exact-head review, publish ready, and stop before merge.

## Plan

Revision 11

## Steps

[
  {
    "id": "S1",
    "action": "Preserve the old #297 broad source candidate as historical non-authoritative evidence; resync with current main and keep #300 review-failed/unpublished while Noether P1s are open.",
    "acceptance_ids": [
      "AC-8",
      "AC-9"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement only the minimum production recovery-to-cleanup authority bridge in #297: recovery must emit or expose cleanup-consumable completed recovery receipt and canonical archive manifest authority without test-authored JSON.",
    "acceptance_ids": [
      "AC-2",
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Validate, independently exact-head review, publish, shepherd CI, and finish #297 so the bridge is terminal and ancestral before #300 resumes.",
    "acceptance_ids": [
      "AC-9"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "After #297 is terminal and ancestral, update #300 so its integration target consumes production-generated bridge artifacts and mechanically invokes or enumerates the approved recovery/cleanup failpoint and adversarial matrix.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-7",
      "AC-8"
    ],
    "status": "completed"
  }
]

## Invariants

- No path name or digest alone grants destructive authority; the exact opened inode and stable manifest must match
- Rejected evidence survives recovery and cleanup remains a distinct explicit operation
- Every namespace transition is preceded and followed by immutable durable receipt truth
- Restart resumes only an unambiguous operation-owned chain and otherwise fails closed
- Canonical lifecycle phase, topology, execution evidence, and prior audit history remain truthful and append-only

## Risks

- TOCTOU between candidate classification and namespace mutation
- Crash windows may make canonical, backup, and archived paths ambiguous
- Recursive cleanup could delete a replacement or unrelated evidence
- An overly permissive recovery path could bypass lifecycle/CAS/topology authority
- Insufficient real failpoint coverage could leave restart invariants unproved

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/297/design.md

Digest: fa657fb40b9571fd0ff37b0099de2546405dbe7fc836ebabd264a8096d305978

## Diagram

.csdlc/prepared/issues/297/diagram.mmd

Digest: 8fea8524f0be5ce3b0cf19b9034a9721dcf92bb3afda875df661679494fc816a

## Stop Conditions

- Issue #296, #294, #291, #292, or unrelated root state would be mutated
- Typed lifecycle reports stale or conflicting topology
- Exclusive store.rs ownership is lost
- Any destructive step lacks exact opened-inode and manifest authority
- Focused validation, exact-head review, publication, or required CI fails

## Handoff

Proceed only after doctor readiness.
