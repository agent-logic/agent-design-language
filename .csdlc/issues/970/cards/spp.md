---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "aprovider-effective-inference-configuration-execution-plan"
issue: 970
task_id: "issue-0970"
run_id: "issue-0970"
version: "1.0.5"
title: "[provider architecture] Make declared AProvider inference configuration effective and observable"
branch: "codex/970-provider-architecture-effective-configuration"
generated_at: "2026-09-15"
card_status: "ready"
status: "in_progress"
activation_state: "bound"
plan_revision: 1
initial_pvf_lane: "provider"
planned_pvf_lane: "provider"
planned_pvf_lane_source: "docs/validation/pvf_lanes.json provider lane and issue #970 acceptance criteria"
estimate_elapsed_seconds: "10800"
estimate_total_tokens: "90000"
estimate_validation_seconds: "1800"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "10"
estimate_confidence: "medium"
estimate_data_source: "bounded source survey and issue acceptance surface"
estimate_source_ref: "issue-970"
issue_goal_ref: "Active issue #970 implementation, native proof, independent review, and draft publication goal"
sprint_goal_ref: "Sprint 5 provider follow-up"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/970"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/970"
  - kind: "stp"
    ref: ".csdlc/issues/970/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/970/cards/sip.md"
scope:
  files:
    - "adl-provider-core normalized substrate, candidate admission, profile materialization, GeneralProvider dispatch, Ollama HTTP codec/tests, and provider operator documentation."
  components:
    - "aprovider-effective-inference-configuration"
  out_of_scope:
    - "No provider lifecycle or recovery redesign, executable provider extensions, model downloads, live hosted calls, paid execution, benchmarks, or hardware provisioning."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Inventory current ad hoc controls; introduce typed effective normalization and codec support enforcement at ProviderInvocationTargetV1; consume the normalized controls in Ollama HTTP; expose redacted projection and fingerprint; prove profile/direct/reload parity and pre-call rejection; update operator docs; complete native proof and exact-head review."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: Closed dependencies #514, #876, and #855; #901 remains separate."
    expected_output: ".csdlc/issues/970/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Issue #970 and current provider-core profile, candidate, substrate, factory, codec, tests, and docs surfaces."
    expected_output: ".csdlc/issues/970/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Typed effective configuration, codec consumption contract, fail-closed admission, Ollama serialization, projection/fingerprint, compatibility documentation, and focused proof."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: AC-1 through AC-8 from the issue-specific STP, with every supplied control either consumed by the selected codec or rejected before provider I/O."
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
    status: "in_progress"
  - step: "Implement the bounded deliverables only."
    status: "pending"
  - step: "Run focused validation and proof gates."
    status: "pending"
  - step: "Record issue-specific SRP findings and VPP/SOR outcome truth."
    status: "pending"
affected_areas:
  - "aprovider-effective-inference-configuration"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Codec-specific constraints differ, legacy profiles use loose keys, and observability must not retain prompts, credentials, private endpoint data, or responses."
test_strategy:
  - "Focused local Rust tests and wire capture, fmt, diff hygiene, native proof, independent exact-head review, then CI."
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
notes: "<notes_risks_inline>"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[provider architecture] Make declared AProvider inference configuration effective and observable`.

Inventory current ad hoc controls; introduce typed effective normalization and codec support enforcement at ProviderInvocationTargetV1; consume the normalized controls in Ollama HTTP; expose redacted projection and fingerprint; prove profile/direct/reload parity and pre-call rejection; update operator docs; complete native proof and exact-head review.

## PVF Lane Plan

- Initial PVF lane from issue creation: `provider`
- Planned PVF lane for execution: `provider`
- Planning lane source: `docs/validation/pvf_lanes.json provider lane and issue #970 acceptance criteria`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `10800`
- Estimated total tokens: `90000`
- Estimated validation seconds: `1800`
- Issue goal token budget: `unknown`
- Variance threshold percent: `10`
- Estimate confidence: `medium`
- Estimate data source: `bounded source survey and issue acceptance surface`
- Estimate source ref: `issue-970`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Goal Accounting Plan

Carry `issue_goal_ref`, `sprint_goal_ref`, and `goal_metrics_rollup_ref` in frontmatter so later tooling can roll planning and outcome metrics up without duplicating machine-local goal details in prose.

## Codex Plan

1. [completed] Confirm dependencies and starting state from the source issue prompt.
2. [in_progress] Inspect repo inputs and target surfaces before editing.
3. [pending] Implement the bounded deliverables only.
4. [pending] Run focused validation and proof gates.
5. [pending] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: Closed dependencies #514, #876, and #855; #901 remains separate.
2. Review repo inputs and scoped surfaces before editing: Issue #970 and current provider-core profile, candidate, substrate, factory, codec, tests, and docs surfaces.
3. Implement only the bounded deliverables: Typed effective configuration, codec consumption contract, fail-closed admission, Ollama serialization, projection/fingerprint, compatibility documentation, and focused proof.
4. Run focused proof gates for acceptance: AC-1 through AC-8 from the issue-specific STP, with every supplied control either consumed by the selected codec or rejected before provider I/O.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- aprovider-effective-inference-configuration

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Codec-specific constraints differ, legacy profiles use loose keys, and observability must not retain prompts, credentials, private endpoint data, or responses.

## Test Strategy

- Focused local Rust tests and wire capture, fmt, diff hygiene, native proof, independent exact-head review, then CI.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

<notes_risks_inline>
