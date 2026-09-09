---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "a2a-canonical-name-routing-execution-plan"
issue: 718
task_id: "issue-0718"
run_id: "issue-0718"
version: "1.0.4"
title: "[v0.92.1][Runtime] Route agent-to-agent communication by canonical agent name"
branch: "codex/718-a2a-canonical-name-routing"
generated_at: "2026-09-09T18:20:00Z"
card_status: "approved"
status: "ready"
activation_state: "approved_for_execution"
plan_revision: 1
initial_pvf_lane: "runtime-focused"
planned_pvf_lane: "runtime-focused-plus-governed-demo"
planned_pvf_lane_source: "issue-718-acceptance"
estimate_elapsed_seconds: "5400"
estimate_total_tokens: "18000"
estimate_validation_seconds: "1200"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "20"
estimate_confidence: "medium"
estimate_data_source: "live issue contract and current Runtime A2A implementation"
estimate_source_ref: "issue #718"
issue_goal_ref: "issue-718-session-goal"
sprint_goal_ref: "v0.92.1-bugfix"
goal_metrics_rollup_ref: "v0.92.1-bugfix"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/718"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/718"
  - kind: "stp"
    ref: ".csdlc/issues/718/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/718/cards/sip.md"
scope:
  files:
    - "Runtime roster/A2A resolution, provider prompt/action schema, public/history/Observatory projections, migration compatibility, Welcome Package, and focused tests"
  components:
    - "a2a-canonical-name-routing"
  out_of_scope:
    - "distributed transport, provider/model changes, Shepherd-specific paths, weakened ACIP, identity redesign"
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Trace every public recipient field, introduce one authoritative canonical-name resolution boundary before signed dispatch, preserve internal IDs behind that boundary, update outward contracts, and prove lifecycle stability plus all-pairs communication."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: merged governed A2A initiation/closed-loop paths and merged #717 Welcome Package"
    expected_output: ".csdlc/issues/718/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: issue #718, live roster naming, current ACIP/A2A carrier, history, lifecycle, OpenAPI, and Observatory code"
    expected_output: ".csdlc/issues/718/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: resolver, typed denials, bounded legacy input, outward name projections, lifecycle stability, prompts/docs, and end-to-end tests"
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: canonical names work uniformly for all agents, invalid targets fail explicitly, addresses survive provider and lifecycle changes, and internal IDs remain private routing/audit detail"
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
  - "a2a-canonical-name-routing"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "identity/display-name ambiguity, time-of-check roster drift, legacy record compatibility, internal-ID leakage, duplicate public addresses"
test_strategy:
  - "focused resolver/denial/history/lifecycle/prompt/schema tests, full resident-pair governed path, formatting, diff hygiene, independent review"
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
notes: "Re-plan if canonical names are not already unique invariants of admission; do not silently choose among ambiguous matches."
---

Canonical Template Source: `docs/templates/prompts/1.0.4/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.1][Runtime] Route agent-to-agent communication by canonical agent name`.

Trace every public recipient field, introduce one authoritative canonical-name resolution boundary before signed dispatch, preserve internal IDs behind that boundary, update outward contracts, and prove lifecycle stability plus all-pairs communication.

## PVF Lane Plan

- Initial PVF lane from issue creation: `runtime-focused`
- Planned PVF lane for execution: `runtime-focused-plus-governed-demo`
- Planning lane source: `issue-718-acceptance`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `5400`
- Estimated total tokens: `18000`
- Estimated validation seconds: `1200`
- Issue goal token budget: `unknown`
- Variance threshold percent: `20`
- Estimate confidence: `medium`
- Estimate data source: `live issue contract and current Runtime A2A implementation`
- Estimate source ref: `issue #718`
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

1. Confirm dependency readiness and starting state: merged governed A2A initiation/closed-loop paths and merged #717 Welcome Package
2. Review repo inputs and scoped surfaces before editing: issue #718, live roster naming, current ACIP/A2A carrier, history, lifecycle, OpenAPI, and Observatory code
3. Implement only the bounded deliverables: resolver, typed denials, bounded legacy input, outward name projections, lifecycle stability, prompts/docs, and end-to-end tests
4. Run focused proof gates for acceptance: canonical names work uniformly for all agents, invalid targets fail explicitly, addresses survive provider and lifecycle changes, and internal IDs remain private routing/audit detail
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- a2a-canonical-name-routing

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- identity/display-name ambiguity, time-of-check roster drift, legacy record compatibility, internal-ID leakage, duplicate public addresses

## Test Strategy

- focused resolver/denial/history/lifecycle/prompt/schema tests, full resident-pair governed path, formatting, diff hygiene, independent review

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Re-plan if canonical names are not already unique invariants of admission; do not silently choose among ambiguous matches.
