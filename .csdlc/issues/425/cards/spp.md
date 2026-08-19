# Structured Planning Prompt

Template: 1.0.0

Issue: 425

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap #425, review the design, bind a FastWork worktree, implement typed recordless/no-projection closeout recovery/classification with focused tests, validate, fresh-review, publish if green, then rerun eligible v0.92 residual closeouts.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap and design-review the #425 recordless closeout recovery contract.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement typed request/result parsing, live authority validation, no-projection classification, contradictory-evidence refusal, and recordless terminal receipt materialization.",
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
    "id": "S3",
    "action": "Add focused positive and negative tests for recordless closeout recovery and no-projection blockers.",
    "acceptance_ids": [
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused validation, fresh exact-head review, publish if green, and rerun v0.92 residual closeout where safe.",
    "acceptance_ids": [
      "AC-9"
    ],
    "status": "pending"
  }
]

## Invariants

- Recordless recovery cannot claim absent review/implementation/card evidence
- Exact live GitHub state must match requested issue, PR, head SHA, merge SHA, and closing linkage
- Contradictory retained publication evidence fails closed
- Normal publication/finish behavior remains strict for active issues

## Risks

- Recordless receipts could be mistaken for normal implementation proof unless explicitly marked
- GitHub closing-linkage parsing could differ from GitHub closure semantics
- Existing retained caches may reveal precedence conflicts that must block closeout

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/425/design.md

Digest: c638a8436ab15d5674274765a3bb846ea607634b0ee7bdbb9e299139f2481e99

## Diagram

.csdlc/prepared/issues/425/diagram.mmd

Digest: cead0c7fe405f4400fff0cca3845eb7ea64b4901928414b0ac8be7c06317481a

## Stop Conditions

- Design review rejects recordless terminal semantics
- Implementation would require product or raw GitHub writes
- Focused tests cannot distinguish contradictory precedence
- Validation/review fails

## Handoff

Proceed only after doctor readiness.
