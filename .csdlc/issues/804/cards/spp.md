---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "sor-authority-notice-execution-plan"
issue: 804
task_id: "issue-0804"
run_id: "issue-0804"
version: "v0.92.1"
title: "[v0.92.1][defect] Align active SOR template authority notice with C-SDLC v3"
branch: "codex/804-sor-authority-notice"
generated_at: "2026-09-09T18:25:45.944998+00:00"
card_status: "ready"
status: "IN_PROGRESS"
activation_state: "active"
plan_revision: 1
initial_pvf_lane: "prompt_template"
planned_pvf_lane: "prompt_template"
planned_pvf_lane_source: "docs/templates/prompts/1.0.4/pvf_lane_policy.json"
estimate_elapsed_seconds: "not_collected"
estimate_total_tokens: "not_collected"
estimate_validation_seconds: "not_collected"
issue_goal_token_budget: "not_collected"
variance_threshold_percent: "10"
estimate_confidence: "not_collected"
estimate_data_source: "not_collected"
estimate_source_ref: "not_collected"
issue_goal_ref: "Issue #804 session goal in current Codex task"
sprint_goal_ref: "not_applicable: standalone defect"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/804"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/804"
  - kind: "stp"
    ref: ".csdlc/issues/804/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/804/cards/sip.md"
scope:
  files:
    - "docs/templates/prompts/1.0.4/sor.md and schemas/sor.structure.json"
  components:
    - "sor-authority-notice"
  out_of_scope:
    - "Historical versions, unrelated cards, runtime and #759 behavior"
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Inspect active notice/schema; replace only obsolete authority text; re-render local cards through native edit; validate parity and negative stale notice; independently review and publish."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: Completed #505/PR591 cutover; no pending dependency"
    expected_output: ".csdlc/issues/804/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Active registry and current v3 authority contract"
    expected_output: ".csdlc/issues/804/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Active SOR notice/schema correction and issue-local validation evidence."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: Active SOR template and schema match v3 authority; fresh native render validates; unrelated/historical cards and runtime unchanged."
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
  - "sor-authority-notice"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Existing historical cards are intentionally not rewritten."
test_strategy:
  - "Native csdlc edit and validate using issue-local requests and active current.json; python3 adl/tools/test_prompt_template_structure_schemas.py; git diff --check; authority parity assertions retained in .csdlc/evidence/804/authority-proof.json"
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
notes: "Historical cards retain source-time text; no historical rewrite or runtime authority change."
---

Canonical Template Source: `docs/templates/prompts/1.0.4/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.1][defect] Align active SOR template authority notice with C-SDLC v3`.

Inspect active notice/schema; replace only obsolete authority text; re-render local cards through native edit; validate parity and negative stale notice; independently review and publish.

## PVF Lane Plan

- Initial PVF lane from issue creation: `prompt_template`
- Planned PVF lane for execution: `prompt_template`
- Planning lane source: `docs/templates/prompts/1.0.4/pvf_lane_policy.json`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `not_collected`
- Estimated total tokens: `not_collected`
- Estimated validation seconds: `not_collected`
- Issue goal token budget: `not_collected`
- Variance threshold percent: `10`
- Estimate confidence: `not_collected`
- Estimate data source: `not_collected`
- Estimate source ref: `not_collected`
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

1. Confirm dependency readiness and starting state: Completed #505/PR591 cutover; no pending dependency
2. Review repo inputs and scoped surfaces before editing: Active registry and current v3 authority contract
3. Implement only the bounded deliverables: Active SOR notice/schema correction and issue-local validation evidence.
4. Run focused proof gates for acceptance: Active SOR template and schema match v3 authority; fresh native render validates; unrelated/historical cards and runtime unchanged.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- sor-authority-notice

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Existing historical cards are intentionally not rewritten.

## Test Strategy

- Native csdlc edit and validate using issue-local requests and active current.json; python3 adl/tools/test_prompt_template_structure_schemas.py; git diff --check; authority parity assertions retained in .csdlc/evidence/804/authority-proof.json

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Historical cards retain source-time text; no historical rewrite or runtime authority change.
