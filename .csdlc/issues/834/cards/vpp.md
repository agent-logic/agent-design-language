---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "issue-834-internal-review-reconciliation-validation-plan"
issue: 834
task_id: "issue-0834"
run_id: "issue-0834"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06.19][review] Reconcile finalized internal-review predecessor"
branch: "codex/834-internal-review-reconciliation"
generated_at: "2026-09-11T02:59:50.407672+00:00"
card_status: "ready"
status: "in_progress"
initial_pvf_lane: "release-evidence"
planned_pvf_lane: "release-evidence"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "small"
validation_resource_profile: "Small local CPU/disk; deterministic captured readbacks and local Git; no credentials in artifacts."
validation_family: "release-evidence"
validation_size_split: "small"
expected_proof_cost: "small CPU; local JSON/Git plus explicit GitHub readback"
planned_validation_seconds: "300"
planned_validation_tokens: "4000"
issue_goal_ref: "Planning #7 issue834 active session goal"
sprint_goal_ref: "not_applicable; issue-local goal under522"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/834"
  - kind: "stp"
    ref: ".csdlc/issues/834/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/834/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/834/cards/spp.md"
selected_lanes:
  - "Required predecessor reconciliation and negative validator contracts."
parallel_groups:
  - "none; focused sequential proof"
validation_commands:
  - "python3 docs/milestones/v0.92.1/evidence/release/tail-06/issue-834/validate.py; python3 docs/milestones/v0.92.1/evidence/release/tail-06/issue-834/test_validate.py"
failure_policy: "Fail closed on stale state, wrong closing PR, nonancestor merge, missing/duplicate finding or unproven owner."
notes: "#833 owns external report retention; #835 owns final gate correction. No release approval."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Inspect final520 disposition and historical external finding; map14 findings to verified merged owners; capture live closure/ancestry; add current reconciliation and fail-closed validator; run negatives; review exacthead and publish with required CI green.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `release-evidence`
- Planned PVF lane for execution: `release-evidence`

## Selected Validation Lanes

- Required predecessor reconciliation and negative validator contracts.

## Parallelization Plan

- Parallel groups: none; focused sequential proof
- Validation runtime class: `small`
- Validation resource profile: `Small local CPU/disk; deterministic captured readbacks and local Git; no credentials in artifacts.`
- Validation family: `release-evidence`
- Validation size split: `small`

## Goal Accounting Hooks

- Issue goal ref: `Planning #7 issue834 active session goal`
- Sprint goal ref: `not_applicable; issue-local goal under522`
- Goal metrics rollup ref: `not_collected`

## Proof Cost / Runtime Expectations

- Expected proof cost: `small CPU; local JSON/Git plus explicit GitHub readback`
- Planned validation seconds: `300`
- Planned validation token budget: `4000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- python3 docs/milestones/v0.92.1/evidence/release/tail-06/issue-834/validate.py; python3 docs/milestones/v0.92.1/evidence/release/tail-06/issue-834/test_validate.py

## Failure Semantics

- Fail closed on stale state, wrong closing PR, nonancestor merge, missing/duplicate finding or unproven owner.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

#833 owns external report retention; #835 owns final gate correction. No release approval.
