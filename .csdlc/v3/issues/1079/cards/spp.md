---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "deepseek-openrouter-reasoning-execution-plan"
issue: 1079
task_id: "issue-1079"
run_id: "issue-1079"
version: "1.0.5"
title: "[v0.92.2][Runtime][OpenRouter] Honor reasoning effort for DeepSeek review turns"
branch: "codex/1079-deepseek-openrouter-reasoning"
generated_at: "2026-09-18T23:46:34Z"
card_status: "ready"
status: "in_progress"
activation_state: "bound"
plan_revision: 1
initial_pvf_lane: "runtime_provider_integration"
planned_pvf_lane: "focused_provider_and_runtime_integration_plus_bounded_live_qualification"
planned_pvf_lane_source: "issue_1079_acceptance"
estimate_elapsed_seconds: "unknown"
estimate_total_tokens: "unknown"
estimate_validation_seconds: "unknown"
issue_goal_token_budget: "unbounded"
variance_threshold_percent: "10"
estimate_confidence: "low"
estimate_data_source: "not_collected"
estimate_source_ref: "issue_1079"
issue_goal_ref: "Issue #1079 active Codex goal"
sprint_goal_ref: "not_assigned"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1079"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1079"
  - kind: "stp"
    ref: ".csdlc/issues/1079/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/1079/cards/sip.md"
scope:
  files:
    - "OpenRouter codec/adapter/tests; Runtime provider registry timeout preservation and authenticated conversation tests; provider inference docs; installed local DeepSeek definition/proof"
  components:
    - "deepseek-openrouter-reasoning"
  out_of_scope:
    - "shared OpenRouter widening, recurring inference, unrelated Runtime repair, billing or credential changes"
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Implement normalized OpenRouter reasoning effort and safe typed Runtime failures, preserve validated provider-specific transport timeouts, validate locally, install the exact-head generation, configure a separate bounded DeepSeek provider, and prove a full issue review plus governed A2A continuation without changing Nexus or Nemotron."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: merged #855 provider lifecycle and #876 hot loading"
    expected_output: ".csdlc/issues/1079/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: issue #1079 and its seven canonical tracked surfaces"
    expected_output: ".csdlc/issues/1079/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: reasoning serialization, conflicting-control rejection, typed provider failures, docs, separate DeepSeek profile, live review and A2A proof"
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: captured low effort; pre-dispatch rejection; substantive full review; governed A2A continuation; Nexus/Nemotron preservation; five typed failure categories"
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
  - "deepseek-openrouter-reasoning"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "provider latency, hidden reasoning budget, ambiguous live outcomes, and last-known-good preservation"
test_strategy:
  - "focused provider and Runtime regressions, provider-core suite/Clippy, native validation, exact-head review, then bounded installed live proof"
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
notes: "Do not retry an ambiguous provider outcome; keep all retained proof redacted and repo-relative."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][Runtime][OpenRouter] Honor reasoning effort for DeepSeek review turns`.

Implement normalized OpenRouter reasoning effort and safe typed Runtime failures, preserve validated provider-specific transport timeouts, validate locally, install the exact-head generation, configure a separate bounded DeepSeek provider, and prove a full issue review plus governed A2A continuation without changing Nexus or Nemotron.

## PVF Lane Plan

- Initial PVF lane from issue creation: `runtime_provider_integration`
- Planned PVF lane for execution: `focused_provider_and_runtime_integration_plus_bounded_live_qualification`
- Planning lane source: `issue_1079_acceptance`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `unknown`
- Estimated total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Issue goal token budget: `unbounded`
- Variance threshold percent: `10`
- Estimate confidence: `low`
- Estimate data source: `not_collected`
- Estimate source ref: `issue_1079`
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

1. Confirm dependency readiness and starting state: merged #855 provider lifecycle and #876 hot loading
2. Review repo inputs and scoped surfaces before editing: issue #1079 and its seven canonical tracked surfaces
3. Implement only the bounded deliverables: reasoning serialization, conflicting-control rejection, typed provider failures, docs, separate DeepSeek profile, live review and A2A proof
4. Run focused proof gates for acceptance: captured low effort; pre-dispatch rejection; substantive full review; governed A2A continuation; Nexus/Nemotron preservation; five typed failure categories
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- deepseek-openrouter-reasoning

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- provider latency, hidden reasoning budget, ambiguous live outcomes, and last-known-good preservation

## Test Strategy

- focused provider and Runtime regressions, provider-core suite/Clippy, native validation, exact-head review, then bounded installed live proof

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Do not retry an ambiguous provider outcome; keep all retained proof redacted and repo-relative.
