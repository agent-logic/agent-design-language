---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "v0922-test-planner-validation-plan"
issue: 894
task_id: "issue-0894"
run_id: "issue-0894"
version: "0.92.2"
title: "[v0.92.2][CF-TESTPLAN] Generate a bounded test plan from review findings"
branch: "codex/894-v0922-test-planner"
generated_at: "2026-09-12T00:10:09.925246+00:00"
card_status: "ready"
status: "planned"
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
issue_goal_ref: "Sprint 4 #930 active goal covers #894 execution; this delegated lane advances #894 without replacing the root sprint goal."
sprint_goal_ref: "v0.92.2 execution Sprint 4; umbrella management owned by #926"
goal_metrics_rollup_ref: ".csdlc/evidence/894/goal-metrics.json (planned, absent until execution)"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/894"
  - kind: "stp"
    ref: ".csdlc/issues/894/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/894/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/894/cards/spp.md"
selected_lanes:
  - "Runtime-focused deterministic local contract/integration lane: six named codefriend_testplan scenarios including installed CLI generation/readback, retained #892 predecessor consumption, concrete behavior/fixture/assertion mapping, path preservation, omission handling, invalid-plan rejection, and source immutability. Rust formatting, focused Clippy with denied warnings, and exact diff hygiene accompany the lane. Standard CI remains required after publication; no optional, paid, provider, or cloud lane is selected."
parallel_groups:
  - "Serialize the Rust test and Clippy commands against the same issue-local target cache; formatting and diff hygiene are independent lightweight checks. Fresh exact-head review follows an immutable clean commit. Standard CI follows native publication."
validation_commands:
  - "Declared focused commands: CARGO_TARGET_DIR=<Git-common issue-894 cache> cargo test --manifest-path adl/Cargo.toml --test codefriend_testplan; CARGO_TARGET_DIR=<Git-common issue-894 cache> cargo clippy --manifest-path adl/Cargo.toml --test codefriend_testplan -- -D warnings; cargo fmt --manifest-path adl/Cargo.toml --check; git diff --check origin/main...HEAD. Missing, skipped, zero-scenario, or failed proof blocks acceptance. Standard CI is observed after native publication."
failure_policy: "Missing, skipped, zero-scenario or failed required proof blocks acceptance. Replan absent/renamed tests explicitly. No canned plan, fixture-only admission, helper-only success, unexecuted platform claim or provider permission inference. Keep machine-readable stdout and redacted stderr; preserve failed evidence and refresh affected exact-head review."
notes: "The selected proof is bounded local CPU/Rust/filesystem with no network, provider credential, paid job, or cloud execution. VPP declares the lane but does not claim its result; actual results are recorded in SOR. Any substantive change refreshes proof and exact-head review."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

With #892 accepted, consume the merged synthesis artifact contract and implement bounded test-plan generation that maps synthesized findings to behavior under test, proposed test location, fixture/input, expected failure before a fix, expected post-fix assertion, validation lane, and source-mutation non-goal. Before bind, recheck shared CodeFriend CLI/action paths and preserve #893/#895 sibling boundaries.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `runtime`
- Planned PVF lane for execution: `runtime`

## Selected Validation Lanes

- Runtime-focused deterministic local contract/integration lane: six named codefriend_testplan scenarios including installed CLI generation/readback, retained #892 predecessor consumption, concrete behavior/fixture/assertion mapping, path preservation, omission handling, invalid-plan rejection, and source immutability. Rust formatting, focused Clippy with denied warnings, and exact diff hygiene accompany the lane. Standard CI remains required after publication; no optional, paid, provider, or cloud lane is selected.

## Parallelization Plan

- Parallel groups: Serialize the Rust test and Clippy commands against the same issue-local target cache; formatting and diff hygiene are independent lightweight checks. Fresh exact-head review follows an immutable clean commit. Standard CI follows native publication.
- Validation runtime class: `bounded_local`
- Validation resource profile: `Isolated local CPU/Rust/filesystem, controlled observation time, bounded admitted fixtures; no network/provider/cloud execution`
- Validation family: `codefriend_action_plans`
- Validation size split: `Focused named tests and touched-owner regressions; macOS/Linux CI separately observed`

## Goal Accounting Hooks

- Issue goal ref: `Sprint 4 #930 active goal covers #894 execution; this delegated lane advances #894 without replacing the root sprint goal.`
- Sprint goal ref: `v0.92.2 execution Sprint 4; umbrella management owned by #926`
- Goal metrics rollup ref: `.csdlc/evidence/894/goal-metrics.json (planned, absent until execution)`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Low-confidence estimate 1200 local validation seconds; excludes cold build, CI queue and independent review`
- Planned validation seconds: `1200`
- Planned validation token budget: `7000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- Declared focused commands: CARGO_TARGET_DIR=<Git-common issue-894 cache> cargo test --manifest-path adl/Cargo.toml --test codefriend_testplan; CARGO_TARGET_DIR=<Git-common issue-894 cache> cargo clippy --manifest-path adl/Cargo.toml --test codefriend_testplan -- -D warnings; cargo fmt --manifest-path adl/Cargo.toml --check; git diff --check origin/main...HEAD. Missing, skipped, zero-scenario, or failed proof blocks acceptance. Standard CI is observed after native publication.

## Failure Semantics

- Missing, skipped, zero-scenario or failed required proof blocks acceptance. Replan absent/renamed tests explicitly. No canned plan, fixture-only admission, helper-only success, unexecuted platform claim or provider permission inference. Keep machine-readable stdout and redacted stderr; preserve failed evidence and refresh affected exact-head review.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

The selected proof is bounded local CPU/Rust/filesystem with no network, provider credential, paid job, or cloud execution. VPP declares the lane but does not claim its result; actual results are recorded in SOR. Any substantive change refreshes proof and exact-head review.
