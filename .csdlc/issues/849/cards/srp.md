---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-merge-linkage-admission-review-prompt"
issue: 849
task_id: "issue-0849"
version: "0.92.2"
title: "[v0.92.2][C-SDLC v3][P2] Preserve publication linkage during native PR merge"
branch: "codex/849-v0922-merge-linkage-admission"
generated_at: "2026-09-12T00:22:05.416044+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/849"
  - kind: "stp"
    ref: ".csdlc/issues/849/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/849/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/849/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/849/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/849/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/849/cards/stp.md"
  - ".csdlc/issues/849/cards/sip.md"
  - ".csdlc/issues/849/cards/vpp.md"
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
  - ".csdlc/issues/849/cards/stp.md"
  - ".csdlc/issues/849/cards/sip.md"
  - ".csdlc/issues/849/cards/vpp.md"
review_results:
  findings_status: "addressed"
  recommended_outcome: "accept"
notes: "Independent sprint8_909 accepted8bbac35e726b421f50c19f1421076f268a0b242b. Subsequent two-line fake-curl stdin drain488c3d92b undergoes source and metadata renewal before publication; no production transport change."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/849/cards/stp.md
- .csdlc/issues/849/cards/sip.md
- .csdlc/issues/849/cards/vpp.md

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

- User review of5b68a266 found P2 URL-form PartOf directives bypass ambiguity/mixed-mode checks. Reproduced canonical PartOf plus conflicting URL before repair. Source01488a4687 detects URL and Markdown-wrapped directives, retaining canonical-only admission. Two earlier P2 metadata findings remain corrected.

### Dispositions

- URL ambiguity repaired with32fake-transport cases: Part of/Part-of, both modes, plain/angle/Markdown URLs including spaced labels, with or without a canonical directive. Every rejection precedes intent and PUT. Independent source review accepted01488a4687; final metadata review retained before publication.

### Recommended Outcome

- accept

## Notes

Independent sprint8_909 accepted8bbac35e726b421f50c19f1421076f268a0b242b. Subsequent two-line fake-curl stdin drain488c3d92b undergoes source and metadata renewal before publication; no production transport change.
