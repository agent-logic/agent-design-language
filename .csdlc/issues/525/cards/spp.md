---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "next-milestone-review-execution-plan"
issue: 525
task_id: "issue-0525"
run_id: "issue-0525"
version: "v0.92.1"
title: "[v0.92.1][TAIL-09] Next milestone review pass"
branch: "codex/525-next-milestone-review"
generated_at: "2026-09-11T00:00:00Z"
card_status: "ready"
status: "READY"
activation_state: "active"
plan_revision: 1
initial_pvf_lane: "docs_only"
planned_pvf_lane: "docs_only"
planned_pvf_lane_source: "docs/validation/pvf_lanes.json"
estimate_elapsed_seconds: "not_collected"
estimate_total_tokens: "not_collected"
estimate_validation_seconds: "not_collected"
issue_goal_token_budget: "not_collected"
variance_threshold_percent: "10"
estimate_confidence: "medium"
estimate_data_source: "operator scope and current planning corpus"
estimate_source_ref: "issue-525"
issue_goal_ref: "Issue #525 session goal"
sprint_goal_ref: "v0.92.1 release tail"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/525"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/525"
  - kind: "stp"
    ref: ".csdlc/issues/525/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/525/cards/sip.md"
scope:
  files:
    - "Canonical v0.92.2 planning documents and issue-local #525 records."
  components:
    - "next-milestone-review"
  out_of_scope:
    - "No issue creation, Terraform apply, cloud mutation, implementation, or customer-scale hosting."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Inspect every canonical projection; add OBS-S3 and ARCH-ADR without issue numbers; reconcile dependencies, denominator, readiness and non-goals; validate; review exact head."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: No current execution dependency; WP-01 performs future issue creation."
    expected_output: ".csdlc/issues/525/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Issue #525, current v0.92.2 documents, #679/PR #685, and #720."
    expected_output: ".csdlc/issues/525/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Two bounded planned rows plus consistent milestone projections and validator coverage."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: All identifiers, counts, dependencies, issue bindings, proof gates and non-goals agree."
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
  - "next-milestone-review"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Cross-document denominator or dependency drift."
test_strategy:
  - "Focused planning self-test, native cards, diff hygiene and bounded review."
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
notes: "Fail closed on projection disagreement."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.1][TAIL-09] Next milestone review pass`.

Inspect every canonical projection; add OBS-S3 and ARCH-ADR without issue numbers; reconcile dependencies, denominator, readiness and non-goals; validate; review exact head.

## PVF Lane Plan

- Initial PVF lane from issue creation: `docs_only`
- Planned PVF lane for execution: `docs_only`
- Planning lane source: `docs/validation/pvf_lanes.json`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `not_collected`
- Estimated total tokens: `not_collected`
- Estimated validation seconds: `not_collected`
- Issue goal token budget: `not_collected`
- Variance threshold percent: `10`
- Estimate confidence: `medium`
- Estimate data source: `operator scope and current planning corpus`
- Estimate source ref: `issue-525`
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

1. Confirm dependency readiness and starting state: No current execution dependency; WP-01 performs future issue creation.
2. Review repo inputs and scoped surfaces before editing: Issue #525, current v0.92.2 documents, #679/PR #685, and #720.
3. Implement only the bounded deliverables: Two bounded planned rows plus consistent milestone projections and validator coverage.
4. Run focused proof gates for acceptance: All identifiers, counts, dependencies, issue bindings, proof gates and non-goals agree.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- next-milestone-review

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Cross-document denominator or dependency drift.

## Test Strategy

- Focused planning self-test, native cards, diff hygiene and bounded review.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Fail closed on projection disagreement.
