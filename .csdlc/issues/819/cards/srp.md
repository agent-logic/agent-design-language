---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "csdlc-v3-retained-proof-review-prompt"
issue: 819
task_id: "issue-0819"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06.08b][quality] Close C-SDLC v3 retained proof gaps"
branch: "codex/819-csdlc-v3-retained-proof"
generated_at: "2026-09-09T22:20:00Z"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/819"
  - kind: "stp"
    ref: ".csdlc/issues/819/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/819/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/819/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/819/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/819/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/819/cards/stp.md"
  - ".csdlc/issues/819/cards/sip.md"
  - ".csdlc/issues/819/cards/vpp.md"
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
  - ".csdlc/issues/819/cards/stp.md"
  - ".csdlc/issues/819/cards/sip.md"
  - ".csdlc/issues/819/cards/vpp.md"
review_results:
  findings_status: "findings_present"
  recommended_outcome: "block"
notes: "Findings-first reviews: /root/fix_814_runtime at b91f486e019640be63400dd17da5dff31f926a1d and /root/fix_815_cloud_auth at 0e83f53acba16cb13b42a752e1dda1e5900bfe41. Both heads were clean and read-only. Current repaired head must receive a fresh independent verdict."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/819/cards/stp.md
- .csdlc/issues/819/cards/sip.md
- .csdlc/issues/819/cards/vpp.md

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

- At b91f486e019640be63400dd17da5dff31f926a1d, /root/fix_814_runtime found two P1, one P2, and one P3: broad root-cause execution promotion, fabricated generic amendment approval, vacuous empty evidence, and trailing-log whitespace. At 0e83f53acba16cb13b42a752e1dda1e5900bfe41, /root/fix_815_cloud_auth found P1 coordinated plan/receipt source-field drift and P2 unspecified amendment-or-removal proposals.

### Dispositions

- All six findings were repaired: execution remains limited to 51 source-supported rows; 101 rows now carry an exact digest-bound remove_from_v0.92.1_retained_release_gate proposal with no product-behavior claim; source assessment, root cause, rationale, and proof boundary bind directly to the canonical source mapping; coordinated plan/receipt negatives enforce them; evidence is nonempty/exact; logs are normalized. Fresh exact-head rereview remains required.

### Recommended Outcome

- block

## Notes

Findings-first reviews: /root/fix_814_runtime at b91f486e019640be63400dd17da5dff31f926a1d and /root/fix_815_cloud_auth at 0e83f53acba16cb13b42a752e1dda1e5900bfe41. Both heads were clean and read-only. Current repaired head must receive a fresh independent verdict.
