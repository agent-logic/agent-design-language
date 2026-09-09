---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "versioned-template-structure-schemas-execution-plan"
issue: 776
task_id: "issue-0776"
run_id: "issue-0776"
version: "1.0.4"
title: "[v0.92.1][TAIL-06.17][csdlc] Resolve structure schemas beside versioned templates"
branch: "codex/776-versioned-template-structure-schemas"
generated_at: "<timestamp>"
card_status: "ready"
status: "ready"
activation_state: "execution_approved"
plan_revision: 1
initial_pvf_lane: "csdlc-v3-local-commands"
planned_pvf_lane: "csdlc-v3-local-commands"
planned_pvf_lane_source: "issue-776"
estimate_elapsed_seconds: "900"
estimate_total_tokens: "12000"
estimate_validation_seconds: "300"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "10"
estimate_confidence: "high"
estimate_data_source: "bounded two-file defect scope"
estimate_source_ref: "issue-776"
issue_goal_ref: "Issue #776 session goal"
sprint_goal_ref: "v0.92.1 TAIL-06.17"
goal_metrics_rollup_ref: "not_applicable"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/776"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/776"
  - kind: "stp"
    ref: ".csdlc/issues/776/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/776/cards/sip.md"
scope:
  files:
    - "csdlc-v3/src/commands/local/mod.rs; csdlc-v3/tests/local_commands.rs"
  components:
    - "versioned-template-structure-schemas"
  out_of_scope:
    - "template/schema edits; registry redesign; validation weakening; Runtime changes"
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Resolve schemas beneath each versioned template parent and validate the schema identity, version, card binding, applicable scaffold lines, declared headings, and locked lines. Prove the real current registry across all six cards, retain focused local-command behavior, obtain exact-head review, and publish without merging."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: native v3 authority is current; #758 waits for the tested fix"
    expected_output: ".csdlc/issues/776/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: native local validator, local_commands integration tests, current 1.0.4 registry"
    expected_output: ".csdlc/issues/776/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: single-parent schema resolution and current-registry six-card regression"
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: all issue acceptance criteria AC-1 through AC-4"
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
  - "versioned-template-structure-schemas"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "The issue's path-only diagnosis omitted that scaffold_lines is a cross-template vocabulary rather than an all-required per-card set; the repair must preserve heading and locked-line validation instead of bypassing structure checks."
test_strategy:
  - "Exact real-current-registry six-card regression; complete 34-test local_commands target; cargo fmt --check; git diff --check."
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

Design-time operative plan for `[v0.92.1][TAIL-06.17][csdlc] Resolve structure schemas beside versioned templates`.

Resolve schemas beneath each versioned template parent and validate the schema identity, version, card binding, applicable scaffold lines, declared headings, and locked lines. Prove the real current registry across all six cards, retain focused local-command behavior, obtain exact-head review, and publish without merging.

## PVF Lane Plan

- Initial PVF lane from issue creation: `csdlc-v3-local-commands`
- Planned PVF lane for execution: `csdlc-v3-local-commands`
- Planning lane source: `issue-776`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `900`
- Estimated total tokens: `12000`
- Estimated validation seconds: `300`
- Issue goal token budget: `unknown`
- Variance threshold percent: `10`
- Estimate confidence: `high`
- Estimate data source: `bounded two-file defect scope`
- Estimate source ref: `issue-776`
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

1. Confirm dependency readiness and starting state: native v3 authority is current; #758 waits for the tested fix
2. Review repo inputs and scoped surfaces before editing: native local validator, local_commands integration tests, current 1.0.4 registry
3. Implement only the bounded deliverables: single-parent schema resolution and current-registry six-card regression
4. Run focused proof gates for acceptance: all issue acceptance criteria AC-1 through AC-4
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- versioned-template-structure-schemas

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- The issue's path-only diagnosis omitted that scaffold_lines is a cross-template vocabulary rather than an all-required per-card set; the repair must preserve heading and locked-line validation instead of bypassing structure checks.

## Test Strategy

- Exact real-current-registry six-card regression; complete 34-test local_commands target; cargo fmt --check; git diff --check.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

<notes_risks_inline>
