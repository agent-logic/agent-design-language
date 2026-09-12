---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "879-github-ingestion-validation-plan"
issue: 879
task_id: "issue-0879"
run_id: "issue-0879"
version: "v0.92.2"
title: "[v0.92.2][CF-ADAPTER-GITHUB] Ingest a pinned GitHub revision or PR into a repository packet"
branch: "codex/879-github-ingestion"
generated_at: "2026-09-11T23:55:26.355672+00:00"
card_status: "ready"
status: "prepared"
initial_pvf_lane: "runtime"
planned_pvf_lane: "runtime"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "focused"
validation_resource_profile: "bounded local CPU/disk; no implicit paid provider or cloud effects"
validation_family: "focused-production-integration"
validation_size_split: "local focused proof then required hosted integration"
expected_proof_cost: "Bounded local CPU/disk; #848 external review and #855 live provider calls separately governed. No paid execution implied."
planned_validation_seconds: "Not estimated; record actual child execution metrics."
planned_validation_tokens: "Not estimated; record actual child execution metrics."
issue_goal_ref: "Issue #879 active execution goal through passing reviewed PR; no merge or cleanup."
sprint_goal_ref: "Current v0.92.2 Sprint2 execution-readiness preparation goal"
goal_metrics_rollup_ref: "Not measured: no child execution"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/879"
  - kind: "stp"
    ref: ".csdlc/issues/879/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/879/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/879/cards/spp.md"
selected_lanes:
  - "runtime"
parallel_groups:
  - "Use Sprint2 execution packet shared-path assignments; serialize shared registration/manifests."
validation_commands:
  - "cargo test --manifest-path adl/Cargo.toml --test codefriend_github_ingestion --test codefriend_ingestion; installed candidate repeat; focused clippy and fmt. See .csdlc/evidence/879/PROOF.json."
failure_policy: "Stop on required proof failure, zero scenarios, stale input or shared ownership conflict; preserve evidence."
notes: "Implementation and focused local/installed proof complete; independent exact-head review, native publication and required CI pending. No live GitHub or provider calls, merge or closeout claimed."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Verify accepted inputs and ownership; implement complete issue; execute source proving cases; independent exact-head review; native publication.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `runtime`
- Planned PVF lane for execution: `runtime`

## Selected Validation Lanes

- runtime

## Parallelization Plan

- Parallel groups: Use Sprint2 execution packet shared-path assignments; serialize shared registration/manifests.
- Validation runtime class: `focused`
- Validation resource profile: `bounded local CPU/disk; no implicit paid provider or cloud effects`
- Validation family: `focused-production-integration`
- Validation size split: `local focused proof then required hosted integration`

## Goal Accounting Hooks

- Issue goal ref: `Issue #879 active execution goal through passing reviewed PR; no merge or cleanup.`
- Sprint goal ref: `Current v0.92.2 Sprint2 execution-readiness preparation goal`
- Goal metrics rollup ref: `Not measured: no child execution`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Bounded local CPU/disk; #848 external review and #855 live provider calls separately governed. No paid execution implied.`
- Planned validation seconds: `Not estimated; record actual child execution metrics.`
- Planned validation token budget: `Not estimated; record actual child execution metrics.`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- cargo test --manifest-path adl/Cargo.toml --test codefriend_github_ingestion --test codefriend_ingestion; installed candidate repeat; focused clippy and fmt. See .csdlc/evidence/879/PROOF.json.

## Failure Semantics

- Stop on required proof failure, zero scenarios, stale input or shared ownership conflict; preserve evidence.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Implementation and focused local/installed proof complete; independent exact-head review, native publication and required CI pending. No live GitHub or provider calls, merge or closeout claimed.
