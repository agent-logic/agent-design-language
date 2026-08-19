# Structured Planning Prompt

Template: 1.0.0

Issue: 421

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bind #421, reproduce the validator_target_missing deadlock, add typed intentional-deletion readiness semantics with base/candidate proof, add positive and adversarial regressions, validate, review, publish, shepherd, finish, clean, and install the terminal owner binaries.

## Plan

Revision 1

## Steps

[
  {
    "id": "step-1",
    "action": "Reproduce and characterize the exact validator_target_missing behavior for an intentional deletion deliverable.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "step-2",
    "action": "Implement explicit intentional-deletion readiness semantics with fail-closed base/candidate proof.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "step-3",
    "action": "Add positive #414-style and adversarial false-deletion regressions.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "step-4",
    "action": "Validate, review, publish, shepherd, finish, clean, install, and report the safe #414 resume operation.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Missing files are not accepted by default.
- Deletion claims are path-specific and issue-owned.
- A claimed deletion must be provable against the governed base revision.
- Review and publication authority remain exact-head and current.
- No hand editing of generated cards or canonical state.

## Risks

- A broad missing-file exception could hide real validator omissions.
- A string marker could be ambiguous or bypass issue-owned scope.
- Base proof could be stale or tied to the wrong revision.
- Tests could exercise synthetic state without proving public readiness behavior.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/421/design.md

Digest: d9c2906a6079f9d902d500dff7e938c2fd78dc5d08f60c9dc6b88569e5f17467

## Diagram

.csdlc/prepared/issues/421/diagram.mmd

Digest: b11d9df8a22557ceb6e7b970ff21e0533ed86a865c52802d6234ae5c57cb885b

## Stop Conditions

- The change requires mutating #414, #268, #269, AWS, or unrelated lifecycle state.
- Deletion proof cannot be tied to the governed base and exact candidate.
- Ordinary missing validator deliverables stop failing closed.
- Review, CI, merge, finish, ancestry, or installed provenance is not current.

## Handoff

Proceed only after doctor readiness.
