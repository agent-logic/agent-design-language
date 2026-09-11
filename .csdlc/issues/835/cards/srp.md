---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "release-gate-projection-review-prompt"
issue: 835
task_id: "issue-0835"
version: "1.0.5"
title: "Recompute v0.92.1 release-gate projection"
branch: "codex/835-release-gate-projection"
generated_at: "2026-09-11"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/835"
  - kind: "stp"
    ref: ".csdlc/issues/835/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/835/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/835/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/835/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/835/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/835/cards/stp.md"
  - ".csdlc/issues/835/cards/sip.md"
  - ".csdlc/issues/835/cards/vpp.md"
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
  - ".csdlc/issues/835/cards/stp.md"
  - ".csdlc/issues/835/cards/sip.md"
  - ".csdlc/issues/835/cards/vpp.md"
review_results:
  findings_status: "no_findings"
  recommended_outcome: "pass"
notes: "review_835 independently checked393 rows/143 removals/14 negative cases. Final source review at f5bf3cce7d84f5ad72fc9a3e213728338c369f0b; only truthful card recording follows. No Rust/cloud tests run."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/835/cards/stp.md
- .csdlc/issues/835/cards/sip.md
- .csdlc/issues/835/cards/vpp.md

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

- Two P2: evidence-map links/anchors lost; all27 stage rows assigned ceremony owner526.

### Dispositions

- Both P2 findings fixed in f5bf3cce7d. Independent re-review reports no remaining actionable findings.

### Recommended Outcome

- pass

## Notes

review_835 independently checked393 rows/143 removals/14 negative cases. Final source review at f5bf3cce7d84f5ad72fc9a3e213728338c369f0b; only truthful card recording follows. No Rust/cloud tests run.
