---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "active-boot-paths-control-plane-guidance-validation-plan"
issue: 837
task_id: "issue-0837"
run_id: "issue-0837"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06.22][architecture] Publish active boot paths and retire stale control-plane guidance"
branch: "codex/837-active-boot-paths-control-plane-guidance"
generated_at: "2026-09-10T23:00:00Z"
card_status: "ready"
status: "READY"
initial_pvf_lane: "review_docs"
planned_pvf_lane: "review_docs"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "small"
validation_resource_profile: "local CPU and filesystem"
validation_family: "documentation_contract"
validation_size_split: "bounded authority docs and boot-path inventory"
expected_proof_cost: "Focused deterministic documentation and contract checks only."
planned_validation_seconds: "180"
planned_validation_tokens: "1500"
issue_goal_ref: "Active issue #837 execution goal"
sprint_goal_ref: "Parent #522"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/837"
  - kind: "stp"
    ref: ".csdlc/issues/837/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/837/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/837/cards/spp.md"
selected_lanes:
  - "review_docs; boot-path contract; negative stale-guidance fixture; diff/path hygiene"
parallel_groups:
  - "Inventory and scan may be independent; final validator serialized."
validation_commands:
  - "bash .csdlc/prepared/issues/837/validate-active-boot-paths.sh; git diff --check"
failure_policy: "Any selector, executable identity, classification, or current-guidance mismatch blocks publication."
notes: "Do not interpret historical evidence or explicit rollback guidance as an ordinary route."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Use a source-backed inventory and scoped current-guidance validator with explicit historical/rollback allowances.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `review_docs`
- Planned PVF lane for execution: `review_docs`

## Selected Validation Lanes

- review_docs; boot-path contract; negative stale-guidance fixture; diff/path hygiene

## Parallelization Plan

- Parallel groups: Inventory and scan may be independent; final validator serialized.
- Validation runtime class: `small`
- Validation resource profile: `local CPU and filesystem`
- Validation family: `documentation_contract`
- Validation size split: `bounded authority docs and boot-path inventory`

## Goal Accounting Hooks

- Issue goal ref: `Active issue #837 execution goal`
- Sprint goal ref: `Parent #522`
- Goal metrics rollup ref: `not_collected`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Focused deterministic documentation and contract checks only.`
- Planned validation seconds: `180`
- Planned validation token budget: `1500`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- bash .csdlc/prepared/issues/837/validate-active-boot-paths.sh; git diff --check

## Failure Semantics

- Any selector, executable identity, classification, or current-guidance mismatch blocks publication.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Do not interpret historical evidence or explicit rollback guidance as an ordinary route.
