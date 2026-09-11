---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-closeout-plan-review-prompt"
issue: 524
task_id: "issue-0524"
version: "1.0.5"
title: "[v0.92.1][TAIL-08] Next-milestone closeout plan"
branch: "codex/524-v0922-closeout-plan"
generated_at: "2026-09-11T02:25:31Z"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/524"
  - kind: "stp"
    ref: ".csdlc/issues/524/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/524/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/524/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/524/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/524/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/524/cards/stp.md"
  - ".csdlc/issues/524/cards/sip.md"
  - ".csdlc/issues/524/cards/vpp.md"
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
  - ".csdlc/issues/524/cards/stp.md"
  - ".csdlc/issues/524/cards/sip.md"
  - ".csdlc/issues/524/cards/vpp.md"
review_results:
  findings_status: "pending"
  recommended_outcome: "pending"
notes: "Earlier review found denominator and placeholder-card defects; those fixes require fresh exact-head verification."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/524/cards/stp.md
- .csdlc/issues/524/cards/sip.md
- .csdlc/issues/524/cards/vpp.md

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

- Prior reviews found implicit denominator, stale claims, placeholder lifecycle truth, bundled transition work, and PAIR vocabulary drift. The branch now has a 43-row denominator, split qualification/operations/pilot results, current predecessor truth, and corrected PAIR terminology; fresh exact-head review remains pending.

### Dispositions

- All prior actionable findings were repaired in the issue branch; no pass is claimed until fresh independent review covers the resulting exact head.

### Recommended Outcome

- pending

## Notes

Earlier review found denominator and placeholder-card defects; those fixes require fresh exact-head verification.
