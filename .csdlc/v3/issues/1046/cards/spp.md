---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "<slug>-execution-plan"
issue: 1046
task_id: "issue-1046"
run_id: "issue-1046"
version: "v0.92.2"
title: "[C-SDLC v3][defect] Amend prepared publication metadata through native guards"
branch: "codex/1046-publication-plan-amendment"
generated_at: "<timestamp>"
card_status: "ready"
status: "pre_execution"
activation_state: "ready_for_binding"
plan_revision: 1
initial_pvf_lane: "owner_binary"
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
    ref: "https://github.com/agent-logic/agent-design-language/issues/1046"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1046"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
scope:
  files:
    - "Issue #1046 and #1044 publication blocker; csdlc-v3 application intent, semantic storage and protocol, preparation/publication guards and focused tests."
  components:
    - "<slug>"
  out_of_scope:
    - "<non_goals_inline>"
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Validate publication closing relations at prepare and allow guarded native amendment of publication title/body/draft before publication, preserving base and exact issue identity."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: No unresolved numeric dependency. Native binding and issue goal required before repair."
    expected_output: "<sip_card>"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Issue #1046 and #1044 publication blocker; csdlc-v3 application intent, semantic storage and protocol, preparation/publication guards and focused tests."
    expected_output: "<stp_card>"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Validate publication closing relations at prepare and allow guarded native amendment of publication title/body/draft before publication, preserving base and exact issue identity."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: Reject malformed or unrelated closing relations before preparation mutates state; amend only publication metadata through typed exact-version admission, retain history, invalidate downstream evidence, reject mixed edit surfaces and stale bindings. Use reviewed isolated candidate to unblock #1044."
    expected_output: "validation evidence recorded in VPP/SOR"
    allowed_mode: "execution_after_approval"
  - id: "step-5"
    description: "Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges."
    expected_output: "reviewed SRP and truthful VPP/SOR"
    allowed_mode: "execution_after_approval"
codex_plan:
  - step: "Confirm dependencies and starting state from the source issue prompt."
    status: "pending"
  - step: "Inspect repo inputs and target surfaces before editing."
    status: "pending"
  - step: "Implement the bounded deliverables only."
    status: "pending"
  - step: "Run focused validation and proof gates."
    status: "pending"
  - step: "Record issue-specific SRP findings and VPP/SOR outcome truth."
    status: "pending"
affected_areas:
  - "<slug>"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "<risks_inline>"
test_strategy:
  - "Focused deterministic local owner-contract tests for preparation validation, publication amendment, immutable history, stale version and mixed/foreign input rejection; formatting and clippy; independent review. No cloud calls or shared owner replacement."
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
notes: "No raw GitHub writes, hand-edited lifecycle state, shared owner replacement, or base/issue identity changes."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[C-SDLC v3][defect] Amend prepared publication metadata through native guards`.

Validate publication closing relations at prepare and allow guarded native amendment of publication title/body/draft before publication, preserving base and exact issue identity.

## PVF Lane Plan

- Initial PVF lane from issue creation: `owner_binary`
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

1. [pending] Confirm dependencies and starting state from the source issue prompt.
2. [pending] Inspect repo inputs and target surfaces before editing.
3. [pending] Implement the bounded deliverables only.
4. [pending] Run focused validation and proof gates.
5. [pending] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: No unresolved numeric dependency. Native binding and issue goal required before repair.
2. Review repo inputs and scoped surfaces before editing: Issue #1046 and #1044 publication blocker; csdlc-v3 application intent, semantic storage and protocol, preparation/publication guards and focused tests.
3. Implement only the bounded deliverables: Validate publication closing relations at prepare and allow guarded native amendment of publication title/body/draft before publication, preserving base and exact issue identity.
4. Run focused proof gates for acceptance: Reject malformed or unrelated closing relations before preparation mutates state; amend only publication metadata through typed exact-version admission, retain history, invalidate downstream evidence, reject mixed edit surfaces and stale bindings. Use reviewed isolated candidate to unblock #1044.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- <slug>

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- <risks_inline>

## Test Strategy

- Focused deterministic local owner-contract tests for preparation validation, publication amendment, immutable history, stale version and mixed/foreign input rejection; formatting and clippy; independent review. No cloud calls or shared owner replacement.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

No raw GitHub writes, hand-edited lifecycle state, shared owner replacement, or base/issue identity changes.
