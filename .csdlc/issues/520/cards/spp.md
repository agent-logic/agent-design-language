---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "tail-04-internal-review-execution-plan"
issue: 520
task_id: "issue-0520"
run_id: "issue-0520"
version: "1.0.5"
title: "[v0.92.1][TAIL-04] Internal review"
branch: "codex/520-internal-review"
generated_at: "2026-09-09T19:19:55Z"
card_status: "approved"
status: "in_progress"
activation_state: "active_second_review"
plan_revision: 1
initial_pvf_lane: "review-complete"
planned_pvf_lane: "review-complete-exact-candidate"
planned_pvf_lane_source: "issue #520 acceptance and internal-review-plan.md"
estimate_elapsed_seconds: "43200"
estimate_total_tokens: "140000"
estimate_validation_seconds: "7200"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "10"
estimate_confidence: "medium"
estimate_data_source: "prior #520 review run and current rerun scope"
estimate_source_ref: "issue #520 historical packet"
issue_goal_ref: "issue-520-internal-review-rerun"
sprint_goal_ref: "v0.92.1-tail-review"
goal_metrics_rollup_ref: "v0.92.1-tail-review"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/520"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/520"
  - kind: "stp"
    ref: ".csdlc/issues/520/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/520/cards/sip.md"
scope:
  files:
    - "TAIL-04 review packet, issue #520 lifecycle truth, complete v0.92.1 base-to-candidate surfaces"
  components:
    - "tail-04-internal-review"
  out_of_scope:
    - "product remediation, external review, release approval, deployment, paid execution"
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "After #758 merges, fetch origin/main, require both #718 and #758 merge commits as ancestors, freeze that exact candidate, rebuild all denominators, rerun every mandatory specialist lane, synthesize every supported finding, validate the packet, and obtain exact-head review."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: #718/PR #809 and #758/PR #805 are merged and present in frozen candidate fb6cbc7f619daa54f901fd2d12f480add682ace3"
    expected_output: ".csdlc/issues/520/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: issue #520, milestone v0.92.1 specification and issue graph, TAIL-04 runbook and validators, merged gate evidence"
    expected_output: ".csdlc/issues/520/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: gate manifest, complete denominators, specialist reports, finding register, synthesis, validation and review evidence"
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: complete inventory and disposition; exact evidence binding; explicit partial/inert/unproven outcomes; no non-proving pass credit; exact-head packet validation"
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
    status: "exact_head_review_passed_publication_in_progress"
affected_areas:
  - "tail-04-internal-review"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "candidate drift, incomplete denominator, lane overlap or omission, unsupported synthesis, stale lifecycle truth"
test_strategy:
  - "gate preflight; production validator negative fixtures; complete specialist lane coverage; exact-head packet review"
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
notes: "Independent exact-head packet review passed at 770980ccb68d06753f8b6b275bd7dffca4038889 with no actionable packet findings. The packet truthfully retains 14 accepted candidate-product findings routed under #522. PR publication and hosted CI remain pending."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.1][TAIL-04] Internal review`.

After #758 merges, fetch origin/main, require both #718 and #758 merge commits as ancestors, freeze that exact candidate, rebuild all denominators, rerun every mandatory specialist lane, synthesize every supported finding, validate the packet, and obtain exact-head review.

## PVF Lane Plan

- Initial PVF lane from issue creation: `review-complete`
- Planned PVF lane for execution: `review-complete-exact-candidate`
- Planning lane source: `issue #520 acceptance and internal-review-plan.md`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `43200`
- Estimated total tokens: `140000`
- Estimated validation seconds: `7200`
- Issue goal token budget: `unknown`
- Variance threshold percent: `10`
- Estimate confidence: `medium`
- Estimate data source: `prior #520 review run and current rerun scope`
- Estimate source ref: `issue #520 historical packet`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Goal Accounting Plan

Carry `issue_goal_ref`, `sprint_goal_ref`, and `goal_metrics_rollup_ref` in frontmatter so later tooling can roll planning and outcome metrics up without duplicating machine-local goal details in prose.

## Codex Plan

1. [completed] Confirm dependencies and starting state from the source issue prompt.
2. [completed] Inspect repo inputs and target surfaces before editing.
3. [completed] Implement the bounded deliverables only.
4. [completed] Run focused validation and proof gates.
5. [exact_head_review_passed_publication_in_progress] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: #718/PR #809 and #758/PR #805 are merged and present in frozen candidate fb6cbc7f619daa54f901fd2d12f480add682ace3
2. Review repo inputs and scoped surfaces before editing: issue #520, milestone v0.92.1 specification and issue graph, TAIL-04 runbook and validators, merged gate evidence
3. Implement only the bounded deliverables: gate manifest, complete denominators, specialist reports, finding register, synthesis, validation and review evidence
4. Run focused proof gates for acceptance: complete inventory and disposition; exact evidence binding; explicit partial/inert/unproven outcomes; no non-proving pass credit; exact-head packet validation
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- tail-04-internal-review

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- candidate drift, incomplete denominator, lane overlap or omission, unsupported synthesis, stale lifecycle truth

## Test Strategy

- gate preflight; production validator negative fixtures; complete specialist lane coverage; exact-head packet review

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Independent exact-head packet review passed at 770980ccb68d06753f8b6b275bd7dffca4038889 with no actionable packet findings. The packet truthfully retains 14 accepted candidate-product findings routed under #522. PR publication and hosted CI remain pending.
