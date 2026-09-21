---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "<slug>-review-prompt"
issue: 1109
task_id: "issue-1109"
version: "1.0.5"
title: "[v0.92.2][CF-ARCH] Generate the complete 4+1 architecture package in Beta 1"
branch: "codex/1109-codefriend-four-plus-one-architecture"
generated_at: "<timestamp>"
card_status: "completed"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1109"
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
  findings_status: "no_findings"
  recommended_outcome: "pass"
notes: "Independent reviewer /root/review_1109_core PASS at94be96e44d437c48a908fffb785c9507fe2593ae. Prior full implementation/presentation/evidence review plus worktree-local fixture/PVF repair; no actionable findings. Native proof passed35tests with unchanged inputs and no timeout/cancellation. Final record commit requires fresh exact-head proof/review before publication. Installed ADL/external demonstrations do not satisfy #915 or hosted/paired #1101. Browser visual inspection remains unclaimed."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

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

- Bounded independent source review found missing actionable gap explanations, omitted isolated-node diagrams, response decode mismatch, attachment link prefix mismatch and fenced-response publication rejection. All repaired. No actionable findings remain in repair review at 69afb828f3 or subsequent HTML/evidence delta review.

### Dispositions

- All five actionable findings fixed with regression coverage. Parent static HTML inspection additionally identified pipe tables rendered as prose; GFM table rendering and actual table-header assertions added and independently reviewed.

### Recommended Outcome

- pass

## Notes

Independent reviewer /root/review_1109_core PASS at94be96e44d437c48a908fffb785c9507fe2593ae. Prior full implementation/presentation/evidence review plus worktree-local fixture/PVF repair; no actionable findings. Native proof passed35tests with unchanged inputs and no timeout/cancellation. Final record commit requires fresh exact-head proof/review before publication. Installed ADL/external demonstrations do not satisfy #915 or hosted/paired #1101. Browser visual inspection remains unclaimed.
