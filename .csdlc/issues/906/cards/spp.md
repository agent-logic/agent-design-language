---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "v0922-process-parser-simplification-execution-plan"
issue: 906
task_id: "issue-0906"
run_id: "issue-0906"
version: "0.92.2"
title: "[v0.92.2][PLAT-RUST] Complete one selected production Rust responsibility refactor"
branch: "codex/906-v0922-process-parser-simplification"
generated_at: "2026-09-12T00:22:05.416044+00:00"
card_status: "ready"
status: "in_progress"
activation_state: "bound"
plan_revision: 1
initial_pvf_lane: "tooling"
planned_pvf_lane: "tooling"
planned_pvf_lane_source: "https://github.com/agent-logic/agent-design-language/issues/906 validation contract; docs/validation/pvf_lanes.json"
estimate_elapsed_seconds: "14400"
estimate_total_tokens: "35000"
estimate_validation_seconds: "2400"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "10"
estimate_confidence: "low"
estimate_data_source: "Conservative planning estimate for bounded Rust implementation plus deterministic installed-consumer fixtures, assuming warm cache; recalibrate against accepted upstream and focused-regression availability; not an imposed token limit"
estimate_source_ref: "https://github.com/agent-logic/agent-design-language/issues/906"
issue_goal_ref: "Active issue #906 implementation, review, publication, and green CI goal"
sprint_goal_ref: "v0.92.2 execution Sprint 7; umbrella management owned by #926"
goal_metrics_rollup_ref: ".csdlc/evidence/906/goal-metrics.json (planned; absent until execution)"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/906"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/906"
  - kind: "stp"
    ref: ".csdlc/issues/906/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/906/cards/sip.md"
scope:
  files:
    - "Select process-status argument parsing/validation from `adl/src/cli/process_cmd.rs`: `ParsedStatus`, `parse_status_args`, `take_value`, `parse_pid`, `parse_port`, `validate_loopback_host`. The real caller is `real_process_status`. Move this coherent responsibility to proposed `adl/src/cli/process_cmd/args.rs` while simplifying redundant target-selection representation and control flow. Preserve the rest of process probing/output. The inspected baseline has 482 lines and four separate optional targets followed by counting and a repeated selection chain ending in `expect`. Record exact baseline hash and recursive source/function/branch inventory before editing. A registered worktree ending `.worktrees/adl-process-status-fanout` has a dirty modification to this exact file. Preserve those bytes, identify its owner and resolve overlap before any implementation. Never take over, reset, cherry-pick or assume abandoned work from this issue's creation. Selection is concrete; execution ownership remains a prerequisite."
  components:
    - "v0922-process-parser-simplification"
  out_of_scope:
    - "acceptance: `bounded_surface`, `behavior_preserved`, `measurable_reduction`, `exact_module_and_invariant_selected_before_creation`, `production_callers_preserved`, `recursive_before_after_inventory`. pvf: `focused_regression`, `before_after_measurement`, `exact_module_and_invariant_selected_before_creation`, `production_callers_preserved`, `recursive_before_after_inventory`, `unselected_surface_rejected`, `facade_only_reduction_claim_rejected`. stop_conditions: `scope_sprawl`, `behavior_redesign`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `production_responsibility_unselected`. non_goals: `repo_wide_rewrite`. Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Resolve dirty process-status-fanout source and CLI-test ownership without copying/resetting inherited work; freeze exact baseline plus recursive source/function/branch inventory and argv behavior corpus; extract ParsedStatus and pure parsing/validation helpers into process_cmd/args.rs and simplify duplicate target-selection representation while retaining production real_process_status caller; prove exact accepted/rejected flags, repeats/order/error order/defaults/zero bounds/loopback rules with parser and installed CLI regressions; compare recursive totals and control-flow simplification, preserve probe/output schema and supported-platform behavior, then independently review exact head."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: #864 WP-01 prerequisite delivered by merged PR #865; all69 identity/creation reviews completed. Recheck accepted evidence and current path ownership before bind; no dependency on entire earlier sprints is added."
    expected_output: ".csdlc/issues/906/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Complete source contract (live issue snapshot; preserve all requirements): # [v0.92.2][PLAT-RUST] Complete one selected production Rust responsibility refactor ## One complete result One selected production Rust responsibility is fully extracted or simplified while preserving observable behavior, with recursive source accounting and focused regressions. Dependencies: WP-01. Numeric identities are attached during native creation. ## Exact responsibility and ownership Select process-status argument parsing/validation from `adl/src/cli/process_cmd.rs`: `ParsedStatus`, `parse_status_args`, `take_value`, `parse_pid`, `parse_port`, `validate_loopback_host`. The real caller is `real_process_status`. Move this coherent responsibility to proposed `adl/src/cli/process_cmd/args.rs` while simplifying redundant target-selection representation and control flow. Preserve the rest of process probing/output. The inspected baseline has 482 lines and four separate optional targets followed by counting and a repeated selection chain ending in `expect`. Record exact baseline hash and recursive source/function/branch inventory before editing. A registered worktree ending `.worktrees/adl-process-status-fanout` has a dirty modification to this exact file. Preserve those bytes, identify its owner and resolve overlap before any implementation. Never take over, reset, cherry-pick or assume abandoned work from this issue's creation. Selection is concrete; execution ownership remains a prerequisite. ## Executed acceptance 1. Deliver the complete pure parser used by production `real_process_status`, preserving accepted/rejected argv, repeated options and option order, error text/order, defaults, mutually exclusive target semantics, zero PID/port rejection and loopback-only host rules. Keep syscall/network probes and output schema unchanged. 2. Reduce duplicated target-selection logic/control-flow complexity with an explicit before/after measure and reviewed rationale. Count recursive source totals and explain additions. A smaller parent facade, moved lines or more tests alone is not reduction; no arbitrary net-line claim substitutes for the actual simplification. 3. Exercise installed `adl process status` via existing `adl/tests/cli_smoke/process_status.rs` and focused parser cases, including malformed/missing/duplicate/conflicting flags and boundary values. Test safety denials without broad process scans or unsafe network targets. Preserve existing public outputs on all supported platforms. 4. Focused regressions, diff/format checks and required CI pass at independent exact-head review. No design-only refactor proposal, unused helper or compilation-only evidence closes the issue. Update narrowly affected process-status documentation if necessary. PVF: deterministic local parser/CLI behavior-preservation contract, small CPU and controlled local process fixtures, required milestone support gate. No broad Rust rewrite, process-control feature changes, unrelated cleanup or other-owner takeover. Stop on unresolved dirty ownership, behavior drift, measurement without simplification or missing/failed proof. ## Global startup and proof boundary All 69 milestone issues must be created and reviewed before any implementation begins, as directed by the operator. Creation/review batches add no execution dependencies beyond that global startup gate and each task's declared prerequisites. Use native C-SDLC v3, current authority/readiness, a bound FastWork worktree and issue-bound goal. Reconcile actual owners before shared-path edits. Preserve repository/provider credentials and inspected source; stdout carries machine output and stderr redacted human diagnostics, including tested compatibility logging when relevant. Deliver one complete result with required tests, failure handling and documentation; no schema/scaffold/unused library/zero-test or authored-packet-only completion. Classify every new proof case by lane, role, determinism, resources and release gate; distinguish local fixture proof, actual hardware/provider runs and required CI. Required proof not executed remains not proved, even when issue creation succeeds. No implementation, paid execution, activation or remote mutation is performed by this issue-creation batch. ## Inherited obligation ledger acceptance: `bounded_surface`, `behavior_preserved`, `measurable_reduction`, `exact_module_and_invariant_selected_before_creation`, `production_callers_preserved`, `recursive_before_after_inventory`. pvf: `focused_regression`, `before_after_measurement`, `exact_module_and_invariant_selected_before_creation`, `production_callers_preserved`, `recursive_before_after_inventory`, `unselected_surface_rejected`, `facade_only_reduction_claim_rejected`. stop_conditions: `scope_sprawl`, `behavior_redesign`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `production_responsibility_unselected`. non_goals: `repo_wide_rewrite`. Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`. ## Canonical execution links Planning owner: #864. Creation/review batch: 7; this grouping adds no execution gate. Execution prerequisite: #864 (WP-01); accepted output is required before dependent execution. Reviewed creation source: `d955fd1bdbe7f79433dbcf1ea8426536ec8274a7`. This issue records a complete task; creation does not claim execution or acceptance."
    expected_output: ".csdlc/issues/906/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Select process-status argument parsing/validation from `adl/src/cli/process_cmd.rs`: `ParsedStatus`, `parse_status_args`, `take_value`, `parse_pid`, `parse_port`, `validate_loopback_host`. The real caller is `real_process_status`. Move this coherent responsibility to proposed `adl/src/cli/process_cmd/args.rs` while simplifying redundant target-selection representation and control flow. Preserve the rest of process probing/output. The inspected baseline has 482 lines and four separate optional targets followed by counting and a repeated selection chain ending in `expect`. Record exact baseline hash and recursive source/function/branch inventory before editing. A registered worktree ending `.worktrees/adl-process-status-fanout` has a dirty modification to this exact file. Preserve those bytes, identify its owner and resolve overlap before any implementation. Never take over, reset, cherry-pick or assume abandoned work from this issue's creation. Selection is concrete; execution ownership remains a prerequisite. One selected production Rust responsibility is fully extracted or simplified while preserving observable behavior, with recursive source accounting and focused regressions. Dependencies: WP-01. Numeric identities are attached during native creation."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: 1. Deliver the complete pure parser used by production `real_process_status`, preserving accepted/rejected argv, repeated options and option order, error text/order, defaults, mutually exclusive target semantics, zero PID/port rejection and loopback-only host rules. Keep syscall/network probes and output schema unchanged. 2. Reduce duplicated target-selection logic/control-flow complexity with an explicit before/after measure and reviewed rationale. Count recursive source totals and explain additions. A smaller parent facade, moved lines or more tests alone is not reduction; no arbitrary net-line claim substitutes for the actual simplification. 3. Exercise installed `adl process status` via existing `adl/tests/cli_smoke/process_status.rs` and focused parser cases, including malformed/missing/duplicate/conflicting flags and boundary values. Test safety denials without broad process scans or unsafe network targets. Preserve existing public outputs on all supported platforms. 4. Focused regressions, diff/format checks and required CI pass at independent exact-head review. No design-only refactor proposal, unused helper or compilation-only evidence closes the issue. Update narrowly affected process-status documentation if necessary. PVF: deterministic local parser/CLI behavior-preservation contract, small CPU and controlled local process fixtures, required milestone support gate. No broad Rust rewrite, process-control feature changes, unrelated cleanup or other-owner takeover. Stop on unresolved dirty ownership, behavior drift, measurement without simplification or missing/failed proof."
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
  - "v0922-process-parser-simplification"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "acceptance: `bounded_surface`, `behavior_preserved`, `measurable_reduction`, `exact_module_and_invariant_selected_before_creation`, `production_callers_preserved`, `recursive_before_after_inventory`. pvf: `focused_regression`, `before_after_measurement`, `exact_module_and_invariant_selected_before_creation`, `production_callers_preserved`, `recursive_before_after_inventory`, `unselected_surface_rejected`, `facade_only_reduction_claim_rejected`. stop_conditions: `scope_sprawl`, `behavior_redesign`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `production_responsibility_unselected`. non_goals: `repo_wide_rewrite`. Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`. #864 WP-01 prerequisite delivered by merged PR #865; all69 identity/creation reviews completed. Recheck accepted evidence and current path ownership before bind; no dependency on entire earlier sprints is added."
test_strategy:
  - "Required tooling lane, deterministic pure-parser/installed CLI behavior preservation with small CPU and isolated owned process fixtures; no broad process scan or unsafe network targets. First enumerate cargo test --manifest-path adl/Cargo.toml --test cli_smoke -- --list; run cargo test --manifest-path adl/Cargo.toml --test cli_smoke process_status plus newly authored parser cases and require nonzero denominator. Record exact source baseline and recursive source/function/branch counts before and after, independently assess removal of duplicated target selection rather than facade shrink. Cover missing/malformed/repeated/conflicting flags, option order and error order, PID/port zero/bounds, loopback-only host, unchanged defaults and output schema. Run cargo fmt --manifest-path adl/Cargo.toml --check and git diff --check and required platform CI separately. New fixture inventory must declare role/determinism/resources/release gate. Compilation, moved lines or additional tests alone do not establish simplification."
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
notes: "Ownership resolution 2026-09-16: historical worktree /Users/daniel/git/agent-design-language/.worktrees/adl-process-status-fanout on branch codex/reduce-process-status-fanout is operator-owned June 19 WIP at 1ea914010e6b96482a95bd6f64c8318f1a19b937. Its four-file dirty patch has SHA-256 aef64c67a22d74ff5dc4962d5c6f25ab09a06ca64bb1763c48984d73558fbf84; no origin or legacy-origin branch or PR exists. Preserve every byte there. Issue #906 will not copy, reset, cherry-pick, or modify that worktree and will execute only in its separate bound FastWork worktree from current origin/main. The source and CLI-test paths are clear through isolation; the unrelated finish files remain untouched."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][PLAT-RUST] Complete one selected production Rust responsibility refactor`.

Resolve dirty process-status-fanout source and CLI-test ownership without copying/resetting inherited work; freeze exact baseline plus recursive source/function/branch inventory and argv behavior corpus; extract ParsedStatus and pure parsing/validation helpers into process_cmd/args.rs and simplify duplicate target-selection representation while retaining production real_process_status caller; prove exact accepted/rejected flags, repeats/order/error order/defaults/zero bounds/loopback rules with parser and installed CLI regressions; compare recursive totals and control-flow simplification, preserve probe/output schema and supported-platform behavior, then independently review exact head.

## PVF Lane Plan

- Initial PVF lane from issue creation: `tooling`
- Planned PVF lane for execution: `tooling`
- Planning lane source: `https://github.com/agent-logic/agent-design-language/issues/906 validation contract; docs/validation/pvf_lanes.json`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `14400`
- Estimated total tokens: `35000`
- Estimated validation seconds: `2400`
- Issue goal token budget: `unknown`
- Variance threshold percent: `10`
- Estimate confidence: `low`
- Estimate data source: `Conservative planning estimate for bounded Rust implementation plus deterministic installed-consumer fixtures, assuming warm cache; recalibrate against accepted upstream and focused-regression availability; not an imposed token limit`
- Estimate source ref: `https://github.com/agent-logic/agent-design-language/issues/906`
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

1. Confirm dependency readiness and starting state: #864 WP-01 prerequisite delivered by merged PR #865; all69 identity/creation reviews completed. Recheck accepted evidence and current path ownership before bind; no dependency on entire earlier sprints is added.
2. Review repo inputs and scoped surfaces before editing: Complete source contract (live issue snapshot; preserve all requirements): # [v0.92.2][PLAT-RUST] Complete one selected production Rust responsibility refactor ## One complete result One selected production Rust responsibility is fully extracted or simplified while preserving observable behavior, with recursive source accounting and focused regressions. Dependencies: WP-01. Numeric identities are attached during native creation. ## Exact responsibility and ownership Select process-status argument parsing/validation from `adl/src/cli/process_cmd.rs`: `ParsedStatus`, `parse_status_args`, `take_value`, `parse_pid`, `parse_port`, `validate_loopback_host`. The real caller is `real_process_status`. Move this coherent responsibility to proposed `adl/src/cli/process_cmd/args.rs` while simplifying redundant target-selection representation and control flow. Preserve the rest of process probing/output. The inspected baseline has 482 lines and four separate optional targets followed by counting and a repeated selection chain ending in `expect`. Record exact baseline hash and recursive source/function/branch inventory before editing. A registered worktree ending `.worktrees/adl-process-status-fanout` has a dirty modification to this exact file. Preserve those bytes, identify its owner and resolve overlap before any implementation. Never take over, reset, cherry-pick or assume abandoned work from this issue's creation. Selection is concrete; execution ownership remains a prerequisite. ## Executed acceptance 1. Deliver the complete pure parser used by production `real_process_status`, preserving accepted/rejected argv, repeated options and option order, error text/order, defaults, mutually exclusive target semantics, zero PID/port rejection and loopback-only host rules. Keep syscall/network probes and output schema unchanged. 2. Reduce duplicated target-selection logic/control-flow complexity with an explicit before/after measure and reviewed rationale. Count recursive source totals and explain additions. A smaller parent facade, moved lines or more tests alone is not reduction; no arbitrary net-line claim substitutes for the actual simplification. 3. Exercise installed `adl process status` via existing `adl/tests/cli_smoke/process_status.rs` and focused parser cases, including malformed/missing/duplicate/conflicting flags and boundary values. Test safety denials without broad process scans or unsafe network targets. Preserve existing public outputs on all supported platforms. 4. Focused regressions, diff/format checks and required CI pass at independent exact-head review. No design-only refactor proposal, unused helper or compilation-only evidence closes the issue. Update narrowly affected process-status documentation if necessary. PVF: deterministic local parser/CLI behavior-preservation contract, small CPU and controlled local process fixtures, required milestone support gate. No broad Rust rewrite, process-control feature changes, unrelated cleanup or other-owner takeover. Stop on unresolved dirty ownership, behavior drift, measurement without simplification or missing/failed proof. ## Global startup and proof boundary All 69 milestone issues must be created and reviewed before any implementation begins, as directed by the operator. Creation/review batches add no execution dependencies beyond that global startup gate and each task's declared prerequisites. Use native C-SDLC v3, current authority/readiness, a bound FastWork worktree and issue-bound goal. Reconcile actual owners before shared-path edits. Preserve repository/provider credentials and inspected source; stdout carries machine output and stderr redacted human diagnostics, including tested compatibility logging when relevant. Deliver one complete result with required tests, failure handling and documentation; no schema/scaffold/unused library/zero-test or authored-packet-only completion. Classify every new proof case by lane, role, determinism, resources and release gate; distinguish local fixture proof, actual hardware/provider runs and required CI. Required proof not executed remains not proved, even when issue creation succeeds. No implementation, paid execution, activation or remote mutation is performed by this issue-creation batch. ## Inherited obligation ledger acceptance: `bounded_surface`, `behavior_preserved`, `measurable_reduction`, `exact_module_and_invariant_selected_before_creation`, `production_callers_preserved`, `recursive_before_after_inventory`. pvf: `focused_regression`, `before_after_measurement`, `exact_module_and_invariant_selected_before_creation`, `production_callers_preserved`, `recursive_before_after_inventory`, `unselected_surface_rejected`, `facade_only_reduction_claim_rejected`. stop_conditions: `scope_sprawl`, `behavior_redesign`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `production_responsibility_unselected`. non_goals: `repo_wide_rewrite`. Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`. ## Canonical execution links Planning owner: #864. Creation/review batch: 7; this grouping adds no execution gate. Execution prerequisite: #864 (WP-01); accepted output is required before dependent execution. Reviewed creation source: `d955fd1bdbe7f79433dbcf1ea8426536ec8274a7`. This issue records a complete task; creation does not claim execution or acceptance.
3. Implement only the bounded deliverables: Select process-status argument parsing/validation from `adl/src/cli/process_cmd.rs`: `ParsedStatus`, `parse_status_args`, `take_value`, `parse_pid`, `parse_port`, `validate_loopback_host`. The real caller is `real_process_status`. Move this coherent responsibility to proposed `adl/src/cli/process_cmd/args.rs` while simplifying redundant target-selection representation and control flow. Preserve the rest of process probing/output. The inspected baseline has 482 lines and four separate optional targets followed by counting and a repeated selection chain ending in `expect`. Record exact baseline hash and recursive source/function/branch inventory before editing. A registered worktree ending `.worktrees/adl-process-status-fanout` has a dirty modification to this exact file. Preserve those bytes, identify its owner and resolve overlap before any implementation. Never take over, reset, cherry-pick or assume abandoned work from this issue's creation. Selection is concrete; execution ownership remains a prerequisite. One selected production Rust responsibility is fully extracted or simplified while preserving observable behavior, with recursive source accounting and focused regressions. Dependencies: WP-01. Numeric identities are attached during native creation.
4. Run focused proof gates for acceptance: 1. Deliver the complete pure parser used by production `real_process_status`, preserving accepted/rejected argv, repeated options and option order, error text/order, defaults, mutually exclusive target semantics, zero PID/port rejection and loopback-only host rules. Keep syscall/network probes and output schema unchanged. 2. Reduce duplicated target-selection logic/control-flow complexity with an explicit before/after measure and reviewed rationale. Count recursive source totals and explain additions. A smaller parent facade, moved lines or more tests alone is not reduction; no arbitrary net-line claim substitutes for the actual simplification. 3. Exercise installed `adl process status` via existing `adl/tests/cli_smoke/process_status.rs` and focused parser cases, including malformed/missing/duplicate/conflicting flags and boundary values. Test safety denials without broad process scans or unsafe network targets. Preserve existing public outputs on all supported platforms. 4. Focused regressions, diff/format checks and required CI pass at independent exact-head review. No design-only refactor proposal, unused helper or compilation-only evidence closes the issue. Update narrowly affected process-status documentation if necessary. PVF: deterministic local parser/CLI behavior-preservation contract, small CPU and controlled local process fixtures, required milestone support gate. No broad Rust rewrite, process-control feature changes, unrelated cleanup or other-owner takeover. Stop on unresolved dirty ownership, behavior drift, measurement without simplification or missing/failed proof.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- v0922-process-parser-simplification

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- acceptance: `bounded_surface`, `behavior_preserved`, `measurable_reduction`, `exact_module_and_invariant_selected_before_creation`, `production_callers_preserved`, `recursive_before_after_inventory`. pvf: `focused_regression`, `before_after_measurement`, `exact_module_and_invariant_selected_before_creation`, `production_callers_preserved`, `recursive_before_after_inventory`, `unselected_surface_rejected`, `facade_only_reduction_claim_rejected`. stop_conditions: `scope_sprawl`, `behavior_redesign`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `production_responsibility_unselected`. non_goals: `repo_wide_rewrite`. Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`. #864 WP-01 prerequisite delivered by merged PR #865; all69 identity/creation reviews completed. Recheck accepted evidence and current path ownership before bind; no dependency on entire earlier sprints is added.

## Test Strategy

- Required tooling lane, deterministic pure-parser/installed CLI behavior preservation with small CPU and isolated owned process fixtures; no broad process scan or unsafe network targets. First enumerate cargo test --manifest-path adl/Cargo.toml --test cli_smoke -- --list; run cargo test --manifest-path adl/Cargo.toml --test cli_smoke process_status plus newly authored parser cases and require nonzero denominator. Record exact source baseline and recursive source/function/branch counts before and after, independently assess removal of duplicated target selection rather than facade shrink. Cover missing/malformed/repeated/conflicting flags, option order and error order, PID/port zero/bounds, loopback-only host, unchanged defaults and output schema. Run cargo fmt --manifest-path adl/Cargo.toml --check and git diff --check and required platform CI separately. New fixture inventory must declare role/determinism/resources/release gate. Compilation, moved lines or additional tests alone do not establish simplification.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Ownership resolution 2026-09-16: historical worktree /Users/daniel/git/agent-design-language/.worktrees/adl-process-status-fanout on branch codex/reduce-process-status-fanout is operator-owned June 19 WIP at 1ea914010e6b96482a95bd6f64c8318f1a19b937. Its four-file dirty patch has SHA-256 aef64c67a22d74ff5dc4962d5c6f25ab09a06ca64bb1763c48984d73558fbf84; no origin or legacy-origin branch or PR exists. Preserve every byte there. Issue #906 will not copy, reset, cherry-pick, or modify that worktree and will execute only in its separate bound FastWork worktree from current origin/main. The source and CLI-test paths are clear through isolation; the unrelated finish files remain untouched.
