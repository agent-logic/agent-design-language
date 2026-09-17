---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "<slug>-execution-plan"
issue: 897
task_id: "issue-0897"
run_id: "issue-0897"
version: "1.0.5"
title: "[v0.92.2][CF-RENDER-HTML] Render an approved review as HTML"
branch: "codex/897-v0922-codefriend-html-renderer"
generated_at: "<timestamp>"
card_status: "ready"
status: "<status>"
activation_state: "<activation_state>"
plan_revision: 1
initial_pvf_lane: "<initial_pvf_lane>"
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
    ref: "https://github.com/agent-logic/agent-design-language/issues/897"
  - kind: "source_issue_prompt"
    ref: "<source_issue_prompt>"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
scope:
  files:
    - "adl/src/codefriend/publication/html.rs; minimal shared publication helpers required for semantic parity; adl/src/codefriend/publication/mod.rs; adl/src/cli/codefriend_cmd.rs; adl/tests/codefriend_render_html.rs; issue-local retained HTML/browser evidence and lifecycle records."
  components:
    - "<slug>"
  out_of_scope:
    - "<non_goals_inline>"
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "<plan_summary>"
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: Issue #896 is satisfied by merged PR #1032 at merge commit 7c09a6526714e562ef8a458652b289ad0e7b63b2; #898 is an independent sibling and not a dependency."
    expected_output: "<sip_card>"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Issue #897; accepted #896 Markdown publication contract and fixtures; governed review, synthesis, remediation and test-plan artifacts; current native C-SDLC authority."
    expected_output: "<stp_card>"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Installed local HTML export, bound manifest, complete semantic parity with the approved Markdown baseline, safe local-only navigation, focused positive and denial tests, actual browser inspection evidence, and truthful review/publication records."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: HTML contains the complete governed claim set and stable finding/evidence/action/test identities; manifest binds exact inputs, renderer, approval and output; stale, unapproved, tampered or unsafe inputs fail closed; hostile markup cannot create active script, attributes or remote fetches; actual output opens and navigates accessibly in a browser; no public hosting, PDF renderer or unrelated sibling scope is absorbed."
    expected_output: "validation evidence recorded in VPP/SOR"
    allowed_mode: "execution_after_approval"
  - id: "step-5"
    description: "Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges."
    expected_output: "reviewed SRP and truthful VPP/SOR"
    allowed_mode: "execution_after_approval"
codex_plan:
  - step: "Confirm dependencies and starting state from the source issue prompt."
    status: "<step_1_status>"
  - step: "Inspect repo inputs and target surfaces before editing."
    status: "<step_2_status>"
  - step: "Implement the bounded deliverables only."
    status: "<step_3_status>"
  - step: "Run focused validation and proof gates."
    status: "<step_4_status>"
  - step: "Record issue-specific SRP findings and VPP/SOR outcome truth."
    status: "<step_5_status>"
affected_areas:
  - "<slug>"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "<risks_inline>"
test_strategy:
  - "Run `cargo test --manifest-path adl/Cargo.toml --test codefriend_render_html`, strict relevant Clippy, formatting, diff hygiene, installed CLI positive/denial proof, browser navigation and no-remote-fetch inspection, then independent exact-head review and required CI."
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
notes: "Preserve #896 create-only anchored output and retained-byte validation invariants. Serialize shared CLI wiring with #898. Do not duplicate a second publication authority model or weaken approval, redaction, path confinement, output freshness or provenance checks."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][CF-RENDER-HTML] Render an approved review as HTML`.

<plan_summary>

## PVF Lane Plan

- Initial PVF lane from issue creation: `<initial_pvf_lane>`
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

1. [<step_1_status>] Confirm dependencies and starting state from the source issue prompt.
2. [<step_2_status>] Inspect repo inputs and target surfaces before editing.
3. [<step_3_status>] Implement the bounded deliverables only.
4. [<step_4_status>] Run focused validation and proof gates.
5. [<step_5_status>] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: Issue #896 is satisfied by merged PR #1032 at merge commit 7c09a6526714e562ef8a458652b289ad0e7b63b2; #898 is an independent sibling and not a dependency.
2. Review repo inputs and scoped surfaces before editing: Issue #897; accepted #896 Markdown publication contract and fixtures; governed review, synthesis, remediation and test-plan artifacts; current native C-SDLC authority.
3. Implement only the bounded deliverables: Installed local HTML export, bound manifest, complete semantic parity with the approved Markdown baseline, safe local-only navigation, focused positive and denial tests, actual browser inspection evidence, and truthful review/publication records.
4. Run focused proof gates for acceptance: HTML contains the complete governed claim set and stable finding/evidence/action/test identities; manifest binds exact inputs, renderer, approval and output; stale, unapproved, tampered or unsafe inputs fail closed; hostile markup cannot create active script, attributes or remote fetches; actual output opens and navigates accessibly in a browser; no public hosting, PDF renderer or unrelated sibling scope is absorbed.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- <slug>

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- <risks_inline>

## Test Strategy

- Run `cargo test --manifest-path adl/Cargo.toml --test codefriend_render_html`, strict relevant Clippy, formatting, diff hygiene, installed CLI positive/denial proof, browser navigation and no-remote-fetch inspection, then independent exact-head review and required CI.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Preserve #896 create-only anchored output and retained-byte validation invariants. Serialize shared CLI wiring with #898. Do not duplicate a second publication authority model or weaken approval, redaction, path confinement, output freshness or provenance checks.
