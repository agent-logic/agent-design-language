---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "release-truth-refresh-review-prompt"
issue: 817
task_id: "issue-0817"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06.12][release] Refresh candidate proof and canonical release truth"
branch: "codex/817-release-truth-refresh"
generated_at: "2026-09-09T22:00:00Z"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/817"
  - kind: "stp"
    ref: ".csdlc/issues/817/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/817/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/817/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/817/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/817/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/817/cards/stp.md"
  - ".csdlc/issues/817/cards/sip.md"
  - ".csdlc/issues/817/cards/vpp.md"
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
  - ".csdlc/issues/817/cards/stp.md"
  - ".csdlc/issues/817/cards/sip.md"
  - ".csdlc/issues/817/cards/vpp.md"
review_results:
  findings_status: "v3f_resolved_pending_final_issue_review"
  recommended_outcome: "pending"
notes: "Independent exact-source V3-F rereview by codex:/root/fix_816_validation PASS at c0a9edef5b2d8d755d4b56d1b5456058b13db1d9 with 163 paths reviewed and no remaining findings. Final whole-issue exact-head review remains pending."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/817/cards/stp.md
- .csdlc/issues/817/cards/sip.md
- .csdlc/issues/817/cards/vpp.md

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

- V3-F review found P2 stale active-template-set guidance, P3 stale compatibility-set status, and P2 a nonexistent historical rendering implication in docs/templates/prompts/README.md.

### Dispositions

- All three findings fixed. README now names 1.0.5 as active, 1.0.0 through 1.0.4 as historical, and historical sets as immutable audit inputs with schema-only metadata checks and no current rendering route.

### Recommended Outcome

- pending

## Notes

Independent exact-source V3-F rereview by codex:/root/fix_816_validation PASS at c0a9edef5b2d8d755d4b56d1b5456058b13db1d9 with 163 paths reviewed and no remaining findings. Final whole-issue exact-head review remains pending.
