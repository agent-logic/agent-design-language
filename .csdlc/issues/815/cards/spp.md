---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "cloud-authorization-authenticity-execution-plan"
issue: 815
task_id: "issue-0815"
run_id: "issue-0815"
version: "1.0.5"
title: "[v0.92.1][TAIL-06.10][security] Authenticate AWS and GCP mutation authorization"
branch: "codex/815-cloud-authorization-authenticity"
generated_at: "2026-09-09T20:50:00Z"
card_status: "approved"
status: "ready"
activation_state: "approved_for_execution"
plan_revision: 1
initial_pvf_lane: "security-local"
planned_pvf_lane: "security-local-negative-matrix"
planned_pvf_lane_source: "issue-815-acceptance"
estimate_elapsed_seconds: "3600"
estimate_total_tokens: "12000"
estimate_validation_seconds: "300"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "25"
estimate_confidence: "medium"
estimate_data_source: "current #727/#731 script size and deterministic local fixture scope"
estimate_source_ref: "issue #815"
issue_goal_ref: "issue-815-session-goal"
sprint_goal_ref: "issue-520-remediation"
goal_metrics_rollup_ref: "issue-522-findings"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/815"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/815"
  - kind: "stp"
    ref: ".csdlc/issues/815/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/815/cards/sip.md"
scope:
  files:
    - "#727/#731 authorization validators and mutation entrypoints, #815 shared verifier/tests, coupled templates/runbooks"
  components:
    - "cloud-authorization-authenticity"
  out_of_scope:
    - "paid cloud mutation, broad credential redesign, historical evidence rewrites, provider-resource changes"
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Introduce one externally anchored detached-signature contract, require it before GCP mutation, hash AWS tfplan bytes directly, derive/compare Terraform JSON from the same plan, bind observed provider identity, and prove every tamper/replay case locally."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: current v0.92.1 main, parent #522, local Python and OpenSSH tooling"
    expected_output: ".csdlc/issues/815/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: issue #815, findings D520-SEC-001/002, #727/#731 scripts and templates"
    expected_output: ".csdlc/issues/815/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: authentic signature validation, exact plan-byte identity, provider identity binding, docs, negative fixtures"
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: forgery, mutation, expiry, and replay reject before provider calls; read-only paths remain usable; no paid mutation"
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
  - "cloud-authorization-authenticity"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "test-mode leakage, signature canonicalization drift, Terraform projection nondeterminism, accidental provider invocation"
test_strategy:
  - "ephemeral signer fixtures, mocked Terraform projection, exact negative assertions, syntax, diff hygiene, independent review"
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
notes: "Production trusted signer path is external and fixed; test overrides are accepted only by the test harness entrypoint and never by live mutation wrappers."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.1][TAIL-06.10][security] Authenticate AWS and GCP mutation authorization`.

Introduce one externally anchored detached-signature contract, require it before GCP mutation, hash AWS tfplan bytes directly, derive/compare Terraform JSON from the same plan, bind observed provider identity, and prove every tamper/replay case locally.

## PVF Lane Plan

- Initial PVF lane from issue creation: `security-local`
- Planned PVF lane for execution: `security-local-negative-matrix`
- Planning lane source: `issue-815-acceptance`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `3600`
- Estimated total tokens: `12000`
- Estimated validation seconds: `300`
- Issue goal token budget: `unknown`
- Variance threshold percent: `25`
- Estimate confidence: `medium`
- Estimate data source: `current #727/#731 script size and deterministic local fixture scope`
- Estimate source ref: `issue #815`
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

1. Confirm dependency readiness and starting state: current v0.92.1 main, parent #522, local Python and OpenSSH tooling
2. Review repo inputs and scoped surfaces before editing: issue #815, findings D520-SEC-001/002, #727/#731 scripts and templates
3. Implement only the bounded deliverables: authentic signature validation, exact plan-byte identity, provider identity binding, docs, negative fixtures
4. Run focused proof gates for acceptance: forgery, mutation, expiry, and replay reject before provider calls; read-only paths remain usable; no paid mutation
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- cloud-authorization-authenticity

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- test-mode leakage, signature canonicalization drift, Terraform projection nondeterminism, accidental provider invocation

## Test Strategy

- ephemeral signer fixtures, mocked Terraform projection, exact negative assertions, syntax, diff hygiene, independent review

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Production trusted signer path is external and fixed; test overrides are accepted only by the test harness entrypoint and never by live mutation wrappers.
