---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "issue-856-native-release-preflight-validation-plan"
issue: 856
task_id: "issue-0856"
run_id: "issue-0856"
version: "v0.92.1"
title: "[v0.92.1][release] Reconcile release versions and restore native v3 ceremony preflight"
branch: "codex/856-native-release-preflight"
generated_at: "2026-09-11T18:46:11.480766+00:00"
card_status: "ready"
status: "in_progress"
initial_pvf_lane: "tooling"
planned_pvf_lane: "tooling"
lane_registry_path: "docs/csdlc-v3/RELEASE_PREFLIGHT.md"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "small"
validation_resource_profile: "local CPU and Git fixtures; no network/cloud mutation"
validation_family: "native release preflight"
validation_size_split: "Focused local contract tests plus required hosted CI"
expected_proof_cost: "small local CPU/Git fixtures"
planned_validation_seconds: "300"
planned_validation_tokens: "unknown"
issue_goal_ref: "Planning #7 issue856 goal"
sprint_goal_ref: "not_applicable"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/856"
  - kind: "stp"
    ref: ".csdlc/issues/856/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/856/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/856/cards/spp.md"
selected_lanes:
  - "Native contract proof; retained registry compatibility contract proof; hosted integrationCI"
parallel_groups:
  - "Independent metadata and shell checks; native build commands serialized"
validation_commands:
  - "Native preflight suite and shell/metadata checks; retained v2 card_identity/gate9 and standalone suite plus strict clippy/fmt; hostedCI"
failure_policy: "Fail closed on any candidate, version, notes or authority mismatch"
notes: "Local gate hashes prove consistency, not semantic review/authentication; candidate release approval remains separate."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Inventory package versions; implement exact-candidate native preflight; prove rejection/nonmutation; independent review; publish and resolve CI.

## Lane Registry Inputs

- Registry path: `docs/csdlc-v3/RELEASE_PREFLIGHT.md`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `tooling`
- Planned PVF lane for execution: `tooling`

## Selected Validation Lanes

- Native contract proof; retained registry compatibility contract proof; hosted integrationCI

## Parallelization Plan

- Parallel groups: Independent metadata and shell checks; native build commands serialized
- Validation runtime class: `small`
- Validation resource profile: `local CPU and Git fixtures; no network/cloud mutation`
- Validation family: `native release preflight`
- Validation size split: `Focused local contract tests plus required hosted CI`

## Goal Accounting Hooks

- Issue goal ref: `Planning #7 issue856 goal`
- Sprint goal ref: `not_applicable`
- Goal metrics rollup ref: `not_collected`

## Proof Cost / Runtime Expectations

- Expected proof cost: `small local CPU/Git fixtures`
- Planned validation seconds: `300`
- Planned validation token budget: `unknown`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- Native preflight suite and shell/metadata checks; retained v2 card_identity/gate9 and standalone suite plus strict clippy/fmt; hostedCI

## Failure Semantics

- Fail closed on any candidate, version, notes or authority mismatch

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Local gate hashes prove consistency, not semantic review/authentication; candidate release approval remains separate.
