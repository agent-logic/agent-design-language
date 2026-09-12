---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "sprint-8-cloud-operations-observatory-review-prompt"
issue: 934
task_id: "issue-0934"
version: "v0.92.2"
title: "[v0.92.2][Sprint 8] Cloud operations and Observatory"
branch: "codex/934-sprint8-coordination"
generated_at: "2026-09-12"
card_status: "in_progress"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/934"
  - kind: "stp"
    ref: ".csdlc/issues/934/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/934/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/934/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/934/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/934/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/934/cards/stp.md"
  - ".csdlc/issues/934/cards/sip.md"
  - ".csdlc/issues/934/cards/vpp.md"
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
  - ".csdlc/issues/934/cards/stp.md"
  - ".csdlc/issues/934/cards/sip.md"
  - ".csdlc/issues/934/cards/vpp.md"
review_results:
  findings_status: "addressed"
  recommended_outcome: "Approve reviewed four-child acceptance packet for umbrella publication; final umbrella CI/merge/terminal steps remain."
notes: "See .csdlc/evidence/934/review/COMBINED_SPRINT_ACCEPTANCE.md and DEPLOYED_CHILD_910_REVIEW.md. Local child tests, actual cloud/browser observations, CI selection and merge/terminal receipts remain distinct proof surfaces."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/934/cards/stp.md
- .csdlc/issues/934/cards/sip.md
- .csdlc/issues/934/cards/vpp.md

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

- Two P2 record findings corrected: stale #910 pending fields and final-head CI association. All four children are reviewed, green, merged and natively closed out; all-four ancestry verified.

### Dispositions

- Source, browser, invalidation and metadata findings resolved. Final #910 CI is run34674381718 at35dd207861; earlier run34674270192 belongs to28fbf649.

### Recommended Outcome

- Approve reviewed four-child acceptance packet for umbrella publication; final umbrella CI/merge/terminal steps remain.

## Notes

See .csdlc/evidence/934/review/COMBINED_SPRINT_ACCEPTANCE.md and DEPLOYED_CHILD_910_REVIEW.md. Local child tests, actual cloud/browser observations, CI selection and merge/terminal receipts remain distinct proof surfaces.
