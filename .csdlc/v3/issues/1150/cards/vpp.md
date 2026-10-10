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
status: "executed_blocked_on_q02_q03_q19"
initial_pvf_lane: "installed_integration"
planned_pvf_lane: "installed_integration"
lane_registry_path: "docs/pvf/validation-lanes.json"
lane_registry_template_set: "current repository registry at execution time"
validation_runtime_class: "multi_platform_installed_integration"
validation_resource_profile: "Bounded local CPU/files and owned nonpublic CI or staging; paid providers require separate issue-specific authority; public deployment, audience activation and live launch are excluded."
validation_family: "independent_installed_qualification"
validation_size_split: "12 contract-derived tuples plus 24 source obligations; actual scenario denominator may only grow explicitly"
expected_proof_cost: "No additional provider cost. Q02/Q03 require an existing second GitHub identity of known invitation state; Q19 requires operator inspection time only."
planned_validation_seconds: "unknown"
planned_validation_tokens: "unknown"
issue_goal_ref: "Active issue #1150 execution-and-qualification goal; provider execution is complete and final acceptance remains blocked on Q02, Q03 and Q19."
sprint_goal_ref: "Sprint-3 umbrella #1229"
goal_metrics_rollup_ref: "Issue #1150 execution checkpoint: 21 PASS / 0 FAIL / 3 INCOMPLETE, with Q02/Q03/Q19 remaining."
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
  - "Retained terminal-accounting verification proved 122 actual POSTs, zero unknown outcomes and no ambiguity replay. Retained evidence review reconciled all 24 first/repeat groups, 36 exports, exact candidate identities, Q08 parity and the PR #31 bridge. Remaining: exercise a real uninvited GitHub account, prove live cross-user run/artifact denial with two real identities, and record human navigation of 12 HTML plus inspection of 12 PDFs."
failure_policy: "Fail closed. Keep NOT QUALIFIED and 0/12 release-accepted cells until Q02, Q03 and Q19 are proved or the issue authority explicitly changes those requirements. Do not infer identity evidence, substitute fixtures for real OAuth, or substitute agent render checks for human inspection."
notes: "The semantic execution passes all 12 cells, but global acceptance remains blocked. The first Linux/Vector quote mismatch remains adverse evidence while its compatible repeat satisfies Q06. Q22 is PASS on the independently reviewed identity bridge. No fresh provider, CI, deployment or service operation is needed."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

The full authorized provider-backed execution, export generation, exact-candidate parity checks, compatibility bridge, terminal reconciliation and independent semantic review have run. The result is 21 PASS / 0 FAIL / 3 INCOMPLETE. Remaining validation is provider-free and limited to Q02, Q03 and Q19.

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

- Issue goal ref: `Active issue #1150 execution-and-qualification goal; provider execution is complete and final acceptance remains blocked on Q02, Q03 and Q19.`
- Sprint goal ref: `Sprint-3 umbrella #1229`
- Goal metrics rollup ref: `Issue #1150 execution checkpoint: 21 PASS / 0 FAIL / 3 INCOMPLETE, with Q02/Q03/Q19 remaining.`

## Proof Cost / Runtime Expectations

- Expected proof cost: `No additional provider cost. Q02/Q03 require an existing second GitHub identity of known invitation state; Q19 requires operator inspection time only.`
- Planned validation seconds: `unknown`
- Planned validation token budget: `unknown`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- Retained terminal-accounting verification proved 122 actual POSTs, zero unknown outcomes and no ambiguity replay. Retained evidence review reconciled all 24 first/repeat groups, 36 exports, exact candidate identities, Q08 parity and the PR #31 bridge. Remaining: exercise a real uninvited GitHub account, prove live cross-user run/artifact denial with two real identities, and record human navigation of 12 HTML plus inspection of 12 PDFs.

## Failure Semantics

- Fail closed. Keep NOT QUALIFIED and 0/12 release-accepted cells until Q02, Q03 and Q19 are proved or the issue authority explicitly changes those requirements. Do not infer identity evidence, substitute fixtures for real OAuth, or substitute agent render checks for human inspection.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

The semantic execution passes all 12 cells, but global acceptance remains blocked. The first Linux/Vector quote mismatch remains adverse evidence while its compatible repeat satisfies Q06. Q22 is PASS on the independently reviewed identity bridge. No fresh provider, CI, deployment or service operation is needed.
