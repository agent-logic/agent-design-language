---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "dynamic-agent-health-task-failures-execution-plan"
issue: 759
task_id: "issue-0759"
run_id: "issue-0759"
version: "1.0.4"
title: "[v0.92.1][TAIL-06.03][runtime] Isolate dynamic-agent health task failures"
branch: "codex/759-dynamic-agent-health-task-failures"
generated_at: "<timestamp>"
card_status: "ready"
status: "ready"
activation_state: "bound_execution_ready"
plan_revision: 1
initial_pvf_lane: "runtime-focused"
planned_pvf_lane: "runtime-focused-defect-regression"
planned_pvf_lane_source: "issue #759 acceptance criteria and AGENTS.md validation expectations"
estimate_elapsed_seconds: "3600"
estimate_total_tokens: "unknown"
estimate_validation_seconds: "1800"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "50"
estimate_confidence: "medium"
estimate_data_source: "issue #759 live contract and current control.rs inspection"
estimate_source_ref: "https://github.com/agent-logic/agent-design-language/issues/759"
issue_goal_ref: "Codex goal: Issue #759 dynamic-agent health sweep remediation"
sprint_goal_ref: "v0.92.1 closeout tail runtime defect lane"
goal_metrics_rollup_ref: "v0.92.1"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/759"
  - kind: "source_issue_prompt"
    ref: "<source_issue_prompt>"
  - kind: "stp"
    ref: ".csdlc/issues/759/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/759/cards/sip.md"
scope:
  files:
    - "adl-runtime-kernel/src/control.rs and narrowly coupled Runtime health tests"
  components:
    - "dynamic-agent-health-task-failures"
  out_of_scope:
    - "provider semantics changes, broad Runtime refactors, hiding task failures"
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Implement the minimal health-sweep fix in `refresh_dynamic_agent_health` so the JoinSet loop drains all results, records task failures against stable agent identity, and preserves successful peer projections."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: issue #759 is open and bound; parent #522/source #520 are evidence inputs, not execution blockers"
    expected_output: ".csdlc/issues/759/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: issue #759, C520-CODE-003, candidate c24f8fa65ce445b03ce6cd69007307291d78b60c, control.rs health sweep"
    expected_output: ".csdlc/issues/759/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: draining JoinSet handling, stable failed identity projection, deterministic multi-agent regression, validation/review evidence"
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: failed/cancelled task does not abort peers; all checks drain; failed id visible; no false healthy downgrade"
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
    status: "pending"
  - step: "Run focused validation and proof gates."
    status: "pending"
  - step: "Record issue-specific SRP findings and VPP/SOR outcome truth."
    status: "pending"
affected_areas:
  - "dynamic-agent-health-task-failures"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "JoinError identity may require an external join-id map or local helper rather than relying on task return values."
test_strategy:
  - "run focused Runtime health regression, Runtime fmt, strict clippy/check; record failures truthfully"
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

Canonical Template Source: `docs/templates/prompts/1.0.4/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.1][TAIL-06.03][runtime] Isolate dynamic-agent health task failures`.

Implement the minimal health-sweep fix in `refresh_dynamic_agent_health` so the JoinSet loop drains all results, records task failures against stable agent identity, and preserves successful peer projections.

## PVF Lane Plan

- Initial PVF lane from issue creation: `runtime-focused`
- Planned PVF lane for execution: `runtime-focused-defect-regression`
- Planning lane source: `issue #759 acceptance criteria and AGENTS.md validation expectations`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `3600`
- Estimated total tokens: `unknown`
- Estimated validation seconds: `1800`
- Issue goal token budget: `unknown`
- Variance threshold percent: `50`
- Estimate confidence: `medium`
- Estimate data source: `issue #759 live contract and current control.rs inspection`
- Estimate source ref: `https://github.com/agent-logic/agent-design-language/issues/759`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Goal Accounting Plan

Carry `issue_goal_ref`, `sprint_goal_ref`, and `goal_metrics_rollup_ref` in frontmatter so later tooling can roll planning and outcome metrics up without duplicating machine-local goal details in prose.

## Codex Plan

1. [completed] Confirm dependencies and starting state from the source issue prompt.
2. [completed] Inspect repo inputs and target surfaces before editing.
3. [pending] Implement the bounded deliverables only.
4. [pending] Run focused validation and proof gates.
5. [pending] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: issue #759 is open and bound; parent #522/source #520 are evidence inputs, not execution blockers
2. Review repo inputs and scoped surfaces before editing: issue #759, C520-CODE-003, candidate c24f8fa65ce445b03ce6cd69007307291d78b60c, control.rs health sweep
3. Implement only the bounded deliverables: draining JoinSet handling, stable failed identity projection, deterministic multi-agent regression, validation/review evidence
4. Run focused proof gates for acceptance: failed/cancelled task does not abort peers; all checks drain; failed id visible; no false healthy downgrade
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- dynamic-agent-health-task-failures

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- JoinError identity may require an external join-id map or local helper rather than relying on task return values.

## Test Strategy

- run focused Runtime health regression, Runtime fmt, strict clippy/check; record failures truthfully

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

<notes_risks_inline>
