# Structured Planning Prompt

Template: 1.0.0

Issue: 5337

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Initialize and bind #5337, verify and semantically normalize all six generated cards, validate current-template and PVF truth, commit issue-local preparation artifacts, run bounded exact-revision subagent review, fix findings, and publish only if the typed lifecycle accepts a preparation-only boundary.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Initialize and bind the issue-local typed lifecycle record in the dedicated #5337 worktree",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Validate and, only through csdlc-edit, normalize six current-template cards for preparation-only truth",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused typed validation and confirm protected-path and dependency boundaries",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Commit the substantive preparation revision and run bounded exact-revision subagent review",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Record findings disposition and publish or retain a truthful blocked preparation handoff through typed lifecycle gates",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- no tracked issue work on main
- all card mutation remains csdlc-edit owned
- only issue-local preparation paths are protected
- WP-02 acceptance precedes future implementation
- v1 is behavioral evidence rather than source authority
- no AWS and no raw gh

## Risks

- preparation language could be mistaken for authorization to implement the corpus
- overbroad protected paths could collide with shared milestone documentation or sibling lanes
- normalization rules could hide semantic mismatches if left unconstrained
- generated cards could remain generic while passing only superficial structure checks

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5337/design.md

Digest: 6e71aba9982df2cfbf79bf7798d795d32ce1c11dbf3e37cc1184b9587ac16a15

## Diagram

.csdlc/prepared/issues/5337/diagram.mmd

Digest: d14510e4439edfcc48a78fb640efdf43feed2dbfeaa0a1d58e2d30507a1209e7

## Stop Conditions

- another live claim or worktree owns #5337
- WP-03 preparation would require shared milestone-doc edits
- a lifecycle route would record product implementation or dependency satisfaction
- current-template provenance or typed card validation cannot be established
- publication would require bypassing current review truth

## Handoff

Proceed only after doctor readiness.
