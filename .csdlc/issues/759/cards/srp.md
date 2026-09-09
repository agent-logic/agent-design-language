---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "dynamic-agent-health-task-failures-review-prompt"
issue: 759
task_id: "issue-0759"
version: "1.0.4"
title: "[v0.92.1][TAIL-06.03][runtime] Isolate dynamic-agent health task failures"
branch: "codex/759-dynamic-agent-health-task-failures"
generated_at: "<timestamp>"
card_status: "completed"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/759"
  - kind: "stp"
    ref: ".csdlc/issues/759/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/759/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/759/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/759/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/759/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/759/cards/stp.md"
  - ".csdlc/issues/759/cards/sip.md"
  - ".csdlc/issues/759/cards/vpp.md"
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
  - ".csdlc/issues/759/cards/stp.md"
  - ".csdlc/issues/759/cards/sip.md"
  - ".csdlc/issues/759/cards/vpp.md"
review_results:
  findings_status: "findings_present"
  recommended_outcome: "block"
notes: "Pre-PR review /root/review_759_381612a2 made no source mutations and found no Runtime source behavior issue. Runtime source behavior, r16 evidence hashes, and #804 routing for the stale active SOR template authority notice were non-findings. The P2 card-truth finding has been remediated locally; the later current-main merge changed no #759 runtime-scoped source/test paths and requires a new exact-head review before publication."
---

Canonical Template Source: `docs/templates/prompts/1.0.4/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/759/cards/stp.md
- .csdlc/issues/759/cards/sip.md
- .csdlc/issues/759/cards/vpp.md

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

When finalizing review, record the machine-readable review result in frontmatter:

```yaml
review_results:
  findings_status: "no_findings | findings_present | review_unavailable | review_timeout | review_cancelled | review_failed"
  recommended_outcome: "pass | block | needs_followup"
```

### Findings

- FAIL P2 — /root/review_759_381612a2 reviewed exact HEAD 381612a2b85f20edd5e5c416b9235dd7725fb7d2 and found stale SRP/SOR truth: SRP still named prior exact HEAD ab7bb7b220c571775763725629421391b4bc8f17, SOR still named stale source SHA 23a5b82e6a353eaf4963357f72b536939430cab5 in integration evidence, and SOR still listed validation-r15.md as the primary proof surface instead of validation-r16.md.

### Dispositions

- Remediated in lifecycle generation 7 by rebinding SRP/SOR truth to the r16 validation package and replacing stale prior-head/r15 proof references. Publication remains blocked until a fresh independent exact-head review passes on the repaired card truth.

### Recommended Outcome

- block

## Notes

Pre-PR review /root/review_759_381612a2 made no source mutations and found no Runtime source behavior issue. Runtime source behavior, r16 evidence hashes, and #804 routing for the stale active SOR template authority notice were non-findings. The P2 card-truth finding has been remediated locally; the later current-main merge changed no #759 runtime-scoped source/test paths and requires a new exact-head review before publication.
