# Structured Planning Prompt

Template: 1.0.0

Issue: 687

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Define one typed readiness state, derive legacy projection fields from it, classify resident and dynamic-agent failures, add focused state-matrix proof, review, and publish without live mutation.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Define the closed inference-readiness taxonomy and central projection mapping.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Wire resident Shepherd recovery and dynamic-agent health refresh to typed classifications.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add focused deterministic tests for every state and API/roster consistency.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused validation and obtain independent exact-head review before typed publication.",
    "acceptance_ids": [
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Only ready inference is communication-eligible
- Unsupported or placeholder adapters never receive production readiness credit
- Canonical agent IDs and names do not change
- Runtime availability is not coupled to one temporarily unavailable inference backend
- No credentials or provider response bodies enter readiness projections

## Risks

- Duplicated string mappings could create roster/API disagreement
- Retryable and terminal provider errors could be misclassified
- Adding fields could break backward-compatible deserialization
- Tests could accidentally perform live provider I/O

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/687/design.md

Digest: 8ac58078fa6608ada8d851d54f7b686840ca7aef5cc8def1c405e4eb1a039bb2

## Diagram

.csdlc/prepared/issues/687/diagram.mmd

Digest: 4613d1499893bcb0f7590c0473f30f44424a5b6aca6607cd837ed6fa0b41e42e

## Stop Conditions

- The change requires a live Runtime restart or provider call
- Scope expands into provider implementation or credential management
- Canonical agent identity would change
- A placeholder could receive ready credit
- Exact-head review has unresolved findings

## Handoff

Proceed only after doctor readiness.
