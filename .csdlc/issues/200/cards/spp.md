# Structured Planning Prompt

Template: 1.0.0

Issue: 200

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After #191 and #201 merge, bind #200, implement the generic reconciliation barrier with a sealed test adapter, prove every durable and authority boundary, independently review, and publish a ready unmerged PR before #203.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "After #191 and #201 merge ancestrally, bind #200 and freeze the opaque token/plan/receipt/permit plus journal/checkpoint/view contracts.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement the barrier, sealed registry, deterministic time carrier, durable phases, restart reconciliation, and test-only deterministic adapter in exact owned paths.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Prove the exact thirty-six-case denominator, strict Clippy, merge-safe receipt, capacity, rollback, crash, and path-safety contracts.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Resolve fresh exact-head review, publish a ready PR closing #200, shepherd hosted CI, and wait for operator review and merge authorization.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "in_progress"
  }
]

## Invariants

- No caller-produced token, adapter, receipt, boolean, or legacy command can create reconciliation authority
- No authority-restoring read or mutation permit exists before exact published-generation parity
- Exact retry never re-executes a completed adapter plan
- Partial multi-step progress is fail-safe and never described as atomic
- Every local durable state is bounded, canonical, locked, checkpointed, and rollback-detecting

## Risks

- A public adapter or receipt seam could convert caller claims into authority
- A permit could expose partially reconciled concrete state before publication
- Ambiguous checkpoint outcomes could duplicate a step or lose a committed result
- Local clocks could make replicas derive different adapter inputs
- A generic barrier could drift into concrete #203/#204 behavior and become oversized

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/200/design.md

Digest: 7ad324d6833f2baa41d41a3115316e62df7cdc2863d54384d61c059316cd44ae

## Diagram

.csdlc/prepared/issues/200/diagram.mmd

Digest: b121a59cca8190c82a2fe1b53a658d318dd2e74f45263861fb4729a14743ab7b

## Stop Conditions

- PR #197 or #201 is not externally reviewed, merged, and ancestral
- The #201 token remains caller-constructible or lacks exact time/membership/checkpoint binding
- A production adapter would require a public trait object, closure, completion boolean, or raw receipt
- A current permit cannot be withheld until state, result, checkpoint, marker, and view agree
- Implementation expands into #203, #204, #199, #202, Guardian/API/WSS, models, AWS, or live qualification
- Any focused proof or independent review has an unresolved actionable finding

## Handoff

Proceed only after doctor readiness.
