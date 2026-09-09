---
issue_card_schema: adl.issue.v1
wp: "TAIL-06.08b"
slug: "csdlc-v3-retained-proof"
title: "[v0.92.1][TAIL-06.08b][quality] Close C-SDLC v3 retained proof gaps"
labels:
  - "track:roadmap"
issue_number: 819
generated_at: "2026-09-09T22:20:00Z"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.1"
required_outcome_type:
  - "quality"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/819"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Child of #522 resolving the C-SDLC v3 slice of D520-RET-001."
pr_start:
  enabled: true
  slug: "csdlc-v3-retained-proof"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-09T22:20:00Z

# Structured Task Prompt

## Summary

Consume the exact retained-v3.json bucket from the #764 denominator once, produce real proof for each retained criterion, and reconcile the bucket to zero unresolved rows.

## Goal

Close exactly the 152 C-SDLC v3 retained-proof gaps with current candidate-bound execution proof or explicit governed disposition.

## Required Outcome

Strict positive and negative reconciliation for 152 C-SDLC v3 rows without synthetic behavioral passes or fabricated operator approval.

## Deliverables

Candidate-bound C-SDLC v3 retained-proof receipts, exact 152-row reconciliation output, validator, and review evidence.

## Acceptance Criteria

Exactly 152 retained-v3 rows are consumed once; every row has candidate-bound behavioral proof or an operator-reviewed amendment/removal; documentation and issue closure do not substitute for execution; the reconciled bucket reports zero unresolved rows with exact-head review identity.

## Repo Inputs

docs/milestones/v0.92.1/evidence/release/tail-06/issue-764/retained-proof-gap-denominator.json and the exact proof producers named by its 152 retained-v3 rows.

## Dependencies

#764 denominator packet is present; parent #522 owns remediation integration.

## Target Files / Surfaces

C-SDLC v3 proof producers named by the 152-row retained-v3 denominator and narrow issue-819 reconciliation/proof artifacts.

## Validation Plan

Validate exact 152-row partition and uniqueness, execute each real proof family at the candidate, validate evidence digests and zero unresolved reconciliation, then run diff hygiene.

## Demo Expectations

Replay each retained proof family through its real producer; no presentation-only demo.

## Non-goals

No corporate Runtime, distributed Runtime, or TAIL-01 rows; no relabeling documentation, ownership, or historical closure as behavioral proof.

## Issue-Graph Notes

Exactly the 152 retained-v3 rows; other retained-proof buckets remain out of scope.

## Notes

Fail closed on missing, duplicate, stale-candidate, documentation-only, or synthetic pass evidence.

## Tooling Notes

Native C-SDLC v3 lifecycle; FastWork issue worktree; exact-head independent review.
