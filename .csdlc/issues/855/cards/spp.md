---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "855-provider-neutral-lifecycle-execution-plan"
issue: 855
task_id: "issue-0855"
run_id: "issue-0855"
version: "v0.92.2"
title: "[v0.92.2][RT-PROVIDER] Provider-neutral dynamic agent lifecycle"
branch: "codex/855-provider-neutral-lifecycle"
generated_at: "2026-09-11T23:55:22.557345+00:00"
card_status: "ready"
status: "prepared"
activation_state: "not_activated"
plan_revision: 1
initial_pvf_lane: "provider"
planned_pvf_lane: "provider"
planned_pvf_lane_source: "Issue #855 acceptance and PVF clauses"
estimate_elapsed_seconds: "Not estimated; record actual child execution metrics."
estimate_total_tokens: "Not estimated; record actual child execution metrics."
estimate_validation_seconds: "Not estimated; record actual child execution metrics."
issue_goal_token_budget: "not specified"
variance_threshold_percent: "Not estimated; record actual child execution metrics."
estimate_confidence: "unestimated"
estimate_data_source: "No child execution baseline measured"
estimate_source_ref: "source issue validation requirements"
issue_goal_ref: "issue855 implementation goal 01a0941a-fa4c-7bc1-8585-7b7e48e2fe00"
sprint_goal_ref: "Current v0.92.2 Sprint2 execution-readiness preparation goal"
goal_metrics_rollup_ref: "Not measured: no child execution"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/855"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/855"
  - kind: "stp"
    ref: ".csdlc/issues/855/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/855/cards/sip.md"
scope:
  files:
    - "- `adl-runtime-kernel/src/control.rs` - `adl-runtime-kernel/src/config.rs` - `adl/src/cli/csmctl_cmd.rs` - Existing provider adapter and capability architecture - `docs/architecture/PROVIDER_CAPABILITY_AND_TRANSPORT_ARCHITECTURE.md` - Issue #602 dynamic agent lifecycle behavior - Issue #854 metered cloud inference safeguards"
  components:
    - "855-provider-neutral-lifecycle"
  out_of_scope:
    - "- Hard-coding only the four providers used by the initial acceptance demonstration. - Embedding provider secrets in agent configuration. - Requiring all providers to support identical optional capabilities. - Restarting Runtime merely to add, replace, or remove an agent. - Changing subscription or cloud billing configuration."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Extract canonical provider implementations into adl-provider-core with ADL compatibility exports; wire a capability registry and accepted definition snapshots into the existing production kernel binary before admission restoration. Complete all issue855 lifecycle and provider proof including separately authorized hosted demonstration."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: Accepted876 PR953 is ancestor of bound baseline b6d110c84e11253b392d0bb078f2fb33a36b9a0c; preserve854 safeguards."
    expected_output: ".csdlc/issues/855/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Extract adl-provider-core canonical DTO/adapters/profiles/validation, retaining ADL facades and accepted876 semantics. Wire process-owned kernel registry into production binary before restore; metadata/capability/endpoint/reference validation, bounded blocking execution and cached truthful projection; provider-neutral canonical operator and A2A dispatch and lifecycle replacement/checkpoint/migration/restore. Extend CLI config fields. Add bounded demo call/input/output/retry/trust controls required for actual hosted qualification. Add direct leaf CI tests in .github/workflows/ci.yaml so moved tests retain execution ownership; preserve old ADL compatibility tests and focused kernel gates. Work surfaces: adl-provider-core, ADL provider/model/CLI facades, kernel control/assembly/telemetry/roster/bin, owner Cargo manifests and locks, issue855 tools/docs and CI provider job."
    expected_output: ".csdlc/issues/855/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Extract adl-provider-core canonical DTO/adapters/profiles/validation, retaining ADL facades and accepted876 semantics. Wire process-owned kernel registry into production binary before restore; metadata/capability/endpoint/reference validation, bounded blocking execution and cached truthful projection; provider-neutral canonical operator and A2A dispatch and lifecycle replacement/checkpoint/migration/restore. Extend CLI config fields. Add bounded demo call/input/output/retry/trust controls required for actual hosted qualification. Add direct leaf CI tests in .github/workflows/ci.yaml so moved tests retain execution ownership; preserve old ADL compatibility tests and focused kernel gates. Work surfaces: adl-provider-core, ADL provider/model/CLI facades, kernel control/assembly/telemetry/roster/bin, owner Cargo manifests and locks, issue855 tools/docs and CI provider job."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: - `csmctl agent add --config <agent.yaml>` accepts every provider registered with the running Runtime and rejects only an unknown provider or a provider whose declared capability requirements are unsatisfied. - Adding or replacing a dynamic agent does not require a Runtime or Guardian restart and does not require editing the Runtime initialization file. - OpenAI/ChatGPT, Anthropic/Claude, Gemini through the appropriate Google provider route, Ollama local, and an OpenAI-compatible local endpoint each pass add, generated conversation, roster projection, checkpoint, removal, and rehydration tests. - Agent-to-agent communication uses the same provider-neutral execution path and canonical agent names for every admitted provider. - Provider adapters declare whether tools, streaming, model discovery, token accounting, and health checks are supported; the Runtime does not infer these capabilities from provider names. - Hosted providers require HTTPS and approved credential references. Local plaintext endpoints remain limited to loopback, private, or explicitly trusted local-network bindings. - Admission performs no recurring paid inference. Health checks and model validation follow provider capabilities and the metered-call safeguards tracked by #854. - Provider failures retain actionable classifications such as credentials, quota, unsupported capability, model unavailable, transport, timeout, and invalid response. - The API and Observatory report the effective provider/model and current capability/readiness state without exposing credentials. - Deterministic tests prove that a newly registered fixture provider works without modifying Runtime admission or conversation match statements."
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
    status: "in_progress"
affected_areas:
  - "855-provider-neutral-lifecycle"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Preserve existing adapter implementations, accepted876 profile expansion and last-known-good atomicity, both Bedrock account hash aliases and URL credential rejection. Avoid package cycle. One kernel config_reload watcher per process. Never buffer streaming and label true streaming. Transport timeouts and bounded calls must outlive cancellation safely without repeated billing."
test_strategy:
  - "Provider 94/94+Runtime mock focused 1/1, kernel control85/85,ADL compatibility18/18,CLI 7/7,OpenAPI 11/11 and Clippy passed; installed five-provider fixture 5/5 plus hosted topology fixture 3/3 passed. Source independent review passedthrough d96. Authorized actual hosted run andrequired CI remain pending; draft publication overlaps waiting withoutclaiming full acceptance."
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
notes: "Composition: leaf provider DTOs are canonical types re-exported by ADL; workflow document remains ADL-owned. Provider sidecar selected once at kernel startup, reload changes provider definitions without initialization edits. Credential references are env names or approved adapter auth modes, resolved only during execution. Bindings re-resolve current validated snapshot on each call and restore; stable model_ref stays explicit while effective native model is projected. New capabilities are conservative adapter declarations. Current snapshot metadata is projected, in-flight calls pin Arc snapshots. No implementation or proof yet."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][RT-PROVIDER] Provider-neutral dynamic agent lifecycle`.

Extract canonical provider implementations into adl-provider-core with ADL compatibility exports; wire a capability registry and accepted definition snapshots into the existing production kernel binary before admission restoration. Complete all issue855 lifecycle and provider proof including separately authorized hosted demonstration.

## PVF Lane Plan

- Initial PVF lane from issue creation: `provider`
- Planned PVF lane for execution: `provider`
- Planning lane source: `Issue #855 acceptance and PVF clauses`
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

1. [completed] Confirm dependencies and starting state from the source issue prompt.
2. [completed] Inspect repo inputs and target surfaces before editing.
3. [completed] Implement the bounded deliverables only.
4. [in_progress] Run focused validation and proof gates.
5. [in_progress] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: Accepted876 PR953 is ancestor of bound baseline b6d110c84e11253b392d0bb078f2fb33a36b9a0c; preserve854 safeguards.
2. Review repo inputs and scoped surfaces before editing: Extract adl-provider-core canonical DTO/adapters/profiles/validation, retaining ADL facades and accepted876 semantics. Wire process-owned kernel registry into production binary before restore; metadata/capability/endpoint/reference validation, bounded blocking execution and cached truthful projection; provider-neutral canonical operator and A2A dispatch and lifecycle replacement/checkpoint/migration/restore. Extend CLI config fields. Add bounded demo call/input/output/retry/trust controls required for actual hosted qualification. Add direct leaf CI tests in .github/workflows/ci.yaml so moved tests retain execution ownership; preserve old ADL compatibility tests and focused kernel gates. Work surfaces: adl-provider-core, ADL provider/model/CLI facades, kernel control/assembly/telemetry/roster/bin, owner Cargo manifests and locks, issue855 tools/docs and CI provider job.
3. Implement only the bounded deliverables: Extract adl-provider-core canonical DTO/adapters/profiles/validation, retaining ADL facades and accepted876 semantics. Wire process-owned kernel registry into production binary before restore; metadata/capability/endpoint/reference validation, bounded blocking execution and cached truthful projection; provider-neutral canonical operator and A2A dispatch and lifecycle replacement/checkpoint/migration/restore. Extend CLI config fields. Add bounded demo call/input/output/retry/trust controls required for actual hosted qualification. Add direct leaf CI tests in .github/workflows/ci.yaml so moved tests retain execution ownership; preserve old ADL compatibility tests and focused kernel gates. Work surfaces: adl-provider-core, ADL provider/model/CLI facades, kernel control/assembly/telemetry/roster/bin, owner Cargo manifests and locks, issue855 tools/docs and CI provider job.
4. Run focused proof gates for acceptance: - `csmctl agent add --config <agent.yaml>` accepts every provider registered with the running Runtime and rejects only an unknown provider or a provider whose declared capability requirements are unsatisfied. - Adding or replacing a dynamic agent does not require a Runtime or Guardian restart and does not require editing the Runtime initialization file. - OpenAI/ChatGPT, Anthropic/Claude, Gemini through the appropriate Google provider route, Ollama local, and an OpenAI-compatible local endpoint each pass add, generated conversation, roster projection, checkpoint, removal, and rehydration tests. - Agent-to-agent communication uses the same provider-neutral execution path and canonical agent names for every admitted provider. - Provider adapters declare whether tools, streaming, model discovery, token accounting, and health checks are supported; the Runtime does not infer these capabilities from provider names. - Hosted providers require HTTPS and approved credential references. Local plaintext endpoints remain limited to loopback, private, or explicitly trusted local-network bindings. - Admission performs no recurring paid inference. Health checks and model validation follow provider capabilities and the metered-call safeguards tracked by #854. - Provider failures retain actionable classifications such as credentials, quota, unsupported capability, model unavailable, transport, timeout, and invalid response. - The API and Observatory report the effective provider/model and current capability/readiness state without exposing credentials. - Deterministic tests prove that a newly registered fixture provider works without modifying Runtime admission or conversation match statements.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- 855-provider-neutral-lifecycle

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Preserve existing adapter implementations, accepted876 profile expansion and last-known-good atomicity, both Bedrock account hash aliases and URL credential rejection. Avoid package cycle. One kernel config_reload watcher per process. Never buffer streaming and label true streaming. Transport timeouts and bounded calls must outlive cancellation safely without repeated billing.

## Test Strategy

- Provider 94/94+Runtime mock focused 1/1, kernel control85/85,ADL compatibility18/18,CLI 7/7,OpenAPI 11/11 and Clippy passed; installed five-provider fixture 5/5 plus hosted topology fixture 3/3 passed. Source independent review passedthrough d96. Authorized actual hosted run andrequired CI remain pending; draft publication overlaps waiting withoutclaiming full acceptance.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Composition: leaf provider DTOs are canonical types re-exported by ADL; workflow document remains ADL-owned. Provider sidecar selected once at kernel startup, reload changes provider definitions without initialization edits. Credential references are env names or approved adapter auth modes, resolved only during execution. Bindings re-resolve current validated snapshot on each call and restore; stable model_ref stays explicit while effective native model is projected. New capabilities are conservative adapter declarations. Current snapshot metadata is projected, in-flight calls pin Arc snapshots. No implementation or proof yet.
