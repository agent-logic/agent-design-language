---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-review-synthesis-review-prompt"
issue: 892
task_id: "issue-0892"
version: "0.92.2"
title: "[v0.92.2][CF-SYNTHESIS] Synthesize completed review perspectives"
branch: "codex/892-v0922-review-synthesis"
generated_at: "2026-09-12T00:09:56.378553+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/892"
  - kind: "stp"
    ref: ".csdlc/issues/892/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/892/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/892/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/892/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/892/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/892/cards/stp.md"
  - ".csdlc/issues/892/cards/sip.md"
  - ".csdlc/issues/892/cards/vpp.md"
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
  - ".csdlc/issues/892/cards/stp.md"
  - ".csdlc/issues/892/cards/sip.md"
  - ".csdlc/issues/892/cards/vpp.md"
review_results:
  findings_status: "review_r2_failed_findings_remediated_pending_fresh_review"
  recommended_outcome: "block_until_fresh_exact_head_review_passes"
notes: "Fresh independent exact-head review is required on the new immutable remediation commit before native publication. Reviewer must verify the r1 finding dispositions, the four focused synthesis tests, the actual predecessor-output artifacts including review-record.json, create-only artifact behavior, and that #892 does not absorb publication/remediation/rendering authority."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/892/cards/stp.md
- .csdlc/issues/892/cards/sip.md
- .csdlc/issues/892/cards/vpp.md

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

- R1 at 0c5e5a8f485a1d41379d78175a49f410ac1cd533 failed on false-merge disagreement handling, stale SOR truth and missing predecessor proof. R2 at 0590b692787244f789ddab29b51e74c875e3c2a0 failed on dangling predecessor proof input and stale SOR execution header.

### Dispositions

- R1 and R2 actionable findings are remediated in implementation/lifecycle truth: distinct claim variants are explicit disagreement, actual accepted #890 predecessor output is self-contained in committed evidence, and the SOR execution header no longer claims unassigned/not_started execution. No findings waived. Fresh independent exact-head review remains required.

### Recommended Outcome

- block_until_fresh_exact_head_review_passes

## Notes

Fresh independent exact-head review is required on the new immutable remediation commit before native publication. Reviewer must verify the r1 finding dispositions, the four focused synthesis tests, the actual predecessor-output artifacts including review-record.json, create-only artifact behavior, and that #892 does not absorb publication/remediation/rendering authority.
