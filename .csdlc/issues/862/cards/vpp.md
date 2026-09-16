---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "v0922-local-command-decomposition-validation-plan"
issue: 862
task_id: "issue-0862"
run_id: "issue-0862"
version: "0.92.2"
title: "[v0.92.2][C-SDLC v3][refactor] Decompose the local command owner"
branch: "codex/862-v0922-local-command-decomposition"
generated_at: "2026-09-16T00:27:06.103043+00:00"
card_status: "ready"
status: "executed_pre_review"
initial_pvf_lane: "tooling"
planned_pvf_lane: "tooling"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "bounded_local"
validation_resource_profile: "Isolated local CPU/Rust/Git/filesystem, controlled clocks and fake authenticated remote transport; no live GitHub mutation/provider/cloud execution"
validation_family: "csdlc_local_decomposition"
validation_size_split: "Focused named route/transaction and golden-contract tests; required CI separately observed"
expected_proof_cost: "Low-confidence estimate 1800 local validation seconds; excludes cold build, CI queue and independent review"
planned_validation_seconds: "1800"
planned_validation_tokens: "9000"
issue_goal_ref: "active Codex goal for Sprint #933 / issue #862"
sprint_goal_ref: "v0.92.2 Sprint 7 coordination issue #933; descriptive only"
goal_metrics_rollup_ref: ".csdlc/evidence/862/goal-metrics.json (planned, absent until execution)"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/862"
  - kind: "stp"
    ref: ".csdlc/issues/862/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/862/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/862/cards/spp.md"
selected_lanes:
  - "tooling: 151 focused tests passed across decomposition, local routes, operational CLI, transactions, foundation, binding and terminal boundaries; strict all-target clippy and formatting passed"
parallel_groups:
  - "Serialize native issue preparation; future isolated deterministic fixtures may parallelize only after ownership/resource review"
validation_commands:
  - "cargo test --manifest-path csdlc-v3/Cargo.toml --test local_module_decomposition --test local_commands --test operational_cli_commands --test transactions --test foundation --test proof_worktree_binding --test terminal_cleanup_cutover_commands; cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings; cargo fmt --manifest-path csdlc-v3/Cargo.toml --check; git diff --check"
failure_policy: "Missing, skipped, zero-scenario or failed required proof blocks acceptance. Replan absent/renamed tests explicitly. No incomplete route migration, replacement god module, facade-only size claim, serialized/public drift or weakened guard. Keep machine-readable stdout and redacted stderr; preserve failed evidence and refresh affected exact-head review."
notes: "#864 WP-01 is accepted via merged PR #865 and the all-69 creation/review gate is satisfied. #849 is accepted via merged PR #952. #862 has no dependency on unfinished SIM-06 through SIM-09 or on an entire earlier sprint. Live ownership reconciliation on 2026-09-15 found the former #867 conflict obsolete: #867 through #871 are closed with merged PRs. The current #872 worktree changes only issue-872 rehearsal scripts and csdlc-v3/tests/copied_record_conversion_rehearsal.rs; it does not modify csdlc-v3/src/commands/local/mod.rs or the #862 focused test owners. No branch, worktree, or PR exists for #862 before this bind. Bind is authorized for execution readiness only. Implementation, validation execution, review, publication, merge, stable binary replacement, and live conversion remain unstarted and unauthorized by this preparation. Recheck live ownership before implementation and coordinate any later shared test or installed-binary write. Sprint 7 coordination is #933; #926 is historical umbrella-creation management, not a child dependency."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Verify accepted #864 baseline and reconcile SIM/local-owner and installed-writer coordination; record recursive source/responsibility and route/serialized-contract inventories; extract binding, editing, validation, registration, transaction and state-persistence responsibilities into cohesive local modules with thin stable dispatch; run each affected production route and authority/idempotency/digest/crash/error golden negatives against isolated state; independently check dependency direction and complete route migration before current review and native publication.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `tooling`
- Planned PVF lane for execution: `tooling`

## Selected Validation Lanes

- tooling: 151 focused tests passed across decomposition, local routes, operational CLI, transactions, foundation, binding and terminal boundaries; strict all-target clippy and formatting passed

## Parallelization Plan

- Parallel groups: Serialize native issue preparation; future isolated deterministic fixtures may parallelize only after ownership/resource review
- Validation runtime class: `bounded_local`
- Validation resource profile: `Isolated local CPU/Rust/Git/filesystem, controlled clocks and fake authenticated remote transport; no live GitHub mutation/provider/cloud execution`
- Validation family: `csdlc_local_decomposition`
- Validation size split: `Focused named route/transaction and golden-contract tests; required CI separately observed`

## Goal Accounting Hooks

- Issue goal ref: `active Codex goal for Sprint #933 / issue #862`
- Sprint goal ref: `v0.92.2 Sprint 7 coordination issue #933; descriptive only`
- Goal metrics rollup ref: `.csdlc/evidence/862/goal-metrics.json (planned, absent until execution)`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Low-confidence estimate 1800 local validation seconds; excludes cold build, CI queue and independent review`
- Planned validation seconds: `1800`
- Planned validation token budget: `9000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- cargo test --manifest-path csdlc-v3/Cargo.toml --test local_module_decomposition --test local_commands --test operational_cli_commands --test transactions --test foundation --test proof_worktree_binding --test terminal_cleanup_cutover_commands; cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings; cargo fmt --manifest-path csdlc-v3/Cargo.toml --check; git diff --check

## Failure Semantics

- Missing, skipped, zero-scenario or failed required proof blocks acceptance. Replan absent/renamed tests explicitly. No incomplete route migration, replacement god module, facade-only size claim, serialized/public drift or weakened guard. Keep machine-readable stdout and redacted stderr; preserve failed evidence and refresh affected exact-head review.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

#864 WP-01 is accepted via merged PR #865 and the all-69 creation/review gate is satisfied. #849 is accepted via merged PR #952. #862 has no dependency on unfinished SIM-06 through SIM-09 or on an entire earlier sprint. Live ownership reconciliation on 2026-09-15 found the former #867 conflict obsolete: #867 through #871 are closed with merged PRs. The current #872 worktree changes only issue-872 rehearsal scripts and csdlc-v3/tests/copied_record_conversion_rehearsal.rs; it does not modify csdlc-v3/src/commands/local/mod.rs or the #862 focused test owners. No branch, worktree, or PR exists for #862 before this bind. Bind is authorized for execution readiness only. Implementation, validation execution, review, publication, merge, stable binary replacement, and live conversion remain unstarted and unauthorized by this preparation. Recheck live ownership before implementation and coordinate any later shared test or installed-binary write. Sprint 7 coordination is #933; #926 is historical umbrella-creation management, not a child dependency.
