---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "tail-01-quality-gate-validation-plan"
issue: 517
task_id: "issue-0517"
run_id: "issue-0517"
version: "v0.92.1"
title: "[v0.92.1][TAIL-01] Quality gate"
branch: "codex/517-tail-01-quality-gate"
generated_at: "2026-09-09T00:56:48.157331+00:00"
card_status: "ready"
status: "in_progress"
initial_pvf_lane: "local_cpu"
planned_pvf_lane: "local_cpu"
lane_registry_path: "docs/templates/prompts/current.json"
lane_registry_template_set: "1.0.4"
validation_runtime_class: "deterministic_local_cpu"
validation_resource_profile: "small"
validation_family: "local_contract"
validation_size_split: "focused"
expected_proof_cost: "Bounded local CPU; no live mutations for test proof"
planned_validation_seconds: "3600"
planned_validation_tokens: "25000"
issue_goal_ref: "issue-517-pr748-review-remediation"
sprint_goal_ref: "not recorded"
goal_metrics_rollup_ref: "not recorded"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/517"
  - kind: "stp"
    ref: ".csdlc/issues/517/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/517/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/517/cards/spp.md"
selected_lanes:
  - "quality-denominator; evidence-accounting; canonical-document-parity"
parallel_groups:
  - "document mapping and issue ownership"
validation_commands:
  - "Focused reconciliation validators, denominator and digest checks, explicit follow-up ownership, canonical YAML parity, independent document review, and required hosted CI."
failure_policy: "Fail closed on an unmet predecessor, incomplete denominator, non-proving lane, candidate drift, or unowned exception."
notes: "Gate remains blocked; proof debt is not resolved by this PR. Publication does not authorize release."
---

Canonical Template Source: `docs/templates/prompts/1.0.4/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Docs-only evidence accounting; actual code and operational proof remain separate issues.

## Lane Registry Inputs

- Registry path: `docs/templates/prompts/current.json`
- Registry template set: `1.0.4`
- Initial PVF lane from issue creation: `local_cpu`
- Planned PVF lane for execution: `local_cpu`

## Selected Validation Lanes

- quality-denominator; evidence-accounting; canonical-document-parity

## Parallelization Plan

- Parallel groups: document mapping and issue ownership
- Validation runtime class: `deterministic_local_cpu`
- Validation resource profile: `small`
- Validation family: `local_contract`
- Validation size split: `focused`

## Goal Accounting Hooks

- Issue goal ref: `issue-517-pr748-review-remediation`
- Sprint goal ref: `not recorded`
- Goal metrics rollup ref: `not recorded`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Bounded local CPU; no live mutations for test proof`
- Planned validation seconds: `3600`
- Planned validation token budget: `25000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- Focused reconciliation validators, denominator and digest checks, explicit follow-up ownership, canonical YAML parity, independent document review, and required hosted CI.

## Failure Semantics

- Fail closed on an unmet predecessor, incomplete denominator, non-proving lane, candidate drift, or unowned exception.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Gate remains blocked; proof debt is not resolved by this PR. Publication does not authorize release.
