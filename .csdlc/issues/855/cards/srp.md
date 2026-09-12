---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "issue-855-provider-neutral-dynamic-agent-lifecycle-review-prompt"
issue: 855
task_id: "issue-0855"
version: "v0.92.2"
title: "[v0.92.2][RT-PROVIDER] Provider-neutral dynamic agent lifecycle"
branch: "codex/855-provider-neutral-lifecycle"
generated_at: "2026-09-12"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/855"
  - kind: "stp"
    ref: ".csdlc/issues/855/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/855/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/855/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/855/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/855/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/855/cards/stp.md"
  - ".csdlc/issues/855/cards/sip.md"
  - ".csdlc/issues/855/cards/vpp.md"
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
  - ".csdlc/issues/855/cards/stp.md"
  - ".csdlc/issues/855/cards/sip.md"
  - ".csdlc/issues/855/cards/vpp.md"
review_results:
  findings_status: "findings_present"
  recommended_outcome: "pass"
notes: "Source and merge review PASS through114d143fdfc7668848e5e45065fb7c9fdb7a558b. Refreshed installed10 passed5/5 and hosted-fixture04 passed3/3 with zero paidcalls, exact binary/report hashes retained. Current CI34681209235 remains pending. Live02 remains failed after3 OpenAIrequests; further paid execution awaits reconciled authorization. Proof-only metadata update requires final acknowledgment."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/855/cards/stp.md
- .csdlc/issues/855/cards/sip.md
- .csdlc/issues/855/cards/vpp.md

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

- Prior CLI deadline, health race, action accounting and mock bound findings resolved. Live02 category-loss remediation and CI TLS repair reviewed at f162669; actual live continuation cause remains unknown. User P2 zero-cap candidate finding resolved by shared strict Runtime-limit validation before generation promotion. Actual reload regression rejects eight invalid replacements while retaining previous generation/digest, then accepts valid recovery. No actionable source findings remain; refreshed installed proof, current CI and full hosted acceptance remain incomplete.

### Dispositions

- review_804 reviewed continuation remediation; root reviewed TLS, strict candidate/LKG validation and exact114d merge preserving both CI jobs. Focused regressions and refreshed installed fixture proof pass. No known actionable source findings remain; full hosted acceptance and current CI are incomplete. Draft publication only.

### Recommended Outcome

- pass

## Notes

Source and merge review PASS through114d143fdfc7668848e5e45065fb7c9fdb7a558b. Refreshed installed10 passed5/5 and hosted-fixture04 passed3/3 with zero paidcalls, exact binary/report hashes retained. Current CI34681209235 remains pending. Live02 remains failed after3 OpenAIrequests; further paid execution awaits reconciled authorization. Proof-only metadata update requires final acknowledgment.
