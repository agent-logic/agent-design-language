---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "v0922-six-resident-qualification-validation-plan"
issue: 900
task_id: "issue-0900"
run_id: "issue-0900"
version: "0.92.2"
title: "[v0.92.2][QUAL-RESIDENT] Execute resident workload and signed restore qualification"
branch: "codex/900-v0922-six-resident-qualification"
generated_at: "2026-09-12T00:15:07.665153+00:00"
card_status: "ready"
status: "planned"
initial_pvf_lane: "runtime"
planned_pvf_lane: "runtime"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "bounded_local_production_qualification_pending_approval"
validation_resource_profile: "Mac16,11 Apple M4 Pro 64 GB; task-owned Ollama 127.0.0.1:11435; three existing local model blobs; serial inference; task-owned Runtime, CSM and continuity processes; isolated worktree evidence; no cloud, downloads, hosted APIs or paid calls"
validation_family: "resident_signed_restore_qualification"
validation_size_split: "Focused local harness negatives, actual production scenarios and required CI reported as separate lanes"
expected_proof_cost: "Up to 5400 seconds wall time, serial model execution, zero provider/API/cloud charges; local compute and energy only. Stop on memory pressure, shared-process collision or unexpected network access."
planned_validation_seconds: "5400"
planned_validation_tokens: "8000"
issue_goal_ref: "Create child issue #900 goal after bind and before implementation; Sprint #931 goal alone is not a substitute"
sprint_goal_ref: "Sprint #931"
goal_metrics_rollup_ref: ".csdlc/evidence/900/goal-metrics.json (planned, absent until execution)"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/900"
  - kind: "stp"
    ref: ".csdlc/issues/900/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/900/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/900/cards/spp.md"
selected_lanes:
  - "runtime; local deterministic integrity/regression plus operator-approved production execution; required qualification gate; local proof passed and CI pending"
parallel_groups:
  - "Serialize native issue preparation; future isolated deterministic fixtures may parallelize only after ownership/resource review"
validation_commands:
  - "Classify new tests at authoring time with lane, proof role, determinism, resource profile and release-gate status in a tightly coupled manifest. Local deterministic contract negatives use isolated copies, controlled clocks and owned subprocesses; role is evidence-integrity/behavior regression, local CPU/Rust/Python/Git, required issue gate. Run the authored focused tests and actual tool/harness invocations with nonzero scenario counts; list exact registered Rust test names first rather than trusting historical filters. Run formatting and relevant shared-owner regressions when source changes. Warm only trusted same-host dependency artifacts; distinguish local evidence from required CI. Resident/provider qualification additionally requires the declared production execution lane in an explicitly approved environment. Before effects, pin candidate/runtime/provider/model/artifact/configuration identities, process ownership, bounds, duration/cost, credential reference and the scope of operator authorization. This creation does not authorize paid cloud/GPU mutation, account changes, service disruption or provider charges. If approved execution cannot be obtained, record not-proven and keep the task incomplete; do not replace it with mock proof. Any ADL AWS observation uses `agent-logic-admin` with verified business identity; credentials and account secrets never enter artifacts. No broad host process scans or killing shared services. Source-grounded planned local commands: `python3 adl/tools/validate_issue268_six_resident_uts_plan.py`; `python3 adl/tools/test_run_issue268_six_resident_uts_cycle.py`; `python3 adl/tools/test_run_issue268_continuity_uts_qualification.py`. The two current tests use fake subprocess harnesses and prove local regression only. Production execution must separately run the six-resident pre/replay/post runner and continuity qualification orchestrator with exact approved runtime/continuity binaries, isolated state/evidence roots, agent-spec/configuration and signed volume identity; resolve full current argv at execution. Never substitute historical scripts, mock receipts or the prior r7i.2xlarge plan for current environment authorization. Formatting and touched-owner regressions use the manifest of each actually changed Rust component. Enumerate actual tests and replan absent/renamed ones explicitly; zero-test filters never prove acceptance. These are future execution commands, not tests run during preparation. `git diff --check` accompanies focused proof. Required qualification harness repairs and scenario proofs are future work; existing source alone is not acceptance. Install the issue candidate through the approved installer only at execution with current provenance; do not replace a shared binary during preparation."
failure_policy: "Missing, skipped, zero-scenario or failed required proof blocks acceptance. Replan absent/renamed tests explicitly. No mock-only success, static reference trace, supplied failure flag, stale candidate or provider permission inference. Actual effects, complete scenario population and independent review remain required. Keep machine-readable stdout and redacted stderr; preserve failed evidence and refresh affected exact-head review."
notes: "Dependencies #852 and #855 are satisfied by merged PRs #963 and #964. The operator-approved local Ollama envelope completed in attempt-17 using task-owned processes and existing blobs; no paid or cloud call and no shared-service mutation occurred. Six role-specific runtime.observe views executed before and after signed restore with receipt-bound argument and effect hashes. All seven integrity negatives denied without effects. Independent code and evidence review passed at 4e9d2663a0091027e52c768d7b7bb3cf2f0296c1; this lifecycle correction requires fresh exact-head confirmation. Hosted CI, merge, terminal reconciliation, and Sprint acceptance remain pending. The separate dirty #851 worktree and its bytes remain untouched."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Wait for accepted #852 and #855, resolve the dirty #851 shared-path owner and approve the exact resource/environment profile; pin six distinct role/model/artifact/configuration identities and complete scenario denominator; execute production tick/ACC/UTS workload effects then signed dehydration/verified restore and exact pending-work resumption; execute isolated tamper/omission/substitution negatives and distinguish local mock regression from actual provider results; independently review exact run/signatures/lineage/effects and hand accepted evidence to QUAL-EVIDENCE.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `runtime`
- Planned PVF lane for execution: `runtime`

## Selected Validation Lanes

- runtime; local deterministic integrity/regression plus operator-approved production execution; required qualification gate; local proof passed and CI pending

## Parallelization Plan

- Parallel groups: Serialize native issue preparation; future isolated deterministic fixtures may parallelize only after ownership/resource review
- Validation runtime class: `bounded_local_production_qualification_pending_approval`
- Validation resource profile: `Mac16,11 Apple M4 Pro 64 GB; task-owned Ollama 127.0.0.1:11435; three existing local model blobs; serial inference; task-owned Runtime, CSM and continuity processes; isolated worktree evidence; no cloud, downloads, hosted APIs or paid calls`
- Validation family: `resident_signed_restore_qualification`
- Validation size split: `Focused local harness negatives, actual production scenarios and required CI reported as separate lanes`

## Goal Accounting Hooks

- Issue goal ref: `Create child issue #900 goal after bind and before implementation; Sprint #931 goal alone is not a substitute`
- Sprint goal ref: `Sprint #931`
- Goal metrics rollup ref: `.csdlc/evidence/900/goal-metrics.json (planned, absent until execution)`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Up to 5400 seconds wall time, serial model execution, zero provider/API/cloud charges; local compute and energy only. Stop on memory pressure, shared-process collision or unexpected network access.`
- Planned validation seconds: `5400`
- Planned validation token budget: `8000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- Classify new tests at authoring time with lane, proof role, determinism, resource profile and release-gate status in a tightly coupled manifest. Local deterministic contract negatives use isolated copies, controlled clocks and owned subprocesses; role is evidence-integrity/behavior regression, local CPU/Rust/Python/Git, required issue gate. Run the authored focused tests and actual tool/harness invocations with nonzero scenario counts; list exact registered Rust test names first rather than trusting historical filters. Run formatting and relevant shared-owner regressions when source changes. Warm only trusted same-host dependency artifacts; distinguish local evidence from required CI. Resident/provider qualification additionally requires the declared production execution lane in an explicitly approved environment. Before effects, pin candidate/runtime/provider/model/artifact/configuration identities, process ownership, bounds, duration/cost, credential reference and the scope of operator authorization. This creation does not authorize paid cloud/GPU mutation, account changes, service disruption or provider charges. If approved execution cannot be obtained, record not-proven and keep the task incomplete; do not replace it with mock proof. Any ADL AWS observation uses `agent-logic-admin` with verified business identity; credentials and account secrets never enter artifacts. No broad host process scans or killing shared services. Source-grounded planned local commands: `python3 adl/tools/validate_issue268_six_resident_uts_plan.py`; `python3 adl/tools/test_run_issue268_six_resident_uts_cycle.py`; `python3 adl/tools/test_run_issue268_continuity_uts_qualification.py`. The two current tests use fake subprocess harnesses and prove local regression only. Production execution must separately run the six-resident pre/replay/post runner and continuity qualification orchestrator with exact approved runtime/continuity binaries, isolated state/evidence roots, agent-spec/configuration and signed volume identity; resolve full current argv at execution. Never substitute historical scripts, mock receipts or the prior r7i.2xlarge plan for current environment authorization. Formatting and touched-owner regressions use the manifest of each actually changed Rust component. Enumerate actual tests and replan absent/renamed ones explicitly; zero-test filters never prove acceptance. These are future execution commands, not tests run during preparation. `git diff --check` accompanies focused proof. Required qualification harness repairs and scenario proofs are future work; existing source alone is not acceptance. Install the issue candidate through the approved installer only at execution with current provenance; do not replace a shared binary during preparation.

## Failure Semantics

- Missing, skipped, zero-scenario or failed required proof blocks acceptance. Replan absent/renamed tests explicitly. No mock-only success, static reference trace, supplied failure flag, stale candidate or provider permission inference. Actual effects, complete scenario population and independent review remain required. Keep machine-readable stdout and redacted stderr; preserve failed evidence and refresh affected exact-head review.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Dependencies #852 and #855 are satisfied by merged PRs #963 and #964. The operator-approved local Ollama envelope completed in attempt-17 using task-owned processes and existing blobs; no paid or cloud call and no shared-service mutation occurred. Six role-specific runtime.observe views executed before and after signed restore with receipt-bound argument and effect hashes. All seven integrity negatives denied without effects. Independent code and evidence review passed at 4e9d2663a0091027e52c768d7b7bb3cf2f0296c1; this lifecycle correction requires fresh exact-head confirmation. Hosted CI, merge, terminal reconciliation, and Sprint acceptance remain pending. The separate dirty #851 worktree and its bytes remain untouched.
