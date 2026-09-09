---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "sor-authority-notice-validation-plan"
issue: 804
task_id: "issue-0804"
run_id: "issue-0804"
version: "v0.92.1"
title: "[v0.92.1][defect] Align active SOR template authority notice with C-SDLC v3"
branch: "codex/804-sor-authority-notice"
generated_at: "2026-09-09T18:25:45.944998+00:00"
card_status: "ready"
status: "IN_PROGRESS"
initial_pvf_lane: "prompt_template"
planned_pvf_lane: "prompt_template"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.4"
validation_runtime_class: "small"
validation_resource_profile: "local CPU, filesystem and Git; no cloud"
validation_family: "prompt_template_contract"
validation_size_split: "single focused local proof"
expected_proof_cost: "small local checks; no Rust rebuild or runtime suite"
planned_validation_seconds: "not_collected"
planned_validation_tokens: "not_collected"
issue_goal_ref: "Issue #804 session goal in current Codex task"
sprint_goal_ref: "not_applicable: standalone defect"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/804"
  - kind: "stp"
    ref: ".csdlc/issues/804/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/804/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/804/cards/spp.md"
selected_lanes:
  - "prompt_template; required issue-local proof"
parallel_groups:
  - "Serialized native edits and validation; independent review after proof"
validation_commands:
  - "Native csdlc edit and validate using issue-local requests and active current.json; python3 adl/tools/test_prompt_template_structure_schemas.py; git diff --check; authority parity assertions retained in .csdlc/evidence/804/authority-proof.json"
failure_policy: "Fix any notice, render or schema mismatch before publication."
notes: "Historical cards retain source-time text; no historical rewrite or runtime authority change."
---

Canonical Template Source: `docs/templates/prompts/1.0.4/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Small deterministic prompt-template contract proof. Native render and six-card validation exercise active schema; Python smoke reads all six schemas. Exact notice/hash checks retain semantic limits.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.4`
- Initial PVF lane from issue creation: `prompt_template`
- Planned PVF lane for execution: `prompt_template`

## Selected Validation Lanes

- prompt_template; required issue-local proof

## Parallelization Plan

- Parallel groups: Serialized native edits and validation; independent review after proof
- Validation runtime class: `small`
- Validation resource profile: `local CPU, filesystem and Git; no cloud`
- Validation family: `prompt_template_contract`
- Validation size split: `single focused local proof`

## Goal Accounting Hooks

- Issue goal ref: `Issue #804 session goal in current Codex task`
- Sprint goal ref: `not_applicable: standalone defect`
- Goal metrics rollup ref: `not_collected`

## Proof Cost / Runtime Expectations

- Expected proof cost: `small local checks; no Rust rebuild or runtime suite`
- Planned validation seconds: `not_collected`
- Planned validation token budget: `not_collected`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- Native csdlc edit and validate using issue-local requests and active current.json; python3 adl/tools/test_prompt_template_structure_schemas.py; git diff --check; authority parity assertions retained in .csdlc/evidence/804/authority-proof.json

## Failure Semantics

- Fix any notice, render or schema mismatch before publication.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Historical cards retain source-time text; no historical rewrite or runtime authority change.
