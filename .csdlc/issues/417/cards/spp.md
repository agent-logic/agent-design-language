# Structured Planning Prompt

Template: 1.0.0

Issue: 417

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bind #417, prove the deadlock, make implemented authored-refresh eligibility recovery-epoch aware, add exact and negative regressions, obtain fresh review, publish, merge, finish, clean, and install the terminal owner binaries.

## Plan

Revision 4

## Steps

[
  {
    "id": "step-1",
    "action": "Reproduce and characterize the exact implemented recovery audit sequence.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "step-2",
    "action": "Implement recovery-epoch-aware authored refresh eligibility with bounded intervening operations.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "step-3",
    "action": "Add exact public-operation regressions and negative authority/provenance assertions.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "step-4",
    "action": "Review, publish, merge, finish, clean, install, and report the safe #414 resume operation.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- The record remains implemented during design recovery.
- The originating recover_review audit event remains the recovery-epoch anchor.
- Only supported recovery repair operations may intervene.
- Review and publication authority remain absent after substantive recovery edits.
- No hand editing of generated cards or canonical state.

## Risks

- Searching too far back could authorize a stale recovery epoch.
- An incomplete operation allowlist could preserve the deadlock or admit unrelated mutations.
- Recovery could accidentally retain review or publication authority.
- A test could construct synthetic state without exercising public typed operations.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/417/design.md

Digest: fa7af980a5c8a4288468733b0801dafc2d905ec737d817cfcf51e9604a4d36f0

## Diagram

.csdlc/prepared/issues/417/diagram.mmd

Digest: de5ef56eab36dfb8b3e86dd8cbba25ffb58a55ed8a015783f7fe5cfab2750b28

## Stop Conditions

- The change requires mutating #414, #268, #269, or AWS.
- Eligibility cannot be tied unambiguously to one originating recovery epoch.
- Focused tests cannot prove downstream authority remains cleared.
- Review, CI, merge, finish, ancestry, or installed provenance is not current.

## Handoff

Proceed only after doctor readiness.
