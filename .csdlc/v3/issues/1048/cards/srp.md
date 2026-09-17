---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "publication-metadata-amendment-review-prompt"
issue: 1048
task_id: "issue-1048"
version: "v0.92.2"
title: "[C-SDLC] Allow typed correction of accepted publication metadata before dispatch"
branch: "codex/1048-publication-metadata-amendment"
generated_at: "2026-09-17T00:17:50.059444+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1048"
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
  recommended_outcome: "approve"
notes: "Independent /root/review_1048 reviewed exact source and documentation HEAD 8a514568145dbe5c5a7ff524e12c9e8d06074c02 against e162c69942f104a0ccb14bdea46719de3ebd531a. All actionable findings resolved; final lifecycle-only delta also requires exact-head review before publication."
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

- Legacy receipt path compatibility, completed publication no-effect guard, and missing reviewed-branch push in manual.

### Dispositions

- All three fixed and independently re-reviewed. Legacy receipt dispatch and pending publication block have passing installed regressions. Completed authenticated no-effect recovery remains statically reviewed only; pre-existing failure to settle recovery is recorded in REVIEW.md.

### Recommended Outcome

- approve

## Notes

Independent /root/review_1048 reviewed exact source and documentation HEAD 8a514568145dbe5c5a7ff524e12c9e8d06074c02 against e162c69942f104a0ccb14bdea46719de3ebd531a. All actionable findings resolved; final lifecycle-only delta also requires exact-head review before publication.
