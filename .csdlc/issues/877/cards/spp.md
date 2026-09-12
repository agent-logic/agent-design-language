---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "877-uts-package-execution-plan"
issue: 877
task_id: "issue-0877"
run_id: "issue-0877"
version: "v0.92.2"
title: "[v0.92.2][PLAT-UTS] Install a versioned UTS package consumed by Runtime tool dispatch"
branch: "codex/877-uts-package"
generated_at: "2026-09-11T23:55:24.168359+00:00"
card_status: "ready"
status: "prepared"
activation_state: "activated"
plan_revision: 1
initial_pvf_lane: "runtime"
planned_pvf_lane: "runtime"
planned_pvf_lane_source: "Issue #877 acceptance and PVF clauses"
estimate_elapsed_seconds: "Not estimated; record actual child execution metrics."
estimate_total_tokens: "Not estimated; record actual child execution metrics."
estimate_validation_seconds: "Not estimated; record actual child execution metrics."
issue_goal_token_budget: "not specified"
variance_threshold_percent: "Not estimated; record actual child execution metrics."
estimate_confidence: "unestimated"
estimate_data_source: "No child execution baseline measured"
estimate_source_ref: "source issue validation requirements"
issue_goal_ref: "Agent-local issue #877 implementation goal created after native doctor passed; parent Sprint 2/#928 goal remains active."
sprint_goal_ref: "Current v0.92.2 Sprint2 execution-readiness preparation goal"
goal_metrics_rollup_ref: "Not measured: no child execution"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/877"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/877"
  - kind: "stp"
    ref: ".csdlc/issues/877/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/877/cards/sip.md"
scope:
  files:
    - "Read `docs/specs/uts/README.md`, `UTS_V1.0_SCHEMA.md`, `UTS_V1.1_SCHEMA.md`, and `adl-spec/schemas/uts/`. Existing types are in `adl/src/uts.rs`; conformance in `adl/src/uts_conformance.rs`; ACC compilation in `adl/src/uts_acc_compiler/`; production consumers include `adl/src/resident_tool_execution.rs`, `adl/src/tool_registry.rs` and `adl/src/governed_executor.rs`. UTS describes tools; ACC retains runtime authority. Documentation calls v1 the guaranteed baseline while source also includes v1.1 types; reconcile actual supported semantics rather than inferring complete v1.1 implementation from type presence.  Selected package location: a new in-repository `adl-uts/` Rust crate, initial package version `0.1.0`, with manifest, canonical types/schema assets and focused tests. Package version and UTS schema version are distinct. Preserve existing supported `uts.v1` and `uts.v1.1` serialization/types and declare their actual implemented compatibility separately; introducing this package does not claim every proposed v1.1 semantic is implemented. Make `adl/Cargo.toml` consume the package and migrate only necessary UTS imports/reexports and the named production dispatch path. Preserve ACC enforcement and unrelated APIs. No external registry publication or standalone repository creation is implied."
  components:
    - "877-uts-package"
  out_of_scope:
    - "PVF: deterministic local schema/package/Runtime integration; role: installability, real consumer and compatibility/authority rejection; resources: bounded CPU/disk and a read-only local tool, no provider/cloud spend; gate: required milestone support result. Record nonzero executed scenarios and exact package/consumer revisions. Stop on ambiguous contract authority, silent breaking behavior, unauthorized tool effect or missing consumer proof. Exclude universal ecosystem standard claims, external package publication, wholesale ACC redesign and unrelated Runtime refactoring."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Extract unchanged UTS types/validators into adl-uts 0.1.0 with schema assets; retain adl::uts compatibility reexport and route registry loading through package validation; prove isolated artifact consumption, before/after conformance and actual Runtime governed observation including denial before effects; document supported semantics; independent review and native PR."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: Accepted merged output required from #864. No sprint-wide barrier or asynchronous closeout dependency."
    expected_output: ".csdlc/issues/877/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Full canonical issue https://github.com/agent-logic/agent-design-language/issues/877; docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md; WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml and ATOMIC_TASK_CONTRACTS_v0.92.2.json."
    expected_output: ".csdlc/issues/877/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Install one versioned UTS package and use it in actual Runtime ACC/UTS tool dispatch with explicit supported-version compatibility. Depends on WP-01/#864. This is one working package/consumer delivery; docs, schema and migration notes support that result. Include production proof, failure handling and operator documentation required by the source issue."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: 1. Build/package/install the selected version from a clean bounded checkout into an isolated consumer environment. One canonical package owns the supported schema/type contract; no silent private fork remains in the named consumer. Include complete installation and migration instructions. 2. Exercise a real Runtime-owned governed tool dispatch that loads/validates the packaged contract, obtains an ACC-governed decision and executes an actual bounded read-only tool. Retain invocation, package/schema version, authority decision and real result. A compiler fixture or package import test alone does not prove consumer integration. 3. Define and execute compatible-version cases and reject unsupported versions, malformed tool declarations, registry-binding mismatch and authority/policy denial before tool effects. Preserve side-effect/replay/idempotence/data-sensitivity semantics; package/schema compatibility cannot grant execution authority. 4. Run schema conformance and named-consumer parity before/after migration. Explicitly distinguish v1 guaranteed semantics from any separately implemented v1.1 surface; reject silent breaking changes. Record package artifact/version and successful isolated consumption. 5. Complete focused source docs and independent review. A published standard document, unused library or fixture-only consumer cannot close the issue."
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
  - "877-uts-package"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Prerequisite output or exact implementation test targets may change; revalidate before child execution."
test_strategy:
  - "1. Build/package/install the selected version from a clean bounded checkout into an isolated consumer environment. One canonical package owns the supported schema/type contract; no silent private fork remains in the named consumer. Include complete installation and migration instructions. 2. Exercise a real Runtime-owned governed tool dispatch that loads/validates the packaged contract, obtains an ACC-governed decision and executes an actual bounded read-only tool. Retain invocation, package/schema version, authority decision and real result. A compiler fixture or package import test alone does not prove consumer integration. 3. Define and execute compatible-version cases and reject unsupported versions, malformed tool declarations, registry-binding mismatch and authority/policy denial before tool effects. Preserve side-effect/replay/idempotence/data-sensitivity semantics; package/schema compatibility cannot grant execution authority. 4. Run schema conformance and named-consumer parity before/after migration. Explicitly distinguish v1 guaranteed semantics from any separately implemented v1.1 surface; reject silent breaking changes. Record package artifact/version and successful isolated consumption. 5. Complete focused source docs and independent review. A published standard document, unused library or fixture-only consumer cannot close the issue. PVF: deterministic local schema/package/Runtime integration; role: installability, real consumer and compatibility/authority rejection; resources: bounded CPU/disk and a read-only local tool, no provider/cloud spend; gate: required milestone support result. Record nonzero executed scenarios and exact package/consumer revisions. Stop on ambiguous contract authority, silent breaking behavior, unauthorized tool effect or missing consumer proof. Exclude universal ecosystem standard claims, external package publication, wholesale ACC redesign and unrelated Runtime refactoring."
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
notes: "Prepared only. Child implementation and its review have not run."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][PLAT-UTS] Install a versioned UTS package consumed by Runtime tool dispatch`.

Extract unchanged UTS types/validators into adl-uts 0.1.0 with schema assets; retain adl::uts compatibility reexport and route registry loading through package validation; prove isolated artifact consumption, before/after conformance and actual Runtime governed observation including denial before effects; document supported semantics; independent review and native PR.

## PVF Lane Plan

- Initial PVF lane from issue creation: `runtime`
- Planned PVF lane for execution: `runtime`
- Planning lane source: `Issue #877 acceptance and PVF clauses`
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
4. [completed] Run focused validation and proof gates.
5. [in_progress] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: Accepted merged output required from #864. No sprint-wide barrier or asynchronous closeout dependency.
2. Review repo inputs and scoped surfaces before editing: Full canonical issue https://github.com/agent-logic/agent-design-language/issues/877; docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md; WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml and ATOMIC_TASK_CONTRACTS_v0.92.2.json.
3. Implement only the bounded deliverables: Install one versioned UTS package and use it in actual Runtime ACC/UTS tool dispatch with explicit supported-version compatibility. Depends on WP-01/#864. This is one working package/consumer delivery; docs, schema and migration notes support that result. Include production proof, failure handling and operator documentation required by the source issue.
4. Run focused proof gates for acceptance: 1. Build/package/install the selected version from a clean bounded checkout into an isolated consumer environment. One canonical package owns the supported schema/type contract; no silent private fork remains in the named consumer. Include complete installation and migration instructions. 2. Exercise a real Runtime-owned governed tool dispatch that loads/validates the packaged contract, obtains an ACC-governed decision and executes an actual bounded read-only tool. Retain invocation, package/schema version, authority decision and real result. A compiler fixture or package import test alone does not prove consumer integration. 3. Define and execute compatible-version cases and reject unsupported versions, malformed tool declarations, registry-binding mismatch and authority/policy denial before tool effects. Preserve side-effect/replay/idempotence/data-sensitivity semantics; package/schema compatibility cannot grant execution authority. 4. Run schema conformance and named-consumer parity before/after migration. Explicitly distinguish v1 guaranteed semantics from any separately implemented v1.1 surface; reject silent breaking changes. Record package artifact/version and successful isolated consumption. 5. Complete focused source docs and independent review. A published standard document, unused library or fixture-only consumer cannot close the issue.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- 877-uts-package

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Prerequisite output or exact implementation test targets may change; revalidate before child execution.

## Test Strategy

- 1. Build/package/install the selected version from a clean bounded checkout into an isolated consumer environment. One canonical package owns the supported schema/type contract; no silent private fork remains in the named consumer. Include complete installation and migration instructions. 2. Exercise a real Runtime-owned governed tool dispatch that loads/validates the packaged contract, obtains an ACC-governed decision and executes an actual bounded read-only tool. Retain invocation, package/schema version, authority decision and real result. A compiler fixture or package import test alone does not prove consumer integration. 3. Define and execute compatible-version cases and reject unsupported versions, malformed tool declarations, registry-binding mismatch and authority/policy denial before tool effects. Preserve side-effect/replay/idempotence/data-sensitivity semantics; package/schema compatibility cannot grant execution authority. 4. Run schema conformance and named-consumer parity before/after migration. Explicitly distinguish v1 guaranteed semantics from any separately implemented v1.1 surface; reject silent breaking changes. Record package artifact/version and successful isolated consumption. 5. Complete focused source docs and independent review. A published standard document, unused library or fixture-only consumer cannot close the issue. PVF: deterministic local schema/package/Runtime integration; role: installability, real consumer and compatibility/authority rejection; resources: bounded CPU/disk and a read-only local tool, no provider/cloud spend; gate: required milestone support result. Record nonzero executed scenarios and exact package/consumer revisions. Stop on ambiguous contract authority, silent breaking behavior, unauthorized tool effect or missing consumer proof. Exclude universal ecosystem standard claims, external package publication, wholesale ACC redesign and unrelated Runtime refactoring.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Prepared only. Child implementation and its review have not run.
