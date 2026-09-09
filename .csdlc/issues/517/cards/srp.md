---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "tail-01-quality-gate-review-prompt"
issue: 517
task_id: "issue-0517"
version: "v0.92.1"
title: "[v0.92.1][TAIL-01] Quality gate"
branch: "codex/517-tail-01-quality-gate"
generated_at: "2026-09-09T00:56:48.157331+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/517"
  - kind: "stp"
    ref: ".csdlc/issues/517/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/517/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/517/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/517/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/517/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/517/cards/stp.md"
  - ".csdlc/issues/517/cards/sip.md"
  - ".csdlc/issues/517/cards/vpp.md"
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
  - ".csdlc/issues/517/cards/stp.md"
  - ".csdlc/issues/517/cards/sip.md"
  - ".csdlc/issues/517/cards/vpp.md"
review_results:
  findings_status: "pending"
  recommended_outcome: "pending"
notes: "Review applies to docs accounting only. Accepted successor closure does not assert direct execution proof. Final committed-head review and hosted CI remain distinct."
---

Canonical Template Source: `docs/templates/prompts/1.0.4/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/517/cards/stp.md
- .csdlc/issues/517/cards/sip.md
- .csdlc/issues/517/cards/vpp.md

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

- Independent bounded reviews verified all245 original row identities,19 successor joins,PR750 terminal artifacts,five exception dispositions,and canonical YAML synchronization. Review corrections to scope,status prose,WP01 dependency guard and HOT01 origin-policy description are applied. Final immutable-head attestation remains required before publication.

### Dispositions

- Four speculative follow-up proposals withdrawn; no issues created from missing crossreferences. All245 rows accounted and five exception groups dispositioned with zero unowned entries. Confirmed native defects separately owned749/751.

### Recommended Outcome

- pending

## Notes

Review applies to docs accounting only. Accepted successor closure does not assert direct execution proof. Final committed-head review and hosted CI remain distinct.
