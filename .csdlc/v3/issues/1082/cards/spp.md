---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "bedrock-converse-resident-bindings-execution-plan"
issue: 1082
task_id: "issue-1082"
run_id: "issue-1082"
version: "1.0.5"
title: "[v0.92.2][Runtime][Bedrock] Adopt Converse and migrate unreliable OpenRouter residents"
branch: "codex/1082-bedrock-converse-resident-bindings"
generated_at: "2026-09-19"
card_status: "ready"
status: "in_progress"
activation_state: "bound"
plan_revision: 1
initial_pvf_lane: "runtime"
planned_pvf_lane: "runtime"
planned_pvf_lane_source: "Issue #1082 acceptance criteria and docs/validation/pvf_lanes.json"
estimate_elapsed_seconds: "10800"
estimate_total_tokens: "unknown"
estimate_validation_seconds: "2400"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "10"
estimate_confidence: "medium"
estimate_data_source: "bounded source survey, issue acceptance surface and two-crate Rust validation profile"
estimate_source_ref: "issue-1082"
issue_goal_ref: "Issue #1082 implementation, bounded live qualification, exact-head review and green draft PR"
sprint_goal_ref: "v0.92.2 runtime provider follow-up"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1082"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1082"
  - kind: "stp"
    ref: ".csdlc/issues/1082/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/1082/cards/sip.md"
scope:
  files:
    - "Provider-core current ProviderSpec and Converse adapter, compatibility adapter, Runtime identity/admission/continuity, health projection and operator docs."
  components:
    - "bedrock-converse-resident-bindings"
  out_of_scope:
    - "No healthy DeepSeek migration, recurring provider probe, cloud/IAM change, blanket model support, unrelated Runtime repair or model-derived resident identity."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Migrate both Bedrock adapters to typed Converse; materialize current Kimi/Nemotron provider profiles with stable/native identity and bounded controls; add typed error and response bounds; introduce explicit canonical-name migration that retains the internal continuity key; prove deterministic behavior, exact model inference and Runtime resident operation; review and publish."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: Merged #854, #855, #876 and #1079."
    expected_output: ".csdlc/issues/1082/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Issue #1082, current provider spec/profile/substrate contracts, current Runtime dynamic resident store and legacy resident state."
    expected_output: ".csdlc/issues/1082/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Converse, current profiles and controls, typed failures, bounded response, explicit continuity migration, documentation, tests, live proof and draft publication."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: AC-1 through AC-9 in the issue-specific STP, with identity, provider, model and office remaining independent."
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
  - "bedrock-converse-resident-bindings"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Ambiguous cloud outcomes cannot be replayed; identity rename must be explicit; live state must preserve legacy conversations and welcome records; current provider definitions must remain distinct from resident identity."
test_strategy:
  - "Focused and component Rust proof, fmt/Clippy/diff hygiene, two bounded live provider calls, Runtime conversation/A2A/readiness proof, native proof, independent exact-head review and CI."
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
notes: "Do not name new residents after providers or models, silently discard legacy continuity, widen to every Bedrock catalog model, migrate healthy DeepSeek, expose AWS identity or credentials, retry ambiguous live calls, or introduce recurring inference."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][Runtime][Bedrock] Adopt Converse and migrate unreliable OpenRouter residents`.

Migrate both Bedrock adapters to typed Converse; materialize current Kimi/Nemotron provider profiles with stable/native identity and bounded controls; add typed error and response bounds; introduce explicit canonical-name migration that retains the internal continuity key; prove deterministic behavior, exact model inference and Runtime resident operation; review and publish.

## PVF Lane Plan

- Initial PVF lane from issue creation: `runtime`
- Planned PVF lane for execution: `runtime`
- Planning lane source: `Issue #1082 acceptance criteria and docs/validation/pvf_lanes.json`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `10800`
- Estimated total tokens: `unknown`
- Estimated validation seconds: `2400`
- Issue goal token budget: `unknown`
- Variance threshold percent: `10`
- Estimate confidence: `medium`
- Estimate data source: `bounded source survey, issue acceptance surface and two-crate Rust validation profile`
- Estimate source ref: `issue-1082`
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

1. Confirm dependency readiness and starting state: Merged #854, #855, #876 and #1079.
2. Review repo inputs and scoped surfaces before editing: Issue #1082, current provider spec/profile/substrate contracts, current Runtime dynamic resident store and legacy resident state.
3. Implement only the bounded deliverables: Converse, current profiles and controls, typed failures, bounded response, explicit continuity migration, documentation, tests, live proof and draft publication.
4. Run focused proof gates for acceptance: AC-1 through AC-9 in the issue-specific STP, with identity, provider, model and office remaining independent.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- bedrock-converse-resident-bindings

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Ambiguous cloud outcomes cannot be replayed; identity rename must be explicit; live state must preserve legacy conversations and welcome records; current provider definitions must remain distinct from resident identity.

## Test Strategy

- Focused and component Rust proof, fmt/Clippy/diff hygiene, two bounded live provider calls, Runtime conversation/A2A/readiness proof, native proof, independent exact-head review and CI.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Do not name new residents after providers or models, silently discard legacy continuity, widen to every Bedrock catalog model, migrate healthy DeepSeek, expose AWS identity or credentials, retry ambiguous live calls, or introduce recurring inference.
