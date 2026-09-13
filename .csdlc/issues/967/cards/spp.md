---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "issue-967-deterministic-hosted-a2a-execution-plan"
issue: 967
task_id: "issue-0967"
run_id: "issue-0967"
version: "v0.92.2"
title: "[v0.92.2][RT-PROVIDER][corrective] Make hosted A2A initiation deterministic across provider output formats"
branch: "codex/967-deterministic-hosted-a2a"
generated_at: "2026-09-12"
card_status: "ready"
status: "completed"
activation_state: "active"
plan_revision: 1
initial_pvf_lane: "provider"
planned_pvf_lane: "provider"
planned_pvf_lane_source: "issue967 scoped Runtime/provider contract"
estimate_elapsed_seconds: "unknown"
estimate_total_tokens: "unknown"
estimate_validation_seconds: "unknown"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "unknown"
estimate_confidence: "low"
estimate_data_source: "unknown"
estimate_source_ref: "https://github.com/agent-logic/agent-design-language/issues/967"
issue_goal_ref: "issue-967"
sprint_goal_ref: "issue-928"
goal_metrics_rollup_ref: "unknown"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/967"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/967"
  - kind: "stp"
    ref: ".csdlc/issues/967/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/967/cards/sip.md"
scope:
  files:
    - "adl-runtime-kernel/src/control.rs; adl-runtime-kernel/tests/openapi_contract.rs; adl/tools/issue855_provider_lifecycle.py; docs/api/runtime-v3/v1/observatory.openapi.json; issue-local cards and proof metadata."
  components:
    - "issue-967-deterministic-hosted-a2a"
  out_of_scope:
    - "No arbitrary JSON extraction from prose; no bypass of signatures, canonical addressing, capability checks or replay protection; no credential, billing, model or cloud configuration changes; no rewriting merged #855/PR964 history."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Implemented and qualified the bounded typed-action repair on the merged baseline: local regression, zero-paid matrices, full CI, and one separately authorized hosted acceptance now pass; PR #968 remains operator-controlled for merge."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: #855 / PR #964 are merged. #967 owns the post-merge correction and is part of #928; prior issue closure is not corrective acceptance."
    expected_output: ".csdlc/issues/967/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Live issue #967, merged #855/PR964, canonical Observatory intent and signed A2A delivery, OpenAPI contract tests and bounded lifecycle harness."
    expected_output: ".csdlc/issues/967/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: adl-runtime-kernel/src/control.rs; adl-runtime-kernel/tests/openapi_contract.rs; adl/tools/issue855_provider_lifecycle.py; docs/api/runtime-v3/v1/observatory.openapi.json; issue-local cards and proof metadata. Make authenticated operator-requested A2A initiation independent of provider reply formatting while retaining one initiating provider call and existing signed peer delivery."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: Refuse invalid, unknown, self-targeted, empty and over-limit actions before provider calls; dispatch one signed peer exchange after an ordinary reply; coalesce identical model/request actions; reject conflicts before peer dispatch; preserve absent-field model actions and replay fingerprints; align OpenAPI and Runtime boundaries; pass zero-paid 5/5 and 3/3 matrices, separately authorized bounded hosted acceptance, exact-head independent review and required CI."
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
  - "issue-967-deterministic-hosted-a2a"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Conflicting actions must fail closed; absent-field fingerprints must remain stable; paid inference needs fresh authorization; inherited working-tree proof must not be labeled exact-head."
test_strategy:
  - "Completed: focused Runtime dispatch 1/1, OpenAPI contracts 11/11, zero-paid five-provider 5/5 and hosted-topology 3/3 matrices, required CI run 34685454688 green, and one authorized no-retry hosted acceptance 3/3 at source 6d7bc805e5714050c85c80d5e9351e40cd10d635 with 16 paid requests under the six-call/provider, 32000-byte input, 256-output-token, $1 and 30-minute configured bounds. Refresh exact-head review/publication after committing this evidence; merge remains operator-controlled."
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
notes: "Preserved failed #855 hosted-live-03 report SHA256 651f9a00fb5b96e2a6b0539d1f9321c4546631a16b78e9c723fe823c0d95d439 and the distinct zero-paid #967 packet. The authorized #967 hosted run passed 3/3 with 16 paid provider requests (OpenAI 6, Anthropic 5, Vertex 5) under the configured caps. Runtime counters do not establish vendor-billed token totals or final invoice cost. PR #968 still requires current exact-head review/publication after this evidence update and operator-controlled merge."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][RT-PROVIDER][corrective] Make hosted A2A initiation deterministic across provider output formats`.

Implemented and qualified the bounded typed-action repair on the merged baseline: local regression, zero-paid matrices, full CI, and one separately authorized hosted acceptance now pass; PR #968 remains operator-controlled for merge.

## PVF Lane Plan

- Initial PVF lane from issue creation: `provider`
- Planned PVF lane for execution: `provider`
- Planning lane source: `issue967 scoped Runtime/provider contract`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `unknown`
- Estimated total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Issue goal token budget: `unknown`
- Variance threshold percent: `unknown`
- Estimate confidence: `low`
- Estimate data source: `unknown`
- Estimate source ref: `https://github.com/agent-logic/agent-design-language/issues/967`
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

1. Confirm dependency readiness and starting state: #855 / PR #964 are merged. #967 owns the post-merge correction and is part of #928; prior issue closure is not corrective acceptance.
2. Review repo inputs and scoped surfaces before editing: Live issue #967, merged #855/PR964, canonical Observatory intent and signed A2A delivery, OpenAPI contract tests and bounded lifecycle harness.
3. Implement only the bounded deliverables: adl-runtime-kernel/src/control.rs; adl-runtime-kernel/tests/openapi_contract.rs; adl/tools/issue855_provider_lifecycle.py; docs/api/runtime-v3/v1/observatory.openapi.json; issue-local cards and proof metadata. Make authenticated operator-requested A2A initiation independent of provider reply formatting while retaining one initiating provider call and existing signed peer delivery.
4. Run focused proof gates for acceptance: Refuse invalid, unknown, self-targeted, empty and over-limit actions before provider calls; dispatch one signed peer exchange after an ordinary reply; coalesce identical model/request actions; reject conflicts before peer dispatch; preserve absent-field model actions and replay fingerprints; align OpenAPI and Runtime boundaries; pass zero-paid 5/5 and 3/3 matrices, separately authorized bounded hosted acceptance, exact-head independent review and required CI.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- issue-967-deterministic-hosted-a2a

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Conflicting actions must fail closed; absent-field fingerprints must remain stable; paid inference needs fresh authorization; inherited working-tree proof must not be labeled exact-head.

## Test Strategy

- Completed: focused Runtime dispatch 1/1, OpenAPI contracts 11/11, zero-paid five-provider 5/5 and hosted-topology 3/3 matrices, required CI run 34685454688 green, and one authorized no-retry hosted acceptance 3/3 at source 6d7bc805e5714050c85c80d5e9351e40cd10d635 with 16 paid requests under the six-call/provider, 32000-byte input, 256-output-token, $1 and 30-minute configured bounds. Refresh exact-head review/publication after committing this evidence; merge remains operator-controlled.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Preserved failed #855 hosted-live-03 report SHA256 651f9a00fb5b96e2a6b0539d1f9321c4546631a16b78e9c723fe823c0d95d439 and the distinct zero-paid #967 packet. The authorized #967 hosted run passed 3/3 with 16 paid provider requests (OpenAI 6, Anthropic 5, Vertex 5) under the configured caps. Runtime counters do not establish vendor-billed token totals or final invoice cost. PR #968 still requires current exact-head review/publication after this evidence update and operator-controlled merge.
