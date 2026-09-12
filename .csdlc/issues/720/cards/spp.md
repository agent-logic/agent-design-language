---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "720-observatory-live-execution-plan"
issue: 720
task_id: "issue-0720"
run_id: "issue-0720"
version: "v0.92.2"
title: "[v0.92.2][Observatory] Remove retained-mode demo hazards"
branch: "codex/720-observatory-live"
generated_at: "2026-09-12T00:09:24.617447+00:00"
card_status: "ready"
status: "prepared"
activation_state: "active"
plan_revision: 1
initial_pvf_lane: "tooling"
planned_pvf_lane: "tooling"
planned_pvf_lane_source: "issue-local UI scope"
estimate_elapsed_seconds: "unknown"
estimate_total_tokens: "unknown"
estimate_validation_seconds: "1800"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "unknown"
estimate_confidence: "medium"
estimate_data_source: "issue-local estimate"
estimate_source_ref: "#720"
issue_goal_ref: "#720 execution goal created before implementation"
sprint_goal_ref: "#934: v0.92.2 Sprint 8 Cloud operations and Observatory"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/720"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/720"
  - kind: "stp"
    ref: ".csdlc/issues/720/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/720/cards/sip.md"
scope:
  files:
    - "demos/html-observatory/{app.js,index.html,README.md,tests/}; focused Observatory validators and .csdlc/evidence/720."
  components:
    - "720-observatory-live"
  out_of_scope:
    - "No cloud deployment, SDK refactor, UI redesign, historical evidence deletion, or merge."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Remove retained telemetry loaders, switches and timers; initialize an empty live shell, retain last-known live snapshot on disconnection with stale notice; preserve separately labelled historical evidence; prove transitions and publish reviewed change."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: Prepared native binding; parent #934 confirmed launch gate and authorized execution. No new #864 dependency."
    expected_output: ".csdlc/issues/720/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Issue #720; demos/html-observatory/app.js and index.html; current regression tests."
    expected_output: ".csdlc/issues/720/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Live-only app.js and index.html; focused executed tests; README evidence boundary; validation packet."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: No Published/Retained controls; startup, navigation and failure never read retained API telemetry; no retained timer; orphan assignments removed; functional live regression proof; truthful documentation."
    expected_output: "validation evidence recorded in VPP/SOR"
    allowed_mode: "execution_after_approval"
  - id: "step-5"
    description: "Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges."
    expected_output: "reviewed SRP and truthful VPP/SOR"
    allowed_mode: "execution_after_approval"
codex_plan:
  - step: "Confirm dependencies and starting state from the source issue prompt."
    status: "pending"
  - step: "Inspect repo inputs and target surfaces before editing."
    status: "pending"
  - step: "Implement the bounded deliverables only."
    status: "pending"
  - step: "Run focused validation and proof gates."
    status: "pending"
  - step: "Record issue-specific SRP findings and VPP/SOR outcome truth."
    status: "pending"
affected_areas:
  - "720-observatory-live"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Disconnected live values must be explicitly stale; initialization must not seed telemetry from an old packet. Historical integration evidence may remain labelled separately."
test_strategy:
  - "node --test demos/html-observatory/tests/*.test.mjs; focused live-only browser or DOM behavioral proof; existing Observatory validator where applicable; exact-head independent review and hosted CI."
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
notes: "Disconnected live values must be explicitly stale; initialization must not seed telemetry from an old packet. Historical integration evidence may remain labelled separately."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][Observatory] Remove retained-mode demo hazards`.

Remove retained telemetry loaders, switches and timers; initialize an empty live shell, retain last-known live snapshot on disconnection with stale notice; preserve separately labelled historical evidence; prove transitions and publish reviewed change.

## PVF Lane Plan

- Initial PVF lane from issue creation: `tooling`
- Planned PVF lane for execution: `tooling`
- Planning lane source: `issue-local UI scope`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `unknown`
- Estimated total tokens: `unknown`
- Estimated validation seconds: `1800`
- Issue goal token budget: `unknown`
- Variance threshold percent: `unknown`
- Estimate confidence: `medium`
- Estimate data source: `issue-local estimate`
- Estimate source ref: `#720`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Goal Accounting Plan

Carry `issue_goal_ref`, `sprint_goal_ref`, and `goal_metrics_rollup_ref` in frontmatter so later tooling can roll planning and outcome metrics up without duplicating machine-local goal details in prose.

## Codex Plan

1. [pending] Confirm dependencies and starting state from the source issue prompt.
2. [pending] Inspect repo inputs and target surfaces before editing.
3. [pending] Implement the bounded deliverables only.
4. [pending] Run focused validation and proof gates.
5. [pending] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: Prepared native binding; parent #934 confirmed launch gate and authorized execution. No new #864 dependency.
2. Review repo inputs and scoped surfaces before editing: Issue #720; demos/html-observatory/app.js and index.html; current regression tests.
3. Implement only the bounded deliverables: Live-only app.js and index.html; focused executed tests; README evidence boundary; validation packet.
4. Run focused proof gates for acceptance: No Published/Retained controls; startup, navigation and failure never read retained API telemetry; no retained timer; orphan assignments removed; functional live regression proof; truthful documentation.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- 720-observatory-live

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Disconnected live values must be explicitly stale; initialization must not seed telemetry from an old packet. Historical integration evidence may remain labelled separately.

## Test Strategy

- node --test demos/html-observatory/tests/*.test.mjs; focused live-only browser or DOM behavioral proof; existing Observatory validator where applicable; exact-head independent review and hosted CI.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Disconnected live values must be explicitly stale; initialization must not seed telemetry from an old packet. Historical integration evidence may remain labelled separately.
