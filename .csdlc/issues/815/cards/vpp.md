---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "cloud-authorization-authenticity-validation-plan"
issue: 815
task_id: "issue-0815"
run_id: "issue-0815"
version: "1.0.5"
title: "[v0.92.1][TAIL-06.10][security] Authenticate AWS and GCP mutation authorization"
branch: "codex/815-cloud-authorization-authenticity"
generated_at: "2026-09-09T20:50:00Z"
card_status: "approved"
status: "ready"
initial_pvf_lane: "security-local"
planned_pvf_lane: "security-local-negative-matrix"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "bounded-local"
validation_resource_profile: "small-local-cpu-filesystem-no-cloud"
validation_family: "cloud-mutation-authorization-authenticity"
validation_size_split: "focused-signature-plan-byte-negative-matrix"
expected_proof_cost: "zero provider spend; local Python, OpenSSH, shell, and mocked Terraform only"
planned_validation_seconds: "300"
planned_validation_tokens: "4000"
issue_goal_ref: "issue-815-session-goal"
sprint_goal_ref: "issue-520-remediation"
goal_metrics_rollup_ref: "issue-522-findings"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/815"
  - kind: "stp"
    ref: ".csdlc/issues/815/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/815/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/815/cards/spp.md"
selected_lanes:
  - "detached-signature success and forgery rejection; packet/plan/sidecar/expiry/account/project replay rejection; read-only no-authority proof; shell/Python syntax; diff hygiene; exact-head review"
parallel_groups:
  - "Run independent AWS and GCP local fixtures together after shared verifier syntax passes; serialize final exact-head review."
validation_commands:
  - "bash .csdlc/prepared/issues/815/test-cloud-authorization.sh; python3 -m py_compile .csdlc/prepared/issues/815/verify-cloud-authorization.py; bash -n affected shell scripts; git diff --check"
failure_policy: "Fail closed on missing/untrusted/malformed signature, noncanonical packet, stale authority, plan or sidecar mismatch, projection mismatch, provider identity replay, unexpected provider call, or actionable review finding."
notes: "No live AWS/GCP mutation or spend is permitted; read-only behavior is tested separately from mutation authorization."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Prove mutation authorization authenticity and exact-plan identity entirely with deterministic local fixtures.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `security-local`
- Planned PVF lane for execution: `security-local-negative-matrix`

## Selected Validation Lanes

- detached-signature success and forgery rejection; packet/plan/sidecar/expiry/account/project replay rejection; read-only no-authority proof; shell/Python syntax; diff hygiene; exact-head review

## Parallelization Plan

- Parallel groups: Run independent AWS and GCP local fixtures together after shared verifier syntax passes; serialize final exact-head review.
- Validation runtime class: `bounded-local`
- Validation resource profile: `small-local-cpu-filesystem-no-cloud`
- Validation family: `cloud-mutation-authorization-authenticity`
- Validation size split: `focused-signature-plan-byte-negative-matrix`

## Goal Accounting Hooks

- Issue goal ref: `issue-815-session-goal`
- Sprint goal ref: `issue-520-remediation`
- Goal metrics rollup ref: `issue-522-findings`

## Proof Cost / Runtime Expectations

- Expected proof cost: `zero provider spend; local Python, OpenSSH, shell, and mocked Terraform only`
- Planned validation seconds: `300`
- Planned validation token budget: `4000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- bash .csdlc/prepared/issues/815/test-cloud-authorization.sh; python3 -m py_compile .csdlc/prepared/issues/815/verify-cloud-authorization.py; bash -n affected shell scripts; git diff --check

## Failure Semantics

- Fail closed on missing/untrusted/malformed signature, noncanonical packet, stale authority, plan or sidecar mismatch, projection mismatch, provider identity replay, unexpected provider call, or actionable review finding.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

No live AWS/GCP mutation or spend is permitted; read-only behavior is tested separately from mutation authorization.
