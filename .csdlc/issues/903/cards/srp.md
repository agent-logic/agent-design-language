---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-mlx-metal-provider-review-prompt"
issue: 903
task_id: "issue-0903"
version: "0.92.2"
title: "[v0.92.2][PLAT-MLX] Bounded MLX and Apple Metal provider adapter"
branch: "codex/903-v0922-mlx-metal-provider"
generated_at: "2026-09-12T00:18:14.454312+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/903"
  - kind: "stp"
    ref: ".csdlc/issues/903/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/903/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/903/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/903/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/903/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/903/cards/stp.md"
  - ".csdlc/issues/903/cards/sip.md"
  - ".csdlc/issues/903/cards/vpp.md"
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
  - ".csdlc/issues/903/cards/stp.md"
  - ".csdlc/issues/903/cards/sip.md"
  - ".csdlc/issues/903/cards/vpp.md"
review_results:
  findings_status: "no_findings"
  recommended_outcome: "pass"
notes: "Reviewer subagent:sprint8_908; changed-path committed-byte scope digest29b75e377b5ede104a5c32ea5b453910abb5c3f9c1fa9b6e3191c259975304e0. RequiredCI pending; supplemental comparison failed/unresolved and trials paused. Final metadata-only review required on resulting publication head."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/903/cards/stp.md
- .csdlc/issues/903/cards/sip.md
- .csdlc/issues/903/cards/vpp.md

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

- Bounded publication review PASS at9e9803b66ae3c9eb1bc92955d5475001445bc423. No new source or evidence blocker. All seven source hashes and private hardware receipt match retained reviewed proof.

### Dispositions

- Prior source defects and stale SOR/SRP findings corrected. Root independently reviewed/executed reviewer-authored integration tests. Publication review covers the original bounded903adapter, not successful-review acceleration or fullCI acceptance.

### Recommended Outcome

- pass

## Notes

Reviewer subagent:sprint8_908; changed-path committed-byte scope digest29b75e377b5ede104a5c32ea5b453910abb5c3f9c1fa9b6e3191c259975304e0. RequiredCI pending; supplemental comparison failed/unresolved and trials paused. Final metadata-only review required on resulting publication head.
