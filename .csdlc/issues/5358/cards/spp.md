# Structured Planning Prompt

Template: 1.0.0

Issue: 5358

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Bind #5358 in its issue worktree, preserve a narrow issue-local claim, generate six typed cards and retained design, validate and doctor them, run bounded review, fix preparation-only findings, and stop at reviewed readiness.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Initialize and bind the typed #5358 record in its dedicated worktree with issue-local protected paths",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Inventory #5540, #5541, #5548, and #5558 as evidence or defect inputs without scope absorption",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Validate all six cards, retained design integrity, typed issue state, and diff hygiene",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run bounded subagent review, correct preparation-only findings, and stop before acceptance execution",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- All lifecycle and card state mutations use independent typed v2 binaries
- Generated Markdown remains binary-owned
- Tracked work occurs only in the #5358 worktree
- Protected paths remain issue-local and do not include shared milestone documents
- Open defect inputs remain independently owned
- Planning and readiness evidence never count as acceptance proof

## Risks

- Closed recovery evidence may be historically accurate but insufficient for current exact-revision acceptance
- Open #5548 or #5558 may keep future acceptance blocked
- Installed binary provenance or selector truth may drift before acceptance execution
- A planning-only SOR or readiness result could be misread as completed acceptance

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5358/design.md

Digest: b875a671de05e1c4f769d96ff26cd104c10d8990faa9463881f3a0f131a5620f

## Diagram

.csdlc/prepared/issues/5358/diagram.mmd

Digest: bef618b962abf9b6892b13a17a56cf7d5eb6897ca34d4c4439db86798b83c94c

## Stop Conditions

- Any protected-path collision or inability to bind exact issue ownership
- Any need to edit shared milestone documents or another issue's projection
- Any request to run acceptance, deploy, publish, merge, or close the issue
- Any validation requiring AWS or raw gh
- Any unresolved corruption in generated card or typed issue state

## Handoff

Proceed only after doctor readiness.
