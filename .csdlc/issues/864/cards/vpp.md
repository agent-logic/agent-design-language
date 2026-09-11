---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "v0922-wp01-validation-plan"
issue: 864
task_id: "issue-0864"
run_id: "issue-0864"
version: "1.0.5"
title: "[v0.92.2][WP-01][planning] Publish and open the CodeFriend Beta 1 execution wave"
branch: "codex/864-v0922-wp01"
generated_at: "2026-09-11T21:26:03Z"
card_status: "ready"
status: "complete"
initial_pvf_lane: "docs"
planned_pvf_lane: "docs"
lane_registry_path: "docs/milestones/v0.92.2/validate_planning.py"
lane_registry_template_set: "planning_contract_v0.92.2"
validation_runtime_class: "short"
validation_resource_profile: "local_cpu"
validation_family: "docs_contract"
validation_size_split: "focused"
expected_proof_cost: "zero_paid_resources"
planned_validation_seconds: "30"
planned_validation_tokens: "unknown"
issue_goal_ref: "issue-864-reconciliation-goal"
sprint_goal_ref: "not_applicable"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/864"
  - kind: "stp"
    ref: ".csdlc/issues/864/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/864/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/864/cards/spp.md"
selected_lanes:
  - "docs planning contract and issue-inventory reconciliation"
parallel_groups:
  - "none"
validation_commands:
  - "python3 docs/milestones/v0.92.2/validate_planning.py --self-test; git diff --check; authenticated issue list readback"
failure_policy: "Fail closed on denominator, duplicate, dependency, tail-order, or issue-binding drift."
notes: "No Runtime or release proof is claimed."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Prove planning consistency and rejection of malformed projections.

## Lane Registry Inputs

- Registry path: `docs/milestones/v0.92.2/validate_planning.py`
- Registry template set: `planning_contract_v0.92.2`
- Initial PVF lane from issue creation: `docs`
- Planned PVF lane for execution: `docs`

## Selected Validation Lanes

- docs planning contract and issue-inventory reconciliation

## Parallelization Plan

- Parallel groups: none
- Validation runtime class: `short`
- Validation resource profile: `local_cpu`
- Validation family: `docs_contract`
- Validation size split: `focused`

## Goal Accounting Hooks

- Issue goal ref: `issue-864-reconciliation-goal`
- Sprint goal ref: `not_applicable`
- Goal metrics rollup ref: `not_collected`

## Proof Cost / Runtime Expectations

- Expected proof cost: `zero_paid_resources`
- Planned validation seconds: `30`
- Planned validation token budget: `unknown`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- python3 docs/milestones/v0.92.2/validate_planning.py --self-test; git diff --check; authenticated issue list readback

## Failure Semantics

- Fail closed on denominator, duplicate, dependency, tail-order, or issue-binding drift.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

No Runtime or release proof is claimed.
