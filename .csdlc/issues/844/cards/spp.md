---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "issue-844-native-pr-merge-execution-plan"
issue: 844
task_id: "issue-0844"
run_id: "issue-0844"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06.24][csdlc] Add a first-class native v3 pull-request merge operation"
branch: "codex/844-native-pr-merge"
generated_at: "2026-09-11T15:54:41.182312+00:00"
card_status: "ready"
status: "in_progress"
activation_state: "ready_to_bind"
plan_revision: 1
initial_pvf_lane: "tooling"
planned_pvf_lane: "tooling"
planned_pvf_lane_source: "Issue844 native owner change"
estimate_elapsed_seconds: "unknown"
estimate_total_tokens: "unknown"
estimate_validation_seconds: "unknown"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "10"
estimate_confidence: "not_collected"
estimate_data_source: "not_collected"
estimate_source_ref: "not_collected"
issue_goal_ref: "Planning #7 active issue844 session goal"
sprint_goal_ref: "not_applicable; issue-local goal"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/844"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/844"
  - kind: "stp"
    ref: ".csdlc/issues/844/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/844/cards/sip.md"
scope:
  files:
    - "csdlc-v3/src/commands/remote, tightly coupled process adapter and CLI routes/tests, docs/csdlc-v3 current workflow guidance."
  components:
    - "issue-844-native-pr-merge"
  out_of_scope:
    - "No raw-gh lifecycle writes, weakened authority/review/CI guards, bulk merge, queue, or finish redesign. No live merge without explicit target authorization."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Inspect transport/authority; implement typed merge and eligibility; add pre/post receipt and replay proof; document and validate; independent review and publish."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: Existing native v3 authority, GitHub mutation intent/reconciliation and finish observation contracts."
    expected_output: ".csdlc/issues/844/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Issue844; current remote mutation implementation, authenticated adapter and terminal finish contracts."
    expected_output: ".csdlc/issues/844/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Typed merge request, fail-closed eligibility, authenticated merge/receipt/replay, focused regressions, command documentation."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: Reject stale head/review, draft, conflicts, wrong identity/base/method,red required checks and unresolved review blocks; reconcile identical already-merged result; uncertain response never dispatches twice; finish observes actual merged result."
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
  - "issue-844-native-pr-merge"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "REST sha guards head, not atomic base/policy; poststate mismatch blocks reconciliation after possible merge. Merge-only; other methods/unknown policies rejected."
test_strategy:
  - "Focused deterministic remote/CLI/terminal tests and owner csdlc lane with fmt/clippy. Test all rejects before mutation, uncertain replay, identity and resulting commit; independent exact-head review; required CI."
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
notes: "Eligibility must be authenticated and fail closed for incomplete pagination or protection data. GitHub exact-head SHA condition constrains race; never fabricate terminal result."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.1][TAIL-06.24][csdlc] Add a first-class native v3 pull-request merge operation`.

Inspect transport/authority; implement typed merge and eligibility; add pre/post receipt and replay proof; document and validate; independent review and publish.

## PVF Lane Plan

- Initial PVF lane from issue creation: `tooling`
- Planned PVF lane for execution: `tooling`
- Planning lane source: `Issue844 native owner change`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `unknown`
- Estimated total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Issue goal token budget: `unknown`
- Variance threshold percent: `10`
- Estimate confidence: `not_collected`
- Estimate data source: `not_collected`
- Estimate source ref: `not_collected`
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

1. Confirm dependency readiness and starting state: Existing native v3 authority, GitHub mutation intent/reconciliation and finish observation contracts.
2. Review repo inputs and scoped surfaces before editing: Issue844; current remote mutation implementation, authenticated adapter and terminal finish contracts.
3. Implement only the bounded deliverables: Typed merge request, fail-closed eligibility, authenticated merge/receipt/replay, focused regressions, command documentation.
4. Run focused proof gates for acceptance: Reject stale head/review, draft, conflicts, wrong identity/base/method,red required checks and unresolved review blocks; reconcile identical already-merged result; uncertain response never dispatches twice; finish observes actual merged result.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- issue-844-native-pr-merge

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- REST sha guards head, not atomic base/policy; poststate mismatch blocks reconciliation after possible merge. Merge-only; other methods/unknown policies rejected.

## Test Strategy

- Focused deterministic remote/CLI/terminal tests and owner csdlc lane with fmt/clippy. Test all rejects before mutation, uncertain replay, identity and resulting commit; independent exact-head review; required CI.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Eligibility must be authenticated and fail closed for incomplete pagination or protection data. GitHub exact-head SHA condition constrains race; never fabricate terminal result.
