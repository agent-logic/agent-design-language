---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "960-shutdown-barrier-execution-plan"
issue: 960
task_id: "issue-0960"
run_id: "issue-0960"
version: "1.0.5"
title: "Fix Runtime shutdown barrier acknowledgment race (v0.92.2)"
branch: "codex/960-shutdown-barrier"
generated_at: "2026-09-12T05:59:59.590095+00:00"
card_status: "ready"
status: "planned"
activation_state: "ready"
plan_revision: 1
initial_pvf_lane: "runtime"
planned_pvf_lane: "runtime"
planned_pvf_lane_source: "issue960 bounded Runtime regression"
estimate_elapsed_seconds: "unknown; no estimate recorded"
estimate_total_tokens: "unknown; no estimate recorded"
estimate_validation_seconds: "unknown; no estimate recorded"
issue_goal_token_budget: "unknown; no estimate recorded"
variance_threshold_percent: "unknown; no estimate recorded"
estimate_confidence: "unknown"
estimate_data_source: "not estimated"
estimate_source_ref: "none"
issue_goal_ref: "Worker #10 task 01a0924e-4813-7101-ac5c-a1a9ac80f4f8 issue #960 goal: reviewed repair, focused proof, native publication and required CI; merge not authorized."
sprint_goal_ref: "#928"
goal_metrics_rollup_ref: "unknown; not yet collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/960"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/960"
  - kind: "stp"
    ref: ".csdlc/issues/960/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/960/cards/sip.md"
scope:
  files:
    - "adl/src/long_lived_agent.rs; adl/src/cli/observability.rs (also included by adl/src/observability.rs); adl/tests/cli_smoke/agent.rs; focused proof under .csdlc/evidence/960."
  components:
    - "960-shutdown-barrier"
  out_of_scope:
    - "No CodeFriend changes, paid calls, longer waits or weaker assertions; no claim that original CI cause is proven."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "1. Preserve source cf25e273a39499c3d1b699fa7d9e461266024e0c diagnostic packet. 2. Add compatible synchronous event-specific sink receipt under publication lock. 3. Consume receipt at shutdown and retain child diagnostics. 4. Run focused deterministic race/failure and Cargo-built local CLI proof. 5. Independent exact-head review, native publication and required CI; no merge authorization."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: No implementation dependency; root owns #960 repair separately from #880."
    expected_output: ".csdlc/issues/960/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: GitHub issue #960 and copied diagnosis packet; root-approved issue body."
    expected_output: ".csdlc/issues/960/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Bounded Runtime fix, focused proof and child failure diagnostics; preserve old CI evidence."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: Exact barrier acknowledgment survives subsequent heartbeat updates; required sink write failures still reject shutdown; deterministic concurrent-writer and failure injection tests plus actual local CLI notice/disposition proof; stderr/exit retained. No sleeps, longer timeouts, weakened assertions or CodeFriend changes."
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
  - "960-shutdown-barrier"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Original CI cause unknown; reproduced supported 1ms heartbeat race is independent evidence."
test_strategy:
  - "Exact barrier acknowledgment survives subsequent heartbeat updates; required sink write failures still reject shutdown; deterministic concurrent-writer and failure injection tests plus actual local CLI notice/disposition proof; stderr/exit retained. No sleeps, longer timeouts, weakened assertions or CodeFriend changes."
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
notes: "Supported heartbeat race proven; original CI cause unknown. Local sink acknowledgment is not remote OTLP delivery."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `Fix Runtime shutdown barrier acknowledgment race (v0.92.2)`.

1. Preserve source cf25e273a39499c3d1b699fa7d9e461266024e0c diagnostic packet. 2. Add compatible synchronous event-specific sink receipt under publication lock. 3. Consume receipt at shutdown and retain child diagnostics. 4. Run focused deterministic race/failure and Cargo-built local CLI proof. 5. Independent exact-head review, native publication and required CI; no merge authorization.

## PVF Lane Plan

- Initial PVF lane from issue creation: `runtime`
- Planned PVF lane for execution: `runtime`
- Planning lane source: `issue960 bounded Runtime regression`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `unknown; no estimate recorded`
- Estimated total tokens: `unknown; no estimate recorded`
- Estimated validation seconds: `unknown; no estimate recorded`
- Issue goal token budget: `unknown; no estimate recorded`
- Variance threshold percent: `unknown; no estimate recorded`
- Estimate confidence: `unknown`
- Estimate data source: `not estimated`
- Estimate source ref: `none`
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

1. Confirm dependency readiness and starting state: No implementation dependency; root owns #960 repair separately from #880.
2. Review repo inputs and scoped surfaces before editing: GitHub issue #960 and copied diagnosis packet; root-approved issue body.
3. Implement only the bounded deliverables: Bounded Runtime fix, focused proof and child failure diagnostics; preserve old CI evidence.
4. Run focused proof gates for acceptance: Exact barrier acknowledgment survives subsequent heartbeat updates; required sink write failures still reject shutdown; deterministic concurrent-writer and failure injection tests plus actual local CLI notice/disposition proof; stderr/exit retained. No sleeps, longer timeouts, weakened assertions or CodeFriend changes.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- 960-shutdown-barrier

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Original CI cause unknown; reproduced supported 1ms heartbeat race is independent evidence.

## Test Strategy

- Exact barrier acknowledgment survives subsequent heartbeat updates; required sink write failures still reject shutdown; deterministic concurrent-writer and failure injection tests plus actual local CLI notice/disposition proof; stderr/exit retained. No sleeps, longer timeouts, weakened assertions or CodeFriend changes.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Supported heartbeat race proven; original CI cause unknown. Local sink acknowledgment is not remote OTLP delivery.
