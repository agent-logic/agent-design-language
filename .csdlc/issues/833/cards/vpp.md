---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "external-review-immutable-candidate-validation-plan"
issue: 833
task_id: "issue-0833"
run_id: "issue-0833"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06.18][review] Re-run external review at immutable candidate"
branch: "codex/833-external-review-immutable-candidate"
generated_at: "2026-09-11T17:24:00Z"
card_status: "ready"
status: "READY"
initial_pvf_lane: "documentation"
planned_pvf_lane: "documentation"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "small"
validation_resource_profile: "local CPU/filesystem; read-only GitHub inspection"
validation_family: "external_review_handoff"
validation_size_split: "single document and issue truth"
expected_proof_cost: "Small."
planned_validation_seconds: "not_collected"
planned_validation_tokens: "not_collected"
issue_goal_ref: "Issue #833 session goal"
sprint_goal_ref: "#522"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/833"
  - kind: "stp"
    ref: ".csdlc/issues/833/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/833/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/833/cards/spp.md"
selected_lanes:
  - "documentation"
parallel_groups:
  - "None."
validation_commands:
  - "Focused shell assertions; git cat-file; git diff --check."
failure_policy: "Missing, malformed, substituted, or assignment/report-mismatched SHA blocks publication."
notes: "No mutable inference."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Small deterministic docs proof.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `documentation`
- Planned PVF lane for execution: `documentation`

## Selected Validation Lanes

- documentation

## Parallelization Plan

- Parallel groups: None.
- Validation runtime class: `small`
- Validation resource profile: `local CPU/filesystem; read-only GitHub inspection`
- Validation family: `external_review_handoff`
- Validation size split: `single document and issue truth`

## Goal Accounting Hooks

- Issue goal ref: `Issue #833 session goal`
- Sprint goal ref: `#522`
- Goal metrics rollup ref: `not_collected`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Small.`
- Planned validation seconds: `not_collected`
- Planned validation token budget: `not_collected`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- Focused shell assertions; git cat-file; git diff --check.

## Failure Semantics

- Missing, malformed, substituted, or assignment/report-mismatched SHA blocks publication.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

No mutable inference.
