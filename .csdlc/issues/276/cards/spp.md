# Structured Planning Prompt

Template: 1.0.0

Issue: 276

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Prepare and bind #276 as the first #114 child after #112/#265/#270 terminal dependencies validate; implementation remains limited to durable journal foundation and starts only after bind.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Refresh and bootstrap the #276 design/card packet from live issue truth and canonical terminal dependency observations.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Validate scope boundaries and non-goals against #114/#276 live issue text.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Run prep validator, typed doctor/validate, and obtain fresh design/readiness review before design approval.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Bind only the dedicated #276 branch/worktree under FastWork after PASS and typed approval.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "completed"
  }
]

## Invariants

- #276 consumes Layer 8 authority and acknowledgement evidence but does not redefine either
- #114 parent remains unbound and is not bound by child execution
- #277/#278 are not absorbed into #276
- Journal recovery must fail closed on corruption or partial-write ambiguity after implementation begins

## Risks

- Durable storage design can accidentally absorb #277 replay/watermark semantics unless non-goals remain explicit
- Retention/deletion primitives must preserve downstream receipt coherence
- Dirty primary root projections may disagree with canonical dependency caches

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/276/design.md

Digest: a55f0219502ec12cfb4e9ba882b74569fd0a3ed7db2fe62a7cfd6818ad716262

## Diagram

.csdlc/prepared/issues/276/diagram.mmd

Digest: e488739c3a623e0723dce9620d729b77b4a9c15ccc35271783c7bd98ab94bb46

## Stop Conditions

- #112, #265, or #270 terminal cache or ancestry validation fails
- Design/readiness review reports unresolved actionable findings
- Scope expands into acknowledgement protocol, replay reconciliation, public API, Observatory restoration, or parent integration proof
- Bind would target anything other than the dedicated #276 FastWork worktree

## Handoff

Proceed only after doctor readiness.
