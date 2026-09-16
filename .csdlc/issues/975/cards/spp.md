---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "merge-graphql-observation-execution-plan"
issue: 975
task_id: "issue-0975"
run_id: "issue-0975"
version: "v0.92.2"
title: "[v0.92.2][C-SDLC] Repair native merge GraphQL observation transport"
branch: "codex/975-v0922-merge-graphql-observation"
generated_at: "2026-09-14T17:17:30.658534+00:00"
card_status: "ready"
status: "IN_PROGRESS"
activation_state: "bound"
plan_revision: 1
initial_pvf_lane: "deterministic_local"
planned_pvf_lane: "deterministic_local"
planned_pvf_lane_source: ".csdlc/evidence/975/VALIDATION.md"
estimate_elapsed_seconds: "unknown"
estimate_total_tokens: "unknown"
estimate_validation_seconds: "unknown"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "10"
estimate_confidence: "unknown"
estimate_data_source: "unknown"
estimate_source_ref: "unknown"
issue_goal_ref: "codex-goal:01a0875a-799c-7422-a9ae-e1eefbd1c932:975"
sprint_goal_ref: "not_applicable: delegated tooling repair"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/975"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/975"
  - kind: "stp"
    ref: ".csdlc/issues/975/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/975/cards/sip.md"
scope:
  files:
    - "csdlc-v3/src/adapters/mod.rs; .csdlc/issues/975; .csdlc/evidence/975/VALIDATION.md"
  components:
    - "merge-graphql-observation"
  out_of_scope:
    - "No928 docs, raw merge, policy bypass, shared binary install or release authorization."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Reproduce production transport failure, use minimal JSON POST fix, validate focused regressions, hand off for independent review before publication."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: Bound975; read-only GitHub access;971 merge held until reviewed repair."
    expected_output: ".csdlc/issues/975/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Issue975 and generated merge-state/linkage queries; original PR971 observation failure."
    expected_output: ".csdlc/issues/975/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Bounded JSON POST transport, production adapter regressions, PVF proof and truthful six-card records."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: Both queries return targeted PR without truncation; generated JSON body bounded64KiB; errors/partial/pagination cannot authorize merge."
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
  - "merge-graphql-observation"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Partial/error responses stay non-proving; command/body isolation must not expose credentials."
test_strategy:
  - "cargo test --manifest-path csdlc-v3/Cargo.toml --lib; cargo test --manifest-path csdlc-v3/Cargo.toml --lib merge_adapter_tests; cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands --test remote_publication_commands; native validate; git diff --check."
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
notes: "Live observation is transport proof only; hosted CI and independent review remain required."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][C-SDLC] Repair native merge GraphQL observation transport`.

Reproduce production transport failure, use minimal JSON POST fix, validate focused regressions, hand off for independent review before publication.

## PVF Lane Plan

- Initial PVF lane from issue creation: `deterministic_local`
- Planned PVF lane for execution: `deterministic_local`
- Planning lane source: `.csdlc/evidence/975/VALIDATION.md`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `unknown`
- Estimated total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Issue goal token budget: `unknown`
- Variance threshold percent: `10`
- Estimate confidence: `unknown`
- Estimate data source: `unknown`
- Estimate source ref: `unknown`
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

1. Confirm dependency readiness and starting state: Bound975; read-only GitHub access;971 merge held until reviewed repair.
2. Review repo inputs and scoped surfaces before editing: Issue975 and generated merge-state/linkage queries; original PR971 observation failure.
3. Implement only the bounded deliverables: Bounded JSON POST transport, production adapter regressions, PVF proof and truthful six-card records.
4. Run focused proof gates for acceptance: Both queries return targeted PR without truncation; generated JSON body bounded64KiB; errors/partial/pagination cannot authorize merge.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- merge-graphql-observation

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Partial/error responses stay non-proving; command/body isolation must not expose credentials.

## Test Strategy

- cargo test --manifest-path csdlc-v3/Cargo.toml --lib; cargo test --manifest-path csdlc-v3/Cargo.toml --lib merge_adapter_tests; cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands --test remote_publication_commands; native validate; git diff --check.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Live observation is transport proof only; hosted CI and independent review remain required.
