---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "909-gcp-move-in-execution-plan"
issue: 909
task_id: "issue-0909"
run_id: "issue-0909"
version: "v0.92.2"
title: "[v0.92.2][OPS-GCP] Produce one apply-ready company GCP move-in execution packet"
branch: "codex/909-gcp-move-in"
generated_at: "<timestamp>"
card_status: "ready"
status: "prepared"
activation_state: "<activation_state>"
plan_revision: 1
initial_pvf_lane: "cloud-operations"
planned_pvf_lane: "cloud-operations"
planned_pvf_lane_source: "#909 acceptance"
estimate_elapsed_seconds: "<estimate_elapsed_seconds>"
estimate_total_tokens: "<estimate_total_tokens>"
estimate_validation_seconds: "<estimate_validation_seconds>"
issue_goal_token_budget: "<issue_goal_token_budget>"
variance_threshold_percent: "<variance_threshold_percent>"
estimate_confidence: "<estimate_confidence>"
estimate_data_source: "<estimate_data_source>"
estimate_source_ref: "<estimate_source_ref>"
issue_goal_ref: "Create Sprint 8 issue #909 session goal after binding and before implementation"
sprint_goal_ref: "Sprint 8 umbrella #934"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/909"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/909"
  - kind: "stp"
    ref: ".csdlc/issues/909/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/909/cards/sip.md"
scope:
  files:
    - ".csdlc/evidence/909 and docs/operations/cloud/gcp"
  components:
    - "909-gcp-move-in"
  out_of_scope:
    - "Runtime deployment, GPU launch, six-resident requalification, cloud mutation."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Verify company GCP identity and source/destination inventory, reconcile organization/bootstrap/platform Terraform to observed state, and deliver a reviewed apply-ready move-in packet. Missing readback or real plan blocks completion."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: #864 accepted planning output and all 69 creation/review gates; umbrella #934 coordinates execution."
    expected_output: ".csdlc/issues/909/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: infra/gcp/organization, infra/gcp/bootstrap, infra/gcp/platform and associated docs/operations/cloud/gcp runbooks"
    expected_output: ".csdlc/issues/909/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Exact inventory; actual bounded read-only Terraform plan; ordered application and rollback; billing, cleanup ownership and residual routing."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: Live company identity and inventory; fmt/validate/plan; consistency, link and redaction proof; independent review."
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
  - "909-gcp-move-in"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Missing company credentials or state, drift, wrong identity, resource ownership collision, irreversible data movement."
test_strategy:
  - "Read-only company identity/resource census; exactly3approved isolated local imports only; Terraform validate and real saved plan; source/link/redaction checks and independent review. No cloud or remote-state changes."
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
notes: "All local implementation/proof and independent acceptance review passed. Future custody/adoption/application remain separately approved prerequisites. Publication/CI/merge not complete."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][OPS-GCP] Produce one apply-ready company GCP move-in execution packet`.

Verify company GCP identity and source/destination inventory, reconcile organization/bootstrap/platform Terraform to observed state, and deliver a reviewed apply-ready move-in packet. Missing readback or real plan blocks completion.

## PVF Lane Plan

- Initial PVF lane from issue creation: `cloud-operations`
- Planned PVF lane for execution: `cloud-operations`
- Planning lane source: `#909 acceptance`
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

1. [completed] Confirm dependencies and starting state from the source issue prompt.
2. [completed] Inspect repo inputs and target surfaces before editing.
3. [completed] Implement the bounded deliverables only.
4. [completed] Run focused validation and proof gates.
5. [completed] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: #864 accepted planning output and all 69 creation/review gates; umbrella #934 coordinates execution.
2. Review repo inputs and scoped surfaces before editing: infra/gcp/organization, infra/gcp/bootstrap, infra/gcp/platform and associated docs/operations/cloud/gcp runbooks
3. Implement only the bounded deliverables: Exact inventory; actual bounded read-only Terraform plan; ordered application and rollback; billing, cleanup ownership and residual routing.
4. Run focused proof gates for acceptance: Live company identity and inventory; fmt/validate/plan; consistency, link and redaction proof; independent review.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- 909-gcp-move-in

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Missing company credentials or state, drift, wrong identity, resource ownership collision, irreversible data movement.

## Test Strategy

- Read-only company identity/resource census; exactly3approved isolated local imports only; Terraform validate and real saved plan; source/link/redaction checks and independent review. No cloud or remote-state changes.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

All local implementation/proof and independent acceptance review passed. Future custody/adoption/application remain separately approved prerequisites. Publication/CI/merge not complete.
