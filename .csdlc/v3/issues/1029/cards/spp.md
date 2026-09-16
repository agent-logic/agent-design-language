---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "<slug>-execution-plan"
issue: 1029
task_id: "issue-1029"
run_id: "issue-1029"
version: "1.0.5"
title: "[v0.92.2][C-SDLC v3][defect] Restore preparation and issue edits for legacy native records"
branch: "codex/1029-legacy-native-record-preparation-and-edits"
generated_at: "<timestamp>"
card_status: "ready"
status: "in_progress"
activation_state: "bound"
plan_revision: 1
initial_pvf_lane: "csdlc"
planned_pvf_lane: "csdlc"
planned_pvf_lane_source: "Issue #1029 native semantic transaction and remote mutation acceptance surface"
estimate_elapsed_seconds: "unknown"
estimate_total_tokens: "unknown"
estimate_validation_seconds: "unknown"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "10"
estimate_confidence: "unknown"
estimate_data_source: "not_collected"
estimate_source_ref: "issue-1029"
issue_goal_ref: "Issue #1029 implementation and reviewed PR"
sprint_goal_ref: "Sprint 11 preparation unblocker"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1029"
  - kind: "source_issue_prompt"
    ref: "<source_issue_prompt>"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
scope:
  files:
    - "csdlc-v3 semantic context, local preparation/edit, remote issue mutation preview/execute admission, recovery, focused tests, operator documentation, and lifecycle evidence."
  components:
    - "<slug>"
  out_of_scope:
    - "No Sprint 11 execution, product dependency satisfaction, release authorization, or general live-conversion qualification."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Admit one explicit writer-fenced semantic preparation path for a structurally complete, unbound legacy native-v3 ready record; bind its repository, branch, worktree, generation and lifecycle digest; reject pending, bound, stale, damaged or mismatched sources; require existing-issue GitHub mutation previews to pass the same semantic admission as execution; document missing and legacy initialization; prove the boundary with focused and adjacent transaction tests."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: No product dependency. This native-v3 defect is the blocker for planning updates to #916-#925 and initialization of #937."
    expected_output: "<sip_card>"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Issue #1029; copied pre-semantic prepared records; absent-record fixture; semantic transaction store; local prepare/edit commands; remote issue mutation and recovery receipts."
    expected_output: "<stp_card>"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: A narrow guarded compatibility path, documented missing-record initialization, preview/execute parity, idempotent recovery, negative guard coverage, and installed-owner proof."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: Valid unbound legacy records can be prepared and edited; absent records can be initialized; status distinguishes structural preparation from dependency readiness; preview exposes execute prerequisites; stale digest, wrong repository, ownership conflicts, pending writes, invalid cards, and unauthorized mutations fail without effects; retries reconcile uncertain writes without duplicates."
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
  - "<slug>"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Compatibility admission must be exact enough that corrupt or cross-linked residue still requires recovery."
test_strategy:
  - "Run focused legacy-record positive and negative tests with nonzero denominators, adjacent semantic transaction tests, native owner lane, formatting, strict lint, diff hygiene, installed binary verification, and independent exact-head review."
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
notes: "The compatibility path preserves all legacy source bytes and does not admit active worktrees, binding records, pending journals, stale lifecycle digests, wrong repository identity or missing cards. Full component validation, exact-head review, publication and CI remain pending."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][C-SDLC v3][defect] Restore preparation and issue edits for legacy native records`.

Admit one explicit writer-fenced semantic preparation path for a structurally complete, unbound legacy native-v3 ready record; bind its repository, branch, worktree, generation and lifecycle digest; reject pending, bound, stale, damaged or mismatched sources; require existing-issue GitHub mutation previews to pass the same semantic admission as execution; document missing and legacy initialization; prove the boundary with focused and adjacent transaction tests.

## PVF Lane Plan

- Initial PVF lane from issue creation: `csdlc`
- Planned PVF lane for execution: `csdlc`
- Planning lane source: `Issue #1029 native semantic transaction and remote mutation acceptance surface`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `unknown`
- Estimated total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Issue goal token budget: `unknown`
- Variance threshold percent: `10`
- Estimate confidence: `unknown`
- Estimate data source: `not_collected`
- Estimate source ref: `issue-1029`
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

1. Confirm dependency readiness and starting state: No product dependency. This native-v3 defect is the blocker for planning updates to #916-#925 and initialization of #937.
2. Review repo inputs and scoped surfaces before editing: Issue #1029; copied pre-semantic prepared records; absent-record fixture; semantic transaction store; local prepare/edit commands; remote issue mutation and recovery receipts.
3. Implement only the bounded deliverables: A narrow guarded compatibility path, documented missing-record initialization, preview/execute parity, idempotent recovery, negative guard coverage, and installed-owner proof.
4. Run focused proof gates for acceptance: Valid unbound legacy records can be prepared and edited; absent records can be initialized; status distinguishes structural preparation from dependency readiness; preview exposes execute prerequisites; stale digest, wrong repository, ownership conflicts, pending writes, invalid cards, and unauthorized mutations fail without effects; retries reconcile uncertain writes without duplicates.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- <slug>

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Compatibility admission must be exact enough that corrupt or cross-linked residue still requires recovery.

## Test Strategy

- Run focused legacy-record positive and negative tests with nonzero denominators, adjacent semantic transaction tests, native owner lane, formatting, strict lint, diff hygiene, installed binary verification, and independent exact-head review.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

The compatibility path preserves all legacy source bytes and does not admit active worktrees, binding records, pending journals, stale lifecycle digests, wrong repository identity or missing cards. Full component validation, exact-head review, publication and CI remain pending.
