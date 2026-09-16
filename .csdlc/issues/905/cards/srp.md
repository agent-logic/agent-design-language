---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-speculative-decoding-retest-review-prompt"
issue: 905
task_id: "issue-0905"
version: "0.92.2"
title: "[v0.92.2][SPEC-RETEST] Speculative-decoding requalification"
branch: "codex/905-v0922-speculative-decoding-retest"
generated_at: "2026-09-12T00:18:14.558347+00:00"
card_status: "draft"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/905"
  - kind: "stp"
    ref: ".csdlc/issues/905/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/905/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/905/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/905/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/905/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/905/cards/stp.md"
  - ".csdlc/issues/905/cards/sip.md"
  - ".csdlc/issues/905/cards/vpp.md"
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
  - ".csdlc/issues/905/cards/stp.md"
  - ".csdlc/issues/905/cards/sip.md"
  - ".csdlc/issues/905/cards/vpp.md"
review_results:
  findings_status: "findings_resolved_re_review_pending"
  recommended_outcome: "block_pending_re_review"
notes: "Both reviewers accepted the retained eight exact pairs and repair_inconclusive disposition without a hardware rerun. Remediation affects harness ownership and failure safety only; measured evidence is unchanged. Publication remains blocked until renewed exact-head review passes."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/905/cards/stp.md
- .csdlc/issues/905/cards/sip.md
- .csdlc/issues/905/cards/vpp.md

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

- Initial review of PR #1004 at e3159b6e5ba225e07d9bc8110ed655939ffe0a04 found two P2s. Follow-up review at b7b73d8a6855abfee47ba0c89f33d1add782e1e6 found a residual alias check/use race and cleanup paths that could still suppress report.json, including nonpositive repeats.

### Dispositions

- Both residual P2s are repaired. Requested aliases are transformed into run-unique names with a cryptographically random 128-bit suffix before collision checks and creation; cleanup removes only successfully created run-scoped names. Validation now occurs inside the reporting scope, guardian kill errors are captured, and unexpected cleanup errors are serialized before report.json is written. Nineteen focused tests pass; exact-head re-review is pending.

### Recommended Outcome

- block_pending_re_review

## Notes

Both reviewers accepted the retained eight exact pairs and repair_inconclusive disposition without a hardware rerun. Remediation affects harness ownership and failure safety only; measured evidence is unchanged. Publication remains blocked until renewed exact-head review passes.
