---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "external-review-immutable-candidate-review-prompt"
issue: 833
task_id: "issue-0833"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06.18][review] Re-run external review at immutable candidate"
branch: "codex/833-external-review-immutable-candidate"
generated_at: "2026-09-11T17:24:00Z"
card_status: "ready"
status: "needs_followup"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/833"
  - kind: "stp"
    ref: ".csdlc/issues/833/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/833/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/833/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/833/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/833/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/833/cards/stp.md"
  - ".csdlc/issues/833/cards/sip.md"
  - ".csdlc/issues/833/cards/vpp.md"
in_scope_surfaces:
  - "tracked changes for this issue branch"
evidence_policy:
  - "Use repository evidence, targeted validation output, and linked issue-bundle artifacts only."
validation_inputs:
  - "Issue-local proofs recorded in the VPP and SOR."
allowed_dispositions:
  - "PASS"
  - "BLOCK"
  - "NEEDS_FOLLOWUP"
reviewer_constraints:
  - "Do not widen issue scope."
  - "Do not merge, publish, or close the issue."
refusal_policy:
  - "Refuse claims that are unsupported by repository evidence."
  - "Refuse approving behavior outside the recorded issue scope."
follow_up_routing:
  - "Route actionable defects back to the issue branch before PR publication."
non_claims:
  - "This prompt does not claim review has already run."
  - "This prompt does not guarantee review quality by itself."
policy_refs:
  - ".csdlc/issues/833/cards/stp.md"
  - ".csdlc/issues/833/cards/sip.md"
  - ".csdlc/issues/833/cards/vpp.md"
review_results:
  findings_status: "findings_present"
  recommended_outcome: "needs_followup"
notes: "Review at bb33c5a found publication-synchronization findings: push PR #853 to the reviewed head, update the PR body to close #833 while leaving #522 open, and retain this SRP finding record before final live-state review."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent review for this issue. The review record below captures the current publication-synchronization findings; final PR settlement still requires a live review after the branch and PR body are updated.

## Scope Basis

- .csdlc/issues/833/cards/stp.md
- .csdlc/issues/833/cards/sip.md
- .csdlc/issues/833/cards/vpp.md

## In-Scope Surfaces

- tracked changes for this issue branch

## Evidence Rules

- Use repository evidence, targeted validation output, and linked issue-bundle artifacts only.

## Validation Inputs

- Issue-local proofs recorded in the VPP and SOR.

## Allowed Dispositions

- PASS
- BLOCK
- NEEDS_FOLLOWUP

## Reviewer Constraints

- Do not widen issue scope.
- Do not merge, publish, or close the issue.

## Refusal Policy

- Refuse claims that are unsupported by repository evidence.
- Refuse approving behavior outside the recorded issue scope.

## Follow-up Routing

- Route actionable defects back to the issue branch before PR publication.

## Non-Claims

- This prompt does not claim review has already run.
- This prompt does not guarantee review quality by itself.

## Review Results

### Findings

- P1 — PR #853 was not yet at the reviewed local head `bb33c5a748cd971d8d12337aa845c0f5c910c4a0`; live GitHub still pointed to `3edc76dbda55169d0d229f41bbec150ac8bea374`, so live PR checks and PR state did not yet cover the truth-repair commits.
- P1 — The live PR body still described the earlier phase-one boundary and said it did not close #833, while the issue-local SOR/remediation truth now supports publication that closes #833 and explicitly leaves #522 open.
- P2 — This SRP still contained pending-review placeholders at the reviewed head.

### Dispositions

- P1 PR-head drift: to be resolved by pushing `codex/833-external-review-immutable-candidate` to the reviewed head and rechecking live PR #853.
- P1 PR-body drift: to be resolved through the typed v3 `github-pr` route after the branch is pushed, with `Closes #833` and no `Closes #522`.
- P2 SRP placeholder drift: resolved by this SRP truth update; final live-state review remains required after publication synchronization.

### Recommended Outcome

- needs_followup

## Notes

The failed external report remains retained as non-proving evidence. The executable addendum and #818 supersession proof remain bounded support for #833 publication only; they do not approve the release and do not close #522.
