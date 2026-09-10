---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "pr-ready-reconciliation-validation-plan"
issue: 824
task_id: "issue-0824"
run_id: "issue-0824"
version: "v0.92.1"
title: "[v0.92.1][tooling] Reconcile native pull-request ready mutations"
branch: "codex/824-pr-ready-reconciliation"
generated_at: "2026-09-10T00:00:00Z"
card_status: "ready"
status: "READY"
initial_pvf_lane: "tooling"
planned_pvf_lane: "tooling"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "small"
validation_resource_profile: "local CPU/filesystem; no network"
validation_family: "github_mutation_reconciliation"
validation_size_split: "focused unit/integration tests and issue packet"
expected_proof_cost: "Small local proof."
planned_validation_seconds: "not_collected"
planned_validation_tokens: "not_collected"
issue_goal_ref: "Issue #824 session goal"
sprint_goal_ref: "Parent #522"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/824"
  - kind: "stp"
    ref: ".csdlc/issues/824/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/824/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/824/cards/spp.md"
selected_lanes:
  - "tooling"
parallel_groups:
  - "Code tests and packet validation."
validation_commands:
  - "Focused cargo tests; packet validator; native validate; fmt/clippy; diff check."
failure_policy: "Any ambiguous, stale, mismatched, or repeated recovery blocks publication."
notes: "Already-ready is idempotent; still-draft is not success."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Deterministic local CPU contract with fake provider transport.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `tooling`
- Planned PVF lane for execution: `tooling`

## Selected Validation Lanes

- tooling

## Parallelization Plan

- Parallel groups: Code tests and packet validation.
- Validation runtime class: `small`
- Validation resource profile: `local CPU/filesystem; no network`
- Validation family: `github_mutation_reconciliation`
- Validation size split: `focused unit/integration tests and issue packet`

## Goal Accounting Hooks

- Issue goal ref: `Issue #824 session goal`
- Sprint goal ref: `Parent #522`
- Goal metrics rollup ref: `not_collected`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Small local proof.`
- Planned validation seconds: `not_collected`
- Planned validation token budget: `not_collected`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- Focused cargo tests; packet validator; native validate; fmt/clippy; diff check.

## Failure Semantics

- Any ambiguous, stale, mismatched, or repeated recovery blocks publication.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Already-ready is idempotent; still-draft is not success.
