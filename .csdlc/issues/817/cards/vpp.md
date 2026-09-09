---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "release-truth-refresh-validation-plan"
issue: 817
task_id: "issue-0817"
run_id: "issue-0817"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06.12][release] Refresh candidate proof and canonical release truth"
branch: "codex/817-release-truth-refresh"
generated_at: "2026-09-09T22:00:00Z"
card_status: "ready"
status: "READY"
initial_pvf_lane: "docs_only"
planned_pvf_lane: "docs_only"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "small"
validation_resource_profile: "local CPU, Git, filesystem, and read-only GitHub; no cloud"
validation_family: "release_truth_and_evidence_integrity"
validation_size_split: "candidate proof, canonical status, links, JSON evidence"
expected_proof_cost: "Small local documentation and validator proof."
planned_validation_seconds: "not_collected"
planned_validation_tokens: "not_collected"
issue_goal_ref: "Issue #817 session goal"
sprint_goal_ref: "Parent #522"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/817"
  - kind: "stp"
    ref: ".csdlc/issues/817/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/817/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/817/cards/spp.md"
selected_lanes:
  - "docs/release validators; link negative; JSON parse sweep; diff hygiene"
parallel_groups:
  - "Candidate/status and link/JSON proof may run independently after repairs."
validation_commands:
  - "python3 .csdlc/prepared/issues/817/validate-release-truth.py; python3 .csdlc/prepared/issues/817/validate-release-truth.py --negative; python3 docs/milestones/v0.92.1/evidence/release/current-status/validate.py; python3 docs/milestones/v0.92.1/evidence/release/current-status/validate.py --negative; cargo test --locked --manifest-path csdlc-v3/Cargo.toml in a clean detached exact-source checkout; parse all repaired GCP-E JSON; native six-card validation; git diff --check"
failure_policy: "Any stale candidate, corrupt link, malformed or empty JSON, or failed exact mapping blocks publication."
notes: "V3-F acceptance remains pending until the final exact-source substantive review receipt and mapping validator pass; no paid cloud proof was rerun."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Prove all six findings with existing focused lanes plus new corruption/format negatives.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `docs_only`
- Planned PVF lane for execution: `docs_only`

## Selected Validation Lanes

- docs/release validators; link negative; JSON parse sweep; diff hygiene

## Parallelization Plan

- Parallel groups: Candidate/status and link/JSON proof may run independently after repairs.
- Validation runtime class: `small`
- Validation resource profile: `local CPU, Git, filesystem, and read-only GitHub; no cloud`
- Validation family: `release_truth_and_evidence_integrity`
- Validation size split: `candidate proof, canonical status, links, JSON evidence`

## Goal Accounting Hooks

- Issue goal ref: `Issue #817 session goal`
- Sprint goal ref: `Parent #522`
- Goal metrics rollup ref: `not_collected`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Small local documentation and validator proof.`
- Planned validation seconds: `not_collected`
- Planned validation token budget: `not_collected`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- python3 .csdlc/prepared/issues/817/validate-release-truth.py; python3 .csdlc/prepared/issues/817/validate-release-truth.py --negative; python3 docs/milestones/v0.92.1/evidence/release/current-status/validate.py; python3 docs/milestones/v0.92.1/evidence/release/current-status/validate.py --negative; cargo test --locked --manifest-path csdlc-v3/Cargo.toml in a clean detached exact-source checkout; parse all repaired GCP-E JSON; native six-card validation; git diff --check

## Failure Semantics

- Any stale candidate, corrupt link, malformed or empty JSON, or failed exact mapping blocks publication.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

V3-F acceptance remains pending until the final exact-source substantive review receipt and mapping validator pass; no paid cloud proof was rerun.
