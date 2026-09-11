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
generated_at: "2026-09-11T21:51:25.506803+00:00"
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
  - "deterministic local planning/docs/Python contract; native six-card validation"
parallel_groups:
  - "none"
validation_commands:
  - "python3 docs/milestones/v0.92.2/validate_planning.py --self-test; all eleven retained issue-launch validators; native six-card validate; relative-link and diff checks; independent live and exact-head review."
failure_policy: "Fail on missing/duplicate issue identities, stale or non-independent reviews/readbacks, native receipt mismatch, missing inherited requirements, graph/projection drift, weakened final completion gates or partial-work acceptance."
notes: "No Runtime/provider/cloud execution applies to this planning correction."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Validate all 69 core issue bindings, 60 birth and nine existing review/native-evidence records, separate sidecar membership, complete task contracts and current card truth.

## Lane Registry Inputs

- Registry path: `docs/milestones/v0.92.2/validate_planning.py`
- Registry template set: `planning_contract_v0.92.2`
- Initial PVF lane from issue creation: `docs`
- Planned PVF lane for execution: `docs`

## Selected Validation Lanes

- deterministic local planning/docs/Python contract; native six-card validation

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

- python3 docs/milestones/v0.92.2/validate_planning.py --self-test; all eleven retained issue-launch validators; native six-card validate; relative-link and diff checks; independent live and exact-head review.

## Failure Semantics

- Fail on missing/duplicate issue identities, stale or non-independent reviews/readbacks, native receipt mismatch, missing inherited requirements, graph/projection drift, weakened final completion gates or partial-work acceptance.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

No Runtime/provider/cloud execution applies to this planning correction.
