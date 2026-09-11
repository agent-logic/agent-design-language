---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "corporate-runtime-retained-proof-validation-plan"
issue: 818
task_id: "issue-0818"
run_id: "issue-0818"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06.08a][quality] Close corporate Runtime retained proof gaps"
branch: "codex/818-corporate-runtime-retained-proof"
generated_at: "2026-09-10T22:36:29Z"
card_status: "ready"
status: "READY"
initial_pvf_lane: "review_tests"
planned_pvf_lane: "review_tests"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "v0.92.1"
validation_runtime_class: "small"
validation_resource_profile: "local-cpu"
validation_family: "deterministic_artifact"
validation_size_split: "focused"
expected_proof_cost: "small"
planned_validation_seconds: "unknown"
planned_validation_tokens: "unknown"
issue_goal_ref: "Issue #818 active session goal"
sprint_goal_ref: "Parent #522"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/818"
  - kind: "stp"
    ref: ".csdlc/issues/818/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/818/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/818/cards/spp.md"
selected_lanes:
  - "review_tests"
parallel_groups:
  - "serial deterministic artifact generation then negative matrix"
validation_commands:
  - "ruby .csdlc/prepared/issues/818/run-retained-corporate-runtime-proof.rb; ruby .csdlc/prepared/issues/818/validate-retained-corporate-runtime-proof.rb; ruby .csdlc/prepared/issues/818/test-retained-corporate-runtime-proof.rb"
failure_policy: "Fail closed on any nonzero exit, denominator drift, source drift, fabricated approval, or unsupported behavioral claim."
notes: "Focused proof passed; independent exact-head review remains pending."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Focused issue-local reconciliation proof for the exact 17-row corporate/Runtime bucket.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `v0.92.1`
- Initial PVF lane from issue creation: `review_tests`
- Planned PVF lane for execution: `review_tests`

## Selected Validation Lanes

- review_tests

## Parallelization Plan

- Parallel groups: serial deterministic artifact generation then negative matrix
- Validation runtime class: `small`
- Validation resource profile: `local-cpu`
- Validation family: `deterministic_artifact`
- Validation size split: `focused`

## Goal Accounting Hooks

- Issue goal ref: `Issue #818 active session goal`
- Sprint goal ref: `Parent #522`
- Goal metrics rollup ref: `not_collected`

## Proof Cost / Runtime Expectations

- Expected proof cost: `small`
- Planned validation seconds: `unknown`
- Planned validation token budget: `unknown`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- ruby .csdlc/prepared/issues/818/run-retained-corporate-runtime-proof.rb; ruby .csdlc/prepared/issues/818/validate-retained-corporate-runtime-proof.rb; ruby .csdlc/prepared/issues/818/test-retained-corporate-runtime-proof.rb

## Failure Semantics

- Fail closed on any nonzero exit, denominator drift, source drift, fabricated approval, or unsupported behavioral claim.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Focused proof passed; independent exact-head review remains pending.
