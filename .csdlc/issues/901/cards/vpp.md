---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "v0922-provider-recovery-qualification-validation-plan"
issue: 901
task_id: "issue-0901"
run_id: "issue-0901"
version: "0.92.2"
title: "[v0.92.2][QUAL-PROVIDER] Execute real provider failure and recovery qualification"
branch: "codex/901-v0922-provider-recovery-qualification"
generated_at: "2026-09-14T18:06:04.530196+00:00"
card_status: "ready"
status: "executed"
initial_pvf_lane: "provider"
planned_pvf_lane: "provider"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "local_regression_and_authorized_production"
validation_resource_profile: "Executed with a task-owned CPU llama-server and adapter process groups on dynamic loopback ports, using the existing gemma:2b artifact. No paid/cloud calls, model downloads, shared Ollama service mutation, account changes, or broad process scans."
validation_family: "provider_failure_recovery_qualification"
validation_size_split: "Focused local harness negatives, actual production scenarios and required CI reported as separate lanes"
expected_proof_cost: "Observed live scenario wall time 11.166 seconds plus task-owned provider startup and reaping; local build was cold after cross-filesystem cache warmup safely skipped."
planned_validation_seconds: "2400"
planned_validation_tokens: "10000"
issue_goal_ref: "thread-goal 01a0924d-fbc0-7d21-b1ec-965c8a9562a4; active Sprint #931 objective explicitly includes child #901 qualification"
sprint_goal_ref: "Sprint 5 umbrella #931; child #901 provider failure and recovery qualification"
goal_metrics_rollup_ref: ".csdlc/evidence/901/goal-metrics.json not collected; active Sprint #931 goal service owns session accounting"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/901"
  - kind: "stp"
    ref: ".csdlc/issues/901/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/901/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/901/cards/spp.md"
selected_lanes:
  - "provider; task-owned local production-adapter and registered Runtime executions plus deterministic evidence-integrity negatives passed locally; exact-head review and hosted CI pending"
parallel_groups:
  - "Serialize task-owned live fault scenarios; deterministic negative fixtures may run separately after harness implementation"
validation_commands:
  - "Standalone adapter live-run-09: four production-adapter scenarios and portable validation passed. Registered Runtime runtime-live-12: executable source cb292d7fbfb55b987a05ce1c0c3bd88954c23ebb; loss, recovery, and interruption passed in one Runtime; six retained raw artifact bindings passed portable validation. Both focused Python suites passed 25 tests total. Strict X.509 verification, py_compile, and git diff --check passed. Independent exact-head review passed at 2930f13da6a48331438bf81f046a9e37a925e973. Hosted CI remains pending."
failure_policy: "Missing, skipped, zero-scenario or failed required proof blocks acceptance. Replan absent/renamed tests explicitly. No mock-only success, static reference trace, supplied failure flag, stale candidate or provider permission inference. Actual effects, complete scenario population and independent review remain required. Keep machine-readable stdout and redacted stderr; preserve failed evidence and refresh affected exact-head review."
notes: "Standalone live-run-09 remains the separately retained real timeout proof. Registered Runtime runtime-live-12 passed at executable source cb292d7fbfb55b987a05ce1c0c3bd88954c23ebb with unchanged Runtime identity, actual provider loss, fresh-provider recovery, client WebSocket interruption, checkpoint, and removal. Its portable report reopens and hash-binds six retained raw artifacts. Earlier Runtime attempts remain private and non-accepting. Independent exact-head review passed at 2930f13da6a48331438bf81f046a9e37a925e973; hosted CI, merge, and closeout remain pending."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Bind from current origin/main; enumerate the merged production adapter and registered-provider entrypoints; author the isolated #901 harness and negative fixtures in the new paths; exercise actual loss, timeout, interruption, and healthy recovery separately through the production adapter; retain exact execution receipts; run focused validation and independent exact-head review.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `provider`
- Planned PVF lane for execution: `provider`

## Selected Validation Lanes

- provider; task-owned local production-adapter and registered Runtime executions plus deterministic evidence-integrity negatives passed locally; exact-head review and hosted CI pending

## Parallelization Plan

- Parallel groups: Serialize task-owned live fault scenarios; deterministic negative fixtures may run separately after harness implementation
- Validation runtime class: `local_regression_and_authorized_production`
- Validation resource profile: `Executed with a task-owned CPU llama-server and adapter process groups on dynamic loopback ports, using the existing gemma:2b artifact. No paid/cloud calls, model downloads, shared Ollama service mutation, account changes, or broad process scans.`
- Validation family: `provider_failure_recovery_qualification`
- Validation size split: `Focused local harness negatives, actual production scenarios and required CI reported as separate lanes`

## Goal Accounting Hooks

- Issue goal ref: `thread-goal 01a0924d-fbc0-7d21-b1ec-965c8a9562a4; active Sprint #931 objective explicitly includes child #901 qualification`
- Sprint goal ref: `Sprint 5 umbrella #931; child #901 provider failure and recovery qualification`
- Goal metrics rollup ref: `.csdlc/evidence/901/goal-metrics.json not collected; active Sprint #931 goal service owns session accounting`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Observed live scenario wall time 11.166 seconds plus task-owned provider startup and reaping; local build was cold after cross-filesystem cache warmup safely skipped.`
- Planned validation seconds: `2400`
- Planned validation token budget: `10000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- Standalone adapter live-run-09: four production-adapter scenarios and portable validation passed. Registered Runtime runtime-live-12: executable source cb292d7fbfb55b987a05ce1c0c3bd88954c23ebb; loss, recovery, and interruption passed in one Runtime; six retained raw artifact bindings passed portable validation. Both focused Python suites passed 25 tests total. Strict X.509 verification, py_compile, and git diff --check passed. Independent exact-head review passed at 2930f13da6a48331438bf81f046a9e37a925e973. Hosted CI remains pending.

## Failure Semantics

- Missing, skipped, zero-scenario or failed required proof blocks acceptance. Replan absent/renamed tests explicitly. No mock-only success, static reference trace, supplied failure flag, stale candidate or provider permission inference. Actual effects, complete scenario population and independent review remain required. Keep machine-readable stdout and redacted stderr; preserve failed evidence and refresh affected exact-head review.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Standalone live-run-09 remains the separately retained real timeout proof. Registered Runtime runtime-live-12 passed at executable source cb292d7fbfb55b987a05ce1c0c3bd88954c23ebb with unchanged Runtime identity, actual provider loss, fresh-provider recovery, client WebSocket interruption, checkpoint, and removal. Its portable report reopens and hash-binds six retained raw artifacts. Earlier Runtime attempts remain private and non-accepting. Independent exact-head review passed at 2930f13da6a48331438bf81f046a9e37a925e973; hosted CI, merge, and closeout remain pending.
