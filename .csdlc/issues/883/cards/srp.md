---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-architecture-impact-review-prompt"
issue: 883
task_id: "issue-0883"
version: "0.92.2"
title: "[v0.92.2][CF-COG-IMPACT] Report the impact of a scoped repository change"
branch: "codex/883-v0922-architecture-impact"
generated_at: "2026-09-12T00:04:57.571871+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/883"
  - kind: "stp"
    ref: ".csdlc/issues/883/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/883/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/883/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/883/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/883/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/883/cards/stp.md"
  - ".csdlc/issues/883/cards/sip.md"
  - ".csdlc/issues/883/cards/vpp.md"
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
  - ".csdlc/issues/883/cards/stp.md"
  - ".csdlc/issues/883/cards/sip.md"
  - ".csdlc/issues/883/cards/vpp.md"
review_results:
  findings_status: "review_unavailable"
  recommended_outcome: "block"
notes: "Independent exact-head implementation review required before publication. Review every acceptance item: 1. Run known-change fixtures through the real `change_impact_reporter`: direct dependents, transitive boundary crossing, cycle, unaffected component and multiple changed roots. Expected impacted sets and explanation paths must match; changes outside scope are explicit. 2. Each reported impact cites source change identity, graph revision, edge evidence and inference. Risk indicators supplement an explanation; an opaque score alone fails acceptance. 3. Unknown edges, unsupported symbol resolution, stale graph/change revision, malformed change input and exceeded bounds fail or report non-proving partial analysis with precise reasons. A missing edge cannot become a safe/no-impact claim. 4. Execute the installed product command, retain output consumable by the product artifact path, and repeat stable fixtures deterministically. Record reviewer calibration and sampled false positives/negatives. 5. Preserve evidence privacy and source immutability; do not run builds or source scripts to guess impact. No unrelated scope, autonomous architecture/source rewrite, public publication, general CI platform or complete speculative memory system. Retain original exclusions: unrelated_scope, schema_or_scaffold_only_completion, automatic_architecture_rewrite. Stop on missing_required_input, required_proof_failed, scope_or_contract_conflict, required_proof_not_executed, partial_artifact_claimed_complete, unsupported_architecture_claim, opaque_score_only; also stop for missing authority, unresolved ownership collision, privacy/provenance failure or incompatible shared contracts. Route a separately discovered concern explicitly rather than silently widening this task."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/883/cards/stp.md
- .csdlc/issues/883/cards/sip.md
- .csdlc/issues/883/cards/vpp.md

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

- Interim independent implementation review by /root/review_987_conflicts found no actionable findings. Final exact-head review of complete work product and calibration remains pending.

### Dispositions

- No findings waived. Publication remains gated on final exact-head review.

### Recommended Outcome

- block

## Notes

Independent exact-head implementation review required before publication. Review every acceptance item: 1. Run known-change fixtures through the real `change_impact_reporter`: direct dependents, transitive boundary crossing, cycle, unaffected component and multiple changed roots. Expected impacted sets and explanation paths must match; changes outside scope are explicit. 2. Each reported impact cites source change identity, graph revision, edge evidence and inference. Risk indicators supplement an explanation; an opaque score alone fails acceptance. 3. Unknown edges, unsupported symbol resolution, stale graph/change revision, malformed change input and exceeded bounds fail or report non-proving partial analysis with precise reasons. A missing edge cannot become a safe/no-impact claim. 4. Execute the installed product command, retain output consumable by the product artifact path, and repeat stable fixtures deterministically. Record reviewer calibration and sampled false positives/negatives. 5. Preserve evidence privacy and source immutability; do not run builds or source scripts to guess impact. No unrelated scope, autonomous architecture/source rewrite, public publication, general CI platform or complete speculative memory system. Retain original exclusions: unrelated_scope, schema_or_scaffold_only_completion, automatic_architecture_rewrite. Stop on missing_required_input, required_proof_failed, scope_or_contract_conflict, required_proof_not_executed, partial_artifact_claimed_complete, unsupported_architecture_claim, opaque_score_only; also stop for missing authority, unresolved ownership collision, privacy/provenance failure or incompatible shared contracts. Route a separately discovered concern explicitly rather than silently widening this task.
