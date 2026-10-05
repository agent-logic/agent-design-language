---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "<slug>-validation-plan"
issue: 1223
task_id: "issue-1223"
run_id: "issue-1223"
version: "1.0.5"
title: "[v0.93.1][RD-07 final] Complete coherent source retirement after the repository split"
branch: "codex/1223-1223-coherent-source-retirement"
generated_at: "<timestamp>"
card_status: "ready"
status: "<status>"
initial_pvf_lane: "local_contract"
planned_pvf_lane: "installed_integration"
lane_registry_path: "<lane_registry_path>"
lane_registry_template_set: "<lane_registry_template_set>"
validation_runtime_class: "local_deterministic_plus_macos_installed_integration"
validation_resource_profile: "small_CPU_for_retirement_guard; bounded_CPU_disk_for_five_public_packages; no_provider_cloud_or_service_changes"
validation_family: "source_retirement_and_public_package_consumption"
validation_size_split: "small_guard_and_mutation_negatives; separate_five_package_and_installed_lanes"
expected_proof_cost: "Warm-cache estimate; each native validator capped at300seconds; timeout remains failure, not waiver."
planned_validation_seconds: "1800"
planned_validation_tokens: "6000"
issue_goal_ref: "issue-1223-preparation-remediation"
sprint_goal_ref: "<sprint_goal_ref>"
goal_metrics_rollup_ref: "<goal_metrics_rollup_ref>"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1223"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
  - kind: "spp"
    ref: "<spp_card>"
selected_lanes:
  - "retirement-contract; five-public-package-tests; installed-public-packages-1223"
parallel_groups:
  - "Serial within the single bound issue owner; reuse exact-candidate producer proof where admitted."
validation_commands:
  - "cargo test --locked --manifest-path tools/source_retirement/validator/Cargo.toml --test retirement_contract; five declared public-package cargo test commands; cargo test --locked --manifest-path tools/source_retirement/validator/Cargo.toml --test installed_public_packages"
failure_policy: "Fail closed on missing original row, retained source/evidence loss, corrupt or absent rollback, new deletion, unclassified direct reference, private dependency, failed consumer or timeout. Guard success is preparation proof only; final installed proof and independent exact-head review remain mandatory."
notes: "PVF: retirement_contract deterministic local contract, no network or provider, small CPU/disk, required preparation and retirement gate. installed_public_packages is real macOS sandboxed installed-integration proof with exact artifacts and scenario counts; required final gate, not yet run. Same-host restored baseline is not independent storage custody; changed-source holds require fresh rollback before deletion."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

<plan_summary>

## Lane Registry Inputs

- Registry path: `<lane_registry_path>`
- Registry template set: `<lane_registry_template_set>`
- Initial PVF lane from issue creation: `local_contract`
- Planned PVF lane for execution: `installed_integration`

## Selected Validation Lanes

- retirement-contract; five-public-package-tests; installed-public-packages-1223

## Parallelization Plan

- Parallel groups: Serial within the single bound issue owner; reuse exact-candidate producer proof where admitted.
- Validation runtime class: `local_deterministic_plus_macos_installed_integration`
- Validation resource profile: `small_CPU_for_retirement_guard; bounded_CPU_disk_for_five_public_packages; no_provider_cloud_or_service_changes`
- Validation family: `source_retirement_and_public_package_consumption`
- Validation size split: `small_guard_and_mutation_negatives; separate_five_package_and_installed_lanes`

## Goal Accounting Hooks

- Issue goal ref: `issue-1223-preparation-remediation`
- Sprint goal ref: `<sprint_goal_ref>`
- Goal metrics rollup ref: `<goal_metrics_rollup_ref>`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Warm-cache estimate; each native validator capped at300seconds; timeout remains failure, not waiver.`
- Planned validation seconds: `1800`
- Planned validation token budget: `6000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- cargo test --locked --manifest-path tools/source_retirement/validator/Cargo.toml --test retirement_contract; five declared public-package cargo test commands; cargo test --locked --manifest-path tools/source_retirement/validator/Cargo.toml --test installed_public_packages

## Failure Semantics

- Fail closed on missing original row, retained source/evidence loss, corrupt or absent rollback, new deletion, unclassified direct reference, private dependency, failed consumer or timeout. Guard success is preparation proof only; final installed proof and independent exact-head review remain mandatory.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

PVF: retirement_contract deterministic local contract, no network or provider, small CPU/disk, required preparation and retirement gate. installed_public_packages is real macOS sandboxed installed-integration proof with exact artifacts and scenario counts; required final gate, not yet run. Same-host restored baseline is not independent storage custody; changed-source holds require fresh rollback before deletion.
