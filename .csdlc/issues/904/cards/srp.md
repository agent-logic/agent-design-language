---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-pair-multinode-experiment-review-prompt"
issue: 904
task_id: "issue-0904"
version: "0.92.2"
title: "[v0.92.2][PLAT-PAIR] NVIDIA PAIR multi-node local-inference experiment"
branch: "codex/904-v0922-pair-multinode-experiment"
generated_at: "2026-09-12T00:18:16.017452+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/904"
  - kind: "stp"
    ref: ".csdlc/issues/904/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/904/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/904/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/904/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/904/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/904/cards/stp.md"
  - ".csdlc/issues/904/cards/sip.md"
  - ".csdlc/issues/904/cards/vpp.md"
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
  - ".csdlc/issues/904/cards/stp.md"
  - ".csdlc/issues/904/cards/sip.md"
  - ".csdlc/issues/904/cards/vpp.md"
review_results:
  findings_status: "addressed_pending_final_rereview"
  recommended_outcome: "pending"
notes: "Independent review verified the five first-round repairs at 8a60b072c. It then found stale 18-test and unknown metrics-source fields plus a synthetic exact snapshot timestamp. Evidence commit 03732162bebdb6567489b7c1aa8a0e239cac4243 and this native card edit correct those remaining findings. Final exact-head review is pending."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/904/cards/stp.md
- .csdlc/issues/904/cards/sip.md
- .csdlc/issues/904/cards/vpp.md

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

- First review: one P1 and four P2 evidence/portability/accounting/wording/resource findings. Second review: two P2 stale SOR metric fields and synthetic exact resource timestamp.

### Dispositions

- All findings addressed: tracked sidecar; required exact resource digest and regression; heterogeneous-deployment wording; bounded observed resource snapshot; current two-node lifecycle truth; 19-test count; tracked metric-source references; unknown exact snapshot time stated explicitly.

### Recommended Outcome

- pending

## Notes

Independent review verified the five first-round repairs at 8a60b072c. It then found stale 18-test and unknown metrics-source fields plus a synthetic exact snapshot timestamp. Evidence commit 03732162bebdb6567489b7c1aa8a0e239cac4243 and this native card edit correct those remaining findings. Final exact-head review is pending.
