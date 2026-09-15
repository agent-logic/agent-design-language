---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "v0922-codefriend-local-fitness-validation-plan"
issue: 887
task_id: "issue-0887"
run_id: "issue-0887"
version: "0.92.2"
title: "[v0.92.2][CF-GOV] Execute local architecture fitness functions"
branch: "not bound yet; proposed codex/887-v0922-codefriend-local-fitness"
generated_at: "2026-09-12T00:05:16.416878+00:00"
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
expected_proof_cost: "Planning estimate: local CPU/disk plus normal CI; reestimate after predecessor integration, not a budget authorization"
planned_validation_seconds: "1800"
planned_validation_tokens: "12000"
issue_goal_ref: "Active whole Sprint3#929 goal; #887 owns local machine-checkable fitness execution and installed proof."
sprint_goal_ref: "Sprint #929, all eight children #882 through #889"
goal_metrics_rollup_ref: ".csdlc/evidence/887/goal-metrics.json (planned; absent until execution)"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/887"
  - kind: "stp"
    ref: ".csdlc/issues/887/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/887/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/887/cards/spp.md"
selected_lanes:
  - "owner_binary; Required local deterministic lane: production-module behavior plus installed `adl codefriend` consumer execution against isolated admitted fixtures, CPU/Rust/filesystem only and controlled clocks. Proof role: semantic correctness and actual consumer integration; required issue acceptance and later CF-PROOF/TAIL-01 input. Run `cargo test --manifest-path adl/Cargo.toml --test codefriend_cf_gov` once authored, the focused touched shared-owner regressions and `cargo fmt --manifest-path adl/Cargo.toml --check`. Record actual nonzero test/fixture denominators and source/binary/OS/toolchain identities. New fixtures declare lane, role, determinism, resource profile and release-gate status in the tightly coupled manifest. CI/macOS/Linux evidence is separate from local proof; no unrun support claim. Use trusted same-host dependency warming where applicable. A schema, scaffold, authored packet, test-only consumer or zero-executed-scenario run cannot close this implementation issue. Report missing or failed proof as such; do not defer unfinished behavior to CF-INTEGRATE."
parallel_groups:
  - "Subprocess fixture tests use explicit isolation around exclusive evidence-store lifetimes; installed proof cases execute serially."
validation_commands:
  - "Required local deterministic lane: production-module behavior plus installed `adl codefriend` consumer execution against isolated admitted fixtures, CPU/Rust/filesystem only and controlled clocks. Proof role: semantic correctness and actual consumer integration; required issue acceptance and later CF-PROOF/TAIL-01 input. Run `cargo test --manifest-path adl/Cargo.toml --test codefriend_cf_gov` once authored, the focused touched shared-owner regressions and `cargo fmt --manifest-path adl/Cargo.toml --check`. Record actual nonzero test/fixture denominators and source/binary/OS/toolchain identities. New fixtures declare lane, role, determinism, resource profile and release-gate status in the tightly coupled manifest. CI/macOS/Linux evidence is separate from local proof; no unrun support claim. Use trusted same-host dependency warming where applicable. A schema, scaffold, authored packet, test-only consumer or zero-executed-scenario run cannot close this implementation issue. Report missing or failed proof as such; do not defer unfinished behavior to CF-INTEGRATE."
failure_policy: "Required failures, skipped or zero-test proof block acceptance. Preserve guards; record durable anomalies; repair and rerun affected proof and independent exact-head review. CI evidence is separate from local proof."
notes: "Local proof passed: 8 fitness tests plus 11 shared evidence tests under llvm-cov; 3 installed pass/fail/error scenarios with repeat, readback, tamper, deletion and source immutability checks. Strict Clippy passed. Fitness library line coverage 216/222 (97.30%); CLI 84/92 (91.30%). Exact-head review, publication and hosted CI pending."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Wait for accepted merged CF-EVIDENCE #881 and refresh its exact admitted evidence contract; define a visible versioned prohibited-module-dependency policy with pass/fail Rust fixtures; implement the local predicate and installed command with identity-bound deterministic artifact and pass/fail/error exits; reject malformed/unsupported policies, missing evidence and runner faults without invoking scripts; prove repeated installed execution, scope/redaction and contract handoff to #888; complete documentation and independent exact-head review.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `owner_binary`
- Planned PVF lane for execution: `owner_binary`

## Selected Validation Lanes

- owner_binary; Required local deterministic lane: production-module behavior plus installed `adl codefriend` consumer execution against isolated admitted fixtures, CPU/Rust/filesystem only and controlled clocks. Proof role: semantic correctness and actual consumer integration; required issue acceptance and later CF-PROOF/TAIL-01 input. Run `cargo test --manifest-path adl/Cargo.toml --test codefriend_cf_gov` once authored, the focused touched shared-owner regressions and `cargo fmt --manifest-path adl/Cargo.toml --check`. Record actual nonzero test/fixture denominators and source/binary/OS/toolchain identities. New fixtures declare lane, role, determinism, resource profile and release-gate status in the tightly coupled manifest. CI/macOS/Linux evidence is separate from local proof; no unrun support claim. Use trusted same-host dependency warming where applicable. A schema, scaffold, authored packet, test-only consumer or zero-executed-scenario run cannot close this implementation issue. Report missing or failed proof as such; do not defer unfinished behavior to CF-INTEGRATE.

## Parallelization Plan

- Parallel groups: Subprocess fixture tests use explicit isolation around exclusive evidence-store lifetimes; installed proof cases execute serially.
- Validation runtime class: `bounded_local_and_required_ci`
- Validation resource profile: `local CPU/disk/Rust and controlled fixtures; normal PR CI separately; no provider calls, paid workflows or repository-script execution`
- Validation family: `codefriend_fitness_consumer_contract`
- Validation size split: `focused per source acceptance; no reflexive full workspace suite`

## Goal Accounting Hooks

- Issue goal ref: `Active whole Sprint3#929 goal; #887 owns local machine-checkable fitness execution and installed proof.`
- Sprint goal ref: `Sprint #929, all eight children #882 through #889`
- Goal metrics rollup ref: `.csdlc/evidence/887/goal-metrics.json (planned; absent until execution)`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Planning estimate: local CPU/disk plus normal CI; reestimate after predecessor integration, not a budget authorization`
- Planned validation seconds: `1800`
- Planned validation token budget: `12000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- Required local deterministic lane: production-module behavior plus installed `adl codefriend` consumer execution against isolated admitted fixtures, CPU/Rust/filesystem only and controlled clocks. Proof role: semantic correctness and actual consumer integration; required issue acceptance and later CF-PROOF/TAIL-01 input. Run `cargo test --manifest-path adl/Cargo.toml --test codefriend_cf_gov` once authored, the focused touched shared-owner regressions and `cargo fmt --manifest-path adl/Cargo.toml --check`. Record actual nonzero test/fixture denominators and source/binary/OS/toolchain identities. New fixtures declare lane, role, determinism, resource profile and release-gate status in the tightly coupled manifest. CI/macOS/Linux evidence is separate from local proof; no unrun support claim. Use trusted same-host dependency warming where applicable. A schema, scaffold, authored packet, test-only consumer or zero-executed-scenario run cannot close this implementation issue. Report missing or failed proof as such; do not defer unfinished behavior to CF-INTEGRATE.

## Failure Semantics

- Required failures, skipped or zero-test proof block acceptance. Preserve guards; record durable anomalies; repair and rerun affected proof and independent exact-head review. CI evidence is separate from local proof.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Local proof passed: 8 fitness tests plus 11 shared evidence tests under llvm-cov; 3 installed pass/fail/error scenarios with repeat, readback, tamper, deletion and source immutability checks. Strict Clippy passed. Fitness library line coverage 216/222 (97.30%); CLI 84/92 (91.30%). Exact-head review, publication and hosted CI pending.
