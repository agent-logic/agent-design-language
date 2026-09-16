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
issue_goal_ref: "not_created; required before implementation"
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
  - "runtime; deterministic semantic correctness plus actual installed consumer integration; issue acceptance and later release input; all runs pending"
parallel_groups:
  - "Serialize native issue preparation; future isolated deterministic fixtures may parallelize only after ownership/resource review"
validation_commands:
  - "Declare each new test in a coupled proof inventory: deterministic local CPU contract/integration lane, isolated source/store and controlled clocks/transport, nonzero success and negative proof, bounded disk/process resources, required Beta feature and CF-INTEGRATE input gate. Provider-generated runs and browser/PDF visual review are separately identified execution/observation evidence, not deterministic guarantees. Run focused installed CLI and consumer tests plus required CI, then independent exact-head review. Redacted human diagnostics stay on stderr and machine output on stdout; test compatibility logging when exposed. Stop on unresolved owner/authority, inconsistent scope/provenance, failed or missing proof, source/credential exposure, stale approval or partial completion claimed as success. No unrelated feature work or customer-scale hosting. Planned focused commands after the selected new suite is implemented: `cargo test --manifest-path adl/Cargo.toml --test codefriend_testplan`; `cargo fmt --manifest-path adl/Cargo.toml --check`; focused touched CLI/admission-owner regressions selected after merged prerequisites. These do not replace actual installed generator/reader or publication admission execution. Re-resolve suite names after prerequisites land and update VPP for changes; never treat a zero-test filter as proof. `git diff --check` accompanies focused proof. Selected new test files do not exist yet; their commands are future proving work, not tests run during preparation. Install the issue candidate through the approved installer only at execution with current provenance; do not replace a shared binary during preparation."
failure_policy: "Missing, skipped, zero-scenario or failed required proof blocks acceptance. Replan absent/renamed tests explicitly. No canned plan, fixture-only admission, helper-only success, unexecuted platform claim or provider permission inference. Keep machine-readable stdout and redacted stderr; preserve failed evidence and refresh affected exact-head review."
notes: "Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts. Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success. Live dependency read on 2026-09-16 confirms #892 (CF-SYNTHESIS) is closed/accepted. Prepared, not bound. All implementation steps, proving runs, implementation review, PR and terminal state remain unstarted. Shared CLI dispatch/library, fixture directories and evidence/publication admission surfaces require current owner coordination before bind. Branch codex/894-v0922-test-planner and its proposed FastWork path are unbound planning values only."
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

- runtime; deterministic semantic correctness plus actual installed consumer integration; issue acceptance and later release input; all runs pending

## Parallelization Plan

- Parallel groups: Serialize native issue preparation; future isolated deterministic fixtures may parallelize only after ownership/resource review
- Validation runtime class: `bounded_local`
- Validation resource profile: `Isolated local CPU/Rust/filesystem, controlled observation time, bounded admitted fixtures; no network/provider/cloud execution`
- Validation family: `codefriend_action_plans`
- Validation size split: `Focused named tests and touched-owner regressions; macOS/Linux CI separately observed`

## Goal Accounting Hooks

- Issue goal ref: `not_created; required before implementation`
- Sprint goal ref: `v0.92.2 execution Sprint 4; umbrella management owned by #926`
- Goal metrics rollup ref: `.csdlc/evidence/894/goal-metrics.json (planned, absent until execution)`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Low-confidence estimate 1200 local validation seconds; excludes cold build, CI queue and independent review`
- Planned validation seconds: `1200`
- Planned validation token budget: `7000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- Declare each new test in a coupled proof inventory: deterministic local CPU contract/integration lane, isolated source/store and controlled clocks/transport, nonzero success and negative proof, bounded disk/process resources, required Beta feature and CF-INTEGRATE input gate. Provider-generated runs and browser/PDF visual review are separately identified execution/observation evidence, not deterministic guarantees. Run focused installed CLI and consumer tests plus required CI, then independent exact-head review. Redacted human diagnostics stay on stderr and machine output on stdout; test compatibility logging when exposed. Stop on unresolved owner/authority, inconsistent scope/provenance, failed or missing proof, source/credential exposure, stale approval or partial completion claimed as success. No unrelated feature work or customer-scale hosting. Planned focused commands after the selected new suite is implemented: `cargo test --manifest-path adl/Cargo.toml --test codefriend_testplan`; `cargo fmt --manifest-path adl/Cargo.toml --check`; focused touched CLI/admission-owner regressions selected after merged prerequisites. These do not replace actual installed generator/reader or publication admission execution. Re-resolve suite names after prerequisites land and update VPP for changes; never treat a zero-test filter as proof. `git diff --check` accompanies focused proof. Selected new test files do not exist yet; their commands are future proving work, not tests run during preparation. Install the issue candidate through the approved installer only at execution with current provenance; do not replace a shared binary during preparation.

## Failure Semantics

- Missing, skipped, zero-scenario or failed required proof blocks acceptance. Replan absent/renamed tests explicitly. No canned plan, fixture-only admission, helper-only success, unexecuted platform claim or provider permission inference. Keep machine-readable stdout and redacted stderr; preserve failed evidence and refresh affected exact-head review.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts. Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success. Live dependency read on 2026-09-16 confirms #892 (CF-SYNTHESIS) is closed/accepted. Prepared, not bound. All implementation steps, proving runs, implementation review, PR and terminal state remain unstarted. Shared CLI dispatch/library, fixture directories and evidence/publication admission surfaces require current owner coordination before bind. Branch codex/894-v0922-test-planner and its proposed FastWork path are unbound planning values only.
