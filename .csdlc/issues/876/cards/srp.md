---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "issue-876-provider-definitions-review-prompt"
issue: 876
task_id: "issue-0876"
version: "v0.92.2"
title: "[v0.92.2][PLAT-PROVIDER] Consume validated editable provider definitions"
branch: "codex/876-provider-definitions"
generated_at: "2026-09-12T02:15:09.413701+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/876"
  - kind: "stp"
    ref: ".csdlc/issues/876/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/876/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/876/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/876/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/876/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/876/cards/stp.md"
  - ".csdlc/issues/876/cards/sip.md"
  - ".csdlc/issues/876/cards/vpp.md"
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
  - ".csdlc/issues/876/cards/stp.md"
  - ".csdlc/issues/876/cards/sip.md"
  - ".csdlc/issues/876/cards/vpp.md"
review_results:
  findings_status: "no_findings"
  recommended_outcome: "pass"
notes: "Independent reviewer fix_941_ci approved exact source 754ea5f02534258f8a29644591133331c28a7059 and independently reran all four provider-definition tests, including seven rejected watcher updates with retained generation/digest and real dispatch. Both user P2s resolved; all original findings and failed fixture logs preserved. Native PR953 body update and source push completed; corrected-head CI and merge remain pending. Final record-only delta requires bounded metadata confirmation."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/876/cards/stp.md
- .csdlc/issues/876/cards/sip.md
- .csdlc/issues/876/cards/vpp.md

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

- Historical user findings at 9495c0e: P2 supported expected-account-sha256 public digest rejected; P2 endpoint/base_url userinfo and credential query bypass. Both resolved in independently approved source 754ea5f; original findings preserved in REVIEW.md. No actionable findings remain.

### Dispositions

- Independent reviewer fix_941_ci approved exact source 754ea5f02534258f8a29644591133331c28a7059 and independently reran all four provider-definition tests, including seven rejected watcher updates with retained generation/digest and real dispatch. Both user P2s resolved; all original findings and failed fixture logs preserved. Native PR953 body update and source push completed; corrected-head CI and merge remain pending. Final record-only delta requires bounded metadata confirmation.

### Recommended Outcome

- pass

## Notes

Independent reviewer fix_941_ci approved exact source 754ea5f02534258f8a29644591133331c28a7059 and independently reran all four provider-definition tests, including seven rejected watcher updates with retained generation/digest and real dispatch. Both user P2s resolved; all original findings and failed fixture logs preserved. Native PR953 body update and source push completed; corrected-head CI and merge remain pending. Final record-only delta requires bounded metadata confirmation.
