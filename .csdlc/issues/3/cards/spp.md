# Structured Planning Prompt

Template: 1.0.0

Issue: 3

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bind the existing issue branch, harden effective remote and exhaustive PR reconciliation, add focused proof and docs, retain the live canary, review, and publish one canonical PR.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Audit the landed split-authority baseline and implement exact fetch/push remote plus exhaustive unambiguous PR reconciliation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Add focused regression tests and public-schema coverage for same-repository and split-authority paths.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Update typed operator and GitHub-client contract documentation for separate code and issue identities.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Retain and validate live PR #5 to legacy issue #5901 canary evidence, then complete exact-head independent review.",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- Issue and code repository identities never collapse implicitly
- Effective push destination is verified before push
- Remote PR matching is exhaustive and unambiguous
- Same-repository compatibility remains fail-closed
- No legacy code repository mutation

## Risks

- Git pushurl substitution bypasses fetch-URL verification
- First-page PR reconciliation misses ambiguity
- Qualified linkage is weakened during compatibility handling
- Documentation teaches the old one-repository contract

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/3/design.md

Digest: 33aeb023438cf13eca9cc81db11d17c782ea1c0c5d9f319fad1709770991edc7

## Diagram

.csdlc/prepared/issues/3/diagram.mmd

Digest: d0c70d826286804717d156c865bdb60416d559c7bb121729eac3f225aa2dec1a

## Stop Conditions

- The change requires legacy repository code mutation
- Same-repository publication must be broken
- Live canary identities cannot be independently verified
- Scope expands into issue migration

## Handoff

Proceed only after doctor readiness.
