---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-architecture-drift-review-prompt"
issue: 886
task_id: "issue-0886"
version: "0.92.2"
title: "[v0.92.2][CF-COG-DRIFT] Report architecture drift between compatible revisions"
branch: "codex/886-v0922-architecture-drift"
generated_at: "2026-09-12T00:04:57.571871+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/886"
  - kind: "stp"
    ref: ".csdlc/issues/886/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/886/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/886/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/886/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/886/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/886/cards/stp.md"
  - ".csdlc/issues/886/cards/sip.md"
  - ".csdlc/issues/886/cards/vpp.md"
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
  - ".csdlc/issues/886/cards/stp.md"
  - ".csdlc/issues/886/cards/sip.md"
  - ".csdlc/issues/886/cards/vpp.md"
review_results:
  findings_status: "no_findings"
  recommended_outcome: "approve"
notes: "Independent exact-head implementation review required before publication. Review every acceptance item: 1. Execute `architecture_drift_reporter` on known graph deltas: introduced/removed edge, changed boundary/layering relationship, unchanged graph and scope-preserving relocation. Retain stable baseline selection, deterministic ordering and evidence-linked explanations. 2. Consume actual CF-COG outputs and the CF-MEMORY comparison boundary through the installed product command, then persist shared-contract drift findings. No private identity schema or test-only consumer. 3. Reject incompatible baselines, deleted/tampered evidence, version mismatch and ambiguous identity; partial/narrower coverage remains explicitly not-comparable and never marks findings resolved. 4. Record confidence/unknowns, reviewer calibration and sampled false-positive dispositions. An opaque score cannot replace observed structural delta and reasoning. 5. A changed-source fixture declares a separate controlled revision. Do not silently change the selected external repository pin to manufacture drift evidence. No unrelated scope, autonomous architecture/source rewrite, public publication, general CI platform or complete speculative memory system. Retain original exclusions: unrelated_scope, schema_or_scaffold_only_completion, automatic_architecture_rewrite. Stop on missing_required_input, required_proof_failed, scope_or_contract_conflict, required_proof_not_executed, partial_artifact_claimed_complete, unsupported_architecture_claim, opaque_score_only; also stop for missing authority, unresolved ownership collision, privacy/provenance failure or incompatible shared contracts. Route a separately discovered concern explicitly rather than silently widening this task."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/886/cards/stp.md
- .csdlc/issues/886/cards/sip.md
- .csdlc/issues/886/cards/vpp.md

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

- Independent /root/repair_882_ci/review_989_integration PASS at fbaa3e7e9b61f87efa0440b057e3ce976998df48; no findings. Reviewer independently reran9 drift tests and11 installed scenarios, verified source hashes and CF-MEMORY compatibility/retention behavior.

### Dispositions

- No findings waived. Final record-only exact-head confirmation required before native review/publication.

### Recommended Outcome

- approve

## Notes

Independent exact-head implementation review required before publication. Review every acceptance item: 1. Execute `architecture_drift_reporter` on known graph deltas: introduced/removed edge, changed boundary/layering relationship, unchanged graph and scope-preserving relocation. Retain stable baseline selection, deterministic ordering and evidence-linked explanations. 2. Consume actual CF-COG outputs and the CF-MEMORY comparison boundary through the installed product command, then persist shared-contract drift findings. No private identity schema or test-only consumer. 3. Reject incompatible baselines, deleted/tampered evidence, version mismatch and ambiguous identity; partial/narrower coverage remains explicitly not-comparable and never marks findings resolved. 4. Record confidence/unknowns, reviewer calibration and sampled false-positive dispositions. An opaque score cannot replace observed structural delta and reasoning. 5. A changed-source fixture declares a separate controlled revision. Do not silently change the selected external repository pin to manufacture drift evidence. No unrelated scope, autonomous architecture/source rewrite, public publication, general CI platform or complete speculative memory system. Retain original exclusions: unrelated_scope, schema_or_scaffold_only_completion, automatic_architecture_rewrite. Stop on missing_required_input, required_proof_failed, scope_or_contract_conflict, required_proof_not_executed, partial_artifact_claimed_complete, unsupported_architecture_claim, opaque_score_only; also stop for missing authority, unresolved ownership collision, privacy/provenance failure or incompatible shared contracts. Route a separately discovered concern explicitly rather than silently widening this task.
