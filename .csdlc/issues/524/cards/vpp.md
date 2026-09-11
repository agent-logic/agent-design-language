---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "v0922-closeout-plan-validation-plan"
issue: 524
task_id: "issue-0524"
run_id: "issue-0524"
version: "v0.92.1"
title: "[v0.92.1][TAIL-08] Next-milestone closeout plan"
branch: "codex/524-v0922-closeout-plan"
generated_at: "2026-09-11T02:25:31Z"
card_status: "ready"
status: "executed"
initial_pvf_lane: "docs-bounded"
planned_pvf_lane: "docs-bounded"
lane_registry_path: "docs/templates/prompts/current.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "deterministic_local_cpu"
validation_resource_profile: "small"
validation_family: "milestone_planning"
validation_size_split: "focused"
expected_proof_cost: "small deterministic local documentation check"
planned_validation_seconds: "60"
planned_validation_tokens: "unknown"
issue_goal_ref: "Issue #524 session goal"
sprint_goal_ref: "v0.92.1 TAIL-08"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/524"
  - kind: "stp"
    ref: ".csdlc/issues/524/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/524/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/524/cards/spp.md"
selected_lanes:
  - "planning-structure; dependency-graph; denominator; source-disposition; diff-hygiene; independent-review"
parallel_groups:
  - "planning validator and diff hygiene may run independently after edits settle"
validation_commands:
  - "python3 docs/milestones/v0.92.2/validate_planning.py --self-test; git diff --check; atomic-result parity check"
failure_policy: "Fail on any count mismatch, duplicate/missing row, invalid dependency, stale source disposition, release-tail drift, or implied execution authority."
notes: "No runtime, cloud, provider, paid, or release execution is claimed."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Validate the complete number-free planning package, its 41 distinct primary results, dependency graph, release tail, and source dispositions.

## Lane Registry Inputs

- Registry path: `docs/templates/prompts/current.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `docs-bounded`
- Planned PVF lane for execution: `docs-bounded`

## Selected Validation Lanes

- planning-structure; dependency-graph; denominator; source-disposition; diff-hygiene; independent-review

## Parallelization Plan

- Parallel groups: planning validator and diff hygiene may run independently after edits settle
- Validation runtime class: `deterministic_local_cpu`
- Validation resource profile: `small`
- Validation family: `milestone_planning`
- Validation size split: `focused`

## Goal Accounting Hooks

- Issue goal ref: `Issue #524 session goal`
- Sprint goal ref: `v0.92.1 TAIL-08`
- Goal metrics rollup ref: `not_collected`

## Proof Cost / Runtime Expectations

- Expected proof cost: `small deterministic local documentation check`
- Planned validation seconds: `60`
- Planned validation token budget: `unknown`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- python3 docs/milestones/v0.92.2/validate_planning.py --self-test; git diff --check; atomic-result parity check

## Failure Semantics

- Fail on any count mismatch, duplicate/missing row, invalid dependency, stale source disposition, release-tail drift, or implied execution authority.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

No runtime, cloud, provider, paid, or release execution is claimed.
