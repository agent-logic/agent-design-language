---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "greeting-recovery-local-model-boundary-execution-plan"
issue: 814
task_id: "issue-0814"
run_id: "issue-0814"
version: "1.0.5"
title: "[v0.92.1][TAIL-06.09][runtime] Repair greeting recovery identity and local-model boundary"
branch: "codex/814-greeting-recovery-local-model-boundary"
generated_at: "<timestamp>"
card_status: "ready"
status: "ready"
activation_state: "active"
plan_revision: 1
initial_pvf_lane: "runtime_focused"
planned_pvf_lane: "runtime_focused"
planned_pvf_lane_source: "issue-814"
estimate_elapsed_seconds: "1800"
estimate_total_tokens: "12000"
estimate_validation_seconds: "900"
issue_goal_token_budget: "not_budgeted"
variance_threshold_percent: "10"
estimate_confidence: "medium"
estimate_data_source: "bounded source and test scope"
estimate_source_ref: "https://github.com/agent-logic/agent-design-language/issues/814"
issue_goal_ref: "codex-goal:issue-814"
sprint_goal_ref: "issue-520-review-remediation"
goal_metrics_rollup_ref: "issue-522"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/814"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/814"
  - kind: "stp"
    ref: ".csdlc/issues/814/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/814/cards/sip.md"
scope:
  files:
    - "adl-runtime-kernel/src/control.rs; adl-runtime-kernel/src/ingress.rs; adl-runtime/tests/shepherd_local_model.rs; tightly coupled tests."
  components:
    - "greeting-recovery-local-model-boundary"
  out_of_scope:
    - "Provider selection, deployment architecture, and broad Runtime refactoring."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Reproduce each review finding with a focused regression, repair the bounded greeting state and URL-validation paths, model stable logical DomainWork identity separately from adapter-attempt execution identity, then run Runtime-focused proof and independent exact-head review."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: #718 and #758 merged; #520 candidate and findings retained."
    expected_output: ".csdlc/issues/814/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Issue #814, parent #522, review #520, Runtime control source, Shepherd local-model tests."
    expected_output: ".csdlc/issues/814/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Crash-safe retry repair; immutable logical work identity; structural localhost URL and redirect validation; focused regressions."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: AC-1 bounded final-attempt recovery; AC-2 stable logical key across retry surfaces; AC-3 unsafe URL and redirect forms rejected; AC-4 proving tests and exact-head review pass."
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
    status: "completed"
  - step: "Record issue-specific SRP findings and VPP/SOR outcome truth."
    status: "in_progress"
affected_areas:
  - "greeting-recovery-local-model-boundary"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Off-by-one recovery can poison durable load; identity drift can duplicate work; URL authority confusion can escape local-only proof."
test_strategy:
  - "Focused Runtime tests; Runtime owner validation lane; cargo fmt; diff hygiene; independent exact-head review."
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
notes: "<notes_risks_inline>"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.1][TAIL-06.09][runtime] Repair greeting recovery identity and local-model boundary`.

Reproduce each review finding with a focused regression, repair the bounded greeting state and URL-validation paths, model stable logical DomainWork identity separately from adapter-attempt execution identity, then run Runtime-focused proof and independent exact-head review.

## PVF Lane Plan

- Initial PVF lane from issue creation: `runtime_focused`
- Planned PVF lane for execution: `runtime_focused`
- Planning lane source: `issue-814`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `1800`
- Estimated total tokens: `12000`
- Estimated validation seconds: `900`
- Issue goal token budget: `not_budgeted`
- Variance threshold percent: `10`
- Estimate confidence: `medium`
- Estimate data source: `bounded source and test scope`
- Estimate source ref: `https://github.com/agent-logic/agent-design-language/issues/814`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Goal Accounting Plan

Carry `issue_goal_ref`, `sprint_goal_ref`, and `goal_metrics_rollup_ref` in frontmatter so later tooling can roll planning and outcome metrics up without duplicating machine-local goal details in prose.

## Codex Plan

1. [completed] Confirm dependencies and starting state from the source issue prompt.
2. [completed] Inspect repo inputs and target surfaces before editing.
3. [completed] Implement the bounded deliverables only.
4. [completed] Run focused validation and proof gates.
5. [in_progress] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: #718 and #758 merged; #520 candidate and findings retained.
2. Review repo inputs and scoped surfaces before editing: Issue #814, parent #522, review #520, Runtime control source, Shepherd local-model tests.
3. Implement only the bounded deliverables: Crash-safe retry repair; immutable logical work identity; structural localhost URL and redirect validation; focused regressions.
4. Run focused proof gates for acceptance: AC-1 bounded final-attempt recovery; AC-2 stable logical key across retry surfaces; AC-3 unsafe URL and redirect forms rejected; AC-4 proving tests and exact-head review pass.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- greeting-recovery-local-model-boundary

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Off-by-one recovery can poison durable load; identity drift can duplicate work; URL authority confusion can escape local-only proof.

## Test Strategy

- Focused Runtime tests; Runtime owner validation lane; cargo fmt; diff hygiene; independent exact-head review.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

<notes_risks_inline>
