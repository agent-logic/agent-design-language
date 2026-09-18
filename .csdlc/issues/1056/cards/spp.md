---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "v0922-server-review-model-access-execution-plan"
issue: 1056
task_id: "issue-1056"
run_id: "issue-1056"
version: "v0.92.2"
title: "[v0.92.2][CF-SERVER] Execute hosted reviews and provide governed model access"
branch: "codex/1056-v0922-server-review-model-access"
generated_at: "2026-09-16T20:16:52.878587+00:00"
card_status: "ready"
status: "pre_execution"
activation_state: "implementation_authorized"
plan_revision: 1
initial_pvf_lane: "runtime"
planned_pvf_lane: "runtime"
planned_pvf_lane_source: "Hosted review and model gateway code; docs/validation/pvf_lanes.json"
estimate_elapsed_seconds: "14400"
estimate_total_tokens: "10000"
estimate_validation_seconds: "3600"
issue_goal_token_budget: "not_set"
variance_threshold_percent: "25"
estimate_confidence: "medium"
estimate_data_source: "implementation_estimate_excludes_external_approval_waits"
estimate_source_ref: "https://github.com/agent-logic/agent-design-language/issues/1056"
issue_goal_ref: "Create issue #1056 goal after native bind, before implementation; Sprint #936 prerequisite"
sprint_goal_ref: "https://github.com/agent-logic/agent-design-language/issues/936"
goal_metrics_rollup_ref: "Future #1056 SOR and child evidence register"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1056"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1056"
  - kind: "stp"
    ref: ".csdlc/issues/1056/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/1056/cards/sip.md"
scope:
  files:
    - "adl/src/codefriend/server.rs; adl/src/bin/codefriend_server.rs; adl/src/codefriend/mod.rs; bounded runner prompt helper; adl/tests/codefriend_server.rs; docs/codefriend/SERVER.md; issue PVF manifest; .github/workflows/ci.yaml workspace coverage evidence paths; focused coverage workflow contract test"
  components:
    - "v0922-server-review-model-access"
  out_of_scope:
    - "No Google sign-in, BYOK, open signup, unrelated repository migration, automatic merge/release, or claim of whole-product qualification."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Implement an Axum HTTP server behind a TLS ingress. An operator-owned credential registry maps opaque token hashes to invited user and optional local-agent scopes with expiry; callers cannot choose identities or provider routes. Admit bounded packets through existing evidence validation into isolated per-user stores. Atomically reserve unique operation IDs before asynchronous hosted review or restricted review-lane model calls, cap concurrency and persistent per-principal quotas, retain terminal state across restart and reject ambiguous replay. Hosted execution calls existing review runner; local gateway generates fixed review prompts from admitted evidence and invokes the existing provider adapter without exposing credentials. Expose authenticated status, cancellation and allowlisted result artifacts. Fail incomplete on restart and never automatically replay provider effects. Add focused HTTP positive/negative tests and executable binary/config docs; real provider deployment proof remains separately authorized before issue acceptance. Validation-diagnostics repair for PR1063 run35377358543: retain existing workspace coverage partition logs in the always-run evidence upload so cancellation preserves test progress. Preserve30-minute job timeout, tests, thresholds and aggregation; do not infer a hung test or raise budget without evidence."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: Existing review runner, evidence admission/store and provider adapter are present at 034ccc2b9068c21833483931c4b3d71b257d85d3. #1057 consumes the server identity contract; #1058 consumes model gateway. User explicitly confirmed ADL implementation now; repository extraction deferred."
    expected_output: ".csdlc/issues/1056/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Live source issue; ADR0084; Sprint936 preparation packet; shared Runtime/provider/evidence APIs."
    expected_output: ".csdlc/issues/1056/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: invited users' authorized website requests execute the existing CodeFriend review pipeline on Agent Logic servers, with isolated run/evidence stores and observable success/failure/cancel states. A credential-protecting model-access service also serves authorized installed local agents using Agent Logic-provided model access. Raw provider credentials never reach the website or local agent."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: real bounded server review with source/output/candidate identity, per-user run/artifact authorization, restricted model requests, explicit resource limits, provider failure/cancel/retry behavior and no successful manifest for failed stages. Distinguish server review execution from local review execution using the shared model-access service. Use existing shared Runtime/provider/evidence owners. No BYOK, customer billing or autonomous source changes. Hosting topology and exact operating limits are implementation decisions that must be recorded before deployment or paid effects; preparation grants no such effects."
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
  - "v0922-server-review-model-access"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Coordinate accepted shared Runtime/provider/evidence and review consumer interfaces. Pin repository ownership and design before implementation; no live or paid operations during preparation."
test_strategy:
  - "cargo test --manifest-path adl/Cargo.toml --test codefriend_server; focused formatting and clippy. HTTP tests cover unauthenticated, expired, cross-user, wrong-scope, excessive input/quota, duplicate request, cancellation and restart. Production runner/provider wiring uses actual existing owners; fake backend fixtures are explicitly component-only. Real bounded deployed/provider proof is required before completion and is not authorized by preparation. Diagnostics-only repair: focused test_coverage_authority_contract.sh and workflow guardrail checks plus independent exact-head review; fresh CI establishes whether retained partition diagnostics identify timeout cause."
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
notes: "No deployment or paid effects yet. Provisioning of website and paired-agent tokens must be completed by #1057/#1058; raw provider credentials remain in the service environment. Local execution sends explicitly selected evidence to Agent Logic for model access. Cancel cannot undo an in-flight provider request; keep capacity occupied until it terminates and deny ambiguous retry."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][CF-SERVER] Execute hosted reviews and provide governed model access`.

Implement an Axum HTTP server behind a TLS ingress. An operator-owned credential registry maps opaque token hashes to invited user and optional local-agent scopes with expiry; callers cannot choose identities or provider routes. Admit bounded packets through existing evidence validation into isolated per-user stores. Atomically reserve unique operation IDs before asynchronous hosted review or restricted review-lane model calls, cap concurrency and persistent per-principal quotas, retain terminal state across restart and reject ambiguous replay. Hosted execution calls existing review runner; local gateway generates fixed review prompts from admitted evidence and invokes the existing provider adapter without exposing credentials. Expose authenticated status, cancellation and allowlisted result artifacts. Fail incomplete on restart and never automatically replay provider effects. Add focused HTTP positive/negative tests and executable binary/config docs; real provider deployment proof remains separately authorized before issue acceptance. Validation-diagnostics repair for PR1063 run35377358543: retain existing workspace coverage partition logs in the always-run evidence upload so cancellation preserves test progress. Preserve30-minute job timeout, tests, thresholds and aggregation; do not infer a hung test or raise budget without evidence.

## PVF Lane Plan

- Initial PVF lane from issue creation: `runtime`
- Planned PVF lane for execution: `runtime`
- Planning lane source: `Hosted review and model gateway code; docs/validation/pvf_lanes.json`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `14400`
- Estimated total tokens: `10000`
- Estimated validation seconds: `3600`
- Issue goal token budget: `not_set`
- Variance threshold percent: `25`
- Estimate confidence: `medium`
- Estimate data source: `implementation_estimate_excludes_external_approval_waits`
- Estimate source ref: `https://github.com/agent-logic/agent-design-language/issues/1056`
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

1. Confirm dependency readiness and starting state: Existing review runner, evidence admission/store and provider adapter are present at 034ccc2b9068c21833483931c4b3d71b257d85d3. #1057 consumes the server identity contract; #1058 consumes model gateway. User explicitly confirmed ADL implementation now; repository extraction deferred.
2. Review repo inputs and scoped surfaces before editing: Live source issue; ADR0084; Sprint936 preparation packet; shared Runtime/provider/evidence APIs.
3. Implement only the bounded deliverables: invited users' authorized website requests execute the existing CodeFriend review pipeline on Agent Logic servers, with isolated run/evidence stores and observable success/failure/cancel states. A credential-protecting model-access service also serves authorized installed local agents using Agent Logic-provided model access. Raw provider credentials never reach the website or local agent.
4. Run focused proof gates for acceptance: real bounded server review with source/output/candidate identity, per-user run/artifact authorization, restricted model requests, explicit resource limits, provider failure/cancel/retry behavior and no successful manifest for failed stages. Distinguish server review execution from local review execution using the shared model-access service. Use existing shared Runtime/provider/evidence owners. No BYOK, customer billing or autonomous source changes. Hosting topology and exact operating limits are implementation decisions that must be recorded before deployment or paid effects; preparation grants no such effects.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- v0922-server-review-model-access

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Coordinate accepted shared Runtime/provider/evidence and review consumer interfaces. Pin repository ownership and design before implementation; no live or paid operations during preparation.

## Test Strategy

- cargo test --manifest-path adl/Cargo.toml --test codefriend_server; focused formatting and clippy. HTTP tests cover unauthenticated, expired, cross-user, wrong-scope, excessive input/quota, duplicate request, cancellation and restart. Production runner/provider wiring uses actual existing owners; fake backend fixtures are explicitly component-only. Real bounded deployed/provider proof is required before completion and is not authorized by preparation. Diagnostics-only repair: focused test_coverage_authority_contract.sh and workflow guardrail checks plus independent exact-head review; fresh CI establishes whether retained partition diagnostics identify timeout cause.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

No deployment or paid effects yet. Provisioning of website and paired-agent tokens must be completed by #1057/#1058; raw provider credentials remain in the service environment. Local execution sends explicitly selected evidence to Agent Logic for model access. Cancel cannot undo an in-flight provider request; keep capacity occupied until it terminates and deny ambiguous retry.
