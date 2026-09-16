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
card_status: "reviewed"
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
  findings_status: "addressed"
  recommended_outcome: "pass"
notes: "Review verified runtime-live-12 through installed #855 owners: actual provider loss, fresh-PID recovery and client interruption in one Runtime incarnation; checkpoint/removal and six retained raw artifact hashes; coherent JSON without artifacts is rejected. Standalone live-run-09 remains the real timeout authority."
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

- The external review found that the original packet bypassed the registered Runtime lifecycle, then found the first portable validator could accept a coherent synthetic report. Both findings were repaired. Independent exact-head review at 2930f13da6a48331438bf81f046a9e37a925e973 reported no actionable findings.

### Dispositions

- Pass for refreshed draft publication. The registered Runtime proof and raw-artifact-bound validator address both P2 findings; CI, merge and closeout remain pending.

### Recommended Outcome

- pass

## Notes

Review verified runtime-live-12 through installed #855 owners: actual provider loss, fresh-PID recovery and client interruption in one Runtime incarnation; checkpoint/removal and six retained raw artifact hashes; coherent JSON without artifacts is rejected. Standalone live-run-09 remains the real timeout authority.
