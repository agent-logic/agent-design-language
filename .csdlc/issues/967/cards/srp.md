---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "issue-967-deterministic-hosted-a2a-review-prompt"
issue: 967
task_id: "issue-0967"
version: "v0.92.2"
title: "[v0.92.2][RT-PROVIDER][corrective] Make hosted A2A initiation deterministic across provider output formats"
branch: "codex/967-deterministic-hosted-a2a"
generated_at: "2026-09-12"
card_status: "completed"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/967"
  - kind: "stp"
    ref: ".csdlc/issues/967/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/967/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/967/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/967/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/967/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/967/cards/stp.md"
  - ".csdlc/issues/967/cards/sip.md"
  - ".csdlc/issues/967/cards/vpp.md"
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
  - ".csdlc/issues/967/cards/stp.md"
  - ".csdlc/issues/967/cards/sip.md"
  - ".csdlc/issues/967/cards/vpp.md"
review_results:
  findings_status: "no_findings"
  recommended_outcome: "approve"
notes: "Independent read-only review covered the full Runtime source, regression, OpenAPI schema, and lifecycle harness. Focused Runtime proof passed 1/1 and OpenAPI contracts passed 11/11. Exact-source zero-paid lifecycle reports pass 5/5 and 3/3. Paid hosted acceptance and CI remain pending and are outside this review result."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/967/cards/stp.md
- .csdlc/issues/967/cards/sip.md
- .csdlc/issues/967/cards/vpp.md

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

- No actionable findings at source revision 4d28de627cffc7c529de5082f0b883f9ab8f083c. The earlier replay-evidence overstatement was resolved by adding omitted-field wire-shape, identical typed-action replay, and changed-action conflict assertions.

### Dispositions

- Initial OpenAPI boundary finding and replay-evidence finding were fixed. Independent exact-source re-review at 4d28de627cffc7c529de5082f0b883f9ab8f083c found no remaining actionable findings.

### Recommended Outcome

- approve

## Notes

Independent read-only review covered the full Runtime source, regression, OpenAPI schema, and lifecycle harness. Focused Runtime proof passed 1/1 and OpenAPI contracts passed 11/11. Exact-source zero-paid lifecycle reports pass 5/5 and 3/3. Paid hosted acceptance and CI remain pending and are outside this review result.
