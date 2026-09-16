---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "960-shutdown-barrier-review-prompt"
issue: 960
task_id: "issue-0960"
version: "1.0.5"
title: "Fix Runtime shutdown barrier acknowledgment race (v0.92.2)"
branch: "codex/960-shutdown-barrier"
generated_at: "2026-09-12T06:12:47.244773+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/960"
  - kind: "stp"
    ref: ".csdlc/issues/960/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/960/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/960/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/960/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/960/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/960/cards/stp.md"
  - ".csdlc/issues/960/cards/sip.md"
  - ".csdlc/issues/960/cards/vpp.md"
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
  - ".csdlc/issues/960/cards/stp.md"
  - ".csdlc/issues/960/cards/sip.md"
  - ".csdlc/issues/960/cards/vpp.md"
review_results:
  findings_status: "no_findings"
  recommended_outcome: "pass"
notes: "Independent reviewer /root/review_960 reviewed the four source files at 91cf2c85e3c85051b4ebcdca2eed9d6c8cdf7909 plus the test import correction; no actionable findings. Verified all seven proof log hashes. Final committed-head renewal is required before native publication."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/960/cards/stp.md
- .csdlc/issues/960/cards/sip.md
- .csdlc/issues/960/cards/vpp.md

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

- No unresolved actionable findings.

### Dispositions

- P3: corrected SPP/VPP plan_summary to accurately describe Cargo-built CLI validation. Original CI causation remains unknown; exact barrier regression is independently reproduced and repaired.

### Recommended Outcome

- pass

## Notes

Independent reviewer /root/review_960 reviewed the four source files at 91cf2c85e3c85051b4ebcdca2eed9d6c8cdf7909 plus the test import correction; no actionable findings. Verified all seven proof log hashes. Final committed-head renewal is required before native publication.
