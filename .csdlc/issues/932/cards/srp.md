---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "932-sprint6-coordination-review-prompt"
issue: 932
task_id: "issue-0932"
version: "1.0.5"
title: "[v0.92.2][Sprint 6] Hardware/provider qualification"
branch: "codex/932-sprint6-coordination"
generated_at: "2026-09-15"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/932"
  - kind: "stp"
    ref: ".csdlc/issues/932/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/932/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/932/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/932/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/932/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/932/cards/stp.md"
  - ".csdlc/issues/932/cards/sip.md"
  - ".csdlc/issues/932/cards/vpp.md"
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
  - ".csdlc/issues/932/cards/stp.md"
  - ".csdlc/issues/932/cards/sip.md"
  - ".csdlc/issues/932/cards/vpp.md"
review_results:
  findings_status: "no_actionable_findings"
  recommended_outcome: "pass"
notes: "Independent reviewer subagent:review_905 verified exact roster, 12-criterion mapping, child closure, merged green PRs, ancestry, terminal receipts, worktree absence, retained limits, normalized cards, JSON and diff hygiene."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/932/cards/stp.md
- .csdlc/issues/932/cards/sip.md
- .csdlc/issues/932/cards/vpp.md

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

- Initial P2: the umbrella packet lacked durable final-head #905 review proof. Fixed by carrying the exact-head review and typed receipt into .csdlc/evidence/932; independent re-review found no actionable findings.

### Dispositions

- P2 resolved at exact head 8b1721db80bd19c877feaa6e42d7ad92104516a7. Review artifact BLAKE3 equals receipt evidence_digest, and receipt revision equals merged PR #1004 head. No remaining actionable finding.

### Recommended Outcome

- pass

## Notes

Independent reviewer subagent:review_905 verified exact roster, 12-criterion mapping, child closure, merged green PRs, ancestry, terminal receipts, worktree absence, retained limits, normalized cards, JSON and diff hygiene.
