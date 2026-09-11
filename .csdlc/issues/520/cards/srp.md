---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "tail-04-internal-review-review-prompt"
issue: 520
task_id: "issue-0520"
version: "1.0.5"
title: "[v0.92.1][TAIL-04] Internal review"
branch: "codex/520-internal-review"
generated_at: "2026-09-09T19:19:55Z"
card_status: "exact_head_packet_review_passed"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/520"
  - kind: "stp"
    ref: ".csdlc/issues/520/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/520/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/520/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/520/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/520/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/520/cards/stp.md"
  - ".csdlc/issues/520/cards/sip.md"
  - ".csdlc/issues/520/cards/vpp.md"
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
  - ".csdlc/issues/520/cards/stp.md"
  - ".csdlc/issues/520/cards/sip.md"
  - ".csdlc/issues/520/cards/vpp.md"
review_results:
  findings_status: "no_actionable_packet_findings_fourteen_candidate_findings_retained"
  recommended_outcome: "packet_approved_for_publication_release_remains_blocked"
notes: "Reviewer task: 01a08d62-1aa2-77c3-9ceb-ef803a85147e. Exact revision: 770980ccb68d06753f8b6b275bd7dffca4038889. Production suite: 20/20 negative cases. Full packet validator: PASS for 6,098 paths, 119 issues, 176 acceptance surfaces, nine assignments, and 14 retained candidate findings."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/520/cards/stp.md
- .csdlc/issues/520/cards/sip.md
- .csdlc/issues/520/cards/vpp.md

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

- Fresh independent exact-head review of 770980ccb68d06753f8b6b275bd7dffca4038889 found no additional actionable P0-P3 packet or lifecycle findings. The review packet itself passes. Its separate release-candidate verdict remains changes required for the 14 accepted product findings routed under #522.

### Dispositions

- All four prior exact-head packet findings were repaired: independent certification, packet-wide machine-local path redaction, exact manifest completeness, and truthful 74-artifact SOR count. The final context-aware portability repair also rejects bare filesystem /root paths while preserving logical actor identifiers. No packet finding is waived.

### Recommended Outcome

- packet_approved_for_publication_release_remains_blocked

## Notes

Reviewer task: 01a08d62-1aa2-77c3-9ceb-ef803a85147e. Exact revision: 770980ccb68d06753f8b6b275bd7dffca4038889. Production suite: 20/20 negative cases. Full packet validator: PASS for 6,098 paths, 119 issues, 176 acceptance surfaces, nine assignments, and 14 retained candidate findings.
