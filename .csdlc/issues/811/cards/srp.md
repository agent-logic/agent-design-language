---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "all-issue-closeout-review-prompt"
issue: 811
task_id: "issue-0811"
version: "1.0.5"
title: "Reconcile every closed issue including no-PR dispositions"
branch: "codex/811-all-issue-closeout"
generated_at: "2026-09-09"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/811"
  - kind: "stp"
    ref: ".csdlc/issues/811/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/811/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/811/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/811/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/811/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/811/cards/stp.md"
  - ".csdlc/issues/811/cards/sip.md"
  - ".csdlc/issues/811/cards/vpp.md"
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
  - ".csdlc/issues/811/cards/stp.md"
  - ".csdlc/issues/811/cards/sip.md"
  - ".csdlc/issues/811/cards/vpp.md"
review_results:
  findings_status: "no_findings"
  recommended_outcome: "pass"
notes: "Independent reviewer review_811 inspected the revised source and regression tests; follow-up found no actionable findings. Review did not independently rerun tests. Final artifact-head review follows before publication."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/811/cards/stp.md
- .csdlc/issues/811/cards/sip.md
- .csdlc/issues/811/cards/vpp.md

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

- Initial P2: arbitrary context HEAD could be persisted. No remaining actionable findings after fix.

### Dispositions

- P2 fixed by actual HEAD comparison before persistence and regression coverage.

### Recommended Outcome

- pass

## Notes

Independent reviewer review_811 inspected the revised source and regression tests; follow-up found no actionable findings. Review did not independently rerun tests. Final artifact-head review follows before publication.
