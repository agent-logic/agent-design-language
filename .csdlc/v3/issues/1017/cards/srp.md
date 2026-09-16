---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-company-domain-migration-review-prompt"
issue: 1017
task_id: "issue-1017"
version: "v0.92.2"
title: "[v0.92.2][corporate][AWS] Migrate v-*.ai domains from personal AWS to the company account"
branch: "codex/1017-v0922-company-domain-migration"
generated_at: "2026-09-16T23:15:26.654477+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1017"
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
  - "<stp_card>"
  - "<sip_card>"
  - "<vpp_card>"
review_results:
  findings_status: "review_unavailable"
  recommended_outcome: "block"
notes: "Independently review exact inventory approval, account verification, backups, complete record parity, separate transfer and DNS recovery, service baseline/postchecks, TTL observation and safe cleanup. Check privacy and remaining blocked domains."
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

When finalizing review, record the machine-readable review result in frontmatter:

```yaml
review_results:
  findings_status: "no_findings | findings_present | review_unavailable | review_timeout | review_cancelled | review_failed"
  recommended_outcome: "pass | block | needs_followup"
```

### Findings

- Implementation and live migration not performed.

### Dispositions

- No implementation findings dispositioned.

### Recommended Outcome

- block

## Notes

Independently review exact inventory approval, account verification, backups, complete record parity, separate transfer and DNS recovery, service baseline/postchecks, TTL observation and safe cleanup. Check privacy and remaining blocked domains.
