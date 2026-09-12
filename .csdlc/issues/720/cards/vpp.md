---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "720-observatory-live-validation-plan"
issue: 720
task_id: "issue-0720"
run_id: "issue-0720"
version: "v0.92.2"
title: "[v0.92.2][Observatory] Remove retained-mode demo hazards"
branch: "codex/720-observatory-live"
generated_at: "2026-09-12T00:09:24.617447+00:00"
card_status: "ready"
status: "prepared"
initial_pvf_lane: "tooling"
planned_pvf_lane: "tooling"
lane_registry_path: "docs/templates/prompts/1.0.5/pvf_lane_policy.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "bounded"
validation_resource_profile: "Local UI regression plus browser Live proof; no cloud deployment"
validation_family: "Local UI regression plus browser Live proof; no cloud deployment"
validation_size_split: "focused UI suite"
expected_proof_cost: "local CPU only"
planned_validation_seconds: "1800"
planned_validation_tokens: "12000"
issue_goal_ref: "#720 execution goal created before implementation"
sprint_goal_ref: "#934: v0.92.2 Sprint 8 Cloud operations and Observatory"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/720"
  - kind: "stp"
    ref: ".csdlc/issues/720/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/720/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/720/cards/spp.md"
selected_lanes:
  - "tooling: deterministic local UI regression, contract and browser proof; release gate for #720; no cloud access"
parallel_groups:
  - "independent regression files"
validation_commands:
  - "node --test demos/html-observatory/tests/*.test.mjs"
failure_policy: "Missing or failed proof blocks acceptance; read failure is not resource absence; local checks do not prove live deployment."
notes: "Disconnected live values must be explicitly stale; initialization must not seed telemetry from an old packet. Historical integration evidence may remain labelled separately."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Remove retained telemetry loaders, switches and timers; initialize an empty live shell, retain last-known live snapshot on disconnection with stale notice; preserve separately labelled historical evidence; prove transitions and publish reviewed change.

## Lane Registry Inputs

- Registry path: `docs/templates/prompts/1.0.5/pvf_lane_policy.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `tooling`
- Planned PVF lane for execution: `tooling`

## Selected Validation Lanes

- tooling: deterministic local UI regression, contract and browser proof; release gate for #720; no cloud access

## Parallelization Plan

- Parallel groups: independent regression files
- Validation runtime class: `bounded`
- Validation resource profile: `Local UI regression plus browser Live proof; no cloud deployment`
- Validation family: `Local UI regression plus browser Live proof; no cloud deployment`
- Validation size split: `focused UI suite`

## Goal Accounting Hooks

- Issue goal ref: `#720 execution goal created before implementation`
- Sprint goal ref: `#934: v0.92.2 Sprint 8 Cloud operations and Observatory`
- Goal metrics rollup ref: `not_collected`

## Proof Cost / Runtime Expectations

- Expected proof cost: `local CPU only`
- Planned validation seconds: `1800`
- Planned validation token budget: `12000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- node --test demos/html-observatory/tests/*.test.mjs

## Failure Semantics

- Missing or failed proof blocks acceptance; read failure is not resource absence; local checks do not prove live deployment.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Disconnected live values must be explicitly stale; initialization must not seed telemetry from an old packet. Historical integration evidence may remain labelled separately.
