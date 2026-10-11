---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "<slug>-execution-plan"
issue: 1223
task_id: "issue-1223"
run_id: "issue-1223"
version: "1.0.5"
title: "[v0.93.1][RD-07 final] Complete coherent source retirement after the repository split"
branch: "codex/1223-1223-coherent-source-retirement"
generated_at: "<timestamp>"
card_status: "ready"
status: "<status>"
activation_state: "<activation_state>"
plan_revision: 1
initial_pvf_lane: "<initial_pvf_lane>"
planned_pvf_lane: "<planned_pvf_lane>"
planned_pvf_lane_source: "<planned_pvf_lane_source>"
estimate_elapsed_seconds: "<estimate_elapsed_seconds>"
estimate_total_tokens: "<estimate_total_tokens>"
estimate_validation_seconds: "<estimate_validation_seconds>"
issue_goal_token_budget: "<issue_goal_token_budget>"
variance_threshold_percent: "<variance_threshold_percent>"
estimate_confidence: "<estimate_confidence>"
estimate_data_source: "<estimate_data_source>"
estimate_source_ref: "<estimate_source_ref>"
issue_goal_ref: "<issue_goal_ref>"
sprint_goal_ref: "<sprint_goal_ref>"
goal_metrics_rollup_ref: "<goal_metrics_rollup_ref>"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1223"
  - kind: "source_issue_prompt"
    ref: "<source_issue_prompt>"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
scope:
  files:
    - "docs/milestones/v0.93.1/repository-decomposition/rd07-1223; tools/source_retirement; native cards; exact reviewed component and retained-consumer changes only. Current phase prepares a reviewable retirement delta and repairs baseline drift; no blanket deletion."
  components:
    - "<slug>"
  out_of_scope:
    - "<non_goals_inline>"
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "<plan_summary>"
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: Operator authorized execution on2026-10-11. C-SDLC205 is explicitly not a prerequisite. Prior non-tail-before1223-beforeSprint4 order remains. Preserve active producer ownership and Runtime/shared tools. Exact correspondence is not functional replacement acceptance."
    expected_output: "<sip_card>"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Issue1223; frozen248f00e359e412f6bc0061ac88be9ff374e3a212; current main9cfd035aac2a44637b3c8026bd40b669579a8c7f integrated into existing bound FastWork branch; refreshed audit4134rows,3222exact successor matches,912nonexact,14changed main paths; existing preservation archive and reviewed preparation evidence."
    expected_output: "<stp_card>"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Refresh all 4134 exact original candidate dispositions against current ADL and successor heads. Preserve existing34 removals and current-main changes. Prepare coherent component-level removal/surgical/justified-retain decisions and all affected consumers. Maintain exact preservation and restore evidence; qualify final public packages and retained boundaries before independently reviewed native publication."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: Account explicitly for all4134 original candidates without counting unresolved holds as complete. Each removal requires a qualified successor or accepted retirement decision, coherent consumer repairs, preservation and restore proof. Protect active native control plane and five public packages. Run final exact-candidate proof and independent review. Obtain scoped retirement decision on concrete reviewed delta before any additional removals."
    expected_output: "validation evidence recorded in VPP/SOR"
    allowed_mode: "execution_after_approval"
  - id: "step-5"
    description: "Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges."
    expected_output: "reviewed SRP and truthful VPP/SOR"
    allowed_mode: "execution_after_approval"
codex_plan:
  - step: "Confirm dependencies and starting state from the source issue prompt."
    status: "<step_1_status>"
  - step: "Inspect repo inputs and target surfaces before editing."
    status: "<step_2_status>"
  - step: "Implement the bounded deliverables only."
    status: "<step_3_status>"
  - step: "Run focused validation and proof gates."
    status: "<step_4_status>"
  - step: "Record issue-specific SRP findings and VPP/SOR outcome truth."
    status: "<step_5_status>"
affected_areas:
  - "<slug>"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "<risks_inline>"
test_strategy:
  - "First refresh inventory/reference guards to current integrated baseline and run focused positive/negative tests. Prepare component correspondence and retained-consumer evidence without repeated builds. At final committed retirement candidate run five public package tests and installed-public-packages proof once, including7consumer cases,21UTSfixtures and5negative cases. All Cargo outputs and worktrees stay on FastWork; no Runtime replacement or restart."
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
notes: "Equality is only candidate correspondence. RD11 selections are not automatically current installed qualification. Preserve source-dependent C-SDLC and public contracts. AWS proposal includes Batch04; do not apply twice. Current plan changes invalidate prior proof/review; missing final proof remains pending."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.93.1][RD-07 final] Complete coherent source retirement after the repository split`.

<plan_summary>

## PVF Lane Plan

- Initial PVF lane from issue creation: `<initial_pvf_lane>`
- Planned PVF lane for execution: `<planned_pvf_lane>`
- Planning lane source: `<planned_pvf_lane_source>`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `<estimate_elapsed_seconds>`
- Estimated total tokens: `<estimate_total_tokens>`
- Estimated validation seconds: `<estimate_validation_seconds>`
- Issue goal token budget: `<issue_goal_token_budget>`
- Variance threshold percent: `<variance_threshold_percent>`
- Estimate confidence: `<estimate_confidence>`
- Estimate data source: `<estimate_data_source>`
- Estimate source ref: `<estimate_source_ref>`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Goal Accounting Plan

Carry `issue_goal_ref`, `sprint_goal_ref`, and `goal_metrics_rollup_ref` in frontmatter so later tooling can roll planning and outcome metrics up without duplicating machine-local goal details in prose.

## Codex Plan

1. [<step_1_status>] Confirm dependencies and starting state from the source issue prompt.
2. [<step_2_status>] Inspect repo inputs and target surfaces before editing.
3. [<step_3_status>] Implement the bounded deliverables only.
4. [<step_4_status>] Run focused validation and proof gates.
5. [<step_5_status>] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: Operator authorized execution on2026-10-11. C-SDLC205 is explicitly not a prerequisite. Prior non-tail-before1223-beforeSprint4 order remains. Preserve active producer ownership and Runtime/shared tools. Exact correspondence is not functional replacement acceptance.
2. Review repo inputs and scoped surfaces before editing: Issue1223; frozen248f00e359e412f6bc0061ac88be9ff374e3a212; current main9cfd035aac2a44637b3c8026bd40b669579a8c7f integrated into existing bound FastWork branch; refreshed audit4134rows,3222exact successor matches,912nonexact,14changed main paths; existing preservation archive and reviewed preparation evidence.
3. Implement only the bounded deliverables: Refresh all 4134 exact original candidate dispositions against current ADL and successor heads. Preserve existing34 removals and current-main changes. Prepare coherent component-level removal/surgical/justified-retain decisions and all affected consumers. Maintain exact preservation and restore evidence; qualify final public packages and retained boundaries before independently reviewed native publication.
4. Run focused proof gates for acceptance: Account explicitly for all4134 original candidates without counting unresolved holds as complete. Each removal requires a qualified successor or accepted retirement decision, coherent consumer repairs, preservation and restore proof. Protect active native control plane and five public packages. Run final exact-candidate proof and independent review. Obtain scoped retirement decision on concrete reviewed delta before any additional removals.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- <slug>

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- <risks_inline>

## Test Strategy

- First refresh inventory/reference guards to current integrated baseline and run focused positive/negative tests. Prepare component correspondence and retained-consumer evidence without repeated builds. At final committed retirement candidate run five public package tests and installed-public-packages proof once, including7consumer cases,21UTSfixtures and5negative cases. All Cargo outputs and worktrees stay on FastWork; no Runtime replacement or restart.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Equality is only candidate correspondence. RD11 selections are not automatically current installed qualification. Preserve source-dependent C-SDLC and public contracts. AWS proposal includes Batch04; do not apply twice. Current plan changes invalidate prior proof/review; missing final proof remains pending.
