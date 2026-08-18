# Structured Planning Prompt

Template: 1.0.0

Issue: 117

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Validate terminal child caches, bind #117 to a FastWork worktree, author parent-only evidence and validator, run focused proof, obtain fresh exact review, then publish/finish if all gates remain green.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Validate canonical terminal caches for #271, #114, #115, #116, #279, #280, #281, and #282.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Author parent-only closeout evidence and deterministic validator in the bound #117 worktree.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Run focused parent validator and diff hygiene proof.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Obtain fresh exact review, publish with closing linkage, shepherd CI, and finish when green.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "completed"
  }
]

## Invariants

- #117 evidence is parent-only and docs/lifecycle scoped.
- Terminal child evidence is consumed read-only from canonical bound worktrees.
- Residual risks and non-claims are explicit; no synthetic runtime/provider/cloud proof is invented.
- The validator is deterministic and credential-free.

## Risks

- Overclaiming umbrella terminal closeout while #110 remains open.
- Treating stale root projections as canonical dependency evidence.
- Accidentally absorbing child-owned product proof or implementation scope.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/117/design.md

Digest: 0c3bdcb8801003259e8526f09cb5775bbe5f07e41dc2ac2f9f3aa5efe720bc6c

## Diagram

.csdlc/prepared/issues/117/diagram.mmd

Digest: 1f3b962568687426bd66d190e0c022dfa9b563bf5d826040773239ac61cc23a8

## Stop Conditions

- Any terminal child cache is non-canonical or not merged/closed.
- Root/worktree collision would overwrite another issue's unique staging.
- Review finds a parent overclaim or child-scope absorption.
- Publication/CI/finish gates are not exact-current green.

## Handoff

Proceed only after doctor readiness.
