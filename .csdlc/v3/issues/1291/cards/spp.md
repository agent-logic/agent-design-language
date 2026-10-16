---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "<slug>-execution-plan"
issue: 1291
task_id: "issue-1291"
run_id: "issue-1291"
version: "1.0.5"
title: "[v0.93.1] Reject overflowing session TTLs and refresh specification ownership"
branch: "codex/1291-session-ttl-spec-entrypoint"
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
    ref: "https://github.com/agent-logic/agent-design-language/issues/1291"
  - kind: "source_issue_prompt"
    ref: "<source_issue_prompt>"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
scope:
  files:
    - "adl/src/session_ledger.rs; tools/public_adl/test_public_session.py; adl-spec/README.md; native generated issue1291 cards."
  components:
    - "<slug>"
  out_of_scope:
    - "No compiler/prompt changes, normative language changes, Runtime source, providers/cloud, shared owner replacement, main edits or internal SSD builds."
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
    description: "Confirm dependency readiness and starting state: External review findings confirmed; native issue1291 creation reconciled. No dependency on separate compiler/prompt corrective implementation."
    expected_output: "<sip_card>"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: adl/src/session_ledger.rs; tools/public_adl/test_public_session.py; adl-spec/README.md; docs/REPOSITORIES.md; external review ADL_INDEPENDENT_REVIEW_F75F0D3.md."
    expected_output: "<stp_card>"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Checked expiration before mutation; ordinary errors for duration/timestamp overflow; focused unchanged-state regression cases; current public CLI/separate Runtime navigation and explicitly historical milestone framing."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: Both EXT03/EXT04 resolved without ledger mutation on overflow or changed normative text; reviewed focused proof and green required CI; native finish and clean after merge."
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
  - "Focused session_ledger Rust unit tests and public session CLI persistence/refusal tests; checked duration and timestamp overflow leave in-memory and durable ledger unchanged; documentation ownership/link inspection. Native proof, independent review, required public CLI CI integration separately."
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
notes: "PVF local_contract deterministic bounded CPU/filesystem; all cache/targets/scratch FastWork. No installed Runtime acceptance claim; v0.87 milestones retain historical truth."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.93.1] Reject overflowing session TTLs and refresh specification ownership`.

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

1. Confirm dependency readiness and starting state: External review findings confirmed; native issue1291 creation reconciled. No dependency on separate compiler/prompt corrective implementation.
2. Review repo inputs and scoped surfaces before editing: adl/src/session_ledger.rs; tools/public_adl/test_public_session.py; adl-spec/README.md; docs/REPOSITORIES.md; external review ADL_INDEPENDENT_REVIEW_F75F0D3.md.
3. Implement only the bounded deliverables: Checked expiration before mutation; ordinary errors for duration/timestamp overflow; focused unchanged-state regression cases; current public CLI/separate Runtime navigation and explicitly historical milestone framing.
4. Run focused proof gates for acceptance: Both EXT03/EXT04 resolved without ledger mutation on overflow or changed normative text; reviewed focused proof and green required CI; native finish and clean after merge.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- <slug>

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- <risks_inline>

## Test Strategy

- Focused session_ledger Rust unit tests and public session CLI persistence/refusal tests; checked duration and timestamp overflow leave in-memory and durable ledger unchanged; documentation ownership/link inspection. Native proof, independent review, required public CLI CI integration separately.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

PVF local_contract deterministic bounded CPU/filesystem; all cache/targets/scratch FastWork. No installed Runtime acceptance claim; v0.87 milestones retain historical truth.
