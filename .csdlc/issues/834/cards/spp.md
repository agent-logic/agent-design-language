---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "issue-834-internal-review-reconciliation-execution-plan"
issue: 834
task_id: "issue-0834"
run_id: "issue-0834"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06.19][review] Reconcile finalized internal-review predecessor"
branch: "codex/834-internal-review-reconciliation"
generated_at: "2026-09-11T02:59:50.407672+00:00"
card_status: "ready"
status: "in_progress"
activation_state: "bound"
plan_revision: 1
initial_pvf_lane: "release-evidence"
planned_pvf_lane: "release-evidence"
planned_pvf_lane_source: "Issue834 deliverables"
estimate_elapsed_seconds: "3600"
estimate_total_tokens: "30000"
estimate_validation_seconds: "300"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "10"
estimate_confidence: "planning_estimate"
estimate_data_source: "issue scope estimate"
estimate_source_ref: "Issue834"
issue_goal_ref: "Planning #7 issue834 active session goal"
sprint_goal_ref: "not_applicable; issue-local goal under522"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/834"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/834"
  - kind: "stp"
    ref: ".csdlc/issues/834/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/834/cards/sip.md"
scope:
  files:
    - "docs/milestones/v0.92.1/evidence/release/tail-06/issue-834 and narrowly scoped current external-review/remediation links."
  components:
    - "issue-834-internal-review-reconciliation"
  out_of_scope:
    - "No historical report rewrite, runtime changes, closure-only semantic proof, merge or release authorization."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Reconcile closed predecessor and all14 merged owners; preserve source-time history; keep #835 final gate corrections and #833 external report/candidate separately owned and underway. Validate bounded packet, review exact head, publish and shepherd required CI."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: #520 closed by merged PR831 observed live; verify merged remediation issues814 through821 and correcting follow-ons."
    expected_output: ".csdlc/issues/834/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: #834; #520 second review; #831; historical third-party finding TPR-002; remediation owners814-821."
    expected_output: ".csdlc/issues/834/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Current predecessor reconciliation JSON and narrative;14-row exact finding ownership map; live closure/ancestry evidence; rejecting validator and focused negative tests."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: Observe closed520 and merged closing831 ancestral to candidate; all14 IDs exactly once with owners/evidence; preserve historical text; reject stale state/wrong closingPR/nonancestor/dropped or duplicatedIDs; independent exact-head review."
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
  - "issue-834-internal-review-reconciliation"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Historical source-time report must remain immutable; live closure alone cannot discharge findings; successor evidence must be merged and ancestral."
test_strategy:
  - "Focused deterministic local Python/Git contract validation with captured live GitHub readbacks. Explicit negatives for every required rejection. Required local evidence proof plus independent review; hosted CI integration."
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
notes: "#833 owns external report retention; #835 owns final gate correction. No release approval."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.1][TAIL-06.19][review] Reconcile finalized internal-review predecessor`.

Reconcile closed predecessor and all14 merged owners; preserve source-time history; keep #835 final gate corrections and #833 external report/candidate separately owned and underway. Validate bounded packet, review exact head, publish and shepherd required CI.

## PVF Lane Plan

- Initial PVF lane from issue creation: `release-evidence`
- Planned PVF lane for execution: `release-evidence`
- Planning lane source: `Issue834 deliverables`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `3600`
- Estimated total tokens: `30000`
- Estimated validation seconds: `300`
- Issue goal token budget: `unknown`
- Variance threshold percent: `10`
- Estimate confidence: `planning_estimate`
- Estimate data source: `issue scope estimate`
- Estimate source ref: `Issue834`
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

1. Confirm dependency readiness and starting state: #520 closed by merged PR831 observed live; verify merged remediation issues814 through821 and correcting follow-ons.
2. Review repo inputs and scoped surfaces before editing: #834; #520 second review; #831; historical third-party finding TPR-002; remediation owners814-821.
3. Implement only the bounded deliverables: Current predecessor reconciliation JSON and narrative;14-row exact finding ownership map; live closure/ancestry evidence; rejecting validator and focused negative tests.
4. Run focused proof gates for acceptance: Observe closed520 and merged closing831 ancestral to candidate; all14 IDs exactly once with owners/evidence; preserve historical text; reject stale state/wrong closingPR/nonancestor/dropped or duplicatedIDs; independent exact-head review.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- issue-834-internal-review-reconciliation

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Historical source-time report must remain immutable; live closure alone cannot discharge findings; successor evidence must be merged and ancestral.

## Test Strategy

- Focused deterministic local Python/Git contract validation with captured live GitHub readbacks. Explicit negatives for every required rejection. Required local evidence proof plus independent review; hosted CI integration.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

#833 owns external report retention; #835 owns final gate correction. No release approval.
