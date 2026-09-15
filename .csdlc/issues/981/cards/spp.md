---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "repository-scoped-issue-creation-receipt-execution-plan"
issue: 981
task_id: "issue-0981"
run_id: "issue-0981"
version: "1.0.5"
title: "[v0.92.2][C-SDLC v3][defect] Do not misclassify repository-scoped issue creation receipts"
branch: "codex/981-repository-scoped-issue-creation-receipt"
generated_at: "2026-09-15"
card_status: "ready"
status: "in_progress"
activation_state: "bound"
plan_revision: 1
initial_pvf_lane: "csdlc"
planned_pvf_lane: "csdlc"
planned_pvf_lane_source: "Issue #981 semantic transaction acceptance surface"
estimate_elapsed_seconds: "unknown"
estimate_total_tokens: "unknown"
estimate_validation_seconds: "unknown"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "10"
estimate_confidence: "unknown"
estimate_data_source: "not_collected"
estimate_source_ref: "issue-981"
issue_goal_ref: "Active issue #981 implementation and reviewed green PR goal"
sprint_goal_ref: "not_applicable: standalone tooling defect"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/981"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/981"
  - kind: "stp"
    ref: ".csdlc/issues/981/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/981/cards/sip.md"
scope:
  files:
    - "csdlc-v3/src/storage/semantic.rs; csdlc-v3/src/commands/remote/mod.rs; csdlc-v3/src/commands/proof/intent.rs; csdlc-v3/tests/transactions.rs; issue-local lifecycle and evidence records."
  components:
    - "repository-scoped-issue-creation-receipt"
  out_of_scope:
    - "No migration bypass, GitHub effect redesign, provider implementation, v2 lifecycle fallback, or unauthorized merge."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Reuse canonical remote intent and receipt validation for the narrow repository-scoped creation exemption; prove the valid case and six fail-closed mismatches; align proof admission with the safe Cargo filter already accepted in the plan; complete native proof, independent review, publication, CI, and terminal closeout."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: No product dependency. The native defect blocks ordinary preparation of #970, so #981 was bootstrapped with an isolated repaired candidate."
    expected_output: ".csdlc/issues/981/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Issue #981, the reconciled native issue-create transaction for #970, canonical remote intent and receipt validators, and semantic transaction fixtures."
    expected_output: ".csdlc/issues/981/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: A narrow canonical repository-scoped creation-receipt classifier, one positive regression, six negative linkage regressions, consistent safe Cargo test-filter admission through native proof, and truthful lifecycle evidence."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: A valid assigned-number receipt linked to a canonical issue-zero creation intent does not trigger legacy migration; operation digest, filename, marker, request, adapter, receipt, intent digest, repository, assigned issue, and head mismatches fail closed; one safe positional Cargo test filter accepted in the retained plan reaches proof while a second remains rejected; native proof executes nonzero tests and focused/full validation pass."
    expected_output: "validation evidence recorded in VPP/SOR"
    allowed_mode: "execution_after_approval"
  - id: "step-5"
    description: "Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges."
    expected_output: "reviewed SRP and truthful VPP/SOR"
    allowed_mode: "execution_after_approval"
codex_plan:
  - step: "Confirm dependencies and starting state from the source issue prompt."
    status: "completed"
  - step: "Inspect repo inputs and target surfaces before editing."
    status: "completed"
  - step: "Implement the bounded deliverables only."
    status: "completed"
  - step: "Run focused validation and proof gates."
    status: "in_progress"
  - step: "Record issue-specific SRP findings and VPP/SOR outcome truth."
    status: "pending"
affected_areas:
  - "repository-scoped-issue-creation-receipt"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Cross-linked or damaged evidence must produce RecoveryRequired instead of being ignored; issue-local receipts must continue to classify as legacy residue."
test_strategy:
  - "Run the positive and negative creation-receipt regressions, semantic_gate_a, the full csdlc-v3 suite, formatting, strict clippy, diff hygiene, and independent exact-head review."
execution_handoff: "Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges."
required_permissions:
  - "workspace-write after execution approval"
stop_conditions:
  - "Stop and re-plan if dependencies are unmet or materially different from this design-time plan."
  - "Stop and update SPP if touched files, proof gates, or validation commands change materially."
  - "Stop and route follow-on work if acceptance requires scope outside this issue."
alternatives_considered:
  - description: "Rely only on transient chat planning."
    reason_not_chosen: "Chat-only planning is not durable or reviewable enough for this workflow surface."
review_hooks:
  - "Check dependency truth, scope truthfulness, touched-file truthfulness, validation sufficiency, and re-plan triggers."
notes: "Implementation and local proof are complete in the bound worktree. The regression covers authentic repository-scoped mutation receipts and fail-closed recovery behavior for damaged or cross-linked evidence. Independent exact-head review, publication, CI, merge, and closeout remain pending."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][C-SDLC v3][defect] Do not misclassify repository-scoped issue creation receipts`.

Reuse canonical remote intent and receipt validation for the narrow repository-scoped creation exemption; prove the valid case and six fail-closed mismatches; align proof admission with the safe Cargo filter already accepted in the plan; complete native proof, independent review, publication, CI, and terminal closeout.

## PVF Lane Plan

- Initial PVF lane from issue creation: `csdlc`
- Planned PVF lane for execution: `csdlc`
- Planning lane source: `Issue #981 semantic transaction acceptance surface`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `unknown`
- Estimated total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Issue goal token budget: `unknown`
- Variance threshold percent: `10`
- Estimate confidence: `unknown`
- Estimate data source: `not_collected`
- Estimate source ref: `issue-981`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Goal Accounting Plan

Carry `issue_goal_ref`, `sprint_goal_ref`, and `goal_metrics_rollup_ref` in frontmatter so later tooling can roll planning and outcome metrics up without duplicating machine-local goal details in prose.

## Codex Plan

1. [completed] Confirm dependencies and starting state from the source issue prompt.
2. [completed] Inspect repo inputs and target surfaces before editing.
3. [completed] Implement the bounded deliverables only.
4. [in_progress] Run focused validation and proof gates.
5. [pending] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: No product dependency. The native defect blocks ordinary preparation of #970, so #981 was bootstrapped with an isolated repaired candidate.
2. Review repo inputs and scoped surfaces before editing: Issue #981, the reconciled native issue-create transaction for #970, canonical remote intent and receipt validators, and semantic transaction fixtures.
3. Implement only the bounded deliverables: A narrow canonical repository-scoped creation-receipt classifier, one positive regression, six negative linkage regressions, consistent safe Cargo test-filter admission through native proof, and truthful lifecycle evidence.
4. Run focused proof gates for acceptance: A valid assigned-number receipt linked to a canonical issue-zero creation intent does not trigger legacy migration; operation digest, filename, marker, request, adapter, receipt, intent digest, repository, assigned issue, and head mismatches fail closed; one safe positional Cargo test filter accepted in the retained plan reaches proof while a second remains rejected; native proof executes nonzero tests and focused/full validation pass.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- repository-scoped-issue-creation-receipt

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Cross-linked or damaged evidence must produce RecoveryRequired instead of being ignored; issue-local receipts must continue to classify as legacy residue.

## Test Strategy

- Run the positive and negative creation-receipt regressions, semantic_gate_a, the full csdlc-v3 suite, formatting, strict clippy, diff hygiene, and independent exact-head review.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Implementation and local proof are complete in the bound worktree. The regression covers authentic repository-scoped mutation receipts and fail-closed recovery behavior for damaged or cross-linked evidence. Independent exact-head review, publication, CI, merge, and closeout remain pending.
