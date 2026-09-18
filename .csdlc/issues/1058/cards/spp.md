---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "v0922-installed-local-review-agent-execution-plan"
issue: 1058
task_id: "issue-1058"
run_id: "issue-1058"
version: "v0.92.2"
title: "[v0.92.2][CF-AGENT] Run website-controlled reviews through an installed local agent"
branch: "codex/1058-v0922-installed-local-review-agent"
generated_at: "2026-09-16T20:16:52.878587+00:00"
card_status: "ready"
status: "pre_execution"
activation_state: "implementation_authorized"
plan_revision: 1
initial_pvf_lane: "runtime"
planned_pvf_lane: "runtime"
planned_pvf_lane_source: "adl/src/codefriend/agent.rs; adl/src/bin/codefriend_agent.rs; adl/src/codefriend/review/runner.rs; adl/src/codefriend/mod.rs; adl/Cargo.toml; adl/tests/codefriend_agent.rs; adl/tests/fixtures/codefriend/agent/PVF.json; docs/codefriend/LOCAL_AGENT.md"
estimate_elapsed_seconds: "14400"
estimate_total_tokens: "10000"
estimate_validation_seconds: "3600"
issue_goal_token_budget: "not_set"
variance_threshold_percent: "25"
estimate_confidence: "medium"
estimate_data_source: "Agent implementation estimate excluding external authorization waits"
estimate_source_ref: "https://github.com/agent-logic/agent-design-language/issues/1058"
issue_goal_ref: "Active Sprint 10 #936 goal explicitly requested by operator"
sprint_goal_ref: "https://github.com/agent-logic/agent-design-language/issues/936"
goal_metrics_rollup_ref: "Future #1058 SOR and child evidence register"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1058"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1058"
  - kind: "stp"
    ref: ".csdlc/issues/1058/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/1058/cards/sip.md"
scope:
  files:
    - "adl/src/codefriend/agent.rs; adl/src/bin/codefriend_agent.rs; adl/src/codefriend/review/runner.rs; adl/src/codefriend/mod.rs; adl/Cargo.toml; adl/tests/codefriend_agent.rs; adl/tests/fixtures/codefriend/agent/PVF.json; docs/codefriend/LOCAL_AGENT.md; .github/workflows/ci.yaml hosted workspace coverage timeout; .csdlc/evidence/1058/WORKSPACE_TIME_BUDGET.md"
  components:
    - "v0922-installed-local-review-agent"
  out_of_scope:
    - "No Google sign-in, BYOK, open signup, unrelated repository migration, automatic merge/release, or claim of whole-product qualification."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Implement an outbound-only installed agent with no local HTTP listener. Pair via a short-lived website-issued one-time code exchanged over validated HTTPS; persist scoped user/agent and model-gateway credentials in owner-only files. A locally authored consent file pins one repository path, immutable revision and exact bounded scope; remote commands can select only its digest and never supply a filesystem path or shell command. Poll versioned website commands using paired credentials; reserve each run durably before acquisition or model dispatch. Reuse local acquisition/admission and four-perspective review orchestration through a bounded gateway executor seam. Each model lane uses #1056 local_model operations; never retry POST after uncertain effect. Reconnect may resume observation of known operation IDs but cannot repeat review dispatch. Poll website cancellation while waiting; do not mark cancelled until execution stops. Persist terminal result/digest for idempotent forwarding; expose only validated review artifacts, not source paths or credentials. Add CLI pair/run/unpair entrypoints and install instructions. #1057 implements website pairing/control endpoints against this protocol; #914 proves the full journey. Review remediation at670b7b74bb: durably distinguish unknown POST effects from acknowledged operations; reconnect observes known IDs with GET/control only, persists completed lane output and resumes orchestration without replay. Retain and cross-check gateway build/provider/model identity across all lanes and RunReport. Reject oversized aggregate before complete status/persistence, preserving a bounded failure report. Confirmed unpair scrubs source/report payloads while retaining run tombstones. CI repair: retain current test/coverage gates and per-test bounds, and increase only the hosted workspace job envelope from30 to35minutes based on run35385414984 completing 2769 tests after19m45 cold compilation and467.932s longest partition, then exhausting the job limit during reporting. Review the bounded workflow change and run focused CI contracts before refreshing native proof/review/publication."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: Shared #1056 protocol accepted for dependent implementation at 0686e60a0bf29b595f266c476b2ae0996cf6fdd7, independently reviewed with seven component tests. Branch stacks on that candidate; no claim of deployed/provider acceptance. #1057 implements the website side of the agent protocol; #914 integrates completed components."
    expected_output: ".csdlc/issues/1058/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Live source issue; ADR0084; Sprint936 preparation packet; shared Runtime/provider/evidence APIs."
    expected_output: ".csdlc/issues/1058/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: a user installs CodeFriend on the selected macOS/Linux environment, pairs it with their invited website session, and starts/observes/cancels local reviews from the website. Review execution occurs on the user's computer; model requests use Agent Logic's authorized model-access service. Installation and pairing must be reproducible from a clean environment."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: authenticated scoped pairing/revocation, correct execution location, bounded source consent and privacy filtering, non-secret user/agent credentials, no embedded provider keys, reconnect and disconnect/cancellation/retry outcomes without duplicate successful dispatch, truthful status/artifact forwarding, and cross-user/expired-agent denial. Preserve evidence identity, retention and exact-artifact approval. BYOK and Google sign-in are excluded from Beta1. Do not claim local execution means no selected evidence is sent to the model service; disclose the actual scoped model input."
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
  - "v0922-installed-local-review-agent"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Requires #1056 accepted shared identity/run-control/model-access contracts before dependent execution. Coordinate disjoint implementation after contract acceptance; full component outcome is required before #914."
test_strategy:
  - "cargo test --manifest-path adl/Cargo.toml --test codefriend_agent; existing codefriend_review_runner regression tests; focused rustfmt and clippy. Deterministic pairing, expiry/revocation, cross-user, consent, path restriction, durable duplicate, crash/reconnect, cancellation, result forwarding and no-secret-output cases. Separate installed macOS/Linux and real-provider/browser journeys are required for acceptance and await bounded operational authorization. Add disconnect/restart observation with one POST per operation, inconsistent identity rejection, multi-lane aggregate boundary, and confirmed-unpair immediate payload cleanup with duplicate denial. Existing source tests are not acceptance of these unresolved findings. For measured hosted workspace envelope repair, run adl/tools/test_ci_runtime_contracts.sh, git diff --check, exact-head independent review and fresh hosted CI. No reduction of test selection, coverage thresholds or per-test deadlines."
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
notes: "Sprint #936 goal remains active by explicit operator instruction; no replacement issue goal. Implement locally now. No paid calls, deployment or merge authority inferred. Website/agent credentials are private secrets even though they are not provider keys; never print them."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][CF-AGENT] Run website-controlled reviews through an installed local agent`.

Implement an outbound-only installed agent with no local HTTP listener. Pair via a short-lived website-issued one-time code exchanged over validated HTTPS; persist scoped user/agent and model-gateway credentials in owner-only files. A locally authored consent file pins one repository path, immutable revision and exact bounded scope; remote commands can select only its digest and never supply a filesystem path or shell command. Poll versioned website commands using paired credentials; reserve each run durably before acquisition or model dispatch. Reuse local acquisition/admission and four-perspective review orchestration through a bounded gateway executor seam. Each model lane uses #1056 local_model operations; never retry POST after uncertain effect. Reconnect may resume observation of known operation IDs but cannot repeat review dispatch. Poll website cancellation while waiting; do not mark cancelled until execution stops. Persist terminal result/digest for idempotent forwarding; expose only validated review artifacts, not source paths or credentials. Add CLI pair/run/unpair entrypoints and install instructions. #1057 implements website pairing/control endpoints against this protocol; #914 proves the full journey. Review remediation at670b7b74bb: durably distinguish unknown POST effects from acknowledged operations; reconnect observes known IDs with GET/control only, persists completed lane output and resumes orchestration without replay. Retain and cross-check gateway build/provider/model identity across all lanes and RunReport. Reject oversized aggregate before complete status/persistence, preserving a bounded failure report. Confirmed unpair scrubs source/report payloads while retaining run tombstones. CI repair: retain current test/coverage gates and per-test bounds, and increase only the hosted workspace job envelope from30 to35minutes based on run35385414984 completing 2769 tests after19m45 cold compilation and467.932s longest partition, then exhausting the job limit during reporting. Review the bounded workflow change and run focused CI contracts before refreshing native proof/review/publication.

## PVF Lane Plan

- Initial PVF lane from issue creation: `runtime`
- Planned PVF lane for execution: `runtime`
- Planning lane source: `adl/src/codefriend/agent.rs; adl/src/bin/codefriend_agent.rs; adl/src/codefriend/review/runner.rs; adl/src/codefriend/mod.rs; adl/Cargo.toml; adl/tests/codefriend_agent.rs; adl/tests/fixtures/codefriend/agent/PVF.json; docs/codefriend/LOCAL_AGENT.md`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `14400`
- Estimated total tokens: `10000`
- Estimated validation seconds: `3600`
- Issue goal token budget: `not_set`
- Variance threshold percent: `25`
- Estimate confidence: `medium`
- Estimate data source: `Agent implementation estimate excluding external authorization waits`
- Estimate source ref: `https://github.com/agent-logic/agent-design-language/issues/1058`
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

1. Confirm dependency readiness and starting state: Shared #1056 protocol accepted for dependent implementation at 0686e60a0bf29b595f266c476b2ae0996cf6fdd7, independently reviewed with seven component tests. Branch stacks on that candidate; no claim of deployed/provider acceptance. #1057 implements the website side of the agent protocol; #914 integrates completed components.
2. Review repo inputs and scoped surfaces before editing: Live source issue; ADR0084; Sprint936 preparation packet; shared Runtime/provider/evidence APIs.
3. Implement only the bounded deliverables: a user installs CodeFriend on the selected macOS/Linux environment, pairs it with their invited website session, and starts/observes/cancels local reviews from the website. Review execution occurs on the user's computer; model requests use Agent Logic's authorized model-access service. Installation and pairing must be reproducible from a clean environment.
4. Run focused proof gates for acceptance: authenticated scoped pairing/revocation, correct execution location, bounded source consent and privacy filtering, non-secret user/agent credentials, no embedded provider keys, reconnect and disconnect/cancellation/retry outcomes without duplicate successful dispatch, truthful status/artifact forwarding, and cross-user/expired-agent denial. Preserve evidence identity, retention and exact-artifact approval. BYOK and Google sign-in are excluded from Beta1. Do not claim local execution means no selected evidence is sent to the model service; disclose the actual scoped model input.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- v0922-installed-local-review-agent

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Requires #1056 accepted shared identity/run-control/model-access contracts before dependent execution. Coordinate disjoint implementation after contract acceptance; full component outcome is required before #914.

## Test Strategy

- cargo test --manifest-path adl/Cargo.toml --test codefriend_agent; existing codefriend_review_runner regression tests; focused rustfmt and clippy. Deterministic pairing, expiry/revocation, cross-user, consent, path restriction, durable duplicate, crash/reconnect, cancellation, result forwarding and no-secret-output cases. Separate installed macOS/Linux and real-provider/browser journeys are required for acceptance and await bounded operational authorization. Add disconnect/restart observation with one POST per operation, inconsistent identity rejection, multi-lane aggregate boundary, and confirmed-unpair immediate payload cleanup with duplicate denial. Existing source tests are not acceptance of these unresolved findings. For measured hosted workspace envelope repair, run adl/tools/test_ci_runtime_contracts.sh, git diff --check, exact-head independent review and fresh hosted CI. No reduction of test selection, coverage thresholds or per-test deadlines.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Sprint #936 goal remains active by explicit operator instruction; no replacement issue goal. Implement locally now. No paid calls, deployment or merge authority inferred. Website/agent credentials are private secrets even though they are not provider keys; never print them.
