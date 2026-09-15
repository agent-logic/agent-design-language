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
card_status: "ready"
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
  findings_status: "no_findings"
  recommended_outcome: "approve"
notes: "Independent reviewer /root/review_902_exact_head reran the real protected 5/5 input, seven focused tests, reduced-member, empty-risk, review-digest and protected-member substitutions, Python compilation, full diff hygiene and the end-to-end fifth-row CLI failure. Exact head was rechecked and no files or remote state were changed. Native review admission is ready; this card-only metadata change requires a final exact-head delta review before publication."
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

- Initial exact-head review found one P1 and three P2s; a follow-up review found the partial failure report discarded prior pass rows. All five findings were repaired. Final independent rereview found no actionable findings at 6dca1dcd7e778c5c99873e88aaeeedc47ae2d665.

### Dispositions

- Exact protected member and producer identities, semantic producer cross-links, typed review evidence, canonical residual risks, complete negative coverage, structured denominators and current SOR truth are enforced. Late-row CLI failure preserves four prior passes and reports the fifth failed with complete=4 and missing=0.

### Recommended Outcome

- approve

## Notes

Independent reviewer /root/review_902_exact_head reran the real protected 5/5 input, seven focused tests, reduced-member, empty-risk, review-digest and protected-member substitutions, Python compilation, full diff hygiene and the end-to-end fifth-row CLI failure. Exact head was rechecked and no files or remote state were changed. Native review admission is ready; this card-only metadata change requires a final exact-head delta review before publication.
