---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "codefriend-four-plus-one-architecture-execution-plan"
issue: 1109
task_id: "issue-1109"
run_id: "issue-1109"
version: "1.0.5"
title: "[v0.92.2][CF-ARCH] Generate the complete 4+1 architecture package in Beta 1"
branch: "codex/1109-codefriend-four-plus-one-architecture"
generated_at: "<timestamp>"
card_status: "ready"
status: "IN_PROGRESS"
activation_state: "bound"
plan_revision: 1
initial_pvf_lane: "runtime_full_validation"
planned_pvf_lane: "runtime_full_validation"
planned_pvf_lane_source: "issue-1109-runtime-code-scope"
estimate_elapsed_seconds: "unknown"
estimate_total_tokens: "unknown"
estimate_validation_seconds: "unknown"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "25"
estimate_confidence: "low"
estimate_data_source: "source-inspection"
estimate_source_ref: "issue-1109"
issue_goal_ref: "issue-1109-current-session"
sprint_goal_ref: "not-assigned"
goal_metrics_rollup_ref: "issue-1109-current-session"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1109"
  - kind: "source_issue_prompt"
    ref: "<source_issue_prompt>"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
scope:
  files:
    - "adl/src/codefriend/architecture/four_plus_one.rs; journey and publication owners; CLI; focused tests; docs/codefriend; adopted Beta1 feature contracts."
  components:
    - "codefriend-four-plus-one-architecture"
  out_of_scope:
    - "No invented runtime topology, universal language support, automatic architecture rewrites, or new lifecycle checkpoints."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Generate a source-bound 4+1 package in the existing CodeFriend journey and publish through existing report owners; qualify complete, incomplete and conflicting evidence."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: #1097 and #1102 are closed; preserve #915 independent qualification ownership. Operator assigned execution in this session."
    expected_output: "<sip_card>"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: adl/src/codefriend/architecture; integration/journey.rs; publication; agent/server journey routes; issue #1109."
    expected_output: "<stp_card>"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Typed evidence-grounded five-view package; shared entity IDs; scenario traces; editable/rendered diagrams; installed journey and Markdown/HTML/PDF retrieval."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: Issue #1109 criteria; no placeholder-only success, fabricated evidence or universal-support claim."
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
  - "codefriend-four-plus-one-architecture"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Untrusted diagram labels and references; declaration versus observation; retained artifact verification; partial output cannot claim complete."
test_strategy:
  - "Focused Cargo contract/generation tests for complete, incomplete, conflicting and tampered evidence; installed generation on ADL and a bounded external repository; HTML/PDF readability; redaction and evidence integrity."
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
notes: "Installed native generation and retrieval passed on ADL and bounded external source. Fourteen package and twenty-one journey tests passed at 69afb828f3. Final presentation repair, exact-head proof/review and PR publication remain; hosted/paired activity integration stays #1101 and independent Beta1 qualification stays #915."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][CF-ARCH] Generate the complete 4+1 architecture package in Beta 1`.

Generate a source-bound 4+1 package in the existing CodeFriend journey and publish through existing report owners; qualify complete, incomplete and conflicting evidence.

## PVF Lane Plan

- Initial PVF lane from issue creation: `runtime_full_validation`
- Planned PVF lane for execution: `runtime_full_validation`
- Planning lane source: `issue-1109-runtime-code-scope`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `unknown`
- Estimated total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Issue goal token budget: `unknown`
- Variance threshold percent: `25`
- Estimate confidence: `low`
- Estimate data source: `source-inspection`
- Estimate source ref: `issue-1109`
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

1. Confirm dependency readiness and starting state: #1097 and #1102 are closed; preserve #915 independent qualification ownership. Operator assigned execution in this session.
2. Review repo inputs and scoped surfaces before editing: adl/src/codefriend/architecture; integration/journey.rs; publication; agent/server journey routes; issue #1109.
3. Implement only the bounded deliverables: Typed evidence-grounded five-view package; shared entity IDs; scenario traces; editable/rendered diagrams; installed journey and Markdown/HTML/PDF retrieval.
4. Run focused proof gates for acceptance: Issue #1109 criteria; no placeholder-only success, fabricated evidence or universal-support claim.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- codefriend-four-plus-one-architecture

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Untrusted diagram labels and references; declaration versus observation; retained artifact verification; partial output cannot claim complete.

## Test Strategy

- Focused Cargo contract/generation tests for complete, incomplete, conflicting and tampered evidence; installed generation on ADL and a bounded external repository; HTML/PDF readability; redaction and evidence integrity.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Installed native generation and retrieval passed on ADL and bounded external source. Fourteen package and twenty-one journey tests passed at 69afb828f3. Final presentation repair, exact-head proof/review and PR publication remain; hosted/paired activity integration stays #1101 and independent Beta1 qualification stays #915.
