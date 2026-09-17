---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "adr-decision-reconciliation-validation-plan"
issue: 945
task_id: "issue-0945"
run_id: "issue-0945"
version: "v0.92.2"
title: "[v0.92.2][ARCH-ADR] Reconcile proposed ADRs with implementation and obtain decision approval"
branch: "codex/945-adr-decision-reconciliation"
generated_at: "2026-09-17T01:12:51.241656+00:00"
card_status: "ready"
status: "pre_execution"
initial_pvf_lane: "docs_only"
planned_pvf_lane: "docs_only"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "local_cpu"
validation_resource_profile: "Bounded local documentation checks and source review"
validation_family: "docs"
validation_size_split: "small"
expected_proof_cost: "600 seconds local document checks estimated; source review and approval waits separate"
planned_validation_seconds: "600"
planned_validation_tokens: "3000"
issue_goal_ref: "Create #945 issue-bound goal after native bind before implementation"
sprint_goal_ref: "https://github.com/agent-logic/agent-design-language/issues/945"
goal_metrics_rollup_ref: "Issue #945 goal tool and SOR"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/945"
  - kind: "stp"
    ref: ".csdlc/issues/945/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/945/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/945/cards/spp.md"
selected_lanes:
  - "docs_only"
parallel_groups:
  - "Read-only evidence gathering may be parallel; source corrections and manifest refresh serial; independent review after stable content."
validation_commands:
  - "PVF docs_only, deterministic local CPU/file checks. Run historical issue-911 validate_packet.py --self-test to preserve source snapshot; run new issue-945 source/hash/link/decision-coverage validator and negative cases. Native semantic_card_projections tests are tooling-only, not architectural acceptance. Independent substantive exact-head review is required. No runtime/provider/cloud execution."
failure_policy: "Failed source/hash/coverage checks or review findings block decision packet readiness. Pending required operator decisions block acceptance and issue closure."
notes: "Source review does not prove runtime behavior. Proposed ADRs remain pending operator decision; preserve historical records and separate #848/#910 obligations."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Read all12 candidates and current sources; record divergences and revise supported wording; preserve historical proposal and all69 mapping; generate current source manifest and per-candidate decision recommendations; run focused validation and independent review; present exact-text decisions for operator approval before applying final statuses or numbering.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `docs_only`
- Planned PVF lane for execution: `docs_only`

## Selected Validation Lanes

- docs_only

## Parallelization Plan

- Parallel groups: Read-only evidence gathering may be parallel; source corrections and manifest refresh serial; independent review after stable content.
- Validation runtime class: `local_cpu`
- Validation resource profile: `Bounded local documentation checks and source review`
- Validation family: `docs`
- Validation size split: `small`

## Goal Accounting Hooks

- Issue goal ref: `Create #945 issue-bound goal after native bind before implementation`
- Sprint goal ref: `https://github.com/agent-logic/agent-design-language/issues/945`
- Goal metrics rollup ref: `Issue #945 goal tool and SOR`

## Proof Cost / Runtime Expectations

- Expected proof cost: `600 seconds local document checks estimated; source review and approval waits separate`
- Planned validation seconds: `600`
- Planned validation token budget: `3000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- PVF docs_only, deterministic local CPU/file checks. Run historical issue-911 validate_packet.py --self-test to preserve source snapshot; run new issue-945 source/hash/link/decision-coverage validator and negative cases. Native semantic_card_projections tests are tooling-only, not architectural acceptance. Independent substantive exact-head review is required. No runtime/provider/cloud execution.

## Failure Semantics

- Failed source/hash/coverage checks or review findings block decision packet readiness. Pending required operator decisions block acceptance and issue closure.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Source review does not prove runtime behavior. Proposed ADRs remain pending operator decision; preserve historical records and separate #848/#910 obligations.
