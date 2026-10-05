---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "cf05-independent-beta1-qualification-validation-plan"
issue: 1150
task_id: "issue-1150"
run_id: "issue-1150"
version: "1.0.5"
title: "[v0.93][CodeFriend] Complete independent Beta 1 installed qualification"
branch: "codex/1150-codefriend-independent-qualification"
generated_at: "2026-10-04T06:03:30Z"
card_status: "blocked"
status: "blocked_on_prerequisites_candidate_and_execution_authority"
initial_pvf_lane: "installed_integration"
planned_pvf_lane: "installed_integration"
lane_registry_path: "docs/pvf/validation-lanes.json"
lane_registry_template_set: "current repository registry at execution time"
validation_runtime_class: "multi_platform_installed_integration"
validation_resource_profile: "Bounded local CPU/files and owned nonpublic CI or staging; paid providers require separate issue-specific authority; public deployment, audience activation and live launch are excluded."
validation_family: "independent_installed_qualification"
validation_size_split: "12 contract-derived tuples plus 24 source obligations; actual scenario denominator may only grow explicitly"
expected_proof_cost: "unknown until installed candidate, provider authority and reusable evidence are admitted"
planned_validation_seconds: "unknown"
planned_validation_tokens: "unknown"
issue_goal_ref: "Create a fresh issue-bound execution goal only after every execution gate is admitted and before qualification starts."
sprint_goal_ref: "Sprint-3 umbrella #1229"
goal_metrics_rollup_ref: "Planning #13 preparation-content review passed; validation execution and qualification review have not run."
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1150"
  - kind: "stp"
    ref: ".git/csdlc-v3/local/projections/1150/cards/stp.md"
  - kind: "sip"
    ref: ".git/csdlc-v3/local/projections/1150/cards/sip.md"
  - kind: "spp"
    ref: ".git/csdlc-v3/local/projections/1150/cards/spp.md"
selected_lanes:
  - "Installed-consumer identity/custody; macOS/Linux x ADL/external repository x hosted-mode build/local-agent website/CLI matrix; waiting-list and hosted-mode local or owned nonpublic staging deployability; external-tester ADL/external OSS/PR-review journeys; refusal/privacy/interruption/recovery/cost/deletion/rollback; human HTML/PDF inspection; independent findings review."
parallel_groups:
  - "Only independent, authority-compatible cells may run in parallel after shared exact-candidate and prerequisite admission; unknown-effect request #1149 remains serialized and never blindly replayed."
validation_commands:
  - "Blocked during preparation. The execution owner must record exact approved commands after candidate custody and provider/resource authority are known, then bind actual scenario IDs to every source-mapped Q row."
failure_policy: "Fail closed. Missing inputs, skipped cells, zero scenarios, unmatched versions, unresolved citations, unknown effects, failed obligations, missing waiting-list or hosted-mode deployability proof, privacy leakage or unresolved actionable findings block qualification. Absence of public deployment alone is not a qualification failure."
notes: "Historical state is 0/12 accepted and 24 unresolved obligations. Six exports across three formats are partial evidence only. Contract-derived tuples are not recovered historical cell IDs."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

The installed independent qualification is planned in a bound preparation context. The exact Q01-Q24 requirement text is recovered but carries no execution credit. Prove the waiting-list and hosted-mode paths are built, tested and deployable through local or owned nonpublic staging evidence; public deployment and live launch are outside v0.93.1 qualification. Do not execute until accepted dependencies, exact candidate custody, applicable provider authority for genuinely provider-backed local scenarios, and complete prompt-only SRP readiness are present.

## Lane Registry Inputs

- Registry path: `docs/pvf/validation-lanes.json`
- Registry template set: `current repository registry at execution time`
- Initial PVF lane from issue creation: `installed_integration`
- Planned PVF lane for execution: `installed_integration`

## Selected Validation Lanes

- Installed-consumer identity/custody; macOS/Linux x ADL/external repository x hosted-mode build/local-agent website/CLI matrix; waiting-list and hosted-mode local or owned nonpublic staging deployability; external-tester ADL/external OSS/PR-review journeys; refusal/privacy/interruption/recovery/cost/deletion/rollback; human HTML/PDF inspection; independent findings review.

## Parallelization Plan

- Parallel groups: Only independent, authority-compatible cells may run in parallel after shared exact-candidate and prerequisite admission; unknown-effect request #1149 remains serialized and never blindly replayed.
- Validation runtime class: `multi_platform_installed_integration`
- Validation resource profile: `Bounded local CPU/files and owned nonpublic CI or staging; paid providers require separate issue-specific authority; public deployment, audience activation and live launch are excluded.`
- Validation family: `independent_installed_qualification`
- Validation size split: `12 contract-derived tuples plus 24 source obligations; actual scenario denominator may only grow explicitly`

## Goal Accounting Hooks

- Issue goal ref: `Create a fresh issue-bound execution goal only after every execution gate is admitted and before qualification starts.`
- Sprint goal ref: `Sprint-3 umbrella #1229`
- Goal metrics rollup ref: `Planning #13 preparation-content review passed; validation execution and qualification review have not run.`

## Proof Cost / Runtime Expectations

- Expected proof cost: `unknown until installed candidate, provider authority and reusable evidence are admitted`
- Planned validation seconds: `unknown`
- Planned validation token budget: `unknown`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- Blocked during preparation. The execution owner must record exact approved commands after candidate custody and provider/resource authority are known, then bind actual scenario IDs to every source-mapped Q row.

## Failure Semantics

- Fail closed. Missing inputs, skipped cells, zero scenarios, unmatched versions, unresolved citations, unknown effects, failed obligations, missing waiting-list or hosted-mode deployability proof, privacy leakage or unresolved actionable findings block qualification. Absence of public deployment alone is not a qualification failure.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Historical state is 0/12 accepted and 24 unresolved obligations. Six exports across three formats are partial evidence only. Contract-derived tuples are not recovered historical cell IDs.
