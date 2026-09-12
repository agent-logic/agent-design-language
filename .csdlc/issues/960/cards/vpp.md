---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "960-shutdown-barrier-validation-plan"
issue: 960
task_id: "issue-0960"
run_id: "issue-0960"
version: "1.0.5"
title: "Fix Runtime shutdown barrier acknowledgment race (v0.92.2)"
branch: "codex/960-shutdown-barrier"
generated_at: "2026-09-12T05:59:59.590095+00:00"
card_status: "ready"
status: "planned"
initial_pvf_lane: "runtime"
planned_pvf_lane: "runtime"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "runtime"
validation_resource_profile: "bounded local CPU/disk and loopback only"
validation_family: "regression"
validation_size_split: "focused"
expected_proof_cost: "unknown; no estimate recorded"
planned_validation_seconds: "unknown; no estimate recorded"
planned_validation_tokens: "unknown; no estimate recorded"
issue_goal_ref: "issue960 within root Sprint2 goal"
sprint_goal_ref: "#928"
goal_metrics_rollup_ref: "unknown; not yet collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/960"
  - kind: "stp"
    ref: ".csdlc/issues/960/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/960/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/960/cards/spp.md"
selected_lanes:
  - "runtime"
parallel_groups:
  - "unit race/failure tests and CLI smoke after build"
validation_commands:
  - "Focused deterministic unit race and sink-error tests; actual local CLI shutdown smoke. Local CPU/disk/mock control plane only, no paid calls. Required runtime regression proof; broader CI integration deferred."
failure_policy: "Fail closed; preserve errors; no retry or assertion weakening."
notes: "Supported heartbeat race proven; original CI cause unknown. Local sink acknowledgment is not remote OTLP delivery."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

1. Preserve source cf25e273a39499c3d1b699fa7d9e461266024e0c diagnostic packet. 2. Add compatible synchronous event-specific sink receipt under publication lock. 3. Consume receipt at shutdown and retain child diagnostics. 4. Run focused deterministic race/failure and installed local CLI proof. 5. Independent exact-head review, native publication and required CI; no merge authorization.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `runtime`
- Planned PVF lane for execution: `runtime`

## Selected Validation Lanes

- runtime

## Parallelization Plan

- Parallel groups: unit race/failure tests and CLI smoke after build
- Validation runtime class: `runtime`
- Validation resource profile: `bounded local CPU/disk and loopback only`
- Validation family: `regression`
- Validation size split: `focused`

## Goal Accounting Hooks

- Issue goal ref: `issue960 within root Sprint2 goal`
- Sprint goal ref: `#928`
- Goal metrics rollup ref: `unknown; not yet collected`

## Proof Cost / Runtime Expectations

- Expected proof cost: `unknown; no estimate recorded`
- Planned validation seconds: `unknown; no estimate recorded`
- Planned validation token budget: `unknown; no estimate recorded`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- Focused deterministic unit race and sink-error tests; actual local CLI shutdown smoke. Local CPU/disk/mock control plane only, no paid calls. Required runtime regression proof; broader CI integration deferred.

## Failure Semantics

- Fail closed; preserve errors; no retry or assertion weakening.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Supported heartbeat race proven; original CI cause unknown. Local sink acknowledgment is not remote OTLP delivery.
