---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "v0922-native-coordination-completion-execution-plan"
issue: 1006
task_id: "issue-1006"
run_id: "issue-1006"
version: "0.92.2"
title: "[v0.92.2][C-SDLC] Support native completion closure for coordination issues"
branch: "codex/1006-v0922-native-coordination-completion"
generated_at: "2026-09-16T02:45:19.992791+00:00"
card_status: "ready"
status: "in_progress"
activation_state: "bound_implementation_active"
plan_revision: 1
initial_pvf_lane: "tooling"
planned_pvf_lane: "tooling"
planned_pvf_lane_source: "Issue #1006 changes native C-SDLC remote/terminal tooling; bounded deterministic contract and installed CLI proof."
estimate_elapsed_seconds: "not_budgeted_by_operator"
estimate_total_tokens: "not_budgeted_by_operator"
estimate_validation_seconds: "not_budgeted_by_operator"
issue_goal_token_budget: "none_assigned"
variance_threshold_percent: "not_applicable"
estimate_confidence: "not_estimated"
estimate_data_source: "No operator budget or elapsed-time limit assigned."
estimate_source_ref: ".git/csdlc-v3/local/invocations/worker10-1006/source-issue.md"
issue_goal_ref: "Active operator-authorized combined #1003 and #1006 implementation goal, parent worker10; no budget assigned."
sprint_goal_ref: "No active sprint execution budget assigned to this repair."
goal_metrics_rollup_ref: "No metrics rollup measured during preparation."
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1006"
  - kind: "source_issue_prompt"
    ref: ".git/csdlc-v3/local/invocations/worker10-1006/source-issue.md"
  - kind: "stp"
    ref: ".csdlc/issues/1006/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/1006/cards/sip.md"
scope:
  files:
    - "csdlc-v3/src/commands/remote/mod.rs; csdlc-v3/src/commands/terminal.rs; bounded remote/terminal owner tests and installed command fixtures; docs/csdlc-v3 completion-route guidance"
  components:
    - "v0922-native-coordination-completion"
  out_of_scope:
    - "No broad lifecycle redesign, no generic raw-gh bypass, no closing unrelated issues, no weakening ordinary implementation or unfinished-child gates, no live closure during fixture validation."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Implementation,123-test local proof and independent source review complete. Record-only review renewal, native publication, required hosted checks and operator merge remain separate; issue goal stays active."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: No additional implementation prerequisite identified; current native authority and existing terminal/remote owners must remain valid. Coordinate overlapping remote owner changes with #1003 before implementation."
    expected_output: ".csdlc/issues/1006/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: .git/csdlc-v3/local/invocations/worker10-1006/source-issue.md; docs/csdlc-v3/CURRENT_AUTHORITY.md; current remote/terminal code; retained #929 independent review and blocked audit in root Git metadata"
    expected_output: ".csdlc/issues/1006/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Typed coordination completion request/authorization contract; bounded verified child-evidence checks; authenticated mutation/reconciliation; native finish compatibility; focused regression and installed fixture proof; operator docs."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: 1. A coordination-only issue with explicit operator authority, exact identity/state and current completed-child evidence closes with completed reason and native finish reconciles it. 2. Missing or stale evidence, unfinished children, wrong issue identity/state and ordinary implementation issues fail closed before mutation. 3. Existing duplicate, superseded and no-op administrative closure retains its semantics. 4. Installed native command fixtures prove the complete close/readback/finish sequence without live GitHub writes; receipts bind exact operation and observed state. 5. Stdout remains JSON; stderr and durable evidence exclude credentials and sensitive payloads."
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
  - "v0922-native-coordination-completion"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Stale child proof; forgeable coordination classification; identity substitution; partial GitHub mutation before uncertain reconciliation; existing admin closure regression."
test_strategy:
  - "Small deterministic C-SDLC owner regression tests and installed CLI fixture proof with controlled GitHub adapter. Cover success followed by finish; wrong identity/state, stale/missing evidence, unfinished children, replay/reconciliation and existing administrative routes. Run strict Clippy and fmt on touched crate; native six-card validate. Hosted required checks remain separately pending."
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
notes: "Coordination classification and evidence freshness must be verified, not self-certified. Prevent stale state/replay, identity substitution and implicit ordinary-issue completion. #929 workaround is historical evidence, not authority for new bypasses."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][C-SDLC] Support native completion closure for coordination issues`.

Implementation,123-test local proof and independent source review complete. Record-only review renewal, native publication, required hosted checks and operator merge remain separate; issue goal stays active.

## PVF Lane Plan

- Initial PVF lane from issue creation: `tooling`
- Planned PVF lane for execution: `tooling`
- Planning lane source: `Issue #1006 changes native C-SDLC remote/terminal tooling; bounded deterministic contract and installed CLI proof.`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `not_budgeted_by_operator`
- Estimated total tokens: `not_budgeted_by_operator`
- Estimated validation seconds: `not_budgeted_by_operator`
- Issue goal token budget: `none_assigned`
- Variance threshold percent: `not_applicable`
- Estimate confidence: `not_estimated`
- Estimate data source: `No operator budget or elapsed-time limit assigned.`
- Estimate source ref: `.git/csdlc-v3/local/invocations/worker10-1006/source-issue.md`
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

1. Confirm dependency readiness and starting state: No additional implementation prerequisite identified; current native authority and existing terminal/remote owners must remain valid. Coordinate overlapping remote owner changes with #1003 before implementation.
2. Review repo inputs and scoped surfaces before editing: .git/csdlc-v3/local/invocations/worker10-1006/source-issue.md; docs/csdlc-v3/CURRENT_AUTHORITY.md; current remote/terminal code; retained #929 independent review and blocked audit in root Git metadata
3. Implement only the bounded deliverables: Typed coordination completion request/authorization contract; bounded verified child-evidence checks; authenticated mutation/reconciliation; native finish compatibility; focused regression and installed fixture proof; operator docs.
4. Run focused proof gates for acceptance: 1. A coordination-only issue with explicit operator authority, exact identity/state and current completed-child evidence closes with completed reason and native finish reconciles it. 2. Missing or stale evidence, unfinished children, wrong issue identity/state and ordinary implementation issues fail closed before mutation. 3. Existing duplicate, superseded and no-op administrative closure retains its semantics. 4. Installed native command fixtures prove the complete close/readback/finish sequence without live GitHub writes; receipts bind exact operation and observed state. 5. Stdout remains JSON; stderr and durable evidence exclude credentials and sensitive payloads.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- v0922-native-coordination-completion

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Stale child proof; forgeable coordination classification; identity substitution; partial GitHub mutation before uncertain reconciliation; existing admin closure regression.

## Test Strategy

- Small deterministic C-SDLC owner regression tests and installed CLI fixture proof with controlled GitHub adapter. Cover success followed by finish; wrong identity/state, stale/missing evidence, unfinished children, replay/reconciliation and existing administrative routes. Run strict Clippy and fmt on touched crate; native six-card validate. Hosted required checks remain separately pending.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Coordination classification and evidence freshness must be verified, not self-certified. Prevent stale state/replay, identity substitution and implicit ordinary-issue completion. #929 workaround is historical evidence, not authority for new bypasses.
