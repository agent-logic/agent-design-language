# Structured Planning Prompt

Template: 1.0.0

Issue: 298

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Approve the child-specific recovery design, bind #298, extract only useful r1 input, implement anchored classification and immutable resumable recovery, prove recovery-only boundaries, obtain fresh exact-head review, publish ready, and stop before merge.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Approve anchor-specific immutable archive/displacement/install plans, non-circular audit commitments, and the operation-owned temporary-node create, identity, write, fsync, and no-replace publish restart protocol.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement the approved typed classify/recover protocol and prove the later ordinary-commit release gate under the issue lock.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Run the initialized, ready, and issue #291 regression proof after the focused protocol proof required by S1 and S2.",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Obtain fresh exact-head human review as a distinct required evidence gate, publish ready, and shepherd required CI without merge.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "completed"
  }
]

## Invariants

- No path name, embedded invalid state, or digest alone grants recovery authority
- Rejected and displaced evidence survive #298 recovery
- Every recovery namespace transition has immutable intent and completion truth
- Restart adopts only exact operation-owned identities and otherwise fails closed
- A complete canonical candidate contains its typed recovery audit before atomic installation

## Risks

- TOCTOU between anchored classification and namespace mutation
- Crash windows may create ambiguous archive, candidate, canonical, or displaced state
- A partial candidate could be mistaken for publishable canonical state
- An overly permissive anchor or lineage check could bypass lifecycle authority
- Insufficient production failpoints could leave restart semantics unproved

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/298/design.md

Digest: 9ebdd2014ce91ab9a9b1946756ee715f245b9128aece8b1dc58c11afe0bfc52d

## Diagram

.csdlc/prepared/issues/298/diagram.mmd

Digest: 4fae9c334fb78962302b7b255acfacc5582d98fd6b3809fde73115634da114fa

## Stop Conditions

- Issue #291, #294, #296, #297, #299, #300, or unrelated root state would be mutated
- Typed lifecycle reports stale or conflicting topology
- Exclusive store.rs ownership is lost
- A recovery mutation lacks retained identity and immutable intent authority
- Focused validation, exact-head review, publication, or required CI fails

## Handoff

Proceed only after doctor readiness.
