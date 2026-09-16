---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "scope-rebind-validator-recovery-execution-plan"
issue: 1003
task_id: "issue-1003"
run_id: "issue-1003"
version: "v0.92.2"
title: "[v0.92.2][C-SDLC] Restore rebind and validator replacement after scope amendments"
branch: "codex/1003-scope-rebind-validator-recovery"
generated_at: "2026-09-16T02:43:15.759077+00:00"
card_status: "ready"
status: "in_progress"
activation_state: "ready_for_binding"
plan_revision: 1
initial_pvf_lane: "csdlc"
planned_pvf_lane: "csdlc"
planned_pvf_lane_source: "issue source and touched native owner"
estimate_elapsed_seconds: "3600"
estimate_total_tokens: "60000"
estimate_validation_seconds: "900"
issue_goal_token_budget: "unbounded"
variance_threshold_percent: "50"
estimate_confidence: "low"
estimate_data_source: "planning estimate, not a limit"
estimate_source_ref: "https://github.com/agent-logic/agent-design-language/issues/1003"
issue_goal_ref: "Worker10 #1003 and #1006 implementation"
sprint_goal_ref: "Not a sprint child; follow-up tooling repair"
goal_metrics_rollup_ref: "session goal"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1003"
  - kind: "source_issue_prompt"
    ref: ".git/csdlc-v3/local/invocations/worker10-1003/source-issue.json"
  - kind: "stp"
    ref: ".csdlc/issues/1003/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/1003/cards/sip.md"
scope:
  files:
    - "csdlc-v3/src/application/intent/{context,local}.rs; semantic binding owner; focused intent/semantic regression tests; docs/csdlc-v3/INTENT_COMMANDS.md"
  components:
    - "scope-rebind-validator-recovery"
  out_of_scope:
    - "No raw state edits, automatic phase advancement, weaker validator admission, fallback authority or changes to #970."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Reproduce both #1003 failure paths in focused tests. Inspect semantic bind operation identity and admission after scope_acceptance. Add the smallest explicit rebind/head-refresh path preserving registered topology and version guards. Permit typed validator replacement after the guarded refresh without executing retained validators. Test negative ownership/authority/recovery and evidence invalidation cases; run installed candidate scenarios, independent review and CI."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: No unmerged implementation dependency; #970 is the reproducer, not a mutation target."
    expected_output: ".csdlc/issues/1003/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Issue #1003 and current native intent/semantic binding, edit, proof, recovery and transaction contracts."
    expected_output: ".csdlc/issues/1003/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Reproduce both #1003 failure paths in focused tests. Inspect semantic bind operation identity and admission after scope_acceptance. Add the smallest explicit rebind/head-refresh path preserving registered topology and version guards. Permit typed validator replacement after the guarded refresh without executing retained validators. Test negative ownership/authority/recovery and evidence invalidation cases; run installed candidate scenarios, independent review and CI."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: A scope-rewound issue rebinds in its registered checkout at unchanged HEAD; changed HEAD refreshes binding without running old validators; typed validator replacement succeeds before proof; stale authority, branch/worktree mismatch, stale requests and pending recovery still fail closed; evidence invalidation remains explicit."
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
  - "scope-rebind-validator-recovery"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Do not refresh changed branch/worktree or ignore stale generated requests; preserve recovery and evidence invalidation."
test_strategy:
  - "Focused Rust intent and semantic-owner tests, installed candidate rebind and validator replacement fixtures, negative identity/admission cases, cargo fmt and strict Clippy, then required GitHub CI tests and coverage. PVF deterministic local contract/regression proof; small CPU/local Git and isolated synthetic transport; required gate; no provider/cloud."
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
notes: "Implemented and locally proved explicit rebind/head refresh plus validator replacement. Independent working-diff review P2 blocked-doctor acceptance was fixed and regressed. Exact-head review and hosted CI pending; no merge authority."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][C-SDLC] Restore rebind and validator replacement after scope amendments`.

Reproduce both #1003 failure paths in focused tests. Inspect semantic bind operation identity and admission after scope_acceptance. Add the smallest explicit rebind/head-refresh path preserving registered topology and version guards. Permit typed validator replacement after the guarded refresh without executing retained validators. Test negative ownership/authority/recovery and evidence invalidation cases; run installed candidate scenarios, independent review and CI.

## PVF Lane Plan

- Initial PVF lane from issue creation: `csdlc`
- Planned PVF lane for execution: `csdlc`
- Planning lane source: `issue source and touched native owner`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `3600`
- Estimated total tokens: `60000`
- Estimated validation seconds: `900`
- Issue goal token budget: `unbounded`
- Variance threshold percent: `50`
- Estimate confidence: `low`
- Estimate data source: `planning estimate, not a limit`
- Estimate source ref: `https://github.com/agent-logic/agent-design-language/issues/1003`
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

1. Confirm dependency readiness and starting state: No unmerged implementation dependency; #970 is the reproducer, not a mutation target.
2. Review repo inputs and scoped surfaces before editing: Issue #1003 and current native intent/semantic binding, edit, proof, recovery and transaction contracts.
3. Implement only the bounded deliverables: Reproduce both #1003 failure paths in focused tests. Inspect semantic bind operation identity and admission after scope_acceptance. Add the smallest explicit rebind/head-refresh path preserving registered topology and version guards. Permit typed validator replacement after the guarded refresh without executing retained validators. Test negative ownership/authority/recovery and evidence invalidation cases; run installed candidate scenarios, independent review and CI.
4. Run focused proof gates for acceptance: A scope-rewound issue rebinds in its registered checkout at unchanged HEAD; changed HEAD refreshes binding without running old validators; typed validator replacement succeeds before proof; stale authority, branch/worktree mismatch, stale requests and pending recovery still fail closed; evidence invalidation remains explicit.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- scope-rebind-validator-recovery

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Do not refresh changed branch/worktree or ignore stale generated requests; preserve recovery and evidence invalidation.

## Test Strategy

- Focused Rust intent and semantic-owner tests, installed candidate rebind and validator replacement fixtures, negative identity/admission cases, cargo fmt and strict Clippy, then required GitHub CI tests and coverage. PVF deterministic local contract/regression proof; small CPU/local Git and isolated synthetic transport; required gate; no provider/cloud.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Implemented and locally proved explicit rebind/head refresh plus validator replacement. Independent working-diff review P2 blocked-doctor acceptance was fixed and regressed. Exact-head review and hosted CI pending; no merge authority.
