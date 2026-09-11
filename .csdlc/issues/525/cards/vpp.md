---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "next-milestone-review-validation-plan"
issue: 525
task_id: "issue-0525"
run_id: "issue-0525"
version: "v0.92.1"
title: "[v0.92.1][TAIL-09] Next milestone review pass"
branch: "codex/525-next-milestone-review"
generated_at: "2026-09-11T00:00:00Z"
card_status: "ready"
status: "READY"
initial_pvf_lane: "docs_only"
planned_pvf_lane: "docs_only"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "small"
validation_resource_profile: "local CPU and filesystem; no providers or cloud"
validation_family: "milestone_planning_consistency"
validation_size_split: "planning corpus and negative self-tests"
expected_proof_cost: "Small local documentation proof."
planned_validation_seconds: "not_collected"
planned_validation_tokens: "not_collected"
issue_goal_ref: "Issue #525 session goal"
sprint_goal_ref: "v0.92.1 release tail"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/525"
  - kind: "stp"
    ref: ".csdlc/issues/525/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/525/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/525/cards/spp.md"
selected_lanes:
  - "docs_only planning validator, YAML/JSON parse, native card validation, diff hygiene"
parallel_groups:
  - "Planning validation and diff hygiene may run independently after edits."
validation_commands:
  - "python3 docs/milestones/v0.92.2/validate_planning.py --self-test; native v3 validate; git diff --check origin/main...HEAD"
failure_policy: "Any missing row, mismatched count, unauthorized issue binding, dependency drift, parser failure, or diff defect blocks publication."
notes: "Validator must make omission of either new row observable."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Validate exact work-package count, uniqueness, issue authority, dependencies, cross-file projection parity and explicit OBS-S3/ARCH-ADR contracts.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `docs_only`
- Planned PVF lane for execution: `docs_only`

## Selected Validation Lanes

- docs_only planning validator, YAML/JSON parse, native card validation, diff hygiene

## Parallelization Plan

- Parallel groups: Planning validation and diff hygiene may run independently after edits.
- Validation runtime class: `small`
- Validation resource profile: `local CPU and filesystem; no providers or cloud`
- Validation family: `milestone_planning_consistency`
- Validation size split: `planning corpus and negative self-tests`

## Goal Accounting Hooks

- Issue goal ref: `Issue #525 session goal`
- Sprint goal ref: `v0.92.1 release tail`
- Goal metrics rollup ref: `not_collected`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Small local documentation proof.`
- Planned validation seconds: `not_collected`
- Planned validation token budget: `not_collected`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- python3 docs/milestones/v0.92.2/validate_planning.py --self-test; native v3 validate; git diff --check origin/main...HEAD

## Failure Semantics

- Any missing row, mismatched count, unauthorized issue binding, dependency drift, parser failure, or diff defect blocks publication.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Validator must make omission of either new row observable.
