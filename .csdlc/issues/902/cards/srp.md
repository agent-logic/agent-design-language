---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-runtime-criterion-evidence-review-prompt"
issue: 902
task_id: "issue-0902"
version: "0.92.2"
title: "[v0.92.2][QUAL-EVIDENCE] Validate criterion-bound Runtime qualification evidence"
branch: "codex/902-v0922-runtime-criterion-evidence"
generated_at: "2026-09-12T00:14:10.636008+00:00"
card_status: "draft"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/902"
  - kind: "stp"
    ref: ".csdlc/issues/902/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/902/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/902/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/902/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/902/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/902/cards/stp.md"
  - ".csdlc/issues/902/cards/sip.md"
  - ".csdlc/issues/902/cards/vpp.md"
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
  - ".csdlc/issues/902/cards/stp.md"
  - ".csdlc/issues/902/cards/sip.md"
  - ".csdlc/issues/902/cards/vpp.md"
review_results:
  findings_status: "remediated_pending_rereview"
  recommended_outcome: "block pending exact-head rereview"
notes: "Review d2522aefa failed with one P1 and three P2s. Review the new committed candidate after remediation, rerun the real validator and focused tests, and specifically retry reduced-member, empty-risk, refreshed execution-log/provenance, coherent-synthetic, and failure-output adversarial cases. Publication remains blocked until no actionable findings."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/902/cards/stp.md
- .csdlc/issues/902/cards/sip.md
- .csdlc/issues/902/cards/vpp.md

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

- Exact-head review at d2522aefa9a165805c582980be7b614208ce9c6e found one P1 and three P2s: reduced protected evidence could pass; required negatives were incomplete; residual risks and failed row denominators were not enforced; SOR retained stale preparation truth.

### Dispositions

- All four findings remediated in the working candidate: exact member/digest sets and semantic cross-links are enforced; typed receipt evidence digests bind retained review artifacts; 21 negative mutations cover partial/log/provenance/synthetic cases; canonical risks and structured failed rows are enforced; stale SOR fields were replaced through native edit. Independent exact-head rereview remains required.

### Recommended Outcome

- block pending exact-head rereview

## Notes

Review d2522aefa failed with one P1 and three P2s. Review the new committed candidate after remediation, rerun the real validator and focused tests, and specifically retry reduced-member, empty-risk, refreshed execution-log/provenance, coherent-synthetic, and failure-output adversarial cases. Publication remains blocked until no actionable findings.
