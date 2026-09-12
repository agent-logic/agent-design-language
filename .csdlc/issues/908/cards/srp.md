---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "908-aws-inventory-review-prompt"
issue: 908
task_id: "issue-0908"
version: "v0.92.2"
title: "[v0.92.2][OPS-AWS] Produce one current AWS inventory packet from the #484 baseline"
branch: "codex/908-aws-inventory"
generated_at: "2026-09-12T00:15:00.514339+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/908"
  - kind: "stp"
    ref: ".csdlc/issues/908/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/908/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/908/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/908/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/908/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/908/cards/stp.md"
  - ".csdlc/issues/908/cards/sip.md"
  - ".csdlc/issues/908/cards/vpp.md"
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
  - ".csdlc/issues/908/cards/stp.md"
  - ".csdlc/issues/908/cards/sip.md"
  - ".csdlc/issues/908/cards/vpp.md"
review_results:
  findings_status: "no_actionable_findings"
  recommended_outcome: "publish after final metadata-delta review"
notes: "Reviewer inspected actual 157-surface/13-bucket packet, recomputed five comparable deltas, verified 146 unchanged baseline hashes, zero census failures, five NoSuchTagSet and complete object listing bounds; independently ran freshness validation and 13 tests. Ownership uncertainty and no-deletion limits explicit."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/908/cards/stp.md
- .csdlc/issues/908/cards/sip.md
- .csdlc/issues/908/cards/vpp.md

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

- Independent subagent sprint8_909 found no actionable findings at 44a03fa29287e9123e74d4f877244acedea2640b

### Dispositions

- Pre-review CloudFront shape finding corrected with Items-list guard and negative test. Final review found no actionable findings.

### Recommended Outcome

- publish after final metadata-delta review

## Notes

Reviewer inspected actual 157-surface/13-bucket packet, recomputed five comparable deltas, verified 146 unchanged baseline hashes, zero census failures, five NoSuchTagSet and complete object listing bounds; independently ran freshness validation and 13 tests. Ownership uncertainty and no-deletion limits explicit.
