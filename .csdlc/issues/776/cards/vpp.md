---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "versioned-template-structure-schemas-validation-plan"
issue: 776
task_id: "issue-0776"
run_id: "issue-0776"
version: "1.0.4"
title: "[v0.92.1][TAIL-06.17][csdlc] Resolve structure schemas beside versioned templates"
branch: "codex/776-versioned-template-structure-schemas"
generated_at: "<timestamp>"
card_status: "ready"
status: "ready"
initial_pvf_lane: "csdlc-v3-local-commands"
planned_pvf_lane: "csdlc-v3-local-commands"
lane_registry_path: "docs/pvf/PVF_REGISTRY.json"
lane_registry_template_set: "1.0.4"
validation_runtime_class: "focused"
validation_resource_profile: "small"
validation_family: "native-csdlc-v3"
validation_size_split: "single integration target"
expected_proof_cost: "small local CPU; no network or cloud"
planned_validation_seconds: "300"
planned_validation_tokens: "2000"
issue_goal_ref: "Issue #776 session goal"
sprint_goal_ref: "v0.92.1 TAIL-06.17"
goal_metrics_rollup_ref: "not_applicable"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/776"
  - kind: "stp"
    ref: ".csdlc/issues/776/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/776/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/776/cards/spp.md"
selected_lanes:
  - "current-registry all-six structure regression; full local_commands target; diff hygiene"
parallel_groups:
  - "serial local proof"
validation_commands:
  - "cargo test --manifest-path csdlc-v3/Cargo.toml --test local_commands current_registry_versioned_templates_resolve_all_structure_schemas -- --exact; cargo test --manifest-path csdlc-v3/Cargo.toml --test local_commands; git diff --check"
failure_policy: "Fail closed on any missing schema, structure mismatch, current-registry drift, focused-test failure, or diff-hygiene error."
notes: "No synthetic-only substitute is accepted for current registry coverage."
---

Canonical Template Source: `docs/templates/prompts/1.0.4/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Prove the real current registry resolves and validates all six versioned card structure schemas without changing their contents.

## Lane Registry Inputs

- Registry path: `docs/pvf/PVF_REGISTRY.json`
- Registry template set: `1.0.4`
- Initial PVF lane from issue creation: `csdlc-v3-local-commands`
- Planned PVF lane for execution: `csdlc-v3-local-commands`

## Selected Validation Lanes

- current-registry all-six structure regression; full local_commands target; diff hygiene

## Parallelization Plan

- Parallel groups: serial local proof
- Validation runtime class: `focused`
- Validation resource profile: `small`
- Validation family: `native-csdlc-v3`
- Validation size split: `single integration target`

## Goal Accounting Hooks

- Issue goal ref: `Issue #776 session goal`
- Sprint goal ref: `v0.92.1 TAIL-06.17`
- Goal metrics rollup ref: `not_applicable`

## Proof Cost / Runtime Expectations

- Expected proof cost: `small local CPU; no network or cloud`
- Planned validation seconds: `300`
- Planned validation token budget: `2000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- cargo test --manifest-path csdlc-v3/Cargo.toml --test local_commands current_registry_versioned_templates_resolve_all_structure_schemas -- --exact; cargo test --manifest-path csdlc-v3/Cargo.toml --test local_commands; git diff --check

## Failure Semantics

- Fail closed on any missing schema, structure mismatch, current-registry drift, focused-test failure, or diff-hygiene error.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

No synthetic-only substitute is accepted for current registry coverage.
