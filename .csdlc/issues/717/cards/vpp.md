---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "polis-capability-orientation-validation-plan"
issue: 717
task_id: "issue-0717"
run_id: "issue-0717"
version: "1.0.4"
title: "[v0.92.1][Runtime] Teach admitted agents about Polis modules and capabilities"
branch: "codex/717-polis-capability-orientation"
generated_at: "2026-09-09T18:30:00Z"
card_status: "approved"
status: "ready"
initial_pvf_lane: "runtime-focused"
planned_pvf_lane: "runtime-focused-docs-contract"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.4"
validation_runtime_class: "bounded-runtime"
validation_resource_profile: "local-cpu"
validation_family: "runtime-agent-orientation"
validation_size_split: "focused-unit-adjacent-regression-docs-contract"
expected_proof_cost: "bounded local validation only"
planned_validation_seconds: "900"
planned_validation_tokens: "12000"
issue_goal_ref: "issue-717-session-goal"
sprint_goal_ref: "v0.92.1-bugfix"
goal_metrics_rollup_ref: "v0.92.1-bugfix"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/717"
  - kind: "stp"
    ref: ".csdlc/issues/717/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/717/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/717/cards/spp.md"
selected_lanes:
  - "focused orientation unit tests; adjacent first-turn delivery tests; Markdown/inventory contract checks; formatting and diff hygiene; bounded subagent review"
parallel_groups:
  - "Run deterministic local test filters together; review only after the candidate is stable."
validation_commands:
  - "cargo test --manifest-path adl-runtime-kernel/Cargo.toml agent_orientation; cargo test --manifest-path adl-runtime-kernel/Cargo.toml orientation; cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml -- --check; git diff --check"
failure_policy: "Fail closed on missing, duplicate, stale, or invented capability identifiers; missing authority distinctions; first-turn/provenance regression; formatting failure; or actionable review finding."
notes: "A prose keyword scan alone is insufficient; inventory identity and exact membership must be validated."
---

Canonical Template Source: `docs/templates/prompts/1.0.4/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Prove source-grounded Welcome Package coverage and deterministic drift rejection without external calls.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.4`
- Initial PVF lane from issue creation: `runtime-focused`
- Planned PVF lane for execution: `runtime-focused-docs-contract`

## Selected Validation Lanes

- focused orientation unit tests; adjacent first-turn delivery tests; Markdown/inventory contract checks; formatting and diff hygiene; bounded subagent review

## Parallelization Plan

- Parallel groups: Run deterministic local test filters together; review only after the candidate is stable.
- Validation runtime class: `bounded-runtime`
- Validation resource profile: `local-cpu`
- Validation family: `runtime-agent-orientation`
- Validation size split: `focused-unit-adjacent-regression-docs-contract`

## Goal Accounting Hooks

- Issue goal ref: `issue-717-session-goal`
- Sprint goal ref: `v0.92.1-bugfix`
- Goal metrics rollup ref: `v0.92.1-bugfix`

## Proof Cost / Runtime Expectations

- Expected proof cost: `bounded local validation only`
- Planned validation seconds: `900`
- Planned validation token budget: `12000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- cargo test --manifest-path adl-runtime-kernel/Cargo.toml agent_orientation; cargo test --manifest-path adl-runtime-kernel/Cargo.toml orientation; cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml -- --check; git diff --check

## Failure Semantics

- Fail closed on missing, duplicate, stale, or invented capability identifiers; missing authority distinctions; first-turn/provenance regression; formatting failure; or actionable review finding.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

A prose keyword scan alone is insufficient; inventory identity and exact membership must be validated.
