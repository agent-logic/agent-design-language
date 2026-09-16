---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "932-sprint6-coordination-validation-plan"
issue: 932
task_id: "issue-0932"
run_id: "issue-0932"
version: "v0.92.2"
title: "[v0.92.2][Sprint 6] Hardware/provider qualification"
branch: "codex/932-sprint6-coordination"
generated_at: "2026-09-15"
card_status: "ready"
status: "complete"
initial_pvf_lane: "sprint-integration"
planned_pvf_lane: "sprint-integration"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "bounded"
validation_resource_profile: "local record and Git checks plus authenticated GitHub observations and hosted CI"
validation_family: "child evidence and source acceptance reconciliation"
validation_size_split: "12 child acceptance checks plus three delivery/terminal rows"
expected_proof_cost: "bounded local record checks and hosted umbrella CI; no repeated hardware run"
planned_validation_seconds: "900"
planned_validation_tokens: "12000"
issue_goal_ref: "Sprint 6 umbrella #932 execution goal"
sprint_goal_ref: "#932"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/932"
  - kind: "stp"
    ref: ".csdlc/issues/932/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/932/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/932/cards/spp.md"
selected_lanes:
  - "sprint-integration"
parallel_groups:
  - "No child execution remains; umbrella evidence checks may run independently before exact-head synthesis."
validation_commands:
  - "jq ledger checks; git merge-base ancestry; authenticated GitHub observations; native validate; git diff --check"
failure_policy: "Any missing or ambiguous criterion remains incomplete and blocks umbrella publication."
notes: "Retain MLX scope limit, PAIR hardware confound and speculative repair_inconclusive disposition."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Reconcile all three completed qualification lanes, independently review the combined proof and close the umbrella through native publication and terminal cleanup.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `sprint-integration`
- Planned PVF lane for execution: `sprint-integration`

## Selected Validation Lanes

- sprint-integration

## Parallelization Plan

- Parallel groups: No child execution remains; umbrella evidence checks may run independently before exact-head synthesis.
- Validation runtime class: `bounded`
- Validation resource profile: `local record and Git checks plus authenticated GitHub observations and hosted CI`
- Validation family: `child evidence and source acceptance reconciliation`
- Validation size split: `12 child acceptance checks plus three delivery/terminal rows`

## Goal Accounting Hooks

- Issue goal ref: `Sprint 6 umbrella #932 execution goal`
- Sprint goal ref: `#932`
- Goal metrics rollup ref: `not_collected`

## Proof Cost / Runtime Expectations

- Expected proof cost: `bounded local record checks and hosted umbrella CI; no repeated hardware run`
- Planned validation seconds: `900`
- Planned validation token budget: `12000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- jq ledger checks; git merge-base ancestry; authenticated GitHub observations; native validate; git diff --check

## Failure Semantics

- Any missing or ambiguous criterion remains incomplete and blocks umbrella publication.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Retain MLX scope limit, PAIR hardware confound and speculative repair_inconclusive disposition.
