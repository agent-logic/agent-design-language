---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "v0922-remediation-planner-validation-plan"
issue: 893
task_id: "issue-0893"
run_id: "issue-0893"
version: "0.92.2"
title: "[v0.92.2][CF-REMEDIATE] Generate a bounded remediation plan from review findings"
branch: "codex/893-v0922-remediation-planner"
generated_at: "2026-09-12T00:10:02.687479+00:00"
card_status: "ready"
status: "executed_pending_review"
initial_pvf_lane: "runtime"
planned_pvf_lane: "runtime"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "bounded_local"
validation_resource_profile: "Isolated local CPU/Rust/filesystem, controlled observation time, bounded admitted fixtures; no network/provider/cloud execution"
validation_family: "codefriend_action_plans"
validation_size_split: "Focused named tests and touched-owner regressions; macOS/Linux CI separately observed"
expected_proof_cost: "Low-confidence estimate 1200 local validation seconds; excludes cold build, CI queue and independent review"
planned_validation_seconds: "1200"
planned_validation_tokens: "7000"
issue_goal_ref: "Sprint 4 #930 active goal covers #893 execution in this session; single goal slot prevented replacing it with a separate child goal."
sprint_goal_ref: "v0.92.2 execution Sprint 4; umbrella management owned by #926"
goal_metrics_rollup_ref: ".csdlc/evidence/893/goal-metrics.json (planned, absent until execution)"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/893"
  - kind: "stp"
    ref: ".csdlc/issues/893/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/893/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/893/cards/spp.md"
selected_lanes:
  - "runtime; deterministic semantic correctness plus installed consumer integration; focused local proof passed for `codefriend_remediate` 5/5, `cargo fmt --check`, and `git diff --check`; CI observation, publication checks, and fresh exact-head review remain pending"
parallel_groups:
  - "Serialize native issue preparation; future isolated deterministic fixtures may parallelize only after ownership/resource review"
validation_commands:
  - "`cargo fmt --manifest-path adl/Cargo.toml --check` passed. `cargo test --manifest-path adl/Cargo.toml --test codefriend_remediate` passed 5/5 after adding the path-extraction regression. `cargo test --manifest-path adl/Cargo.toml --test codefriend_synthesis` previously passed 5/5 and was not rerun for the path-only remediation. `git diff --check` passed. These focused local checks prove deterministic remediation-plan generation/reading, dot-directory path preservation, root-file path inclusion and synthesis regression scope; CI and independent exact-head review remain pending."
failure_policy: "Missing, skipped, zero-scenario or failed required proof blocks acceptance. Replan absent/renamed tests explicitly. No canned plan, fixture-only admission, helper-only success, unexecuted platform claim or provider permission inference. Keep machine-readable stdout and redacted stderr; preserve failed evidence and refresh affected exact-head review."
notes: "#892 dependency was terminal before implementation. The implemented planner consumes `codefriend.review_synthesis.v1` JSON, generates `codefriend.remediation_plan.v1`, writes create-only output artifacts in a fresh directory, preserves omitted untraceable findings with reasons, rejects unsafe paths, missing acceptance trace and dependency cycles, and exposes an installed CLI reader. No autonomous source repair, issue creation, external provider call, cloud resource, publication, or merge is claimed."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

After accepted #892 and shared-owner recheck, freeze completed-synthesis input and traceability contracts; implement bounded evidence-linked remediation generation with one repair per action and acyclic ordering; wire the installed generator and complete plan reader; execute real synthesis fixtures and unrelated-action, unsupported-path, missing-evidence, cycle, invalid-input and source-mutation negatives; document limitations and obtain independent exact-head review.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `runtime`
- Planned PVF lane for execution: `runtime`

## Selected Validation Lanes

- runtime; deterministic semantic correctness plus installed consumer integration; focused local proof passed for `codefriend_remediate` 5/5, `cargo fmt --check`, and `git diff --check`; CI observation, publication checks, and fresh exact-head review remain pending

## Parallelization Plan

- Parallel groups: Serialize native issue preparation; future isolated deterministic fixtures may parallelize only after ownership/resource review
- Validation runtime class: `bounded_local`
- Validation resource profile: `Isolated local CPU/Rust/filesystem, controlled observation time, bounded admitted fixtures; no network/provider/cloud execution`
- Validation family: `codefriend_action_plans`
- Validation size split: `Focused named tests and touched-owner regressions; macOS/Linux CI separately observed`

## Goal Accounting Hooks

- Issue goal ref: `Sprint 4 #930 active goal covers #893 execution in this session; single goal slot prevented replacing it with a separate child goal.`
- Sprint goal ref: `v0.92.2 execution Sprint 4; umbrella management owned by #926`
- Goal metrics rollup ref: `.csdlc/evidence/893/goal-metrics.json (planned, absent until execution)`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Low-confidence estimate 1200 local validation seconds; excludes cold build, CI queue and independent review`
- Planned validation seconds: `1200`
- Planned validation token budget: `7000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- `cargo fmt --manifest-path adl/Cargo.toml --check` passed. `cargo test --manifest-path adl/Cargo.toml --test codefriend_remediate` passed 5/5 after adding the path-extraction regression. `cargo test --manifest-path adl/Cargo.toml --test codefriend_synthesis` previously passed 5/5 and was not rerun for the path-only remediation. `git diff --check` passed. These focused local checks prove deterministic remediation-plan generation/reading, dot-directory path preservation, root-file path inclusion and synthesis regression scope; CI and independent exact-head review remain pending.

## Failure Semantics

- Missing, skipped, zero-scenario or failed required proof blocks acceptance. Replan absent/renamed tests explicitly. No canned plan, fixture-only admission, helper-only success, unexecuted platform claim or provider permission inference. Keep machine-readable stdout and redacted stderr; preserve failed evidence and refresh affected exact-head review.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

#892 dependency was terminal before implementation. The implemented planner consumes `codefriend.review_synthesis.v1` JSON, generates `codefriend.remediation_plan.v1`, writes create-only output artifacts in a fresh directory, preserves omitted untraceable findings with reasons, rejects unsafe paths, missing acceptance trace and dependency cycles, and exposes an installed CLI reader. No autonomous source repair, issue creation, external provider call, cloud resource, publication, or merge is claimed.
