# Structured Planning Prompt

Template: 1.0.0

Issue: 400

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Add a narrow typed recovery operation or bounded authorization path, prove positive and negative behavior with focused tests, then validate and publish after exact review.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap, review, and bind #400 through typed C-SDLC v2.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement the narrow recovery operation/authorization path for SPP step and STP dependency truth repair.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Add focused positive and negative tests for #117-style recovery and guardrails.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run focused validation, exact review, publication, and CI shepherding.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  }
]

## Invariants

- all lifecycle mutation routes remain typed
- generation and digest CAS must match before mutation
- audit evidence is append-only
- publication still requires fresh exact-head review

## Risks

- over-broad recovery could bypass review truth
- SPP design-plan status semantics may conflict with execution truth if not explicitly bounded
- dependency repair must not become a generic implemented-phase STP rewrite

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/400/design.md

Digest: dffcc62edb01e76e836542b3190f65c6952c913bc6f7ccfbc92395704bbe2777

## Diagram

.csdlc/prepared/issues/400/diagram.mmd

Digest: f722953c905cb1c24fa7fe389c76a7a3b2efd2192b27faa5cd4072d62befa5d4

## Stop Conditions

- typed lifecycle refuses bootstrap/bind
- existing #400 owner/worktree collision is found
- implementation requires broad lifecycle reset or raw card edits
- focused validation or exact review fails

## Handoff

Proceed only after doctor readiness.
