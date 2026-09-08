# Structured Planning Prompt

Template: 1.0.0

Issue: 680

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap #680, bind a FastWork worktree, add the first-class Moonshot/Kimi setup/profile/provider-kind path, validate with focused deterministic tests and strict lint, then obtain exact implementation review before typed publication.

## Plan

Revision 2

## Steps

[
  {
    "id": "step-1",
    "action": "Confirm current Moonshot/Kimi model naming from official sources and preserve it in implementation evidence.",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "step-2",
    "action": "Add or adjust provider profile/setup/provider-kind surfaces for first-class Moonshot/Kimi K3 support.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "step-3",
    "action": "Add deterministic tests for profile, setup/help, auth request construction, and failure classification without live credentials.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "step-4",
    "action": "Run focused validation, strict Clippy, fmt/diff checks, record SOR truth, exact review, and publish PR.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Credential material never appears in committed files or logs.
- Existing kimi:k2.5 behavior remains compatible.
- Provider setup and profile identity stay deterministic and reviewable.
- Offline validation is not overstated as live provider proof.

## Risks

- Moonshot catalog naming may differ between platform docs and model-card/API surfaces.
- Provider builder and hosted adapter paths may have separate identity lists that must not drift.
- Strict Clippy may expose adjacent provider-surface warnings.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/680/design.md

Digest: a3c90cc471c42e2254bac1a0c0c7f465a3223cac5d7d3dc7d4cfcbffdd42a32b

## Diagram

.csdlc/prepared/issues/680/diagram.mmd

Digest: df55fdd651b5667f53393cb73c5f3fa98d5e722ed8b9a4c47f43bdc6c17acfc3

## Stop Conditions

- Typed lifecycle bootstrap/bind refuses the issue state.
- Current Moonshot model id cannot be determined without credentialed live API access.
- Implementation requires live paid provider proof not authorized by the operator.
- Root checkout gains unrelated tracked dirt or another owner claims #680.

## Handoff

Proceed only after doctor readiness.
