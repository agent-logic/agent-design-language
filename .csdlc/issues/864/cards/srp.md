---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-wp01-review-prompt"
issue: 864
task_id: "issue-0864"
version: "1.0.5"
title: "[v0.92.2][WP-01][planning] Publish and open the CodeFriend Beta 1 execution wave"
branch: "codex/864-v0922-wp01"
generated_at: "2026-09-11T21:51:25.506803+00:00"
card_status: "completed"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/864"
  - kind: "stp"
    ref: ".csdlc/issues/864/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/864/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/864/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/864/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/864/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/864/cards/stp.md"
  - ".csdlc/issues/864/cards/sip.md"
  - ".csdlc/issues/864/cards/vpp.md"
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
  - ".csdlc/issues/864/cards/stp.md"
  - ".csdlc/issues/864/cards/sip.md"
  - ".csdlc/issues/864/cards/vpp.md"
review_results:
  findings_status: "no_findings"
  recommended_outcome: "pass"
notes: "Reviewed implementation revision: 9de70e4e064076487379a11a98af087fa8ba484c. Reviewer: /root/task_contract_review. 89 negative fixtures and diff hygiene passed. This subsequent card-only recording commit does not claim its own self-referential SHA; a separate exact-tip review and typed receipt bind the published head. No merge, release approval or child creation is implied."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/864/cards/stp.md
- .csdlc/issues/864/cards/sip.md
- .csdlc/issues/864/cards/vpp.md

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

- /root/task_contract_review completed independent implementation review at 9de70e4e064076487379a11a98af087fa8ba484c: PASS, no actionable findings. Operator FAIL at 3fc16781e3779bca99866693aa37169d385a0233 superseded the earlier PASS at that revision and is retained as the source of this remediation.

### Dispositions

- P1 corrected: TAIL-10 explicitly depends on OBS-S3 and ARCH-ADR and requires their own acceptance completion; 14 added negative cases reject weakened edges, criteria and projections. P2 corrected: this native-rendered SRP records the completed review, reviewer and exact implementation revision with no_findings/pass; the obsolete pending state is removed.

### Recommended Outcome

- pass

## Notes

Reviewed implementation revision: 9de70e4e064076487379a11a98af087fa8ba484c. Reviewer: /root/task_contract_review. 89 negative fixtures and diff hygiene passed. This subsequent card-only recording commit does not claim its own self-referential SHA; a separate exact-tip review and typed receipt bind the published head. No merge, release approval or child creation is implied.
