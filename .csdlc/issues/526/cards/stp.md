# Structured Task Prompt

Template: 1.0.0

Issue: 526

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Prepare and, only after explicit authorization, perform exactly one v0.92.1 release ceremony; no feature work or unrelated cleanup.

## Deliverables

- Final release notes bound to the approved candidate
- Operator authorization record
- Tag and release readback receipt
- Tail merge and ancestry census

## Acceptance

1. AC-1: Every prior tail issue has a reviewed-green merge ancestral to the candidate
2. AC-2: Release notes describe the exact candidate without unsupported claims
3. AC-3: Explicit operator authorization identifies the candidate, tag, and release operation
4. AC-4: Tag and published release resolve to the exact approved candidate
5. AC-5: Live readback and immutable receipt capture notes, tag, release URL or identifier, timestamps, and hashes
6. AC-6: No feature implementation, unreviewed merge, or unrelated cleanup occurs

## Dependencies

- Reviewed merge for #525 / TAIL-09
- Reviewed-green ancestral merges for all prior v0.92.1 tail issues
- Explicit operator authorization for the exact release candidate and mutation

## Inputs

- docs/milestones/v0.92.1/RELEASE_PLAN_v0.92.1.md
- docs/milestones/v0.92.1/MILESTONE_CHECKLIST_v0.92.1.md
- docs/milestones/v0.92.1/RELEASE_NOTES_v0.92.1.md
- agent-logic/agent-design-language#526

## Non Goals

- Feature implementation
- Unreviewed merge
- Successor milestone execution
- Administrative finish or worktree cleanup
- Release mutation before authorization
