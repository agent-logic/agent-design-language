---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "admission-a2a-outbox-validation-plan"
issue: 758
task_id: "issue-0758"
run_id: "issue-0758"
version: "1.0.4"
title: "[v0.92.1][TAIL-06.02][runtime] Persist and recover admission-triggered A2A initiation"
branch: "codex/758-admission-a2a-outbox"
generated_at: "2026-09-09T00:00:00Z"
card_status: "approved"
status: "ready"
initial_pvf_lane: "runtime-focused"
planned_pvf_lane: "runtime-focused-plus-demo"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.4"
validation_runtime_class: "bounded-runtime"
validation_resource_profile: "local-cpu-plus-existing-real-provider-demo"
validation_family: "runtime-recovery-and-a2a"
validation_size_split: "focused-unit-integration-owner-lane-demo"
expected_proof_cost: "bounded local validation plus one real-agent demo"
planned_validation_seconds: "1800"
planned_validation_tokens: "20000"
issue_goal_ref: "issue-758-session-goal"
sprint_goal_ref: "issue-522-remediation-wave"
goal_metrics_rollup_ref: "issue-522-remediation-wave"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/758"
  - kind: "stp"
    ref: ".csdlc/issues/758/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/758/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/758/cards/spp.md"
selected_lanes:
  - "runtime focused unit/integration; persistence restart/replay; conversation-ledger deduplication; Runtime owner lane; autonomous real-agent demo"
parallel_groups:
  - "Run independent unit/integration tests in parallel where the harness is isolated; serialize restart and real-agent demo proof."
validation_commands:
  - "cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml <focused issue-758 tests>; bash adl/tools/run_owner_validation_lane.sh runtime; issue-specific autonomous A2A demo command selected from the existing demo surface"
failure_policy: "Fail closed on any product test, recovery gap, duplicate greeting, missing disposition, owner-lane failure, or non-proving demo. Fix failures before publication."
notes: "A passing transport test is insufficient; proof must exercise persistence and recovery after the admission response boundary."
---

Canonical Template Source: `docs/templates/prompts/1.0.4/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Prove the durable obligation from admission commit through retries, restart recovery, ledger deduplication, observable terminal state, and a real autonomous greeting.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.4`
- Initial PVF lane from issue creation: `runtime-focused`
- Planned PVF lane for execution: `runtime-focused-plus-demo`

## Selected Validation Lanes

- runtime focused unit/integration; persistence restart/replay; conversation-ledger deduplication; Runtime owner lane; autonomous real-agent demo

## Parallelization Plan

- Parallel groups: Run independent unit/integration tests in parallel where the harness is isolated; serialize restart and real-agent demo proof.
- Validation runtime class: `bounded-runtime`
- Validation resource profile: `local-cpu-plus-existing-real-provider-demo`
- Validation family: `runtime-recovery-and-a2a`
- Validation size split: `focused-unit-integration-owner-lane-demo`

## Goal Accounting Hooks

- Issue goal ref: `issue-758-session-goal`
- Sprint goal ref: `issue-522-remediation-wave`
- Goal metrics rollup ref: `issue-522-remediation-wave`

## Proof Cost / Runtime Expectations

- Expected proof cost: `bounded local validation plus one real-agent demo`
- Planned validation seconds: `1800`
- Planned validation token budget: `20000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml <focused issue-758 tests>; bash adl/tools/run_owner_validation_lane.sh runtime; issue-specific autonomous A2A demo command selected from the existing demo surface

## Failure Semantics

- Fail closed on any product test, recovery gap, duplicate greeting, missing disposition, owner-lane failure, or non-proving demo. Fix failures before publication.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

A passing transport test is insufficient; proof must exercise persistence and recovery after the admission response boundary.
