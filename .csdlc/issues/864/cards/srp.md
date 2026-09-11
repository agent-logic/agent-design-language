---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-wp01-review-prompt"
issue: 864
task_id: "issue-0864"
version: "1.0.5"
title: "[v0.92.2][WP-01][planning] Publish and open the CodeFriend Beta 1 execution wave"
branch: "codex/864-v0922-wp01"
generated_at: "2026-09-11T21:26:03Z"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/864"
  - kind: "stp"
    ref: ".csdlc/issues/864/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/864/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/864/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/864/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/864/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/864/cards/stp.md"
  - ".csdlc/issues/864/cards/sip.md"
  - ".csdlc/issues/864/cards/vpp.md"
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
  - ".csdlc/issues/864/cards/stp.md"
  - ".csdlc/issues/864/cards/sip.md"
  - ".csdlc/issues/864/cards/vpp.md"
review_results:
  findings_status: "pass_no_actionable_findings"
  recommended_outcome: "publish_for_operator_review"
notes: "All 42 prospective rows remain number-free and no child issues were created. Publication grants review visibility only, not child creation, merge, or release authority."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/864/cards/stp.md
- .csdlc/issues/864/cards/sip.md
- .csdlc/issues/864/cards/vpp.md

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

- Independent rereview passed with no actionable findings after native v3 card normalization.

### Dispositions

- The initial lifecycle-card placeholder finding is fixed; planning arithmetic, dependencies, exact existing bindings, decision-only ARCH-SPLIT scope, and the canonical closeout tail were independently verified.

### Recommended Outcome

- publish_for_operator_review

## Notes

All 42 prospective rows remain number-free and no child issues were created. Publication grants review visibility only, not child creation, merge, or release authority.
