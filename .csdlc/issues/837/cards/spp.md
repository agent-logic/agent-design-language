---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "active-boot-paths-control-plane-guidance-execution-plan"
issue: 837
task_id: "issue-0837"
run_id: "issue-0837"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06.22][architecture] Publish active boot paths and retire stale control-plane guidance"
branch: "codex/837-active-boot-paths-control-plane-guidance"
generated_at: "2026-09-10T23:00:00Z"
card_status: "ready"
status: "READY"
activation_state: "active"
plan_revision: 1
initial_pvf_lane: "review_docs"
planned_pvf_lane: "review_docs"
planned_pvf_lane_source: "docs/validation/pvf_lanes.json"
estimate_elapsed_seconds: "1800"
estimate_total_tokens: "not_collected"
estimate_validation_seconds: "180"
issue_goal_token_budget: "not_collected"
variance_threshold_percent: "10"
estimate_confidence: "medium"
estimate_data_source: "Issue scope and focused docs-validator precedent"
estimate_source_ref: "Issue #837 authored contract"
issue_goal_ref: "Active issue #837 execution goal"
sprint_goal_ref: "Parent #522"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/837"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/837"
  - kind: "stp"
    ref: ".csdlc/issues/837/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/837/cards/sip.md"
scope:
  files:
    - "Issue #837 inventory, active authority docs, narrow current-guidance repairs, and focused validator."
  components:
    - "active-boot-paths-control-plane-guidance"
  out_of_scope:
    - "No rollback-source deletion or lifecycle/product merger."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Inventory executable/source/selector/config evidence per subsystem, repair stale current guidance, add a deterministic ambiguity guard, review, and publish."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: #505 / PR #591 and parent #522."
    expected_output: ".csdlc/issues/837/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: AGENTS.md; CURRENT_AUTHORITY.md; authority selector; installed owner manifests; CLI help and Runtime/Guardian sources."
    expected_output: ".csdlc/issues/837/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Boot-path table, repairs, validator, negative fixture, review evidence."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: One ordinary v3 lifecycle path; product surfaces classified; v2 rollback-only; all active surfaces agree."
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
  - "active-boot-paths-control-plane-guidance"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Historical or rollback text could be falsely classified as current guidance."
test_strategy:
  - "Source inventory, focused docs/path contract, injected stale-v2-current negative, diff hygiene."
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
notes: "The validator must use explicit scoped allowlists rather than broad repository prose bans."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.1][TAIL-06.22][architecture] Publish active boot paths and retire stale control-plane guidance`.

Inventory executable/source/selector/config evidence per subsystem, repair stale current guidance, add a deterministic ambiguity guard, review, and publish.

## PVF Lane Plan

- Initial PVF lane from issue creation: `review_docs`
- Planned PVF lane for execution: `review_docs`
- Planning lane source: `docs/validation/pvf_lanes.json`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `1800`
- Estimated total tokens: `not_collected`
- Estimated validation seconds: `180`
- Issue goal token budget: `not_collected`
- Variance threshold percent: `10`
- Estimate confidence: `medium`
- Estimate data source: `Issue scope and focused docs-validator precedent`
- Estimate source ref: `Issue #837 authored contract`
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

1. Confirm dependency readiness and starting state: #505 / PR #591 and parent #522.
2. Review repo inputs and scoped surfaces before editing: AGENTS.md; CURRENT_AUTHORITY.md; authority selector; installed owner manifests; CLI help and Runtime/Guardian sources.
3. Implement only the bounded deliverables: Boot-path table, repairs, validator, negative fixture, review evidence.
4. Run focused proof gates for acceptance: One ordinary v3 lifecycle path; product surfaces classified; v2 rollback-only; all active surfaces agree.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- active-boot-paths-control-plane-guidance

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Historical or rollback text could be falsely classified as current guidance.

## Test Strategy

- Source inventory, focused docs/path contract, injected stale-v2-current negative, diff hygiene.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

The validator must use explicit scoped allowlists rather than broad repository prose bans.
