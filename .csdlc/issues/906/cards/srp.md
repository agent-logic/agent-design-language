---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0922-process-parser-simplification-review-prompt"
issue: 906
task_id: "issue-0906"
version: "0.92.2"
title: "[v0.92.2][PLAT-RUST] Complete one selected production Rust responsibility refactor"
branch: "codex/906-v0922-process-parser-simplification"
generated_at: "2026-09-12T00:22:05.416044+00:00"
card_status: "ready"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/906"
  - kind: "stp"
    ref: ".csdlc/issues/906/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/906/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/906/cards/spp.md"
  - kind: "vpp"
    ref: ".csdlc/issues/906/cards/vpp.md"
  - kind: "sor"
    ref: ".csdlc/issues/906/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - ".csdlc/issues/906/cards/stp.md"
  - ".csdlc/issues/906/cards/sip.md"
  - ".csdlc/issues/906/cards/vpp.md"
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
  - ".csdlc/issues/906/cards/stp.md"
  - ".csdlc/issues/906/cards/sip.md"
  - ".csdlc/issues/906/cards/vpp.md"
review_results:
  findings_status: "review_unavailable"
  recommended_outcome: "block"
notes: "Independent exact-head implementation review required before publication. Review every acceptance item: 1. Deliver the complete pure parser used by production `real_process_status`, preserving accepted/rejected argv, repeated options and option order, error text/order, defaults, mutually exclusive target semantics, zero PID/port rejection and loopback-only host rules. Keep syscall/network probes and output schema unchanged. 2. Reduce duplicated target-selection logic/control-flow complexity with an explicit before/after measure and reviewed rationale. Count recursive source totals and explain additions. A smaller parent facade, moved lines or more tests alone is not reduction; no arbitrary net-line claim substitutes for the actual simplification. 3. Exercise installed `adl process status` via existing `adl/tests/cli_smoke/process_status.rs` and focused parser cases, including malformed/missing/duplicate/conflicting flags and boundary values. Test safety denials without broad process scans or unsafe network targets. Preserve existing public outputs on all supported platforms. 4. Focused regressions, diff/format checks and required CI pass at independent exact-head review. No design-only refactor proposal, unused helper or compilation-only evidence closes the issue. Update narrowly affected process-status documentation if necessary. PVF: deterministic local parser/CLI behavior-preservation contract, small CPU and controlled local process fixtures, required milestone support gate. No broad Rust rewrite, process-control feature changes, unrelated cleanup or other-owner takeover. Stop on unresolved dirty ownership, behavior drift, measurement without simplification or missing/failed proof. acceptance: `bounded_surface`, `behavior_preserved`, `measurable_reduction`, `exact_module_and_invariant_selected_before_creation`, `production_callers_preserved`, `recursive_before_after_inventory`. pvf: `focused_regression`, `before_after_measurement`, `exact_module_and_invariant_selected_before_creation`, `production_callers_preserved`, `recursive_before_after_inventory`, `unselected_surface_rejected`, `facade_only_reduction_claim_rejected`. stop_conditions: `scope_sprawl`, `behavior_redesign`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `production_responsibility_unselected`. non_goals: `repo_wide_rewrite`. Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/srp.md`

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue. Review results are intentionally absent before implementation exists and must be finalized before PR publication.

## Scope Basis

- .csdlc/issues/906/cards/stp.md
- .csdlc/issues/906/cards/sip.md
- .csdlc/issues/906/cards/vpp.md

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

Independent exact-head implementation review required before publication. Review every acceptance item: 1. Deliver the complete pure parser used by production `real_process_status`, preserving accepted/rejected argv, repeated options and option order, error text/order, defaults, mutually exclusive target semantics, zero PID/port rejection and loopback-only host rules. Keep syscall/network probes and output schema unchanged. 2. Reduce duplicated target-selection logic/control-flow complexity with an explicit before/after measure and reviewed rationale. Count recursive source totals and explain additions. A smaller parent facade, moved lines or more tests alone is not reduction; no arbitrary net-line claim substitutes for the actual simplification. 3. Exercise installed `adl process status` via existing `adl/tests/cli_smoke/process_status.rs` and focused parser cases, including malformed/missing/duplicate/conflicting flags and boundary values. Test safety denials without broad process scans or unsafe network targets. Preserve existing public outputs on all supported platforms. 4. Focused regressions, diff/format checks and required CI pass at independent exact-head review. No design-only refactor proposal, unused helper or compilation-only evidence closes the issue. Update narrowly affected process-status documentation if necessary. PVF: deterministic local parser/CLI behavior-preservation contract, small CPU and controlled local process fixtures, required milestone support gate. No broad Rust rewrite, process-control feature changes, unrelated cleanup or other-owner takeover. Stop on unresolved dirty ownership, behavior drift, measurement without simplification or missing/failed proof. acceptance: `bounded_surface`, `behavior_preserved`, `measurable_reduction`, `exact_module_and_invariant_selected_before_creation`, `production_callers_preserved`, `recursive_before_after_inventory`. pvf: `focused_regression`, `before_after_measurement`, `exact_module_and_invariant_selected_before_creation`, `production_callers_preserved`, `recursive_before_after_inventory`, `unselected_surface_rejected`, `facade_only_reduction_claim_rejected`. stop_conditions: `scope_sprawl`, `behavior_redesign`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `production_responsibility_unselected`. non_goals: `repo_wide_rewrite`. Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`.
