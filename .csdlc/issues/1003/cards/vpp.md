---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "scope-rebind-validator-recovery-validation-plan"
issue: 1003
task_id: "issue-1003"
run_id: "issue-1003"
version: "v0.92.2"
title: "[v0.92.2][C-SDLC] Restore rebind and validator replacement after scope amendments"
branch: "codex/1003-scope-rebind-validator-recovery"
generated_at: "2026-09-16T02:43:15.759077+00:00"
card_status: "ready"
status: "planned"
initial_pvf_lane: "csdlc"
planned_pvf_lane: "csdlc"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "small"
validation_resource_profile: "local CPU and temporary Git fixtures"
validation_family: "native-owner-contract"
validation_size_split: "focused local then CI integration"
expected_proof_cost: "local CPU; no paid resources"
planned_validation_seconds: "900"
planned_validation_tokens: "15000"
issue_goal_ref: "Worker10 #1003 and #1006 implementation"
sprint_goal_ref: "Not a sprint child; follow-up tooling repair"
goal_metrics_rollup_ref: "session goal"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1003"
  - kind: "stp"
    ref: ".csdlc/issues/1003/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/1003/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/1003/cards/spp.md"
selected_lanes:
  - "Focused csdlc owner regressions and installed candidate; hosted tests/coverage after publication"
parallel_groups:
  - "Separate issue worktrees; serialize shared build cache writes"
validation_commands:
  - "Focused Rust intent and semantic-owner tests, installed candidate rebind and validator replacement fixtures, negative identity/admission cases, cargo fmt and strict Clippy, then required GitHub CI tests and coverage. PVF deterministic local contract/regression proof; small CPU/local Git and isolated synthetic transport; required gate; no provider/cloud."
failure_policy: "Any regression or missing required proof blocks publication."
notes: "Local contract and installed candidate proof recorded in .csdlc/evidence/1003/LOCAL_PROOF.json. Required hosted tests and coverage deferred until PR publication; no shared binary upgrade."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Reproduce both #1003 failure paths in focused tests. Inspect semantic bind operation identity and admission after scope_acceptance. Add the smallest explicit rebind/head-refresh path preserving registered topology and version guards. Permit typed validator replacement after the guarded refresh without executing retained validators. Test negative ownership/authority/recovery and evidence invalidation cases; run installed candidate scenarios, independent review and CI.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `csdlc`
- Planned PVF lane for execution: `csdlc`

## Selected Validation Lanes

- Focused csdlc owner regressions and installed candidate; hosted tests/coverage after publication

## Parallelization Plan

- Parallel groups: Separate issue worktrees; serialize shared build cache writes
- Validation runtime class: `small`
- Validation resource profile: `local CPU and temporary Git fixtures`
- Validation family: `native-owner-contract`
- Validation size split: `focused local then CI integration`

## Goal Accounting Hooks

- Issue goal ref: `Worker10 #1003 and #1006 implementation`
- Sprint goal ref: `Not a sprint child; follow-up tooling repair`
- Goal metrics rollup ref: `session goal`

## Proof Cost / Runtime Expectations

- Expected proof cost: `local CPU; no paid resources`
- Planned validation seconds: `900`
- Planned validation token budget: `15000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- Focused Rust intent and semantic-owner tests, installed candidate rebind and validator replacement fixtures, negative identity/admission cases, cargo fmt and strict Clippy, then required GitHub CI tests and coverage. PVF deterministic local contract/regression proof; small CPU/local Git and isolated synthetic transport; required gate; no provider/cloud.

## Failure Semantics

- Any regression or missing required proof blocks publication.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Local contract and installed candidate proof recorded in .csdlc/evidence/1003/LOCAL_PROOF.json. Required hosted tests and coverage deferred until PR publication; no shared binary upgrade.
