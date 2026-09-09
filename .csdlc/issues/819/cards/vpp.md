---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "csdlc-v3-retained-proof-validation-plan"
issue: 819
task_id: "issue-0819"
run_id: "issue-0819"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06.08b][quality] Close C-SDLC v3 retained proof gaps"
branch: "codex/819-csdlc-v3-retained-proof"
generated_at: "2026-09-09T22:20:00Z"
card_status: "ready"
status: "READY"
initial_pvf_lane: "review_tests"
planned_pvf_lane: "review_tests"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "medium"
validation_resource_profile: "local CPU and filesystem; no cloud"
validation_family: "retained_proof_reconciliation"
validation_size_split: "152 rows: 114 candidate-bound execution receipts and 38 explicit governed non-pass amendments"
expected_proof_cost: "Approximately 45 seconds of local CPU proof plus deterministic reconciliation validation; no cloud resources."
planned_validation_seconds: "60"
planned_validation_tokens: "4000"
issue_goal_ref: "Active issue #819 session goal"
sprint_goal_ref: "Parent #522"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/819"
  - kind: "stp"
    ref: ".csdlc/issues/819/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/819/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/819/cards/spp.md"
selected_lanes:
  - "review_tests; exact-denominator reconciliation; candidate-bound C-SDLC v3 full suite; all-target clippy; current V3-A contract; diff and publication hygiene"
parallel_groups:
  - "The test, clippy, and current-contract producers are recorded separately; final row reconciliation and negative validation are serialized."
validation_commands:
  - "ruby .csdlc/prepared/issues/819/build-retained-v3-plan.rb; ruby .csdlc/prepared/issues/819/run-retained-v3-proof.rb; ruby .csdlc/prepared/issues/819/validate-retained-v3-proof.rb; ruby .csdlc/prepared/issues/819/test-retained-v3-proof.rb; git diff --check"
failure_policy: "Any missing, duplicate, stale, non-executed, or synthetic row blocks publication."
notes: "Passing commands alone are insufficient: every execution row must name an observed passing test, candidate artifacts must match exact Git bytes, and all governed amendments must retain behavioral_pass_claim=false."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Use exact-denominator checks plus real replayed C-SDLC v3 proof-family commands and candidate-bound receipt validation.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `review_tests`
- Planned PVF lane for execution: `review_tests`

## Selected Validation Lanes

- review_tests; exact-denominator reconciliation; candidate-bound C-SDLC v3 full suite; all-target clippy; current V3-A contract; diff and publication hygiene

## Parallelization Plan

- Parallel groups: The test, clippy, and current-contract producers are recorded separately; final row reconciliation and negative validation are serialized.
- Validation runtime class: `medium`
- Validation resource profile: `local CPU and filesystem; no cloud`
- Validation family: `retained_proof_reconciliation`
- Validation size split: `152 rows: 114 candidate-bound execution receipts and 38 explicit governed non-pass amendments`

## Goal Accounting Hooks

- Issue goal ref: `Active issue #819 session goal`
- Sprint goal ref: `Parent #522`
- Goal metrics rollup ref: `not_collected`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Approximately 45 seconds of local CPU proof plus deterministic reconciliation validation; no cloud resources.`
- Planned validation seconds: `60`
- Planned validation token budget: `4000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- ruby .csdlc/prepared/issues/819/build-retained-v3-plan.rb; ruby .csdlc/prepared/issues/819/run-retained-v3-proof.rb; ruby .csdlc/prepared/issues/819/validate-retained-v3-proof.rb; ruby .csdlc/prepared/issues/819/test-retained-v3-proof.rb; git diff --check

## Failure Semantics

- Any missing, duplicate, stale, non-executed, or synthetic row blocks publication.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Passing commands alone are insufficient: every execution row must name an observed passing test, candidate artifacts must match exact Git bytes, and all governed amendments must retain behavioral_pass_claim=false.
