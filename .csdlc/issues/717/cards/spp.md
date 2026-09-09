---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "polis-capability-orientation-execution-plan"
issue: 717
task_id: "issue-0717"
run_id: "issue-0717"
version: "1.0.4"
title: "[v0.92.2][Runtime] Teach admitted agents about Polis modules and capabilities"
branch: "codex/717-polis-capability-orientation"
generated_at: "2026-09-09T18:30:00Z"
card_status: "approved"
status: "ready"
activation_state: "approved_for_execution"
plan_revision: 1
initial_pvf_lane: "runtime-focused"
planned_pvf_lane: "runtime-focused-docs-contract"
planned_pvf_lane_source: "issue-717-acceptance"
estimate_elapsed_seconds: "3600"
estimate_total_tokens: "12000"
estimate_validation_seconds: "900"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "20"
estimate_confidence: "medium"
estimate_data_source: "issue contract and current code inspection"
estimate_source_ref: "issue #717"
issue_goal_ref: "issue-717-session-goal"
sprint_goal_ref: "v0.92.1-bugfix"
goal_metrics_rollup_ref: "v0.92.1-bugfix"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/717"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/717"
  - kind: "stp"
    ref: ".csdlc/issues/717/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/717/cards/sip.md"
scope:
  files:
    - "Welcome Package, agent_orientation validation/tests, and bounded v0.92.1/v0.92.2 planning reconciliation"
  components:
    - "polis-capability-orientation"
  out_of_scope:
    - "new authority, service behavior changes, #718 implementation, live provider/cloud mutation"
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Expand the compact Welcome Package, bind its operational service entries to REQUIRED_OPERATIONAL_ADAPTERS, validate all declared capability markers exactly once, and reconcile milestone truth."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: completed #708/#709 delivery path and current Runtime registries"
    expected_output: ".csdlc/issues/717/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: issue #717, operational adapter inventory, architecture docs, explainer docs, existing orientation code"
    expected_output: ".csdlc/issues/717/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: welcome prose, inventory guard, negative tests, planning updates, focused proof"
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: complete capability map and authority distinctions delivered before first turn; deterministic drift rejection; provenance preserved"
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
    status: "pending"
  - step: "Record issue-specific SRP findings and VPP/SOR outcome truth."
    status: "pending"
affected_areas:
  - "polis-capability-orientation"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "stale prose, duplicate registry truth, excess prompt size, false deployment availability claims"
test_strategy:
  - "agent_orientation tests, adjacent first-turn delivery tests, formatting, diff hygiene, and bounded subagent review"
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
notes: "Use stable machine-readable markers solely as validation anchors; keep natural-language guidance primary."
---

Canonical Template Source: `docs/templates/prompts/1.0.4/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][Runtime] Teach admitted agents about Polis modules and capabilities`.

Expand the compact Welcome Package, bind its operational service entries to REQUIRED_OPERATIONAL_ADAPTERS, validate all declared capability markers exactly once, and reconcile milestone truth.

## PVF Lane Plan

- Initial PVF lane from issue creation: `runtime-focused`
- Planned PVF lane for execution: `runtime-focused-docs-contract`
- Planning lane source: `issue-717-acceptance`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `3600`
- Estimated total tokens: `12000`
- Estimated validation seconds: `900`
- Issue goal token budget: `unknown`
- Variance threshold percent: `20`
- Estimate confidence: `medium`
- Estimate data source: `issue contract and current code inspection`
- Estimate source ref: `issue #717`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Goal Accounting Plan

Carry `issue_goal_ref`, `sprint_goal_ref`, and `goal_metrics_rollup_ref` in frontmatter so later tooling can roll planning and outcome metrics up without duplicating machine-local goal details in prose.

## Codex Plan

1. [completed] Confirm dependencies and starting state from the source issue prompt.
2. [completed] Inspect repo inputs and target surfaces before editing.
3. [in_progress] Implement the bounded deliverables only.
4. [pending] Run focused validation and proof gates.
5. [pending] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: completed #708/#709 delivery path and current Runtime registries
2. Review repo inputs and scoped surfaces before editing: issue #717, operational adapter inventory, architecture docs, explainer docs, existing orientation code
3. Implement only the bounded deliverables: welcome prose, inventory guard, negative tests, planning updates, focused proof
4. Run focused proof gates for acceptance: complete capability map and authority distinctions delivered before first turn; deterministic drift rejection; provenance preserved
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- polis-capability-orientation

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- stale prose, duplicate registry truth, excess prompt size, false deployment availability claims

## Test Strategy

- agent_orientation tests, adjacent first-turn delivery tests, formatting, diff hygiene, and bounded subagent review

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Use stable machine-readable markers solely as validation anchors; keep natural-language guidance primary.
