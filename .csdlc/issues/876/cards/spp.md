---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "876-provider-definitions-execution-plan"
issue: 876
task_id: "issue-0876"
run_id: "issue-0876"
version: "v0.92.2"
title: "[v0.92.2][PLAT-PROVIDER] Consume validated editable provider definitions"
branch: "codex/876-provider-definitions"
generated_at: "2026-09-11T23:55:23.353658+00:00"
card_status: "ready"
status: "in_progress"
activation_state: "not_activated"
plan_revision: 1
initial_pvf_lane: "provider"
planned_pvf_lane: "provider"
planned_pvf_lane_source: "Issue #876 acceptance and PVF clauses"
estimate_elapsed_seconds: "Not estimated; record actual child execution metrics."
estimate_total_tokens: "Not estimated; record actual child execution metrics."
estimate_validation_seconds: "Not estimated; record actual child execution metrics."
issue_goal_token_budget: "not specified"
variance_threshold_percent: "Not estimated; record actual child execution metrics."
estimate_confidence: "unestimated"
estimate_data_source: "No child execution baseline measured"
estimate_source_ref: "source issue validation requirements"
issue_goal_ref: "Active issue-bound #876 implementation and passing reviewed PR goal"
sprint_goal_ref: "Sprint 2 #928 execution goal"
goal_metrics_rollup_ref: "Not measured: no child execution"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/876"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/876"
  - kind: "stp"
    ref: ".csdlc/issues/876/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/876/cards/sip.md"
scope:
  files:
    - "Extend `adl/src/provider/reload.rs`, `adl/src/provider/profiles.rs`, and the minimum wiring in `adl/src/provider/mod.rs` and `adl/src/execute/runner.rs`. Read `docs/providers/provider-profile-hot-loading.md` and `docs/provider/inference-profiles.md`: existing `ProviderReloadOwner`/`ProviderReloadSnapshot` already validate a provider-only sidecar and use the kernel watcher. Reuse that production owner; do not add another registry/watcher. Own focused reload/profile tests and accompanying provider docs. Add a named data/schema/example file only after inventorying the current format and recording its exact path; do not expand into a provider rewrite."
  components:
    - "876-provider-definitions"
  out_of_scope:
    - "PVF lane: deterministic local provider integration/contract; role: production definition consumption, atomic reload and invalid-input rejection; resources: bounded local CPU/filesystem and controlled loopback endpoint, no paid inference; gate: required provider-platform completion and downstream RT-PROVIDER input. Live/provider-cost execution needs separate explicit authority. Stop on missing hot-load authority, unresolved owner collision, hardcoded instance data, credential capture, failed or missing proving scenario, or incompatible unreviewed format changes. Exclude provider behavior rewrite, agent lifecycle, MLX/PAIR implementation, benchmark marketing, broad Runtime changes and schema-only completion."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Extend the existing provider reload owner; reject credential-shaped nested values without rejecting legitimate model identifiers or approved credential references. Prove real loopback endpoint/model/profile consumption, barrier-synchronized old in-flight and new dispatch, whole-map atomicity, and last-known-good dispatch after invalid edits. Run focused provider tests, preserve diagnostics evidence, obtain exact-head independent review and publish a passing PR."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: Accepted merged output required from #864, #854, #622. No sprint-wide barrier or asynchronous closeout dependency."
    expected_output: ".csdlc/issues/876/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Full canonical issue https://github.com/agent-logic/agent-design-language/issues/876; docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md; WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml and ATOMIC_TASK_CONTRACTS_v0.92.2.json."
    expected_output: ".csdlc/issues/876/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: The Runtime provider-definition loader consumes operator-editable endpoint/profile data through the existing adapter boundary, atomically retains the last-known-good snapshot on invalid replacement, and exposes the change to subsequent real production dispatch. Depends on WP-01/#864, RT-COST/#854 and the verified merged provider hot-loading predecessor #622. RT-PROVIDER/#855 separately owns dynamic agent lifecycle; this issue supplies configuration, not attach/detach implementation. Include production proof, failure handling and operator documentation required by the source issue."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: 1. Load valid editable definitions using the production entrypoint, execute a local controlled provider request and prove the observed endpoint/model/profile matches the selected validated snapshot. Edit definitions without rebuilding; a later dispatch uses the new complete snapshot. In-flight dispatch retains its original snapshot and concurrent readers never observe a mixed map. 2. Preserve existing endpoint/profile behavior, declared compatibility and provider generations. Separate instance data from adapter behavior; document supported fields and explicit unsupported cases. No hardcoded instance branch may stand in for consumption. 3. Through the production loader reject malformed, unsupported, incomplete and credential-shaped definitions, including secret values hidden under neutral nested keys. Keep only approved credential references. Invalid initial configuration fails closed; invalid replacement retains the prior whole snapshot and yields a bounded diagnostic. Execute last-known-good readback, not just schema validation. 4. Verify #622 hot-load authority and #854 cost-control behavior are consumed. Reload must not introduce recurring metered inference probes. Tests use a deterministic local provider endpoint and make no hosted-provider qualification claim. 5. Retain executed endpoint/profile parity, reload concurrency, schema negatives and secret-redaction proof. A configuration schema or unused parser is insufficient; real dispatch is the consumer."
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
  - "876-provider-definitions"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Prerequisite output or exact implementation test targets may change; revalidate before child execution."
test_strategy:
  - "1. Load valid editable definitions using the production entrypoint, execute a local controlled provider request and prove the observed endpoint/model/profile matches the selected validated snapshot. Edit definitions without rebuilding; a later dispatch uses the new complete snapshot. In-flight dispatch retains its original snapshot and concurrent readers never observe a mixed map. 2. Preserve existing endpoint/profile behavior, declared compatibility and provider generations. Separate instance data from adapter behavior; document supported fields and explicit unsupported cases. No hardcoded instance branch may stand in for consumption. 3. Through the production loader reject malformed, unsupported, incomplete and credential-shaped definitions, including secret values hidden under neutral nested keys. Keep only approved credential references. Invalid initial configuration fails closed; invalid replacement retains the prior whole snapshot and yields a bounded diagnostic. Execute last-known-good readback, not just schema validation. 4. Verify #622 hot-load authority and #854 cost-control behavior are consumed. Reload must not introduce recurring metered inference probes. Tests use a deterministic local provider endpoint and make no hosted-provider qualification claim. 5. Retain executed endpoint/profile parity, reload concurrency, schema negatives and secret-redaction proof. A configuration schema or unused parser is insufficient; real dispatch is the consumer. PVF lane: deterministic local provider integration/contract; role: production definition consumption, atomic reload and invalid-input rejection; resources: bounded local CPU/filesystem and controlled loopback endpoint, no paid inference; gate: required provider-platform completion and downstream RT-PROVIDER input. Live/provider-cost execution needs separate explicit authority. Stop on missing hot-load authority, unresolved owner collision, hardcoded instance data, credential capture, failed or missing proving scenario, or incompatible unreviewed format changes. Exclude provider behavior rewrite, agent lifecycle, MLX/PAIR implementation, benchmark marketing, broad Runtime changes and schema-only completion."
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
notes: "Bound implementation and focused local proof recorded in .csdlc/evidence/876/IMPLEMENTATION_PROOF.md. Independent exact-head review and PR CI remain required; no merge or closeout claim."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][PLAT-PROVIDER] Consume validated editable provider definitions`.

Extend the existing provider reload owner; reject credential-shaped nested values without rejecting legitimate model identifiers or approved credential references. Prove real loopback endpoint/model/profile consumption, barrier-synchronized old in-flight and new dispatch, whole-map atomicity, and last-known-good dispatch after invalid edits. Run focused provider tests, preserve diagnostics evidence, obtain exact-head independent review and publish a passing PR.

## PVF Lane Plan

- Initial PVF lane from issue creation: `provider`
- Planned PVF lane for execution: `provider`
- Planning lane source: `Issue #876 acceptance and PVF clauses`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `Not estimated; record actual child execution metrics.`
- Estimated total tokens: `Not estimated; record actual child execution metrics.`
- Estimated validation seconds: `Not estimated; record actual child execution metrics.`
- Issue goal token budget: `not specified`
- Variance threshold percent: `Not estimated; record actual child execution metrics.`
- Estimate confidence: `unestimated`
- Estimate data source: `No child execution baseline measured`
- Estimate source ref: `source issue validation requirements`
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

1. Confirm dependency readiness and starting state: Accepted merged output required from #864, #854, #622. No sprint-wide barrier or asynchronous closeout dependency.
2. Review repo inputs and scoped surfaces before editing: Full canonical issue https://github.com/agent-logic/agent-design-language/issues/876; docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md; WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml and ATOMIC_TASK_CONTRACTS_v0.92.2.json.
3. Implement only the bounded deliverables: The Runtime provider-definition loader consumes operator-editable endpoint/profile data through the existing adapter boundary, atomically retains the last-known-good snapshot on invalid replacement, and exposes the change to subsequent real production dispatch. Depends on WP-01/#864, RT-COST/#854 and the verified merged provider hot-loading predecessor #622. RT-PROVIDER/#855 separately owns dynamic agent lifecycle; this issue supplies configuration, not attach/detach implementation. Include production proof, failure handling and operator documentation required by the source issue.
4. Run focused proof gates for acceptance: 1. Load valid editable definitions using the production entrypoint, execute a local controlled provider request and prove the observed endpoint/model/profile matches the selected validated snapshot. Edit definitions without rebuilding; a later dispatch uses the new complete snapshot. In-flight dispatch retains its original snapshot and concurrent readers never observe a mixed map. 2. Preserve existing endpoint/profile behavior, declared compatibility and provider generations. Separate instance data from adapter behavior; document supported fields and explicit unsupported cases. No hardcoded instance branch may stand in for consumption. 3. Through the production loader reject malformed, unsupported, incomplete and credential-shaped definitions, including secret values hidden under neutral nested keys. Keep only approved credential references. Invalid initial configuration fails closed; invalid replacement retains the prior whole snapshot and yields a bounded diagnostic. Execute last-known-good readback, not just schema validation. 4. Verify #622 hot-load authority and #854 cost-control behavior are consumed. Reload must not introduce recurring metered inference probes. Tests use a deterministic local provider endpoint and make no hosted-provider qualification claim. 5. Retain executed endpoint/profile parity, reload concurrency, schema negatives and secret-redaction proof. A configuration schema or unused parser is insufficient; real dispatch is the consumer.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- 876-provider-definitions

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Prerequisite output or exact implementation test targets may change; revalidate before child execution.

## Test Strategy

- 1. Load valid editable definitions using the production entrypoint, execute a local controlled provider request and prove the observed endpoint/model/profile matches the selected validated snapshot. Edit definitions without rebuilding; a later dispatch uses the new complete snapshot. In-flight dispatch retains its original snapshot and concurrent readers never observe a mixed map. 2. Preserve existing endpoint/profile behavior, declared compatibility and provider generations. Separate instance data from adapter behavior; document supported fields and explicit unsupported cases. No hardcoded instance branch may stand in for consumption. 3. Through the production loader reject malformed, unsupported, incomplete and credential-shaped definitions, including secret values hidden under neutral nested keys. Keep only approved credential references. Invalid initial configuration fails closed; invalid replacement retains the prior whole snapshot and yields a bounded diagnostic. Execute last-known-good readback, not just schema validation. 4. Verify #622 hot-load authority and #854 cost-control behavior are consumed. Reload must not introduce recurring metered inference probes. Tests use a deterministic local provider endpoint and make no hosted-provider qualification claim. 5. Retain executed endpoint/profile parity, reload concurrency, schema negatives and secret-redaction proof. A configuration schema or unused parser is insufficient; real dispatch is the consumer. PVF lane: deterministic local provider integration/contract; role: production definition consumption, atomic reload and invalid-input rejection; resources: bounded local CPU/filesystem and controlled loopback endpoint, no paid inference; gate: required provider-platform completion and downstream RT-PROVIDER input. Live/provider-cost execution needs separate explicit authority. Stop on missing hot-load authority, unresolved owner collision, hardcoded instance data, credential capture, failed or missing proving scenario, or incompatible unreviewed format changes. Exclude provider behavior rewrite, agent lifecycle, MLX/PAIR implementation, benchmark marketing, broad Runtime changes and schema-only completion.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Bound implementation and focused local proof recorded in .csdlc/evidence/876/IMPLEMENTATION_PROOF.md. Independent exact-head review and PR CI remain required; no merge or closeout claim.
