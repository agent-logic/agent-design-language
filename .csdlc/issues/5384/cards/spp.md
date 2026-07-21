# Structured Planning Prompt

Template: 1.0.0

Issue: 5384

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Render and validate the six typed cards, retain a complete issue-local evidence-consumer design, obtain bounded preparation review, approve the design through typed v2, bind only the three lifecycle paths, and push the preparation branch while all implementation steps remain gated.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Render and validate all six current native typed cards plus issue-specific design and diagram",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Validate COTS, budget, PVF, protected-path, and complete predecessor promotion gates",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run bounded preparation subagent review and repair every actionable finding",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Approve and bind the preparation-only claim, commit and push without publication",
    "acceptance_ids": [
      "AC-4",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "After a future separately authorized promotion gate passes, execute integrated WP-14A acceptance and handoff",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  }
]

## Invariants

- no tracked root-main writes
- no product or shared-document paths in the preparation claim
- no implementation until every predecessor gate fact passes at one refreshed origin/main revision
- no manual edits to rendered cards or lifecycle state
- no fake approvals, waivers, inferred terminal state, or prose-only evidence
- no PR, publication, AWS, Runtime v2, or raw gh

## Risks

- stale issue prose or a closed GitHub issue can be mistaken for typed terminal truth
- a broad preparation claim could accidentally authorize product work
- nested Runtime and workcell inputs can be omitted from the direct WP-14A child list
- planning estimates can be misreported as execution evidence
- a predecessor closed without merged ancestry cannot satisfy the operator's strict promotion rule

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5384/design.md

Digest: bec76e87d1e5520193a101ef4cbd9e4d90b77526334e3991524782e0fc68fe77

## Diagram

.csdlc/prepared/issues/5384/diagram.mmd

Digest: 5dcaa986a3318096cb25ba898cc6ffa1f26790c0b952b0679195992ec7db91f2

## Stop Conditions

- any request requires a path outside the three protected preparation paths
- any declared predecessor lacks merged, typed closed_out, receipt, or ancestry proof
- current-template identity or structure validation fails
- bounded review reports an actionable finding that cannot be fixed within preparation scope
- another claim, branch, or worktree collides with #5384

## Handoff

Proceed only after doctor readiness.
