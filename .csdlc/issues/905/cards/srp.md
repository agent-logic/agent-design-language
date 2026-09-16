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
  findings_status: "findings_resolved_final_review_pending"
  recommended_outcome: "block_pending_final_review"
notes: "review_905 reviewed d032988e8287872dc332d4aa3d9fc293e8ee0632 and f3e46f75438ea64bf46f4137e962e9a2f87ef705. The final remediation rejects aggregate-only qualification and records repair/unqualified disposition. Publication is blocked until final exact-head clearance."
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

- First review found four P2 proof/lifecycle defects; remediation review confirmed three resolved and found one remaining P2 statistical robustness defect plus a stale P3 pair count. The aggregate gain was driven by a regime shift/outliers while speculative lost end-to-end in three of four blocks.

### Dispositions

- All findings are now addressed. The harness and evidence retain per-block end-to-end/decode benefits, medians, win counts and a 3-of-4 robustness gate. The gate classifies this run repair_inconclusive, so cards no longer claim keep/high confidence. SOR pair count is eight. Final exact-head review remains pending.

### Recommended Outcome

- block_pending_final_review

## Notes

review_905 reviewed d032988e8287872dc332d4aa3d9fc293e8ee0632 and f3e46f75438ea64bf46f4137e962e9a2f87ef705. The final remediation rejects aggregate-only qualification and records repair/unqualified disposition. Publication is blocked until final exact-head clearance.
