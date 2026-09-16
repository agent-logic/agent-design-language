---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-remediation-planner-review-prompt"
issue: 893
task_id: "issue-0893"
version: "0.92.2"
title: "[v0.92.2][CF-REMEDIATE] Generate a bounded remediation plan from review findings"
branch: "codex/893-v0922-remediation-planner"
generated_at: "2026-09-12T00:10:02.687479+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/893"
  - kind: "stp"
    ref: ".csdlc/issues/893/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/893/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/893/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/893/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/893/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/893/cards/stp.md"
  - ".csdlc/issues/893/cards/sip.md"
  - ".csdlc/issues/893/cards/vpp.md"
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
  - ".csdlc/issues/893/cards/stp.md"
  - ".csdlc/issues/893/cards/sip.md"
  - ".csdlc/issues/893/cards/vpp.md"
review_results:
  findings_status: "metadata_finding_remediated_pending_fresh_review"
  recommended_outcome: "review_required"
notes: "A different fresh reviewer must verify the generation-17 replay-result repair at the new immutable head. Product parent a7cddf07dea4f557ed089e0be02adffbe3985717 remains unchanged and reviewed PASS."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/893/cards/stp.md
- .csdlc/issues/893/cards/sip.md
- .csdlc/issues/893/cards/vpp.md

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

- Fresh metadata review of a3b1bc0e178b4bbd26da3bd173b1ed482a308b68 by fresh-session:c0e6ed7d-f990-4f36-a21f-55f542dd7c50 returned FAIL with one P2 truth mismatch and no product finding: replay_command included rustfmt and exact-range diff hygiene, while replay_result omitted those results and instead included a separate review PASS.

### Dispositions

- P2 accepted and remediated through the native SOR editor. replay_result now exactly reports the 9 remediation, 8 test-plan, 7 UX, rustfmt, and exact-range diff outputs produced by replay_command; independent review remains recorded separately in SRP/integration verification truth.

### Recommended Outcome

- review_required

## Notes

A different fresh reviewer must verify the generation-17 replay-result repair at the new immutable head. Product parent a7cddf07dea4f557ed089e0be02adffbe3985717 remains unchanged and reviewed PASS.
