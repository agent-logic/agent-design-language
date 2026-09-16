---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "<slug>-execution-plan"
issue: 1039
task_id: "issue-1039"
run_id: "issue-1039"
version: "1.0.5"
title: "[v0.92.2][C-SDLC v3][defect] Make terminal cleanup preview and execute agree for stale tracked projections"
branch: "codex/1039-terminal-cleanup-projection-parity"
generated_at: "<timestamp>"
card_status: "ready"
status: "ready"
activation_state: "prepared"
plan_revision: 1
initial_pvf_lane: "csdlc"
planned_pvf_lane: "csdlc"
planned_pvf_lane_source: "Issue #1039 terminal cleanup control-plane surface"
estimate_elapsed_seconds: "unknown"
estimate_total_tokens: "unknown"
estimate_validation_seconds: "unknown"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "10"
estimate_confidence: "unknown"
estimate_data_source: "not_collected"
estimate_source_ref: "issue-1039"
issue_goal_ref: "Issue #1039 implementation and reviewed PR"
sprint_goal_ref: "not_applicable: standalone closeout defect"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1039"
  - kind: "source_issue_prompt"
    ref: "<source_issue_prompt>"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
scope:
  files:
    - "Semantic context, terminal cleanup adapter, focused installed regression, issue-local proof and review evidence."
  components:
    - "<slug>"
  out_of_scope:
    - "No broad projection bypass, legacy adoption, or dirty cleanup."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Add one cleanup-only semantic context constructor that accepts canonical ProjectionRepairRequired state without rewriting tracked projections, then prove token parity and unchanged negative guards."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: No product dependency; #1039 is the tooling blocker for #978/#1018 cleanup."
    expected_output: "<sip_card>"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Issue #1039; live #1018 preview/execute failure; semantic context and terminal cleanup code; installed cleanup fixtures."
    expected_output: "<stp_card>"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Cleanup-scoped projection-tolerant semantic admission plus exact positive and negative regression proof."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: Preview token executes once for the exact clean terminal candidate; changed authority/head/topology/receipt/archive/token and dirty worktrees still fail closed."
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
    status: "pending"
  - step: "Implement the bounded deliverables only."
    status: "pending"
  - step: "Run focused validation and proof gates."
    status: "pending"
  - step: "Record issue-specific SRP findings and VPP/SOR outcome truth."
    status: "pending"
affected_areas:
  - "<slug>"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Cleanup removes a checkout, so the exception must remain exact and command-scoped."
test_strategy:
  - "Focused regression, adjacent semantic cleanup suite, native proof, fmt, strict clippy, diff hygiene, independent review."
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
notes: "Do not reuse this admission for proof, review, publication, or ordinary mutation."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][C-SDLC v3][defect] Make terminal cleanup preview and execute agree for stale tracked projections`.

Add one cleanup-only semantic context constructor that accepts canonical ProjectionRepairRequired state without rewriting tracked projections, then prove token parity and unchanged negative guards.

## PVF Lane Plan

- Initial PVF lane from issue creation: `csdlc`
- Planned PVF lane for execution: `csdlc`
- Planning lane source: `Issue #1039 terminal cleanup control-plane surface`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `unknown`
- Estimated total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Issue goal token budget: `unknown`
- Variance threshold percent: `10`
- Estimate confidence: `unknown`
- Estimate data source: `not_collected`
- Estimate source ref: `issue-1039`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Goal Accounting Plan

Carry `issue_goal_ref`, `sprint_goal_ref`, and `goal_metrics_rollup_ref` in frontmatter so later tooling can roll planning and outcome metrics up without duplicating machine-local goal details in prose.

## Codex Plan

1. [completed] Confirm dependencies and starting state from the source issue prompt.
2. [pending] Inspect repo inputs and target surfaces before editing.
3. [pending] Implement the bounded deliverables only.
4. [pending] Run focused validation and proof gates.
5. [pending] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: No product dependency; #1039 is the tooling blocker for #978/#1018 cleanup.
2. Review repo inputs and scoped surfaces before editing: Issue #1039; live #1018 preview/execute failure; semantic context and terminal cleanup code; installed cleanup fixtures.
3. Implement only the bounded deliverables: Cleanup-scoped projection-tolerant semantic admission plus exact positive and negative regression proof.
4. Run focused proof gates for acceptance: Preview token executes once for the exact clean terminal candidate; changed authority/head/topology/receipt/archive/token and dirty worktrees still fail closed.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- <slug>

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Cleanup removes a checkout, so the exception must remain exact and command-scoped.

## Test Strategy

- Focused regression, adjacent semantic cleanup suite, native proof, fmt, strict clippy, diff hygiene, independent review.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Do not reuse this admission for proof, review, publication, or ordinary mutation.
