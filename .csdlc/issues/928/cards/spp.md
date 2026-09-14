---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "combined-sprint-review-execution-plan"
issue: 928
task_id: "issue-0928"
run_id: "issue-0928"
version: "v0.92.2"
title: "[v0.92.2][Sprint 2] Runtime/provider foundations and ingestion"
branch: "codex/928-combined-sprint-review"
generated_at: "2026-09-12T06:54:28.990815+00:00"
card_status: "ready"
status: "COMPLETED"
activation_state: "executed"
plan_revision: 1
initial_pvf_lane: "docs_diff_check"
planned_pvf_lane: "docs_diff_check"
planned_pvf_lane_source: "Review preparation is docs and evidence only; final review lanes are independently selected from actual code scope."
estimate_elapsed_seconds: "unknown"
estimate_total_tokens: "unknown"
estimate_validation_seconds: "unknown"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "unknown"
estimate_confidence: "unknown"
estimate_data_source: "unknown"
estimate_source_ref: "unknown"
issue_goal_ref: "Active Sprint 2 completion objective under #928."
sprint_goal_ref: "#928"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/928"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/928"
  - kind: "stp"
    ref: ".csdlc/issues/928/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/928/cards/sip.md"
scope:
  files:
    - ".csdlc/evidence/928; .csdlc/issues/928; .csdlc/transactions/completed/928; docs/milestones/v0.92.2/SPRINT_v0.92.2.md"
  components:
    - "combined-sprint-review"
  out_of_scope:
    - "No child implementation, new product scope, paid execution, historical evidence rewrite, automatic merge, release, public publication, or claim of child terminal closeout."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Reconcile all nine original Sprint 2 children at one closing revision, account for corrective #967 separately, run all seven combined-review lanes, update planning truth, validate the packet, obtain exact-head review, and publish #928 for operator-controlled merge."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: All nine original children and corrective #967 are closed by reviewed, green, merged PRs and ancestral to the review revision."
    expected_output: ".csdlc/issues/928/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: #848, #854, #855, #876, #877, #878, #879, #880, #881; corrective #967/#968; issue-level exact-head reviews; accepted-head CI; hosted acceptance; Git ancestry; milestone Sprint plan."
    expected_output: ".csdlc/issues/928/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: .csdlc/evidence/928 findings-first sprint review, seven lane records, exact child and corrective ledgers, corrected Sprint 2 planning record, and truthful lifecycle cards."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: Preserve all nine original children exactly once; account for #967 separately; bind accepted heads, CI, merges and ancestry to one revision; record seven lane results, limitations and closeout truth; leave merge operator-controlled."
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
  - "combined-sprint-review"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "unknown"
test_strategy:
  - "Native six-card validation; JSON and Markdown hygiene; exact nine-child plus separate corrective accounting; live issue/PR/check observation; Git ancestry; independent exact-head review; CI after publication."
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
notes: "P3 CLI help discoverability defect; no retained GitHub/CI acquisition-to-store end-to-end test; child native finish/cleanup remains asynchronous; #848 is planning evidence rather than extraction proof; hosted packet does not prove invoice cost."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][Sprint 2] Runtime/provider foundations and ingestion`.

Reconcile all nine original Sprint 2 children at one closing revision, account for corrective #967 separately, run all seven combined-review lanes, update planning truth, validate the packet, obtain exact-head review, and publish #928 for operator-controlled merge.

## PVF Lane Plan

- Initial PVF lane from issue creation: `docs_diff_check`
- Planned PVF lane for execution: `docs_diff_check`
- Planning lane source: `Review preparation is docs and evidence only; final review lanes are independently selected from actual code scope.`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `unknown`
- Estimated total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Issue goal token budget: `unknown`
- Variance threshold percent: `unknown`
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
5. [completed] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: All nine original children and corrective #967 are closed by reviewed, green, merged PRs and ancestral to the review revision.
2. Review repo inputs and scoped surfaces before editing: #848, #854, #855, #876, #877, #878, #879, #880, #881; corrective #967/#968; issue-level exact-head reviews; accepted-head CI; hosted acceptance; Git ancestry; milestone Sprint plan.
3. Implement only the bounded deliverables: .csdlc/evidence/928 findings-first sprint review, seven lane records, exact child and corrective ledgers, corrected Sprint 2 planning record, and truthful lifecycle cards.
4. Run focused proof gates for acceptance: Preserve all nine original children exactly once; account for #967 separately; bind accepted heads, CI, merges and ancestry to one revision; record seven lane results, limitations and closeout truth; leave merge operator-controlled.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- combined-sprint-review

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- unknown

## Test Strategy

- Native six-card validation; JSON and Markdown hygiene; exact nine-child plus separate corrective accounting; live issue/PR/check observation; Git ancestry; independent exact-head review; CI after publication.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

P3 CLI help discoverability defect; no retained GitHub/CI acquisition-to-store end-to-end test; child native finish/cleanup remains asynchronous; #848 is planning evidence rather than extraction proof; hosted packet does not prove invoice cost.
