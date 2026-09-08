# Structured Planning Prompt

Template: 1.0.0

Issue: 645

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap and bind #645, reproduce the typed publication mismatch with offline test fixtures and current live read-only PR state, add the fail-closed closing relation guard, and validate the C-SDLC v2 publication lane.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Bind #645 into a FastWork issue worktree and confirm live read-only PR #644 still has no GitHub closing relation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Add or tighten the csdlc-publish closing-mode reconciliation guard against live PR-state linked_issue/linkage_source.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Preserve explicit non-closing checkpoint semantics for stacked dependency PRs.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run focused C-SDLC v2 publication regression tests, formatting, and diff hygiene.",
    "acceptance_ids": [
      "AC-1",
      "AC-6"
    ],
    "status": "completed"
  }
]

## Invariants

- GitHub closingIssuesReferences is required for terminal closing authority
- Body keyword parsing remains necessary but not sufficient
- Non-default stack-base PRs fail closed unless GitHub reports the exact closing relation
- Checkpoint/dependency publication is explicit and non-terminal
- Main remains inspection-only

## Risks

- A helper may still infer linked_issue from PR body after the guard is added
- Existing publish tests may assume body keyword validation alone is enough
- Non-closing checkpoint paths may need clearer linkage-mode output to avoid overclaim
- Live GitHub API shape could differ from offline fixtures

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/645/design.md

Digest: 0bc4f8014dee0971babeb07386088a49551df6391cd1c072e7051cddd5b37f96

## Diagram

.csdlc/prepared/issues/645/diagram.mmd

Digest: 93e135c5fb690336255a542ea32c12205800d5c953053476bb9262521b49717a

## Stop Conditions

- The issue cannot be bootstrapped or bound without writing tracked work on main
- The live PR-state relation cannot be verified read-only
- The fix requires mutating PR #644 or bypassing typed lifecycle writes
- Focused regression tests cannot model closingIssuesReferences absence deterministically

## Handoff

Proceed only after doctor readiness.
