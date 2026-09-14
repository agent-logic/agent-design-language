---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-provider-recovery-qualification-review-prompt"
issue: 901
task_id: "issue-0901"
version: "0.92.2"
title: "[v0.92.2][QUAL-PROVIDER] Execute real provider failure and recovery qualification"
branch: "codex/901-v0922-provider-recovery-qualification"
generated_at: "2026-09-14T18:06:04.530196+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/901"
  - kind: "stp"
    ref: ".csdlc/issues/901/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/901/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/901/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/901/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/901/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/901/cards/stp.md"
  - ".csdlc/issues/901/cards/sip.md"
  - ".csdlc/issues/901/cards/vpp.md"
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
  - ".csdlc/issues/901/cards/stp.md"
  - ".csdlc/issues/901/cards/sip.md"
  - ".csdlc/issues/901/cards/vpp.md"
review_results:
  findings_status: "review_refresh_pending"
  recommended_outcome: "block_pending_review"
notes: "Review the exact head and both complementary packets. Verify that standalone adapter evidence still proves timeout and adapter interruption, registered Runtime evidence proves loss and recovery with a fresh provider PID in one unchanged Runtime incarnation, client interruption is distinct, CSM registration/checkpoint/removal are real, and no private paths, prompts, generated text, credentials or paid calls enter the portable packet."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/901/cards/stp.md
- .csdlc/issues/901/cards/sip.md
- .csdlc/issues/901/cards/vpp.md

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

- The external P2 found that the original packet bypassed the registered Runtime lifecycle. Remediation adds an installed #855 Runtime harness and a passing runtime-live-09 receipt. Fresh independent exact-head review has not yet run.

### Dispositions

- P2 locally addressed; block publication readiness until exact-head independent review validates the new Runtime harness, retained timeout boundary, and portable receipt.

### Recommended Outcome

- block_pending_review

## Notes

Review the exact head and both complementary packets. Verify that standalone adapter evidence still proves timeout and adapter interruption, registered Runtime evidence proves loss and recovery with a fresh provider PID in one unchanged Runtime incarnation, client interruption is distinct, CSM registration/checkpoint/removal are real, and no private paths, prompts, generated text, credentials or paid calls enter the portable packet.
