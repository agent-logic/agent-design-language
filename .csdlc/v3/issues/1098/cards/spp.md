---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "<slug>-execution-plan"
issue: 1098
task_id: "issue-1098"
run_id: "issue-1098"
version: "1.0.5"
title: "[v0.92.2][C-SDLC v3][defect] Preserve staged cleanup evidence and reconcile publication retry outcomes"
branch: "codex/1098-cleanup-index-and-retry-outcome-safety"
generated_at: "<timestamp>"
card_status: "ready"
status: "in_progress"
activation_state: "prepared"
plan_revision: 1
initial_pvf_lane: "csdlc"
planned_pvf_lane: "csdlc"
planned_pvf_lane_source: "Issue #1098 reproduced review findings"
estimate_elapsed_seconds: "1500"
estimate_total_tokens: "18000"
estimate_validation_seconds: "300"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "10"
estimate_confidence: "medium"
estimate_data_source: "prior_focused_runs"
estimate_source_ref: "PR #1093 review regression runs"
issue_goal_ref: "Issue #1098 repair PR"
sprint_goal_ref: "<sprint_goal_ref>"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1098"
  - kind: "source_issue_prompt"
    ref: "<source_issue_prompt>"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
scope:
  files:
    - "csdlc-v3/src/commands/terminal/intent_archive.rs; csdlc-v3/src/application/intent/remote.rs; csdlc-v3/src/commands/remote; csdlc-v3/tests/installed_intent_commands.rs."
  components:
    - "<slug>"
  out_of_scope:
    - "No merge, shared binary replacement, unrelated cleanup, or authority weakening."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Bind #1098; add regression tests; refuse distinct index content during cleanup; authenticate historical retry outcomes; validate; independently review; publish repair PR."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: PR #1093 merged."
    expected_output: "<sip_card>"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Issue #1098, merged PR #1093, retained review regressions, intent_archive.rs, semantic remote recovery, and installed_intent_commands.rs."
    expected_output: "<stp_card>"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Bounded repairs, installed regressions, truthful proof and independent review."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: Distinct staged evidence is preserved by refusing cleanup before destructive effects; successful or uncertain publication retries are never classified absent using an earlier absence receipt; fresh authenticated absence still permits historical retirement; focused installed regressions and independent review pass."
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
  - "Cleanup must fail before removing staged evidence; recovery must remain uncertain on readback failure."
test_strategy:
  - "Run focused deterministic local owner and installed integration regressions with synthetic GitHub transport, then fmt, strict Clippy, diff hygiene, native exact-head proof and independent review. Required CI covers integration; no live cleanup used as a test."
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
notes: "Implementation and focused tests complete. Current exact-head proof, final independent review and PR publication remain; CI is separately required for handoff."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][C-SDLC v3][defect] Preserve staged cleanup evidence and reconcile publication retry outcomes`.

Bind #1098; add regression tests; refuse distinct index content during cleanup; authenticate historical retry outcomes; validate; independently review; publish repair PR.

## PVF Lane Plan

- Initial PVF lane from issue creation: `csdlc`
- Planned PVF lane for execution: `csdlc`
- Planning lane source: `Issue #1098 reproduced review findings`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `1500`
- Estimated total tokens: `18000`
- Estimated validation seconds: `300`
- Issue goal token budget: `unknown`
- Variance threshold percent: `10`
- Estimate confidence: `medium`
- Estimate data source: `prior_focused_runs`
- Estimate source ref: `PR #1093 review regression runs`
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

1. Confirm dependency readiness and starting state: PR #1093 merged.
2. Review repo inputs and scoped surfaces before editing: Issue #1098, merged PR #1093, retained review regressions, intent_archive.rs, semantic remote recovery, and installed_intent_commands.rs.
3. Implement only the bounded deliverables: Bounded repairs, installed regressions, truthful proof and independent review.
4. Run focused proof gates for acceptance: Distinct staged evidence is preserved by refusing cleanup before destructive effects; successful or uncertain publication retries are never classified absent using an earlier absence receipt; fresh authenticated absence still permits historical retirement; focused installed regressions and independent review pass.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- <slug>

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Cleanup must fail before removing staged evidence; recovery must remain uncertain on readback failure.

## Test Strategy

- Run focused deterministic local owner and installed integration regressions with synthetic GitHub transport, then fmt, strict Clippy, diff hygiene, native exact-head proof and independent review. Required CI covers integration; no live cleanup used as a test.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Implementation and focused tests complete. Current exact-head proof, final independent review and PR publication remain; CI is separately required for handoff.
