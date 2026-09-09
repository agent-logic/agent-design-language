---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "tail-04-internal-review-review-prompt"
issue: 520
task_id: "issue-0520"
version: "1.0.5"
title: "[v0.92.1][TAIL-04] Internal review"
branch: "codex/520-internal-review"
generated_at: "2026-09-09T19:19:55Z"
card_status: "exact_head_review_findings_in_repair"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/520"
  - kind: "stp"
    ref: ".csdlc/issues/520/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/520/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/520/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/520/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/520/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/520/cards/stp.md"
  - ".csdlc/issues/520/cards/sip.md"
  - ".csdlc/issues/520/cards/vpp.md"
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
  - ".csdlc/issues/520/cards/stp.md"
  - ".csdlc/issues/520/cards/sip.md"
  - ".csdlc/issues/520/cards/vpp.md"
review_results:
  findings_status: "six_packet_findings_open_in_addition_to_fourteen_product_findings"
  recommended_outcome: "changes_required_do_not_publish"
notes: "Reviewer: subagent:/root/fix_814_runtime. Reviewed revision: 48a2003d6ca7ea0e785f58990c18c82c846f71c9. No publication or pass is claimed. The 14 product findings remain valid candidate defects but their complete denominator review evidence must be rebuilt truthfully."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/520/cards/stp.md
- .csdlc/issues/520/cards/sip.md
- .csdlc/issues/520/cards/vpp.md

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

- Independent exact-head review of 48a2003d6ca7ea0e785f58990c18c82c846f71c9 found six packet defects: two P1 and three P2 methodology/proof defects plus one P3 lifecycle-truth defect. The assembler manufactured lane observations; acceptance rows were non-terminal; summaries contradicted findings; test proof covered one unrelated command; locators were not line-bound; and review-plan/assignment state was stale.

### Dispositions

- All six packet findings are accepted. The assembler now requires completed independent specialist inputs, the validator requires terminal acceptance dispositions and bounded locators, summary outcomes derive from findings, test proof requires multiple declared invocations, and lifecycle truth is being refreshed. A fresh exact-head review is mandatory after all replacement evidence is assembled.

### Recommended Outcome

- changes_required_do_not_publish

## Notes

Reviewer: subagent:/root/fix_814_runtime. Reviewed revision: 48a2003d6ca7ea0e785f58990c18c82c846f71c9. No publication or pass is claimed. The 14 product findings remain valid candidate defects but their complete denominator review evidence must be rebuilt truthfully.
