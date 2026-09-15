---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-architecture-structure-review-prompt"
issue: 882
task_id: "issue-0882"
version: "0.92.2"
title: "[v0.92.2][CF-COG] Report repository dependency and boundary structure"
branch: "codex/882-v0922-architecture-structure"
generated_at: "2026-09-12T00:04:57.571871+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/882"
  - kind: "stp"
    ref: ".csdlc/issues/882/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/882/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/882/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/882/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/882/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/882/cards/stp.md"
  - ".csdlc/issues/882/cards/sip.md"
  - ".csdlc/issues/882/cards/vpp.md"
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
  - ".csdlc/issues/882/cards/stp.md"
  - ".csdlc/issues/882/cards/sip.md"
  - ".csdlc/issues/882/cards/vpp.md"
review_results:
  findings_status: "interim_findings_fixed_final_review_pending"
  recommended_outcome: "block"
notes: "Independent exact-head implementation review required before publication. Review every acceptance item: 1. Execute the production architecture path against known graph fixtures containing an allowed layering pattern, forbidden dependency/cycle and a bounded coupling/connascence case. Findings identify exact evidence objects, source locations, revision, inference, confidence or unknown, and an actionable explanation. 2. Demonstrate the actual `repository_structure_reporter` receives CF-EVIDENCE-admitted packets and produces persisted findings consumed by the product artifact path; a test-only graph builder or authored report is insufficient. 3. Unsupported language constructs, unresolved external dependencies, truncated scope and malformed/tampered evidence produce explicit partial/unknown/error outcomes. Never present an incomplete graph as a complete architecture assessment. 4. Review a fixed positive/negative sample, retain expected outcomes, false positives and reviewer calibration decisions. Repeat identical inputs deterministically. Explain limits on connascence detected from this selected bounded Rust analysis. 5. Preserve canonical identities and deterministic graph ordering without following repository scripts, hidden network requests or source mutation. No unrelated scope, autonomous architecture/source rewrite, public publication, general CI platform or complete speculative memory system. Retain original exclusions: unrelated_scope, schema_or_scaffold_only_completion, automatic_architecture_rewrite. Stop on unsupported_architecture_claim, opaque_score_only, required_proof_not_executed, partial_artifact_claimed_complete; also stop for missing authority, unresolved ownership collision, privacy/provenance failure or incompatible shared contracts. Route a separately discovered concern explicitly rather than silently widening this task."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/882/cards/stp.md
- .csdlc/issues/882/cards/sip.md
- .csdlc/issues/882/cards/vpp.md

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

- Interim /root/sprint3_preparation_review found malformed root plus manifest panic, missing extern-crate unknown, and policy absent from run compatibility. All three corrected; two failure regressions reproduced then passed. Final committed-head review remains required.

### Dispositions

- No implementation findings have been accepted or waived.

### Recommended Outcome

- block

## Notes

Independent exact-head implementation review required before publication. Review every acceptance item: 1. Execute the production architecture path against known graph fixtures containing an allowed layering pattern, forbidden dependency/cycle and a bounded coupling/connascence case. Findings identify exact evidence objects, source locations, revision, inference, confidence or unknown, and an actionable explanation. 2. Demonstrate the actual `repository_structure_reporter` receives CF-EVIDENCE-admitted packets and produces persisted findings consumed by the product artifact path; a test-only graph builder or authored report is insufficient. 3. Unsupported language constructs, unresolved external dependencies, truncated scope and malformed/tampered evidence produce explicit partial/unknown/error outcomes. Never present an incomplete graph as a complete architecture assessment. 4. Review a fixed positive/negative sample, retain expected outcomes, false positives and reviewer calibration decisions. Repeat identical inputs deterministically. Explain limits on connascence detected from this selected bounded Rust analysis. 5. Preserve canonical identities and deterministic graph ordering without following repository scripts, hidden network requests or source mutation. No unrelated scope, autonomous architecture/source rewrite, public publication, general CI platform or complete speculative memory system. Retain original exclusions: unrelated_scope, schema_or_scaffold_only_completion, automatic_architecture_rewrite. Stop on unsupported_architecture_claim, opaque_score_only, required_proof_not_executed, partial_artifact_claimed_complete; also stop for missing authority, unresolved ownership collision, privacy/provenance failure or incompatible shared contracts. Route a separately discovered concern explicitly rather than silently widening this task.
