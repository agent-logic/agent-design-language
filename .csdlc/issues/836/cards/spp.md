---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "recursive-rust-size-execution-plan"
issue: 836
task_id: "issue-0836"
run_id: "issue-0836"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06.21][quality] Publish recursive code-size and relocation evidence"
branch: "codex/836-recursive-rust-size"
generated_at: "2026-09-11T02:59:11.281951+00:00"
card_status: "ready"
status: "in_progress"
activation_state: "bound"
plan_revision: 1
initial_pvf_lane: "local_contract"
planned_pvf_lane: "local_contract"
planned_pvf_lane_source: "#836 deterministic source-accounting evidence"
estimate_elapsed_seconds: "unknown"
estimate_total_tokens: "unknown"
estimate_validation_seconds: "unknown"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "10"
estimate_confidence: "unknown"
estimate_data_source: "not collected"
estimate_source_ref: "not collected"
issue_goal_ref: "issue-836-active-session-goal"
sprint_goal_ref: "not collected"
goal_metrics_rollup_ref: "not collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/836"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/836"
  - kind: "stp"
    ref: ".csdlc/issues/836/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/836/cards/sip.md"
scope:
  files:
    - "Issue-836 measurement script and focused tests; docs/milestones/v0.92.1/evidence/refactoring/rust-01/**; narrowly corrected current release documentation."
  components:
    - "recursive-rust-size"
  out_of_scope:
    - "No LoC quota; no Rust runtime behavior changes; source size does not prove behavior; no broad validation suite."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Freeze PR547 baseline/candidate and recursive resilience source plus declared test scope; build deterministic Git-object inventory and diff/relocation report; retain exact evidence; correct unsupported release claims; validate and review before publication."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: #499 closed; PR547 merged as e986de6d06aacd385de93dd033def77a718c1581. Use its first parent as pre-refactor baseline and merged revision as retained post-refactor candidate, resolved to full SHAs."
    expected_output: ".csdlc/issues/836/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: #836 TPR-004; #499 and merged PR547; retained validation-impact validator; v0.92.1 Rust refactoring feature and planning docs."
    expected_output: ".csdlc/issues/836/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Recursive tracked .rs inventory at both exact revisions; byte-stable size/diff report; rename and line-relocation evidence; current-doc claims audit; negative guardrails."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: All tracked Rust files under declared recursive scope included at both revisions; additions and deletions separate from relocation; exact revisions and deterministic output; unsupported claims corrected; focused validator and independent exact-head review pass."
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
  - "recursive-rust-size"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Identical lines are relocation candidates, not proof of semantic movement; rename heuristics must disclose threshold; blank/comment lines must be defined; missing revisions and narrowed scope must fail closed."
test_strategy:
  - "Small deterministic temporary-Git fixtures for nested files, additions/deletions, renames, partial relocation and revision identity; repeat-byte output check; source inventory cross-check; current-document claim audit."
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
notes: "Current native binding and SOR supersede pre-execution prompt wording; no runtime edits and no release approval."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.1][TAIL-06.21][quality] Publish recursive code-size and relocation evidence`.

Freeze PR547 baseline/candidate and recursive resilience source plus declared test scope; build deterministic Git-object inventory and diff/relocation report; retain exact evidence; correct unsupported release claims; validate and review before publication.

## PVF Lane Plan

- Initial PVF lane from issue creation: `local_contract`
- Planned PVF lane for execution: `local_contract`
- Planning lane source: `#836 deterministic source-accounting evidence`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `unknown`
- Estimated total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Issue goal token budget: `unknown`
- Variance threshold percent: `10`
- Estimate confidence: `unknown`
- Estimate data source: `not collected`
- Estimate source ref: `not collected`
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

1. Confirm dependency readiness and starting state: #499 closed; PR547 merged as e986de6d06aacd385de93dd033def77a718c1581. Use its first parent as pre-refactor baseline and merged revision as retained post-refactor candidate, resolved to full SHAs.
2. Review repo inputs and scoped surfaces before editing: #836 TPR-004; #499 and merged PR547; retained validation-impact validator; v0.92.1 Rust refactoring feature and planning docs.
3. Implement only the bounded deliverables: Recursive tracked .rs inventory at both exact revisions; byte-stable size/diff report; rename and line-relocation evidence; current-doc claims audit; negative guardrails.
4. Run focused proof gates for acceptance: All tracked Rust files under declared recursive scope included at both revisions; additions and deletions separate from relocation; exact revisions and deterministic output; unsupported claims corrected; focused validator and independent exact-head review pass.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- recursive-rust-size

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Identical lines are relocation candidates, not proof of semantic movement; rename heuristics must disclose threshold; blank/comment lines must be defined; missing revisions and narrowed scope must fail closed.

## Test Strategy

- Small deterministic temporary-Git fixtures for nested files, additions/deletions, renames, partial relocation and revision identity; repeat-byte output check; source inventory cross-check; current-document claim audit.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Current native binding and SOR supersede pre-execution prompt wording; no runtime edits and no release approval.
