---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-test-planner-review-prompt"
issue: 894
task_id: "issue-0894"
version: "0.92.2"
title: "[v0.92.2][CF-TESTPLAN] Generate a bounded test plan from review findings"
branch: "codex/894-v0922-test-planner"
generated_at: "2026-09-12T00:10:09.925246+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/894"
  - kind: "stp"
    ref: ".csdlc/issues/894/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/894/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/894/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/894/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/894/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/894/cards/stp.md"
  - ".csdlc/issues/894/cards/sip.md"
  - ".csdlc/issues/894/cards/vpp.md"
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
  - ".csdlc/issues/894/cards/stp.md"
  - ".csdlc/issues/894/cards/sip.md"
  - ".csdlc/issues/894/cards/vpp.md"
review_results:
  findings_status: "fresh_review_findings_remediated_pending_distinct_fresh_review"
  recommended_outcome: "review_required"
notes: "The prior FAIL remains retained and authoritative for 7b6245f9. Remediation is local and publication remains held until a different fresh reviewer returns PASS on the new exact head."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/894/cards/stp.md
- .csdlc/issues/894/cards/sip.md
- .csdlc/issues/894/cards/vpp.md

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

- Fresh no-context exact-head review of 7b6245f93c482c0f0622c4b07850fa800c9f2845 by fresh-session:04af6f7c-8784-4703-b326-921b1a5b2c24 returned FAIL. P1: generation accepted a standalone synthesis without validating the complete accepted #892 bundle. P1: the plan reader accepted tampered or incompletely mapped plans because it did not validate the manifest and retained synthesis together. P1: the accepted DNS finding still mapped to a generic root integration test rather than the crate's runnable unit-test/vector surface. P2: the source-immutability test asserted immutability without measuring the inspected repository tree.

### Dispositions

- All four findings from fresh-session:04af6f7c-8784-4703-b326-921b1a5b2c24 are accepted and remediated. Generation now requires the canonical synthesis.json plus sibling synthesis manifest and complete review record, validates the record, recomputes synthesis, and verifies all identities, digests and counts. Generated bundles retain the source manifest and review record; the reader verifies every artifact and recomputes the canonical plan, rejecting plan, manifest, synthesis, provenance, count, and finding-partition tampering. The DNS result now targets the existing dnsmsg-parser inline test module with the exact existing MINFO message/RDATA vector, a distinct explicit second RDATA byte string, exact first/second outputs, and exact buffer-length assertion. Installed CLI proof materializes the admitted inspected repository and compares its complete before/after file-byte inventory, including a dot-directory marker. Eight focused tests, strict Clippy, formatting, and diff hygiene pass. A different fresh no-context reviewer is required on the new immutable commit.

### Recommended Outcome

- review_required

## Notes

The prior FAIL remains retained and authoritative for 7b6245f9. Remediation is local and publication remains held until a different fresh reviewer returns PASS on the new exact head.
