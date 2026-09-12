---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "855-provider-neutral-lifecycle-validation-plan"
issue: 855
task_id: "issue-0855"
run_id: "issue-0855"
version: "v0.92.2"
title: "[v0.92.2][RT-PROVIDER] Provider-neutral dynamic agent lifecycle"
branch: "codex/855-provider-neutral-lifecycle"
generated_at: "2026-09-11T23:55:22.557345+00:00"
card_status: "ready"
status: "prepared"
initial_pvf_lane: "provider"
planned_pvf_lane: "provider"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "focused"
validation_resource_profile: "bounded local CPU/disk; no implicit paid provider or cloud effects"
validation_family: "focused-production-integration"
validation_size_split: "local focused proof then required hosted integration"
expected_proof_cost: "Bounded local CPU/disk; #848 external review and #855 live provider calls separately governed. No paid execution implied."
planned_validation_seconds: "Not estimated; record actual child execution metrics."
planned_validation_tokens: "Not estimated; record actual child execution metrics."
issue_goal_ref: "issue855 implementation goal 01a0941a-fa4c-7bc1-8585-7b7e48e2fe00"
sprint_goal_ref: "Current v0.92.2 Sprint2 execution-readiness preparation goal"
goal_metrics_rollup_ref: "Not measured: no child execution"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/855"
  - kind: "stp"
    ref: ".csdlc/issues/855/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/855/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/855/cards/spp.md"
selected_lanes:
  - "provider: deterministic core transport, ADL compatibility, kernel lifecycle, installed five-provider fixture matrix, bounded authorized hosted demonstration; CI provider-core job and required package integration"
parallel_groups:
  - "Use Sprint2 execution packet shared-path assignments; serialize shared registration/manifests."
validation_commands:
  - "Full leaf94/94 plus one Runtime mock constructor regression; selected kernel control85/85, ADL compatibility18/18, CLI 7/7, API11/11; affectedClippy and CIpath/runtimecontracts pass; installedfixture5/5 and hostedtopologyfixture3/3 at 61095. Actual hosted run approved for six calls per provider,32000UTF8inputbytes percall,256outputtokens,zero retries,stop after first failure,$1 ceiling,30 minutes including cleanup; current Google project and named models fixed. Hosted sandbox preflight failed before calls; permission recovery pending. RequiredCI will run on draft PR."
failure_policy: "Stop on required proof failure, zero scenarios, stale input or shared ownership conflict; preserve evidence."
notes: "Prepared only. Child implementation and its review have not run."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Verify accepted inputs and ownership; implement complete issue; execute source proving cases; independent exact-head review; native publication.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `provider`
- Planned PVF lane for execution: `provider`

## Selected Validation Lanes

- provider: deterministic core transport, ADL compatibility, kernel lifecycle, installed five-provider fixture matrix, bounded authorized hosted demonstration; CI provider-core job and required package integration

## Parallelization Plan

- Parallel groups: Use Sprint2 execution packet shared-path assignments; serialize shared registration/manifests.
- Validation runtime class: `focused`
- Validation resource profile: `bounded local CPU/disk; no implicit paid provider or cloud effects`
- Validation family: `focused-production-integration`
- Validation size split: `local focused proof then required hosted integration`

## Goal Accounting Hooks

- Issue goal ref: `issue855 implementation goal 01a0941a-fa4c-7bc1-8585-7b7e48e2fe00`
- Sprint goal ref: `Current v0.92.2 Sprint2 execution-readiness preparation goal`
- Goal metrics rollup ref: `Not measured: no child execution`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Bounded local CPU/disk; #848 external review and #855 live provider calls separately governed. No paid execution implied.`
- Planned validation seconds: `Not estimated; record actual child execution metrics.`
- Planned validation token budget: `Not estimated; record actual child execution metrics.`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- Full leaf94/94 plus one Runtime mock constructor regression; selected kernel control85/85, ADL compatibility18/18, CLI 7/7, API11/11; affectedClippy and CIpath/runtimecontracts pass; installedfixture5/5 and hostedtopologyfixture3/3 at 61095. Actual hosted run approved for six calls per provider,32000UTF8inputbytes percall,256outputtokens,zero retries,stop after first failure,$1 ceiling,30 minutes including cleanup; current Google project and named models fixed. Hosted sandbox preflight failed before calls; permission recovery pending. RequiredCI will run on draft PR.

## Failure Semantics

- Stop on required proof failure, zero scenarios, stale input or shared ownership conflict; preserve evidence.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Prepared only. Child implementation and its review have not run.
