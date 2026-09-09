---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "tail-04-internal-review-validation-plan"
issue: 520
task_id: "issue-0520"
run_id: "issue-0520"
version: "1.0.5"
title: "[v0.92.1][TAIL-04] Internal review"
branch: "codex/520-internal-review"
generated_at: "2026-09-09T19:19:55Z"
card_status: "approved"
status: "ready_waiting_on_758"
initial_pvf_lane: "review-complete"
planned_pvf_lane: "review-complete-exact-candidate"
lane_registry_path: ".csdlc/prepared/issues/520/internal-review-plan.md"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "bounded repository review"
validation_resource_profile: "local CPU plus independent review agents; no paid provider or cloud runtime"
validation_family: "complete v0.92.1 internal review packet"
validation_size_split: "gate preflight, denominator build, parallel specialist lanes, synthesis, packet validation, exact-head review"
expected_proof_cost: "bounded local repository review"
planned_validation_seconds: "7200"
planned_validation_tokens: "50000"
issue_goal_ref: "issue-520-internal-review-rerun"
sprint_goal_ref: "v0.92.1-tail-review"
goal_metrics_rollup_ref: "v0.92.1-tail-review"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/520"
  - kind: "stp"
    ref: ".csdlc/issues/520/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/520/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/520/cards/spp.md"
selected_lanes:
  - "gate and candidate integrity; complete issue, pull-request, path, and acceptance denominators; code, architecture, dependency, security, tests, docs, lifecycle, demos/proof, provider/cloud, and acceptance specialist review; finding synthesis; packet schema/redaction/portability; independent exact-head review"
parallel_groups:
  - "Run independent specialist lanes in parallel only after the immutable candidate and complete denominator are frozen; synthesize after all lanes report."
validation_commands:
  - "ruby docs/milestones/v0.92.1/evidence/release/tail-04/build-denominator.rb <exact-origin-main-sha>; ruby .csdlc/prepared/issues/520/validate-internal-review.rb; ruby .csdlc/prepared/issues/520/test-production-validator.rb; git diff --check"
failure_policy: "Fail closed on an unmerged gate, candidate drift, missing merge ancestry, incomplete denominator, absent or non-proving mandatory lane, unsupported finding, count/schema mismatch, redaction or portability failure, or stale exact-head review."
notes: "A historical pass, green CI, or prior report is context only and cannot prove the new candidate."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Prove the complete rerun packet is tied to one exact post-gate candidate and covers every declared surface without sampling or silent omission.

## Lane Registry Inputs

- Registry path: `.csdlc/prepared/issues/520/internal-review-plan.md`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `review-complete`
- Planned PVF lane for execution: `review-complete-exact-candidate`

## Selected Validation Lanes

- gate and candidate integrity; complete issue, pull-request, path, and acceptance denominators; code, architecture, dependency, security, tests, docs, lifecycle, demos/proof, provider/cloud, and acceptance specialist review; finding synthesis; packet schema/redaction/portability; independent exact-head review

## Parallelization Plan

- Parallel groups: Run independent specialist lanes in parallel only after the immutable candidate and complete denominator are frozen; synthesize after all lanes report.
- Validation runtime class: `bounded repository review`
- Validation resource profile: `local CPU plus independent review agents; no paid provider or cloud runtime`
- Validation family: `complete v0.92.1 internal review packet`
- Validation size split: `gate preflight, denominator build, parallel specialist lanes, synthesis, packet validation, exact-head review`

## Goal Accounting Hooks

- Issue goal ref: `issue-520-internal-review-rerun`
- Sprint goal ref: `v0.92.1-tail-review`
- Goal metrics rollup ref: `v0.92.1-tail-review`

## Proof Cost / Runtime Expectations

- Expected proof cost: `bounded local repository review`
- Planned validation seconds: `7200`
- Planned validation token budget: `50000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- ruby docs/milestones/v0.92.1/evidence/release/tail-04/build-denominator.rb <exact-origin-main-sha>; ruby .csdlc/prepared/issues/520/validate-internal-review.rb; ruby .csdlc/prepared/issues/520/test-production-validator.rb; git diff --check

## Failure Semantics

- Fail closed on an unmerged gate, candidate drift, missing merge ancestry, incomplete denominator, absent or non-proving mandatory lane, unsupported finding, count/schema mismatch, redaction or portability failure, or stale exact-head review.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

A historical pass, green CI, or prior report is context only and cannot prove the new candidate.
