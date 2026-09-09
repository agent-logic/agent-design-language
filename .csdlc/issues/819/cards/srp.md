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
  findings_status: "no_findings"
  recommended_outcome: "pass"
notes: "Findings-first reviews: /root/fix_814_runtime at b91f486e019640be63400dd17da5dff31f926a1d and /root/fix_815_cloud_auth at 0e83f53acba16cb13b42a752e1dda1e5900bfe41 and cd4e93ac186ff9ad9679cc57123759d47f21b163. Final independent rereview by /root/fix_815_cloud_auth at clean immutable 2e315ddcd7b410457544f2b843f93ea8e058cf18 passed with no actionable findings. Operator approval of the 101 exact removal proposals remains a separate merge-time release gate."
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

- At b91f486e019640be63400dd17da5dff31f926a1d, /root/fix_814_runtime found two P1, one P2, and one P3: broad root-cause execution promotion, fabricated generic amendment approval, vacuous empty evidence, and trailing-log whitespace. At 0e83f53acba16cb13b42a752e1dda1e5900bfe41, /root/fix_815_cloud_auth found P1 coordinated plan/receipt source-field drift and P2 unspecified amendment-or-removal proposals. At cd4e93ac186ff9ad9679cc57123759d47f21b163, /root/fix_815_cloud_auth found one P1: source and denominator pointers/bytes were not bound to the canonical frozen candidate and row owner was not bound to the denominator.

### Dispositions

- All seven findings were repaired: execution remains limited to 51 source-supported rows; 101 rows carry an exact digest-bound remove_from_v0.92.1_retained_release_gate proposal with no product-behavior claim; semantic fields bind to canonical source mapping bytes loaded from exact candidate fb6cbc7f619daa54f901fd2d12f480add682ace3; denominator and source pointers are fixed and their working bytes must equal candidate bytes; owner binds to the candidate denominator; forged-pointer, owner-drift, coordinated plan/receipt, and empty-evidence negatives enforce fail-closed behavior; logs are normalized. /root/fix_815_cloud_auth independently rereviewed clean exact head 2e315ddcd7b410457544f2b843f93ea8e058cf18 and returned PASS with no P0-P3 findings.

### Recommended Outcome

- pass

## Notes

Findings-first reviews: /root/fix_814_runtime at b91f486e019640be63400dd17da5dff31f926a1d and /root/fix_815_cloud_auth at 0e83f53acba16cb13b42a752e1dda1e5900bfe41 and cd4e93ac186ff9ad9679cc57123759d47f21b163. Final independent rereview by /root/fix_815_cloud_auth at clean immutable 2e315ddcd7b410457544f2b843f93ea8e058cf18 passed with no actionable findings. Operator approval of the 101 exact removal proposals remains a separate merge-time release gate.
