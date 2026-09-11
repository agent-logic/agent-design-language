---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "<slug>-execution-plan"
issue: 526
task_id: "issue-0526"
run_id: "issue-0526"
version: "v0.92.1"
title: "[v0.92.1][TAIL-10] Release ceremony"
branch: "codex/526-release-ceremony"
generated_at: "<timestamp>"
card_status: "ready"
status: "<status>"
activation_state: "<activation_state>"
plan_revision: 1
initial_pvf_lane: "release-evidence"
planned_pvf_lane: "release-evidence"
planned_pvf_lane_source: "#526 tail-merge-census, notes-parity and tag/release readback"
estimate_elapsed_seconds: "<estimate_elapsed_seconds>"
estimate_total_tokens: "<estimate_total_tokens>"
estimate_validation_seconds: "<estimate_validation_seconds>"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "<variance_threshold_percent>"
estimate_confidence: "<estimate_confidence>"
estimate_data_source: "<estimate_data_source>"
estimate_source_ref: "<estimate_source_ref>"
issue_goal_ref: "<issue_goal_ref>"
sprint_goal_ref: "<sprint_goal_ref>"
goal_metrics_rollup_ref: "<goal_metrics_rollup_ref>"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/526"
  - kind: "source_issue_prompt"
    ref: "<source_issue_prompt>"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
scope:
  files:
    - "docs/milestones/v0.92.1/evidence/release/tail-10/**; release notes only if a bounded correction is needed."
  components:
    - "<slug>"
  out_of_scope:
    - "No tag creation, release publication, merge, issue closure or cleanup; no replacement of missing prerequisite proof."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Assemble live prerequisite and canonical evidence inputs; record exact missing gates, notes hash and proposed tag; prepare execution/readback checklist; review preparation and stop before operator release approval."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: Preparation can proceed now. Ceremony requires all prior reviewed-green ancestral merges including #522 and #525, refreshed final gate, exact notes/candidate, explicit operator authorization."
    expected_output: "<sip_card>"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: #526, #525 and all prior TAIL issues; current-status projection and release notes; canonical release plan."
    expected_output: "<stp_card>"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Release evidence report JSON/Markdown; tail issue/PR ancestry census; pending approval and tag/release receipt fields; ordered ceremony checklist."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: Preparation truth separates open requirements from completed proof. Actual ceremony cannot claim ready until all prior tail merges, exact-candidate notes and operator authorization are verified."
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
    status: "completed"
affected_areas:
  - "<slug>"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Merged preparation PRs do not discharge final proof; current status may be stale; docs CI is not full release coverage."
test_strategy:
  - "Native card validation, source hash/path checks, live issue/PR/tag/release readback, deterministic prerequisite census and bounded independent preparation review."
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
notes: "Preparation complete; actual ceremony awaits #522/#525, final proof, version/authority preflight repair and exact operator approval."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.1][TAIL-10] Release ceremony`.

Assemble live prerequisite and canonical evidence inputs; record exact missing gates, notes hash and proposed tag; prepare execution/readback checklist; review preparation and stop before operator release approval.

## PVF Lane Plan

- Initial PVF lane from issue creation: `release-evidence`
- Planned PVF lane for execution: `release-evidence`
- Planning lane source: `#526 tail-merge-census, notes-parity and tag/release readback`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `<estimate_elapsed_seconds>`
- Estimated total tokens: `<estimate_total_tokens>`
- Estimated validation seconds: `<estimate_validation_seconds>`
- Issue goal token budget: `unknown`
- Variance threshold percent: `<variance_threshold_percent>`
- Estimate confidence: `<estimate_confidence>`
- Estimate data source: `<estimate_data_source>`
- Estimate source ref: `<estimate_source_ref>`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Goal Accounting Plan

Carry `issue_goal_ref`, `sprint_goal_ref`, and `goal_metrics_rollup_ref` in frontmatter so later tooling can roll planning and outcome metrics up without duplicating machine-local goal details in prose.

## Codex Plan

1. [completed] Confirm dependencies and starting state from the source issue prompt.
2. [completed] Inspect repo inputs and target surfaces before editing.
3. [completed] Implement the bounded deliverables only.
4. [completed] Run focused validation and proof gates.
5. [completed] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: Preparation can proceed now. Ceremony requires all prior reviewed-green ancestral merges including #522 and #525, refreshed final gate, exact notes/candidate, explicit operator authorization.
2. Review repo inputs and scoped surfaces before editing: #526, #525 and all prior TAIL issues; current-status projection and release notes; canonical release plan.
3. Implement only the bounded deliverables: Release evidence report JSON/Markdown; tail issue/PR ancestry census; pending approval and tag/release receipt fields; ordered ceremony checklist.
4. Run focused proof gates for acceptance: Preparation truth separates open requirements from completed proof. Actual ceremony cannot claim ready until all prior tail merges, exact-candidate notes and operator authorization are verified.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- <slug>

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Merged preparation PRs do not discharge final proof; current status may be stale; docs CI is not full release coverage.

## Test Strategy

- Native card validation, source hash/path checks, live issue/PR/tag/release readback, deterministic prerequisite census and bounded independent preparation review.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Preparation complete; actual ceremony awaits #522/#525, final proof, version/authority preflight repair and exact operator approval.
