---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "v0922-codefriend-fitness-ci-validation-plan"
issue: 888
task_id: "issue-0888"
run_id: "issue-0888"
version: "0.92.2"
title: "[v0.92.2][CF-GOV-CI] Execute architecture fitness functions as a CI gate"
branch: "codex/888-v0922-codefriend-fitness-ci"
generated_at: "2026-09-12T00:05:18.245012+00:00"
card_status: "ready"
status: "planned"
initial_pvf_lane: "owner_binary"
planned_pvf_lane: "owner_binary"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "bounded_local_and_required_ci"
validation_resource_profile: "local CPU/disk/Rust and controlled fixtures; normal PR CI separately; no provider calls, paid workflows or repository-script execution"
validation_family: "codefriend_fitness_consumer_contract"
validation_size_split: "focused per source acceptance; no reflexive full workspace suite"
expected_proof_cost: "Local CPU/disk plus normal automatic PR CI. Retained numeric values are planning estimates only; the user authorized no time or token cutoff. Required proof must run to completion."
planned_validation_seconds: "2400"
planned_validation_tokens: "16000"
issue_goal_ref: "Active whole Sprint 3 #929 goal includes #888 and all eight children; no replacement goal and no operator time or token cutoff."
sprint_goal_ref: "Active whole Sprint 3 #929 goal includes #888 and all eight children; no replacement goal and no operator time or token cutoff."
goal_metrics_rollup_ref: ".csdlc/evidence/888/goal-metrics.json (planned; absent until execution)"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/888"
  - kind: "stp"
    ref: ".csdlc/issues/888/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/888/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/888/cards/spp.md"
selected_lanes:
  - "owner_binary; Required local deterministic lane: production-module behavior plus installed `adl codefriend` consumer execution against isolated admitted fixtures, CPU/Rust/filesystem only and controlled clocks. Proof role: semantic correctness and actual consumer integration; required issue acceptance and later CF-PROOF/TAIL-01 input. Run `cargo test --manifest-path adl/Cargo.toml --test codefriend_cf_gov_ci` once authored, the focused touched shared-owner regressions and `cargo fmt --manifest-path adl/Cargo.toml --check`. Record actual nonzero test/fixture denominators and source/binary/OS/toolchain identities. New fixtures declare lane, role, determinism, resource profile and release-gate status in the tightly coupled manifest. CI/macOS/Linux evidence is separate from local proof; no unrun support claim. Use trusted same-host dependency warming where applicable. A schema, scaffold, authored packet, test-only consumer or zero-executed-scenario run cannot close this implementation issue. Report missing or failed proof as such; do not defer unfinished behavior to CF-INTEGRATE."
parallel_groups:
  - "serial within this issue; independent fixtures may parallelize only with isolated state"
validation_commands:
  - "Required local deterministic lane: production-module behavior plus installed `adl codefriend` consumer execution against isolated admitted fixtures, CPU/Rust/filesystem only and controlled clocks. Proof role: semantic correctness and actual consumer integration; required issue acceptance and later CF-PROOF/TAIL-01 input. Run `cargo test --manifest-path adl/Cargo.toml --test codefriend_cf_gov_ci` once authored, the focused touched shared-owner regressions and `cargo fmt --manifest-path adl/Cargo.toml --check`. Record actual nonzero test/fixture denominators and source/binary/OS/toolchain identities. New fixtures declare lane, role, determinism, resource profile and release-gate status in the tightly coupled manifest. CI/macOS/Linux evidence is separate from local proof; no unrun support claim. Use trusted same-host dependency warming where applicable. A schema, scaffold, authored packet, test-only consumer or zero-executed-scenario run cannot close this implementation issue. Report missing or failed proof as such; do not defer unfinished behavior to CF-INTEGRATE."
failure_policy: "Required failures, skipped or zero-test proof block acceptance. Preserve guards; record durable anomalies; repair and rerun affected proof and independent exact-head review. CI evidence is separate from local proof."
notes: "Execution prerequisite #887 is satisfied by accepted PR #989 merge 49ca9f2fcf785a2f8e6b75676279db4f0936f925 and passing hosted CI. #888 is bound in its registered FastWork worktree and integrates all seven merged Sprint #929 siblings. Preserve the existing dependency graph and serialize shared CLI edits under Worker #10. VPP remains the proof plan, not execution evidence. Require focused tests, coverage, isolated installed candidate and pass/fail/error cases, and actual automatic hosted job/artifact evidence. Numeric estimates impose no execution time or token cutoff."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Use accepted merged local runner #887 and preserve its exit/artifact contract; implement the narrow installed CI adapter and shell entrypoint without duplicating policy; add a minimally privileged dedicated workflow using the same exact candidate/policy/fixture inputs; prove original exit preservation and reject missing/truncated or identity-mismatched artifacts; obtain actual passing, violating and runner-error CI job evidence through normal PR checks and compare local results; reconcile artifact redaction, docs and independent exact-head review.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `owner_binary`
- Planned PVF lane for execution: `owner_binary`

## Selected Validation Lanes

- owner_binary; Required local deterministic lane: production-module behavior plus installed `adl codefriend` consumer execution against isolated admitted fixtures, CPU/Rust/filesystem only and controlled clocks. Proof role: semantic correctness and actual consumer integration; required issue acceptance and later CF-PROOF/TAIL-01 input. Run `cargo test --manifest-path adl/Cargo.toml --test codefriend_cf_gov_ci` once authored, the focused touched shared-owner regressions and `cargo fmt --manifest-path adl/Cargo.toml --check`. Record actual nonzero test/fixture denominators and source/binary/OS/toolchain identities. New fixtures declare lane, role, determinism, resource profile and release-gate status in the tightly coupled manifest. CI/macOS/Linux evidence is separate from local proof; no unrun support claim. Use trusted same-host dependency warming where applicable. A schema, scaffold, authored packet, test-only consumer or zero-executed-scenario run cannot close this implementation issue. Report missing or failed proof as such; do not defer unfinished behavior to CF-INTEGRATE.

## Parallelization Plan

- Parallel groups: serial within this issue; independent fixtures may parallelize only with isolated state
- Validation runtime class: `bounded_local_and_required_ci`
- Validation resource profile: `local CPU/disk/Rust and controlled fixtures; normal PR CI separately; no provider calls, paid workflows or repository-script execution`
- Validation family: `codefriend_fitness_consumer_contract`
- Validation size split: `focused per source acceptance; no reflexive full workspace suite`

## Goal Accounting Hooks

- Issue goal ref: `Active whole Sprint 3 #929 goal includes #888 and all eight children; no replacement goal and no operator time or token cutoff.`
- Sprint goal ref: `Active whole Sprint 3 #929 goal includes #888 and all eight children; no replacement goal and no operator time or token cutoff.`
- Goal metrics rollup ref: `.csdlc/evidence/888/goal-metrics.json (planned; absent until execution)`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Local CPU/disk plus normal automatic PR CI. Retained numeric values are planning estimates only; the user authorized no time or token cutoff. Required proof must run to completion.`
- Planned validation seconds: `2400`
- Planned validation token budget: `16000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- Required local deterministic lane: production-module behavior plus installed `adl codefriend` consumer execution against isolated admitted fixtures, CPU/Rust/filesystem only and controlled clocks. Proof role: semantic correctness and actual consumer integration; required issue acceptance and later CF-PROOF/TAIL-01 input. Run `cargo test --manifest-path adl/Cargo.toml --test codefriend_cf_gov_ci` once authored, the focused touched shared-owner regressions and `cargo fmt --manifest-path adl/Cargo.toml --check`. Record actual nonzero test/fixture denominators and source/binary/OS/toolchain identities. New fixtures declare lane, role, determinism, resource profile and release-gate status in the tightly coupled manifest. CI/macOS/Linux evidence is separate from local proof; no unrun support claim. Use trusted same-host dependency warming where applicable. A schema, scaffold, authored packet, test-only consumer or zero-executed-scenario run cannot close this implementation issue. Report missing or failed proof as such; do not defer unfinished behavior to CF-INTEGRATE.

## Failure Semantics

- Required failures, skipped or zero-test proof block acceptance. Preserve guards; record durable anomalies; repair and rerun affected proof and independent exact-head review. CI evidence is separate from local proof.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Execution prerequisite #887 is satisfied by accepted PR #989 merge 49ca9f2fcf785a2f8e6b75676279db4f0936f925 and passing hosted CI. #888 is bound in its registered FastWork worktree and integrates all seven merged Sprint #929 siblings. Preserve the existing dependency graph and serialize shared CLI edits under Worker #10. VPP remains the proof plan, not execution evidence. Require focused tests, coverage, isolated installed candidate and pass/fail/error cases, and actual automatic hosted job/artifact evidence. Numeric estimates impose no execution time or token cutoff.
