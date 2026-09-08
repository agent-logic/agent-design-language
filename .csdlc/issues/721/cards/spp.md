# Structured Planning Prompt

Template: 1.0.0

Issue: 721

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Initialize #721 lifecycle state, complete v3 issue-create parity, fix terminal authority reporting, document the real operator path, validate focused v3 surfaces, and publish for review.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Confirm #721 scope and preserve existing issue-create parity patch in the bound worktree.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Fix terminal finish authority reporting and add focused regression coverage.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused remote, terminal, docs/help, fmt, and clippy validation.",
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
    "id": "S4",
    "action": "Record adjacent v3 defects as backlog issues and update execution truth.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- No raw gh lifecycle writes.
- No main-branch implementation edits.
- Remote mutation must write durable intent before external mutation.
- Uncertain mutation outcome must reconcile before replay.
- Pre-cutover behavior must fail closed.

## Risks

- Authority-reporting changes can overstate operational authority if not tied to persisted terminal state.
- Issue-create reconciliation can duplicate issues if marker search or receipt replay is weak.
- Simplifying the operator path can accidentally bypass typed receipts.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

docs/milestones/v0.92.1/evidence/csdlc-v3/issue-721/design.md

Digest: 77319b8ff88b927b9d0b4e8c5419bdf3e1d745b5295a444f56a62747ff9f0773

## Diagram

docs/milestones/v0.92.1/evidence/csdlc-v3/issue-721/diagram.mmd

Digest: 0e45c5281eeeb455b4047224ddf3899f7ccfc321dc7fd20d34f7fd2cb6eddd5f

## Stop Conditions

- Canonical authority checks would need to be weakened.
- Validation shows issue-create can duplicate issues under retry.
- Terminal authority reporting cannot be fixed without widening into v2 removal.

## Handoff

Proceed only after doctor readiness.
