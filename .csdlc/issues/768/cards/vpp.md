---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "tail-02-candidate-safe-reproduction-validation-plan"
issue: 768
task_id: "issue-0768"
run_id: "issue-0768"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06.12][docs] Make the TAIL-02 reproduction route candidate-safe"
branch: "codex/768-tail-02-candidate-safe-reproduction"
generated_at: "<timestamp>"
card_status: "ready"
status: "<status>"
initial_pvf_lane: "docs_contract"
planned_pvf_lane: "docs_contract"
lane_registry_path: "<lane_registry_path>"
lane_registry_template_set: "<lane_registry_template_set>"
validation_runtime_class: "deterministic_local_cpu"
validation_resource_profile: "small"
validation_family: "documentation_contract"
validation_size_split: "focused"
expected_proof_cost: "<expected_proof_cost>"
planned_validation_seconds: "120"
planned_validation_tokens: "3000"
issue_goal_ref: "<issue_goal_ref>"
sprint_goal_ref: "<sprint_goal_ref>"
goal_metrics_rollup_ref: "<goal_metrics_rollup_ref>"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/768"
  - kind: "stp"
    ref: ".csdlc/issues/768/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/768/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/768/cards/spp.md"
selected_lanes:
  - "historical_source; candidate_linkage; tracked_growth; negative_guards; diff_hygiene"
parallel_groups:
  - "<parallel_groups_inline>"
validation_commands:
  - "ruby validator --historical; ruby validator --candidate-linkage; ruby validator --all; ruby tracked-growth fixture; git diff --check"
failure_policy: "Fail on identity, ancestry, content hash, decision, reconciliation, or denominator-boundary drift."
notes: "<notes_risks_inline>"
---

Canonical Template Source: `docs/templates/prompts/1.0.4/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Use deterministic local Git/Ruby proof only; preserve all existing candidate guards and prove tracked growth independently.

## Lane Registry Inputs

- Registry path: `<lane_registry_path>`
- Registry template set: `<lane_registry_template_set>`
- Initial PVF lane from issue creation: `docs_contract`
- Planned PVF lane for execution: `docs_contract`

## Selected Validation Lanes

- historical_source; candidate_linkage; tracked_growth; negative_guards; diff_hygiene

## Parallelization Plan

- Parallel groups: <parallel_groups_inline>
- Validation runtime class: `deterministic_local_cpu`
- Validation resource profile: `small`
- Validation family: `documentation_contract`
- Validation size split: `focused`

## Goal Accounting Hooks

- Issue goal ref: `<issue_goal_ref>`
- Sprint goal ref: `<sprint_goal_ref>`
- Goal metrics rollup ref: `<goal_metrics_rollup_ref>`

## Proof Cost / Runtime Expectations

- Expected proof cost: `<expected_proof_cost>`
- Planned validation seconds: `120`
- Planned validation token budget: `3000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- ruby validator --historical; ruby validator --candidate-linkage; ruby validator --all; ruby tracked-growth fixture; git diff --check

## Failure Semantics

- Fail on identity, ancestry, content hash, decision, reconciliation, or denominator-boundary drift.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

<notes_risks_inline>
