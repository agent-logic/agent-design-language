---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "tail-02-candidate-safe-reproduction-execution-plan"
issue: 768
task_id: "issue-0768"
run_id: "issue-0768"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06.12][docs] Make the TAIL-02 reproduction route candidate-safe"
branch: "codex/768-tail-02-candidate-safe-reproduction"
generated_at: "<timestamp>"
card_status: "ready"
status: "implemented"
activation_state: "executed"
plan_revision: 1
initial_pvf_lane: "docs_contract"
planned_pvf_lane: "docs_contract"
planned_pvf_lane_source: "issue_768"
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
    ref: "https://github.com/agent-logic/agent-design-language/issues/768"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/768"
  - kind: "stp"
    ref: ".csdlc/issues/768/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/768/cards/sip.md"
scope:
  files:
    - ".csdlc/prepared/issues/518/validate-documentation-handoff.rb; tail-02/README.md; issue-768 regression fixture"
  components:
    - "tail-02-candidate-safe-reproduction"
  out_of_scope:
    - "Historical hash rewrites, evidence regeneration, release approval, unrelated validator redesign."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Read frozen inventory from the recorded source head, verify current linkage separately through ancestry and original TAIL-03 hashes, preserve existing release-decision guards, and prove later tracked growth in an isolated clone."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: #522 parent; #520 finding source."
    expected_output: ".csdlc/issues/768/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: <repo_inputs_inline>"
    expected_output: ".csdlc/issues/768/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Two explicit validator modes, corrected reproduction instructions, tracked-growth and retained-guard regression proof."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: <acceptance_criteria_inline>"
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
  - "tail-02-candidate-safe-reproduction"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Accidentally weakening existing fail-closed release checks or binding current working-tree bytes as historical evidence."
test_strategy:
  - "Ruby syntax; historical/candidate/all modes; ten negative fixtures; TAIL-03 zero diff; independent review."
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

Canonical Template Source: `docs/templates/prompts/1.0.4/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.1][TAIL-06.12][docs] Make the TAIL-02 reproduction route candidate-safe`.

Read frozen inventory from the recorded source head, verify current linkage separately through ancestry and original TAIL-03 hashes, preserve existing release-decision guards, and prove later tracked growth in an isolated clone.

## PVF Lane Plan

- Initial PVF lane from issue creation: `docs_contract`
- Planned PVF lane for execution: `docs_contract`
- Planning lane source: `issue_768`
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

1. [completed] Confirm dependencies and starting state from the source issue prompt.
2. [completed] Inspect repo inputs and target surfaces before editing.
3. [completed] Implement the bounded deliverables only.
4. [completed] Run focused validation and proof gates.
5. [completed] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: #522 parent; #520 finding source.
2. Review repo inputs and scoped surfaces before editing: <repo_inputs_inline>
3. Implement only the bounded deliverables: Two explicit validator modes, corrected reproduction instructions, tracked-growth and retained-guard regression proof.
4. Run focused proof gates for acceptance: <acceptance_criteria_inline>
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- tail-02-candidate-safe-reproduction

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Accidentally weakening existing fail-closed release checks or binding current working-tree bytes as historical evidence.

## Test Strategy

- Ruby syntax; historical/candidate/all modes; ten negative fixtures; TAIL-03 zero diff; independent review.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

<notes_risks_inline>
