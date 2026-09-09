---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "<slug>-review-prompt"
issue: 799
task_id: "issue-0799"
version: "v0.92.1"
title: "[v0.92.1][docs] Complete #767 final release-status refresh"
branch: "codex/799-final-release-status"
generated_at: "<timestamp>"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/799"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
  - kind: "spp"
    ref: "<spp_card>"
  - kind: "vpp"
    ref: "<vpp_card>"
  - kind: "sor"
    ref: "<sor_card>"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - "<stp_card>"
  - "<sip_card>"
  - "<vpp_card>"
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
  - "<stp_card>"
  - "<sip_card>"
  - "<vpp_card>"
review_results:
  findings_status: "<findings_status>"
  recommended_outcome: "<recommended_outcome>"
notes: "Review by review_767_refresh. Exact committed head will receive a final review before native publication."
---

Canonical Template Source: `docs/templates/prompts/1.0.4/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- <stp_card>
- <sip_card>
- <vpp_card>

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

- No actionable findings. Reviewer verified C01/C02 ancestry at 11a3fe88, 198/29/3 retained partition, closed-owner removal and explicit C03/C05 debt under #522.

### Dispositions

- No unresolved documentation findings; remaining product proof belongs to the named open owners.

### Recommended Outcome

- <recommended_outcome>

## Notes

Review by review_767_refresh. Exact committed head will receive a final review before native publication.
