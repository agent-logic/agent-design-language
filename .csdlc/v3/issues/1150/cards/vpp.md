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
card_status: "ready"
status: "executed_qualified"
initial_pvf_lane: "installed_integration"
planned_pvf_lane: "installed_integration"
lane_registry_path: "docs/pvf/validation-lanes.json"
lane_registry_template_set: "current repository registry at execution time"
validation_runtime_class: "multi_platform_installed_integration"
validation_resource_profile: "Bounded local CPU/files and owned nonpublic CI or staging; paid providers require separate issue-specific authority; public deployment, audience activation and live launch are excluded."
validation_family: "independent_installed_qualification"
validation_size_split: "12 contract-derived tuples plus 24 source obligations; actual scenario denominator may only grow explicitly"
expected_proof_cost: "No additional provider cost. Remaining validation is local and native lifecycle proof only."
planned_validation_seconds: "unknown"
planned_validation_tokens: "unknown"
issue_goal_ref: "Issue #1150 final qualification is complete; PR publication and merge remain."
sprint_goal_ref: "Sprint-3 umbrella #1229"
goal_metrics_rollup_ref: "Issue #1150 final qualification: 24 PASS / 0 FAIL / 0 INCOMPLETE; 12/12 release-accepted."
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
  - "Focused checkpoint test; JSON parsing and digest verification; exact-aa2 candidate/pass/fail/error exits 0/0/1/2 with local_result_parity true; native csdlc validate and proof; independent exact-head review before merge."
failure_policy: "Fail closed on any digest, denominator, exact-head, native validation, review, or required-check mismatch. Preserve the Linux Vector first failure as adverse evidence."
notes: "The Q03 second-user denial is operator-witnessed and corroborated by bound subject enforcement evidence rather than retained as a raw HTTP capture. Exact provider billing, public deployment and launch are not claimed."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

All required qualification lanes have executed. Q02 and Q03 pass on real identity admission/isolation evidence, Q08 passes exact-aa2 four-case parity, Q19 has operator acceptance over the unchanged 24-report denominator, and the candidate bridge/output refresh pass independent review.

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

- Issue goal ref: `Issue #1150 final qualification is complete; PR publication and merge remain.`
- Sprint goal ref: `Sprint-3 umbrella #1229`
- Goal metrics rollup ref: `Issue #1150 final qualification: 24 PASS / 0 FAIL / 0 INCOMPLETE; 12/12 release-accepted.`

## Proof Cost / Runtime Expectations

- Expected proof cost: `No additional provider cost. Remaining validation is local and native lifecycle proof only.`
- Planned validation seconds: `unknown`
- Planned validation token budget: `unknown`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- Focused checkpoint test; JSON parsing and digest verification; exact-aa2 candidate/pass/fail/error exits 0/0/1/2 with local_result_parity true; native csdlc validate and proof; independent exact-head review before merge.

## Failure Semantics

- Fail closed on any digest, denominator, exact-head, native validation, review, or required-check mismatch. Preserve the Linux Vector first failure as adverse evidence.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

The Q03 second-user denial is operator-witnessed and corroborated by bound subject enforcement evidence rather than retained as a raw HTTP capture. Exact provider billing, public deployment and launch are not claimed.
