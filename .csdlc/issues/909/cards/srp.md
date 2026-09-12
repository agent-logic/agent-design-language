---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "909-gcp-move-in-review-prompt"
issue: 909
task_id: "issue-0909"
version: "v0.92.2"
title: "[v0.92.2][OPS-GCP] Produce one apply-ready company GCP move-in execution packet"
branch: "codex/909-gcp-move-in"
generated_at: "<timestamp>"
card_status: "blocked"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/909"
  - kind: "stp"
    ref: ".csdlc/issues/909/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/909/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/909/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/909/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/909/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/909/cards/stp.md"
  - ".csdlc/issues/909/cards/sip.md"
  - ".csdlc/issues/909/cards/vpp.md"
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
  - ".csdlc/issues/909/cards/stp.md"
  - ".csdlc/issues/909/cards/sip.md"
  - ".csdlc/issues/909/cards/vpp.md"
review_results:
  findings_status: "no_actionable_packet_findings_acceptance_blocked"
  recommended_outcome: "Retain reviewed incomplete packet; obtain platform state/custodian and verified directory group identity before completion or publication."
notes: "Reviewer subagent:sprint8_908 verified18 source/lock digests,20 policy identities/effective readbacks,113 quotas per project, actual private binary plan hashes/address actions/before-after projections, bootstrap private state SHA/MD5/serial and remote-generation claim; read-only inspection, no cloud calls. Review proves packet correctness, not apply-ready completeness."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/909/cards/stp.md
- .csdlc/issues/909/cards/sip.md
- .csdlc/issues/909/cards/vpp.md

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

- Checkpoint null organization-policy projection finding repaired. Final independent packet review at e7cd268a71cd9b5b99143c5dbdc997473196e68c has no actionable findings. Required platform plan and target group remain unresolved.

### Dispositions

- Recorded actual constraint identities and20 effective host policies, quota values, exact plan before/after fields, state digest comparison and explicit403/404 group lookup ambiguity. No waiver of missing acceptance evidence.

### Recommended Outcome

- Retain reviewed incomplete packet; obtain platform state/custodian and verified directory group identity before completion or publication.

## Notes

Reviewer subagent:sprint8_908 verified18 source/lock digests,20 policy identities/effective readbacks,113 quotas per project, actual private binary plan hashes/address actions/before-after projections, bootstrap private state SHA/MD5/serial and remote-generation claim; read-only inspection, no cloud calls. Review proves packet correctness, not apply-ready completeness.
