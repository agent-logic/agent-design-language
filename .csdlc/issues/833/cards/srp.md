---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "external-review-immutable-candidate-review-prompt"
issue: 833
task_id: "issue-0833"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06.18][review] Re-run external review at immutable candidate"
branch: "codex/833-external-review-immutable-candidate"
generated_at: "2026-09-11T17:24:00Z"
card_status: "changes_requested"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/833"
  - kind: "stp"
    ref: ".csdlc/issues/833/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/833/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/833/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/833/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/833/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/833/cards/stp.md"
  - ".csdlc/issues/833/cards/sip.md"
  - ".csdlc/issues/833/cards/vpp.md"
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
  - ".csdlc/issues/833/cards/stp.md"
  - ".csdlc/issues/833/cards/sip.md"
  - ".csdlc/issues/833/cards/vpp.md"
review_results:
  findings_status: "two_publication_truth_findings_resolved_pending_fresh_review"
  recommended_outcome: "fresh_exact_head_review_required_do_not_close_833"
notes: "Reviewer: subagent /root/review_833_release_fixes. Reviewed revision: 2dfd01343816beb5cac6df2f644141be973f6fc2. Scope otherwise passed: no product code or #856-owned version/ceremony files remain; all focused validators and diff hygiene passed. #522 and #833 remain open."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/833/cards/stp.md
- .csdlc/issues/833/cards/sip.md
- .csdlc/issues/833/cards/vpp.md

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

- Independent exact-head review of 2dfd01343816beb5cac6df2f644141be973f6fc2 found P1: PR #853 incorrectly used Closes #833 despite unresolved #821 and #856 blockers; P2: SRP and SOR were stale and did not record current validation or review truth.

### Dispositions

- P1 resolved through authenticated native-v3 github-pr mutation: PR #853 now uses Part of #833 and explicitly does not auto-close #833. P2 resolved by this typed card update. A fresh exact-head review is required after committing these lifecycle changes.

### Recommended Outcome

- fresh_exact_head_review_required_do_not_close_833

## Notes

Reviewer: subagent /root/review_833_release_fixes. Reviewed revision: 2dfd01343816beb5cac6df2f644141be973f6fc2. Scope otherwise passed: no product code or #856-owned version/ceremony files remain; all focused validators and diff hygiene passed. #522 and #833 remain open.
