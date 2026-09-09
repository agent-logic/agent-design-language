---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "runtime-a2a-closed-loop-execution-plan"
issue: 784
task_id: "issue-0784"
run_id: "issue-0784"
version: "1.0.4"
title: "[v0.92.1][Runtime] Return completed A2A replies to the initiating agent"
branch: "codex/784-runtime-a2a-closed-loop"
generated_at: "2026-09-09T16:40:00Z"
card_status: "approved"
status: "ready"
activation_state: "approved_for_execution"
plan_revision: 1
initial_pvf_lane: "runtime-focused"
planned_pvf_lane: "runtime-focused-plus-demo"
planned_pvf_lane_source: "issue-784-acceptance"
estimate_elapsed_seconds: "3600"
estimate_total_tokens: "12000"
estimate_validation_seconds: "900"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "20"
estimate_confidence: "medium"
estimate_data_source: "live defect reproduction and code inspection"
estimate_source_ref: "issue #784"
issue_goal_ref: "issue-784-session-goal"
sprint_goal_ref: "v0.92.1-closeout"
goal_metrics_rollup_ref: "v0.92.1-closeout"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/784"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/784"
  - kind: "stp"
    ref: ".csdlc/issues/784/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/784/cards/sip.md"
scope:
  files:
    - "adl-runtime-kernel/src/control.rs, adl-runtime-kernel/src/assembly.rs, and focused A2A tests"
  components:
    - "runtime-a2a-closed-loop"
  out_of_scope:
    - "canonical-name routing, cross-node transport, Shepherd identity, provider budgets"
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Reuse persisted A2A terminal data, project it as bounded causal context into the initiator's later model turn, and prove delegate-reply-synthesize behavior."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: existing A2A dispatch, terminal result, conversation checkpoint, and restore paths"
    expected_output: ".csdlc/issues/784/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: issue #784, live cooperation findings, current Runtime control and prompt assembly"
    expected_output: ".csdlc/issues/784/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: prior-result projection, prompt integration, deterministic three-call test, focused proof"
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: peer success or typed failure reaches the initiator context once with causal IDs and no operator relay"
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
    status: "in_progress"
  - step: "Run focused validation and proof gates."
    status: "in_progress"
  - step: "Record issue-specific SRP findings and VPP/SOR outcome truth."
    status: "pending"
affected_areas:
  - "runtime-a2a-closed-loop"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "duplicate context, unbounded prompt growth, loss of typed failure data, checkpoint incompatibility"
test_strategy:
  - "focused delegate/reply/synthesize test, adjacent conversation/A2A tests, formatting, diff hygiene, bounded review"
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
notes: "Re-plan if the existing terminal record cannot supply a bounded causal context without schema expansion."
---

Canonical Template Source: `docs/templates/prompts/1.0.4/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.1][Runtime] Return completed A2A replies to the initiating agent`.

Reuse persisted A2A terminal data, project it as bounded causal context into the initiator's later model turn, and prove delegate-reply-synthesize behavior.

## PVF Lane Plan

- Initial PVF lane from issue creation: `runtime-focused`
- Planned PVF lane for execution: `runtime-focused-plus-demo`
- Planning lane source: `issue-784-acceptance`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `3600`
- Estimated total tokens: `12000`
- Estimated validation seconds: `900`
- Issue goal token budget: `unknown`
- Variance threshold percent: `20`
- Estimate confidence: `medium`
- Estimate data source: `live defect reproduction and code inspection`
- Estimate source ref: `issue #784`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Goal Accounting Plan

Carry `issue_goal_ref`, `sprint_goal_ref`, and `goal_metrics_rollup_ref` in frontmatter so later tooling can roll planning and outcome metrics up without duplicating machine-local goal details in prose.

## Codex Plan

1. [completed] Confirm dependencies and starting state from the source issue prompt.
2. [completed] Inspect repo inputs and target surfaces before editing.
3. [in_progress] Implement the bounded deliverables only.
4. [in_progress] Run focused validation and proof gates.
5. [pending] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: existing A2A dispatch, terminal result, conversation checkpoint, and restore paths
2. Review repo inputs and scoped surfaces before editing: issue #784, live cooperation findings, current Runtime control and prompt assembly
3. Implement only the bounded deliverables: prior-result projection, prompt integration, deterministic three-call test, focused proof
4. Run focused proof gates for acceptance: peer success or typed failure reaches the initiator context once with causal IDs and no operator relay
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- runtime-a2a-closed-loop

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- duplicate context, unbounded prompt growth, loss of typed failure data, checkpoint incompatibility

## Test Strategy

- focused delegate/reply/synthesize test, adjacent conversation/A2A tests, formatting, diff hygiene, bounded review

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Re-plan if the existing terminal record cannot supply a bounded causal context without schema expansion.
