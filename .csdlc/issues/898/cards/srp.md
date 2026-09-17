---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-codefriend-pdf-renderer-review-prompt"
issue: 898
task_id: "issue-0898"
version: "0.92.2"
title: "[v0.92.2][CF-RENDER-PDF] Render an approved review as a verified PDF"
branch: "not bound yet; proposed codex/898-v0922-codefriend-pdf-renderer"
generated_at: "2026-09-12T00:10:10.730683+00:00"
card_status: "draft"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/898"
  - kind: "stp"
    ref: ".csdlc/issues/898/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/898/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/898/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/898/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/898/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/898/cards/stp.md"
  - ".csdlc/issues/898/cards/sip.md"
  - ".csdlc/issues/898/cards/vpp.md"
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
  - ".csdlc/issues/898/cards/stp.md"
  - ".csdlc/issues/898/cards/sip.md"
  - ".csdlc/issues/898/cards/vpp.md"
review_results:
  findings_status: "review_unavailable"
  recommended_outcome: "block"
notes: "Independent exact-head implementation review required before publication. Review every acceptance item: 1. Render actual governed inputs as a complete PDF and manifest with the same claim/finding set, scope, citations, uncertainty, disagreements and actionable plans as approved Markdown. Pin the chosen rendering engine/version in the implementation plan and manifest; keep Rust entrypoint ownership even if a bounded renderer subprocess is required. 2. Extract text to check semantic parity and render every representative page for visual inspection. Exercise long findings, long URLs/code, tables, page breaks and non-ASCII text; reject clipped/overlapping/missing content, broken citations and unreadable output. A zero-byte or merely parseable PDF is insufficient. 3. Enforce approval freshness, exact rendering/target identity, immutable provenance and redaction before export. Reject unapproved/tampered input, secret leakage, claim drift and unsafe external-resource inclusion; renderer failures leave explicit failed/withheld state, not a success manifest. 4. Open the emitted PDF and retain visual review plus content/manifest hashes. Produce local output only; no public hosting or submission is implied. Schema, mock or snapshot-only completion is forbidden."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/898/cards/stp.md
- .csdlc/issues/898/cards/sip.md
- .csdlc/issues/898/cards/vpp.md

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

- Implementation review has not run; no implementation exists from this preparation.

### Dispositions

- No implementation findings have been accepted or waived.

### Recommended Outcome

- block

## Notes

Independent exact-head implementation review required before publication. Review every acceptance item: 1. Render actual governed inputs as a complete PDF and manifest with the same claim/finding set, scope, citations, uncertainty, disagreements and actionable plans as approved Markdown. Pin the chosen rendering engine/version in the implementation plan and manifest; keep Rust entrypoint ownership even if a bounded renderer subprocess is required. 2. Extract text to check semantic parity and render every representative page for visual inspection. Exercise long findings, long URLs/code, tables, page breaks and non-ASCII text; reject clipped/overlapping/missing content, broken citations and unreadable output. A zero-byte or merely parseable PDF is insufficient. 3. Enforce approval freshness, exact rendering/target identity, immutable provenance and redaction before export. Reject unapproved/tampered input, secret leakage, claim drift and unsafe external-resource inclusion; renderer failures leave explicit failed/withheld state, not a success manifest. 4. Open the emitted PDF and retain visual review plus content/manifest hashes. Produce local output only; no public hosting or submission is implied. Schema, mock or snapshot-only completion is forbidden.
