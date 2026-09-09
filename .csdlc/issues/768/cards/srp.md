---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "tail-02-candidate-safe-reproduction-review-prompt"
issue: 768
task_id: "issue-0768"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06.12][docs] Make the TAIL-02 reproduction route candidate-safe"
branch: "codex/768-tail-02-candidate-safe-reproduction"
generated_at: "<timestamp>"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/768"
  - kind: "stp"
    ref: ".csdlc/issues/768/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/768/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/768/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/768/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/768/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/768/cards/stp.md"
  - ".csdlc/issues/768/cards/sip.md"
  - ".csdlc/issues/768/cards/vpp.md"
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
  - ".csdlc/issues/768/cards/stp.md"
  - ".csdlc/issues/768/cards/sip.md"
  - ".csdlc/issues/768/cards/vpp.md"
review_results:
  findings_status: "<findings_status>"
  recommended_outcome: "pass"
notes: "Independent reviewer /root/execute_768/review_768 reviewed exact head d1bdc791a5995dc1669510accfd63892137f853f."
---

Canonical Template Source: `docs/templates/prompts/1.0.4/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/768/cards/stp.md
- .csdlc/issues/768/cards/sip.md
- .csdlc/issues/768/cards/vpp.md

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

- Initial P1 removed nine fail-closed checks and P2 retained a stale Cargo command; both were fixed. Final exact-head review found no remaining findings.

### Dispositions

- P1 fixed by restoring all nine checks plus ten negatives. P2 fixed by classifying Cargo evidence as historical. Final review pass.

### Recommended Outcome

- pass

## Notes

Independent reviewer /root/execute_768/review_768 reviewed exact head d1bdc791a5995dc1669510accfd63892137f853f.
