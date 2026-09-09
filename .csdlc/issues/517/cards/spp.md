---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "tail-01-quality-gate-execution-plan"
issue: 517
task_id: "issue-0517"
run_id: "issue-0517"
version: "v0.92.1"
title: "[v0.92.1][TAIL-01] Quality gate"
branch: "codex/517-tail-01-quality-gate"
generated_at: "2026-09-09T00:56:48.157331+00:00"
card_status: "ready"
status: "in_progress"
activation_state: "bound"
plan_revision: 1
initial_pvf_lane: "local_cpu"
planned_pvf_lane: "local_cpu"
planned_pvf_lane_source: "Operator clarified docs-only accounting scope."
estimate_elapsed_seconds: "21600"
estimate_total_tokens: "80000"
estimate_validation_seconds: "3600"
issue_goal_token_budget: "not explicitly budgeted"
variance_threshold_percent: "50"
estimate_confidence: "low"
estimate_data_source: "Original issue planning estimate"
estimate_source_ref: ".csdlc/prepared/issues/517/bootstrap-request.json"
issue_goal_ref: "issue-517-post-merge-evidence-reconciliation"
sprint_goal_ref: "not recorded"
goal_metrics_rollup_ref: "not recorded"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/517"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/517"
  - kind: "stp"
    ref: ".csdlc/issues/517/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/517/cards/sip.md"
scope:
  files:
    - "Quality-gate documentation, reconciliation evidence and validators, canonical milestone specifications, and issue517 typed cards. Native implementation belongs to separate issues."
  components:
    - "tail-01-quality-gate"
  out_of_scope:
    - "No code implementation, cloud execution, release ceremony, merge, fabricated proof, silent acceptance changes, or writes on main."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Docs-only accounting: reconcile every original row against evidence and explicit amendments, synchronize canonical specifications, and assign actual implementation or proof gaps to separate issues. Preserve historical assessments and release conditions."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: INT-01/#516 reviewed merge before execution"
    expected_output: ".csdlc/issues/517/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: agent-logic/agent-design-language#517; agent-logic/agent-design-language#516; docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#TAIL-01; docs/milestones/v0.92.1/SPRINT_v0.92.1.md"
    expected_output: ".csdlc/issues/517/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Exact required-lane denominator; Machine-readable lane results; QUALITY_GATE_v0.92.1.md decision record; Issue-owned retained evidence"
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: AC-1: Every required proving lane passes; AC-2: Skipped, absent, zero-test, stale, and non-proving results fail closed; AC-3: The exact candidate revision and complete denominator are recorded; AC-4: Every exception has an explicit owner and no unresolved exception remains; AC-1 and no-unresolved-exception release conditions remain unmet; the decision records this truth."
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
  - "tail-01-quality-gate"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "An incomplete denominator could create a false green; Zero-test output could be mistaken for proof; Candidate drift could stale the decision; An exception could lack an owner"
test_strategy:
  - "Exact row, digest and evidence reconciliation; complete exception ownership; focused canonical YAML parity; independent docs review and required CI."
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
notes: "Accounting completeness does not assert missing runtime or operational proof. Preserve explicit gap ownership and historical release requirements; do not implement code here."
---

Canonical Template Source: `docs/templates/prompts/1.0.4/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.1][TAIL-01] Quality gate`.

Docs-only accounting: reconcile every original row against evidence and explicit amendments, synchronize canonical specifications, and assign actual implementation or proof gaps to separate issues. Preserve historical assessments and release conditions.

## PVF Lane Plan

- Initial PVF lane from issue creation: `local_cpu`
- Planned PVF lane for execution: `local_cpu`
- Planning lane source: `Operator clarified docs-only accounting scope.`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `21600`
- Estimated total tokens: `80000`
- Estimated validation seconds: `3600`
- Issue goal token budget: `not explicitly budgeted`
- Variance threshold percent: `50`
- Estimate confidence: `low`
- Estimate data source: `Original issue planning estimate`
- Estimate source ref: `.csdlc/prepared/issues/517/bootstrap-request.json`
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

1. Confirm dependency readiness and starting state: INT-01/#516 reviewed merge before execution
2. Review repo inputs and scoped surfaces before editing: agent-logic/agent-design-language#517; agent-logic/agent-design-language#516; docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#TAIL-01; docs/milestones/v0.92.1/SPRINT_v0.92.1.md
3. Implement only the bounded deliverables: Exact required-lane denominator; Machine-readable lane results; QUALITY_GATE_v0.92.1.md decision record; Issue-owned retained evidence
4. Run focused proof gates for acceptance: AC-1: Every required proving lane passes; AC-2: Skipped, absent, zero-test, stale, and non-proving results fail closed; AC-3: The exact candidate revision and complete denominator are recorded; AC-4: Every exception has an explicit owner and no unresolved exception remains; AC-1 and no-unresolved-exception release conditions remain unmet; the decision records this truth.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- tail-01-quality-gate

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- An incomplete denominator could create a false green; Zero-test output could be mistaken for proof; Candidate drift could stale the decision; An exception could lack an owner

## Test Strategy

- Exact row, digest and evidence reconciliation; complete exception ownership; focused canonical YAML parity; independent docs review and required CI.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Accounting completeness does not assert missing runtime or operational proof. Preserve explicit gap ownership and historical release requirements; do not implement code here.
