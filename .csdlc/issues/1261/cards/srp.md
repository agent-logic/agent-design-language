---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "<slug>-review-prompt"
issue: 1261
task_id: "issue-1261"
version: "1.0.5"
title: "[v0.93.1][defect] Report dashboard JavaScript validation as skipped or failed when Node is missing"
branch: "codex/1261-dashboard-node-validation-truth"
generated_at: "<timestamp>"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1261"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
  - kind: "spp"
    ref: "<spp_card>"
  - kind: "vpp"
    ref: "<vpp_card>"
  - kind: "sor"
    ref: "<sor_card>"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - "<stp_card>"
  - "<sip_card>"
  - "<vpp_card>"
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
review_scope: "<review_scope>"
review_instructions: "<review_instructions>"
review_evidence_requirements: "<review_evidence_requirements>"
reviewer_constraints: "<reviewer_constraints>"
refusal_policy: "<refusal_policy>"
follow_up_routing: "<follow_up_routing>"
non_claims:
  - "This prompt does not claim review has already run."
  - "This prompt does not guarantee review quality by itself."
policy_refs:
  - "<stp_card>"
  - "<sip_card>"
  - "<vpp_card>"
review_results: {"status":"not_run"}
notes: "<review_notes>"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- <stp_card>
- <sip_card>
- <vpp_card>

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

When finalizing review, submit exactly these typed result fields. `outcome` must match `review_results.status`:

```yaml
review_results:
  status: "pass | block | needs_followup"
reviewer: "independent reviewer identity"
findings:
  - "substantive finding; use an empty list when there are no findings"
outcome: "pass | block | needs_followup"
```

### Reviewer

<reviewer>

### Findings

- <findings>

### Outcome

<outcome>

## Notes

<review_notes>
