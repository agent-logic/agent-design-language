---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "v0922-test-planner-execution-plan"
issue: 894
task_id: "issue-0894"
run_id: "issue-0894"
version: "0.92.2"
title: "[v0.92.2][CF-TESTPLAN] Generate a bounded test plan from review findings"
branch: "codex/894-v0922-test-planner"
generated_at: "2026-09-12T00:10:09.925246+00:00"
card_status: "ready"
status: "planned"
activation_state: "bound_implemented_pending_fresh_review"
plan_revision: 1
initial_pvf_lane: "runtime"
planned_pvf_lane: "runtime"
planned_pvf_lane_source: "https://github.com/agent-logic/agent-design-language/issues/894 deterministic production consumer acceptance; docs/validation/pvf_lanes.json"
estimate_elapsed_seconds: "unknown"
estimate_total_tokens: "unknown"
estimate_validation_seconds: "1200"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "10"
estimate_confidence: "low"
estimate_data_source: "bounded focused validation estimate; implementation owner recalibrates after prerequisite source is available"
estimate_source_ref: "https://github.com/agent-logic/agent-design-language/issues/894"
issue_goal_ref: "Sprint 4 #930 active goal covers #894 execution; this delegated lane advances #894 without replacing the root sprint goal."
sprint_goal_ref: "v0.92.2 execution Sprint 4; umbrella management owned by #926"
goal_metrics_rollup_ref: ".csdlc/evidence/894/goal-metrics.json (planned, absent until execution)"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/894"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/894"
  - kind: "stp"
    ref: ".csdlc/issues/894/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/894/cards/sip.md"
scope:
  files:
    - "Implement `plan tests` behavior under the selected `adl codefriend` entrypoint. Proposed cohesive modules under `adl/src/codefriend/`: actions/test_plan.rs. The CLI registration is `adl/src/cli/codefriend_cmd.rs` (CF-SHELL owns operator-control additions); coordinate shared wiring with predecessor owners. Add focused `adl/tests/codefriend_testplan.rs` and bounded fixture outputs. Exact flags are documented/tested during implementation; command wording denotes the selected behavior, not an already installed command."
  components:
    - "v0922-test-planner"
  out_of_scope:
    - "acceptance: `finding_to_behavior_trace`, `concrete_test_plan_consumed`. pvf: `finding_to_behavior_trace`, `concrete_test_plan_consumed`, `implementation_mirroring_rejected`, `source_mutation_rejected`, `action_plan_schema`. stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `source_mutation`, `test_without_behavior_mapping`. non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `security_tournament`, `autonomous_fixing`. Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`. Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`. Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts. Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. No schema/scaffold/unused helper/test-only caller/zero-execution result can close it. No autonomous source changes, paid calls, external publication or cloud resources are authorized merely by creation. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "With #892 accepted, consume the merged synthesis artifact contract and implement bounded test-plan generation that maps synthesized findings to behavior under test, proposed test location, fixture/input, expected failure before a fix, expected post-fix assertion, validation lane, and source-mutation non-goal. Before bind, recheck shared CodeFriend CLI/action paths and preserve #893/#895 sibling boundaries."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: #892 (CF-SYNTHESIS) is closed/accepted as of live read on 2026-09-16; remaining pre-bind gate is active path ownership/collision recheck."
    expected_output: ".csdlc/issues/894/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Full live source contract, retained without dropping requirements: # [v0.92.2][CF-TESTPLAN] Generate a bounded test plan from review findings ## One complete result The test planner consumes synthesized findings and maps each selected finding to a concrete behavior, test location, fixture and expected failure without changing source. Dependencies: CF-SYNTHESIS. Consume accepted merged predecessor output; logical IDs receive canonical issue numbers during creation. Batch membership adds no all-to-all gate. ## Production ownership Implement `plan tests` behavior under the selected `adl codefriend` entrypoint. Proposed cohesive modules under `adl/src/codefriend/`: actions/test_plan.rs. The CLI registration is `adl/src/cli/codefriend_cmd.rs` (CF-SHELL owns operator-control additions); coordinate shared wiring with predecessor owners. Add focused `adl/tests/codefriend_testplan.rs` and bounded fixture outputs. Exact flags are documented/tested during implementation; command wording denotes the selected behavior, not an already installed command. ## Acceptance and executed evidence 1. Consume actual CF-SYNTHESIS output and generate a readable test plan mapping every selected finding to behavior under test, source evidence, concrete proposed test location, fixture/input, expected failure before a fix, expected result after it and suitable assertion. Distinguish proposed paths from existing files. 2. Explain why each case detects the reported behavior; reject tests that merely mirror implementation or validate an unrelated schema. Record missing test infrastructure, nondeterminism and unsupported claims explicitly. Include validation lane, role and resource requirements suitable for the repository under review. 3. Run production generation and reader on predecessor output plus known findings; independently check useful behavior/fixture/assertion mapping. Reject missing evidence, incompatible runs, vague test placeholders and unmapped selected findings. 4. Prove generation changes no inspected source and creates no issue or test file in that repository. The product supplies a complete actionable plan; test implementation remains human-controlled. A canned packet or plan-only implementation is insufficient. ## Shared execution boundary Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts. Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. No schema/scaffold/unused helper/test-only caller/zero-execution result can close it. No autonomous source changes, paid calls, external publication or cloud resources are authorized merely by creation. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success. ## PVF and completion Declare each new test in a coupled proof inventory: deterministic local CPU contract/integration lane, isolated source/store and controlled clocks/transport, nonzero success and negative proof, bounded disk/process resources, required Beta feature and CF-INTEGRATE input gate. Provider-generated runs and browser/PDF visual review are separately identified execution/observation evidence, not deterministic guarantees. Run focused installed CLI and consumer tests plus required CI, then independent exact-head review. Redacted human diagnostics stay on stderr and machine output on stdout; test compatibility logging when exposed. Stop on unresolved owner/authority, inconsistent scope/provenance, failed or missing proof, source/credential exposure, stale approval or partial completion claimed as success. No unrelated feature work or customer-scale hosting. ## Inherited obligation ledger acceptance: `finding_to_behavior_trace`, `concrete_test_plan_consumed`. pvf: `finding_to_behavior_trace`, `concrete_test_plan_consumed`, `implementation_mirroring_rejected`, `source_mutation_rejected`, `action_plan_schema`. stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `source_mutation`, `test_without_behavior_mapping`. non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `security_tournament`, `autonomous_fixing`. Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`. Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`. ## Canonical execution links Planning owner: #864. Creation/review batch: 4; this grouping adds no execution gate. Execution prerequisite: #892 (CF-SYNTHESIS); accepted output is required before dependent execution. Reviewed creation source: `54e5d8e100f39c644ca0aa03e3985ce66beb529e`. This issue records a complete task; creation does not claim execution or acceptance. <!-- csdlc-v3-operation:982d5a8ecfa8cf361e7bec1c51dbacacba009f8d326e7c9d710f883eeac809d9 -->"
    expected_output: ".csdlc/issues/894/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Implement `plan tests` behavior under the selected `adl codefriend` entrypoint. Proposed cohesive modules under `adl/src/codefriend/`: actions/test_plan.rs. The CLI registration is `adl/src/cli/codefriend_cmd.rs` (CF-SHELL owns operator-control additions); coordinate shared wiring with predecessor owners. Add focused `adl/tests/codefriend_testplan.rs` and bounded fixture outputs. Exact flags are documented/tested during implementation; command wording denotes the selected behavior, not an already installed command. The test planner consumes synthesized findings and maps each selected finding to a concrete behavior, test location, fixture and expected failure without changing source. Dependencies: CF-SYNTHESIS. Consume accepted merged predecessor output; logical IDs receive canonical issue numbers during creation. Batch membership adds no all-to-all gate."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: 1. Consume actual CF-SYNTHESIS output and generate a readable test plan mapping every selected finding to behavior under test, source evidence, concrete proposed test location, fixture/input, expected failure before a fix, expected result after it and suitable assertion. Distinguish proposed paths from existing files. 2. Explain why each case detects the reported behavior; reject tests that merely mirror implementation or validate an unrelated schema. Record missing test infrastructure, nondeterminism and unsupported claims explicitly. Include validation lane, role and resource requirements suitable for the repository under review. 3. Run production generation and reader on predecessor output plus known findings; independently check useful behavior/fixture/assertion mapping. Reject missing evidence, incompatible runs, vague test placeholders and unmapped selected findings. 4. Prove generation changes no inspected source and creates no issue or test file in that repository. The product supplies a complete actionable plan; test implementation remains human-controlled. A canned packet or plan-only implementation is insufficient. acceptance: `finding_to_behavior_trace`, `concrete_test_plan_consumed`. pvf: `finding_to_behavior_trace`, `concrete_test_plan_consumed`, `implementation_mirroring_rejected`, `source_mutation_rejected`, `action_plan_schema`. stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `source_mutation`, `test_without_behavior_mapping`. non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `security_tournament`, `autonomous_fixing`. Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`. Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`."
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
  - "v0922-test-planner"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "acceptance: `finding_to_behavior_trace`, `concrete_test_plan_consumed`. pvf: `finding_to_behavior_trace`, `concrete_test_plan_consumed`, `implementation_mirroring_rejected`, `source_mutation_rejected`, `action_plan_schema`. stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `source_mutation`, `test_without_behavior_mapping`. non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `security_tournament`, `autonomous_fixing`. Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`. Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`. Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts. Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success. Live dependency read on 2026-09-16 confirms #892 (CF-SYNTHESIS) is closed/accepted; before bind, recheck active CodeFriend path ownership."
test_strategy:
  - "Declare each new test in a coupled proof inventory: deterministic local CPU contract/integration lane, isolated source/store and controlled clocks/transport, nonzero success and negative proof, bounded disk/process resources, required Beta feature and CF-INTEGRATE input gate. Provider-generated runs and browser/PDF visual review are separately identified execution/observation evidence, not deterministic guarantees. Run focused installed CLI and consumer tests plus required CI, then independent exact-head review. Redacted human diagnostics stay on stderr and machine output on stdout; test compatibility logging when exposed. Stop on unresolved owner/authority, inconsistent scope/provenance, failed or missing proof, source/credential exposure, stale approval or partial completion claimed as success. No unrelated feature work or customer-scale hosting. Planned focused commands after the selected new suite is implemented: `cargo test --manifest-path adl/Cargo.toml --test codefriend_testplan`; `cargo fmt --manifest-path adl/Cargo.toml --check`; focused touched CLI/admission-owner regressions selected after merged prerequisites. These do not replace actual installed generator/reader or publication admission execution. Re-resolve suite names after prerequisites land and update VPP for changes; never treat a zero-test filter as proof. `git diff --check` accompanies focused proof. Selected new test files do not exist yet; their commands are future proving work, not tests run during preparation. Install the issue candidate through the approved installer only at execution with current provenance; do not replace a shared binary during preparation."
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
notes: "The planned generator, installed CLI wiring, retained predecessor synthesis proof, negative validation, source-immutability proof, current-main reconciliation, and #893 sibling-scope cleanup are implemented. The plan remains a planning artifact; execution results are recorded in SOR. Fresh exact-head review, native publication, and required standard CI remain separate gates."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][CF-TESTPLAN] Generate a bounded test plan from review findings`.

With #892 accepted, consume the merged synthesis artifact contract and implement bounded test-plan generation that maps synthesized findings to behavior under test, proposed test location, fixture/input, expected failure before a fix, expected post-fix assertion, validation lane, and source-mutation non-goal. Before bind, recheck shared CodeFriend CLI/action paths and preserve #893/#895 sibling boundaries.

## PVF Lane Plan

- Initial PVF lane from issue creation: `runtime`
- Planned PVF lane for execution: `runtime`
- Planning lane source: `https://github.com/agent-logic/agent-design-language/issues/894 deterministic production consumer acceptance; docs/validation/pvf_lanes.json`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `unknown`
- Estimated total tokens: `unknown`
- Estimated validation seconds: `1200`
- Issue goal token budget: `unknown`
- Variance threshold percent: `10`
- Estimate confidence: `low`
- Estimate data source: `bounded focused validation estimate; implementation owner recalibrates after prerequisite source is available`
- Estimate source ref: `https://github.com/agent-logic/agent-design-language/issues/894`
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

1. Confirm dependency readiness and starting state: #892 (CF-SYNTHESIS) is closed/accepted as of live read on 2026-09-16; remaining pre-bind gate is active path ownership/collision recheck.
2. Review repo inputs and scoped surfaces before editing: Full live source contract, retained without dropping requirements: # [v0.92.2][CF-TESTPLAN] Generate a bounded test plan from review findings ## One complete result The test planner consumes synthesized findings and maps each selected finding to a concrete behavior, test location, fixture and expected failure without changing source. Dependencies: CF-SYNTHESIS. Consume accepted merged predecessor output; logical IDs receive canonical issue numbers during creation. Batch membership adds no all-to-all gate. ## Production ownership Implement `plan tests` behavior under the selected `adl codefriend` entrypoint. Proposed cohesive modules under `adl/src/codefriend/`: actions/test_plan.rs. The CLI registration is `adl/src/cli/codefriend_cmd.rs` (CF-SHELL owns operator-control additions); coordinate shared wiring with predecessor owners. Add focused `adl/tests/codefriend_testplan.rs` and bounded fixture outputs. Exact flags are documented/tested during implementation; command wording denotes the selected behavior, not an already installed command. ## Acceptance and executed evidence 1. Consume actual CF-SYNTHESIS output and generate a readable test plan mapping every selected finding to behavior under test, source evidence, concrete proposed test location, fixture/input, expected failure before a fix, expected result after it and suitable assertion. Distinguish proposed paths from existing files. 2. Explain why each case detects the reported behavior; reject tests that merely mirror implementation or validate an unrelated schema. Record missing test infrastructure, nondeterminism and unsupported claims explicitly. Include validation lane, role and resource requirements suitable for the repository under review. 3. Run production generation and reader on predecessor output plus known findings; independently check useful behavior/fixture/assertion mapping. Reject missing evidence, incompatible runs, vague test placeholders and unmapped selected findings. 4. Prove generation changes no inspected source and creates no issue or test file in that repository. The product supplies a complete actionable plan; test implementation remains human-controlled. A canned packet or plan-only implementation is insufficient. ## Shared execution boundary Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts. Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. No schema/scaffold/unused helper/test-only caller/zero-execution result can close it. No autonomous source changes, paid calls, external publication or cloud resources are authorized merely by creation. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success. ## PVF and completion Declare each new test in a coupled proof inventory: deterministic local CPU contract/integration lane, isolated source/store and controlled clocks/transport, nonzero success and negative proof, bounded disk/process resources, required Beta feature and CF-INTEGRATE input gate. Provider-generated runs and browser/PDF visual review are separately identified execution/observation evidence, not deterministic guarantees. Run focused installed CLI and consumer tests plus required CI, then independent exact-head review. Redacted human diagnostics stay on stderr and machine output on stdout; test compatibility logging when exposed. Stop on unresolved owner/authority, inconsistent scope/provenance, failed or missing proof, source/credential exposure, stale approval or partial completion claimed as success. No unrelated feature work or customer-scale hosting. ## Inherited obligation ledger acceptance: `finding_to_behavior_trace`, `concrete_test_plan_consumed`. pvf: `finding_to_behavior_trace`, `concrete_test_plan_consumed`, `implementation_mirroring_rejected`, `source_mutation_rejected`, `action_plan_schema`. stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `source_mutation`, `test_without_behavior_mapping`. non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `security_tournament`, `autonomous_fixing`. Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`. Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`. ## Canonical execution links Planning owner: #864. Creation/review batch: 4; this grouping adds no execution gate. Execution prerequisite: #892 (CF-SYNTHESIS); accepted output is required before dependent execution. Reviewed creation source: `54e5d8e100f39c644ca0aa03e3985ce66beb529e`. This issue records a complete task; creation does not claim execution or acceptance. <!-- csdlc-v3-operation:982d5a8ecfa8cf361e7bec1c51dbacacba009f8d326e7c9d710f883eeac809d9 -->
3. Implement only the bounded deliverables: Implement `plan tests` behavior under the selected `adl codefriend` entrypoint. Proposed cohesive modules under `adl/src/codefriend/`: actions/test_plan.rs. The CLI registration is `adl/src/cli/codefriend_cmd.rs` (CF-SHELL owns operator-control additions); coordinate shared wiring with predecessor owners. Add focused `adl/tests/codefriend_testplan.rs` and bounded fixture outputs. Exact flags are documented/tested during implementation; command wording denotes the selected behavior, not an already installed command. The test planner consumes synthesized findings and maps each selected finding to a concrete behavior, test location, fixture and expected failure without changing source. Dependencies: CF-SYNTHESIS. Consume accepted merged predecessor output; logical IDs receive canonical issue numbers during creation. Batch membership adds no all-to-all gate.
4. Run focused proof gates for acceptance: 1. Consume actual CF-SYNTHESIS output and generate a readable test plan mapping every selected finding to behavior under test, source evidence, concrete proposed test location, fixture/input, expected failure before a fix, expected result after it and suitable assertion. Distinguish proposed paths from existing files. 2. Explain why each case detects the reported behavior; reject tests that merely mirror implementation or validate an unrelated schema. Record missing test infrastructure, nondeterminism and unsupported claims explicitly. Include validation lane, role and resource requirements suitable for the repository under review. 3. Run production generation and reader on predecessor output plus known findings; independently check useful behavior/fixture/assertion mapping. Reject missing evidence, incompatible runs, vague test placeholders and unmapped selected findings. 4. Prove generation changes no inspected source and creates no issue or test file in that repository. The product supplies a complete actionable plan; test implementation remains human-controlled. A canned packet or plan-only implementation is insufficient. acceptance: `finding_to_behavior_trace`, `concrete_test_plan_consumed`. pvf: `finding_to_behavior_trace`, `concrete_test_plan_consumed`, `implementation_mirroring_rejected`, `source_mutation_rejected`, `action_plan_schema`. stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `source_mutation`, `test_without_behavior_mapping`. non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `security_tournament`, `autonomous_fixing`. Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`. Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- v0922-test-planner

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- acceptance: `finding_to_behavior_trace`, `concrete_test_plan_consumed`. pvf: `finding_to_behavior_trace`, `concrete_test_plan_consumed`, `implementation_mirroring_rejected`, `source_mutation_rejected`, `action_plan_schema`. stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `source_mutation`, `test_without_behavior_mapping`. non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `security_tournament`, `autonomous_fixing`. Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`. Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`. Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts. Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success. Live dependency read on 2026-09-16 confirms #892 (CF-SYNTHESIS) is closed/accepted; before bind, recheck active CodeFriend path ownership.

## Test Strategy

- Declare each new test in a coupled proof inventory: deterministic local CPU contract/integration lane, isolated source/store and controlled clocks/transport, nonzero success and negative proof, bounded disk/process resources, required Beta feature and CF-INTEGRATE input gate. Provider-generated runs and browser/PDF visual review are separately identified execution/observation evidence, not deterministic guarantees. Run focused installed CLI and consumer tests plus required CI, then independent exact-head review. Redacted human diagnostics stay on stderr and machine output on stdout; test compatibility logging when exposed. Stop on unresolved owner/authority, inconsistent scope/provenance, failed or missing proof, source/credential exposure, stale approval or partial completion claimed as success. No unrelated feature work or customer-scale hosting. Planned focused commands after the selected new suite is implemented: `cargo test --manifest-path adl/Cargo.toml --test codefriend_testplan`; `cargo fmt --manifest-path adl/Cargo.toml --check`; focused touched CLI/admission-owner regressions selected after merged prerequisites. These do not replace actual installed generator/reader or publication admission execution. Re-resolve suite names after prerequisites land and update VPP for changes; never treat a zero-test filter as proof. `git diff --check` accompanies focused proof. Selected new test files do not exist yet; their commands are future proving work, not tests run during preparation. Install the issue candidate through the approved installer only at execution with current provenance; do not replace a shared binary during preparation.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

The planned generator, installed CLI wiring, retained predecessor synthesis proof, negative validation, source-immutability proof, current-main reconciliation, and #893 sibling-scope cleanup are implemented. The plan remains a planning artifact; execution results are recorded in SOR. Fresh exact-head review, native publication, and required standard CI remain separate gates.
