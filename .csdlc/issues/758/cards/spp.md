---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "admission-a2a-outbox-execution-plan"
issue: 758
task_id: "issue-0758"
run_id: "issue-0758"
version: "1.0.4"
title: "[v0.92.1][TAIL-06.02][runtime] Persist and recover admission-triggered A2A initiation"
branch: "codex/758-admission-a2a-outbox"
generated_at: "2026-09-09T00:00:00Z"
card_status: "approved"
status: "ready"
activation_state: "approved_for_execution"
plan_revision: 1
initial_pvf_lane: "runtime-focused"
planned_pvf_lane: "runtime-focused-plus-demo"
planned_pvf_lane_source: "issue-758-acceptance"
estimate_elapsed_seconds: "unknown"
estimate_total_tokens: "unknown"
estimate_validation_seconds: "1800"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "10"
estimate_confidence: "low"
estimate_data_source: "issue complexity review"
estimate_source_ref: "issue #758"
issue_goal_ref: "issue-758-session-goal"
sprint_goal_ref: "issue-522-remediation-wave"
goal_metrics_rollup_ref: "issue-522-remediation-wave"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/758"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/758"
  - kind: "stp"
    ref: ".csdlc/issues/758/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/758/cards/sip.md"
scope:
  files:
    - "adl-runtime-kernel/src/control.rs and narrowly coupled persistence, health, test, demo, and PVF surfaces"
  components:
    - "admission-a2a-outbox"
  out_of_scope:
    - "exactly-once provider delivery; a general workflow engine; unrelated runtime refactors"
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Inspect the admission/A2A durability boundary, add the smallest durable idempotent intent and worker recovery path, expose disposition, prove failure/restart/dedup behavior, then run the autonomous demo."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: #759 health-task isolation should land first; #757 crash-consistent dynamic-agent removal is the production-code prerequisite because #758 extends the same durable aggregate and transaction; existing A2A transport and conversation ledger remain authoritative"
    expected_output: ".csdlc/issues/758/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: issue #758, review #520 findings, current admission and A2A control paths, durable runtime state and health contracts"
    expected_output: ".csdlc/issues/758/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: durable intent, bounded recovery worker, stable ledger key, disposition projection, tests, demo proof"
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: persistence-before-acknowledgement; restart/replay recovery; bounded retries; deduplication; observable states; autonomous real-agent proof"
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
  - "admission-a2a-outbox"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "crash-window ordering, conflict with #757 aggregate migration, uncertain provider completion, retry storms, duplicate ledger turns, incompatible persisted-state evolution"
test_strategy:
  - "targeted recovery/dedup tests, focused Runtime owner lane, exact-head review, and real-agent autonomous greeting demo"
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
notes: "Do not implement a competing store shape before #757 lands; rebase on its atomic removal transaction, then add greeting obligations without weakening removal recovery."
---

Canonical Template Source: `docs/templates/prompts/1.0.4/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.1][TAIL-06.02][runtime] Persist and recover admission-triggered A2A initiation`.

Inspect the admission/A2A durability boundary, add the smallest durable idempotent intent and worker recovery path, expose disposition, prove failure/restart/dedup behavior, then run the autonomous demo.

## PVF Lane Plan

- Initial PVF lane from issue creation: `runtime-focused`
- Planned PVF lane for execution: `runtime-focused-plus-demo`
- Planning lane source: `issue-758-acceptance`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `unknown`
- Estimated total tokens: `unknown`
- Estimated validation seconds: `1800`
- Issue goal token budget: `unknown`
- Variance threshold percent: `10`
- Estimate confidence: `low`
- Estimate data source: `issue complexity review`
- Estimate source ref: `issue #758`
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

1. Confirm dependency readiness and starting state: #759 health-task isolation should land first; #757 crash-consistent dynamic-agent removal is the production-code prerequisite because #758 extends the same durable aggregate and transaction; existing A2A transport and conversation ledger remain authoritative
2. Review repo inputs and scoped surfaces before editing: issue #758, review #520 findings, current admission and A2A control paths, durable runtime state and health contracts
3. Implement only the bounded deliverables: durable intent, bounded recovery worker, stable ledger key, disposition projection, tests, demo proof
4. Run focused proof gates for acceptance: persistence-before-acknowledgement; restart/replay recovery; bounded retries; deduplication; observable states; autonomous real-agent proof
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- admission-a2a-outbox

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- crash-window ordering, conflict with #757 aggregate migration, uncertain provider completion, retry storms, duplicate ledger turns, incompatible persisted-state evolution

## Test Strategy

- targeted recovery/dedup tests, focused Runtime owner lane, exact-head review, and real-agent autonomous greeting demo

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Do not implement a competing store shape before #757 lands; rebase on its atomic removal transaction, then add greeting obligations without weakening removal recovery.
