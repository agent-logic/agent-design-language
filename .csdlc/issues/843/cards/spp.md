---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "legacy-ready-intent-recovery-execution-plan"
issue: 843
task_id: "issue-0843"
run_id: "issue-0843"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06.23][tooling] Recover retained ready intents without target identity"
branch: "codex/843-legacy-ready-intent-recovery"
generated_at: "2026-09-11T15:33:13Z"
card_status: "ready"
status: "IN_PROGRESS"
activation_state: "active"
plan_revision: 1
initial_pvf_lane: "tooling"
planned_pvf_lane: "tooling"
planned_pvf_lane_source: "docs/validation/pvf_lanes.json"
estimate_elapsed_seconds: "unknown"
estimate_total_tokens: "unknown"
estimate_validation_seconds: "unknown"
issue_goal_token_budget: "not_collected"
variance_threshold_percent: "10"
estimate_confidence: "medium"
estimate_data_source: "bounded follow-on after reproduced defect"
estimate_source_ref: "issue #843"
issue_goal_ref: "Active #522 remediation goal including child #843"
sprint_goal_ref: "Parent #522"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/843"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/843"
  - kind: "stp"
    ref: ".csdlc/issues/843/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/843/cards/sip.md"
scope:
  files:
    - "csdlc-v3 remote mutation state machine, focused tests, and issue #843 proof packet."
  components:
    - "legacy-ready-intent-recovery"
  out_of_scope:
    - "No broad GitHub mutation redesign, raw gh lifecycle write, or stale #840 ready claim."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Legacy ready recovery repaired and tested;225 locked suite tests, strict Clippy, current V3-F binding and15 negative cases pass. Publish reviewed fix; refresh release projection only after merge."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: Parent #522 and merged #824; #835 proof remains failed pending this repair."
    expected_output: ".csdlc/issues/843/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Issue #843; parent #522; #835 current-candidate finding; merged #824/PR #830 implementation."
    expected_output: ".csdlc/issues/843/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Narrow recovery fix, regression tests, issue proof, and refreshed #835 handoff."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: Automatic matching legacy recovery; mismatch rejection; idempotent repeated recovery; focused V3-F proof and exact-head review."
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
  - "legacy-ready-intent-recovery"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Never dispatch until exact authenticated target identity is durably retained."
test_strategy:
  - "Focused remote tests, exact legacy-intent regression, V3-F current-source validator, fmt, clippy, diff hygiene, and independent exact-head review."
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
notes: "Fail closed before mutation."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.1][TAIL-06.23][tooling] Recover retained ready intents without target identity`.

Legacy ready recovery repaired and tested;225 locked suite tests, strict Clippy, current V3-F binding and15 negative cases pass. Publish reviewed fix; refresh release projection only after merge.

## PVF Lane Plan

- Initial PVF lane from issue creation: `tooling`
- Planned PVF lane for execution: `tooling`
- Planning lane source: `docs/validation/pvf_lanes.json`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `unknown`
- Estimated total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Issue goal token budget: `not_collected`
- Variance threshold percent: `10`
- Estimate confidence: `medium`
- Estimate data source: `bounded follow-on after reproduced defect`
- Estimate source ref: `issue #843`
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

1. Confirm dependency readiness and starting state: Parent #522 and merged #824; #835 proof remains failed pending this repair.
2. Review repo inputs and scoped surfaces before editing: Issue #843; parent #522; #835 current-candidate finding; merged #824/PR #830 implementation.
3. Implement only the bounded deliverables: Narrow recovery fix, regression tests, issue proof, and refreshed #835 handoff.
4. Run focused proof gates for acceptance: Automatic matching legacy recovery; mismatch rejection; idempotent repeated recovery; focused V3-F proof and exact-head review.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- legacy-ready-intent-recovery

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Never dispatch until exact authenticated target identity is durably retained.

## Test Strategy

- Focused remote tests, exact legacy-intent regression, V3-F current-source validator, fmt, clippy, diff hygiene, and independent exact-head review.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Fail closed before mutation.
