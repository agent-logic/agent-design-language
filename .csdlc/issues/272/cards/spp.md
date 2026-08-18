# Structured Planning Prompt

Template: 1.0.0

Issue: 272

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap and review the exact foundation design, bind from current main, implement only the frozen three product/test paths, prove durable reconcile-before-publish and redaction behavior, obtain fresh exact-head review, publish, shepherd required CI, and finish #272 before releasing #273/#274.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Validate the six prerequisite terminal caches and exact current-main preparation base, run the issue-owned preparation validator, obtain fresh design review, approve, doctor, and bind.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement the bounded durable store, exact binding types, Pending/Reconciled/Published transitions, restart reconciliation, capacity and path safety, and redacted base projection only in the frozen allowlist.",
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
    "id": "S3",
    "action": "Run the focused foundation target serially, strict Clippy, exact changed-path scope validation, typed lifecycle validation, and diff hygiene; preserve immutable evidence.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Finalize implemented truth, assign and record a new fresh-session exact-head review, repair every actionable finding, and rerun the proving lanes after substantive changes.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Publish a ready PR closing #272, shepherd ordinary required checks, run typed finish, validate the canonical terminal cache, and prove merge ancestry before releasing child successors.",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "completed"
  }
]

## Invariants

- No foundation view is visible from Pending or unreconciled state
- Every visible generation binds one exact sealed authority cut and #203 published receipt
- Retry and restart return only the exact previously published result or fail closed
- The node-local store is a bounded replica and cannot mint authority
- The redacted projection reveals no raw authority material and decides no eligibility
- #203 registry and #273/#274/#275 behavior remain outside #272

## Risks

- The preserved monolithic #205 design could leak eligibility lifecycle into #272
- A stale or conflicting receipt could be accepted after restart
- Partial durable state could be exposed as published
- The base projection could leak identity or timing material
- Later #273/#274 work could collide on the single module unless serialized or redesigned

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/272/design.md

Digest: e7310ec3111d2ce4ad6f6fceb408e34460a03b40cc1846a1f3b7e1caf063e627

## Diagram

.csdlc/prepared/issues/272/diagram.mmd

Digest: 6fcd3a8b32e10173cc5463decf28bbea840bc95a8546f668826d7fb94a2822bc

## Stop Conditions

- Any prerequisite terminal cache is noncanonical or nonancestral
- The preparation base or owned-path collision truth changes before bind
- The terminal producer cannot supply the exact framed canonical ADL-SERVING-AUTHORITY-FOUNDATION-BINDING-V1 bytes matching its sealed #203 result_sha256
- Implementation requires any path outside the frozen allowlist
- Any validation, review, CI, terminal-cache, or ancestry gate fails

## Handoff

Proceed only after doctor readiness.
