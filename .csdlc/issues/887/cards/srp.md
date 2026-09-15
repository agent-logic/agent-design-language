---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-codefriend-local-fitness-review-prompt"
issue: 887
task_id: "issue-0887"
version: "0.92.2"
title: "[v0.92.2][CF-GOV] Execute local architecture fitness functions"
branch: "not bound yet; proposed codex/887-v0922-codefriend-local-fitness"
generated_at: "2026-09-12T00:05:16.416878+00:00"
card_status: "draft"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/887"
  - kind: "stp"
    ref: ".csdlc/issues/887/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/887/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/887/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/887/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/887/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/887/cards/stp.md"
  - ".csdlc/issues/887/cards/sip.md"
  - ".csdlc/issues/887/cards/vpp.md"
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
  - ".csdlc/issues/887/cards/stp.md"
  - ".csdlc/issues/887/cards/sip.md"
  - ".csdlc/issues/887/cards/vpp.md"
review_results:
  findings_status: "passed"
  recommended_outcome: "publish_after_final_record_head_review"
notes: "Reviewer independently verified source/binary hashes, 8 fitness tests, 11 evidence regressions, strict Clippy, 3 installed scenarios, and library/CLI coverage. CI integration belongs to #888."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/887/cards/stp.md
- .csdlc/issues/887/cards/sip.md
- .csdlc/issues/887/cards/vpp.md

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

- Independent reviewer /root/sprint3_preparation_review PASS at 6e14e86bfb4df7d37d14227dff3f0c0bea4e921b; no actionable findings. Original raw identifier finding fixed and tested.

### Dispositions

- Raw identifier bypass fixed with normalized segments and three regression cases. Exact-head product/proof review passed. Final record/doc-only commit will be re-reviewed before native publication.

### Recommended Outcome

- publish_after_final_record_head_review

## Notes

Reviewer independently verified source/binary hashes, 8 fitness tests, 11 evidence regressions, strict Clippy, 3 installed scenarios, and library/CLI coverage. CI integration belongs to #888.
