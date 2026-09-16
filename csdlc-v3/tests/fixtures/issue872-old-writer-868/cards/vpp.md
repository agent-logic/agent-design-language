---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "v0922-installed-command-contract-validation-plan"
issue: 868
task_id: "issue-0868"
run_id: "issue-0868"
version: "0.92.2"
title: "[v0.92.2][SIM-02] One current installed command contract"
branch: "codex/868-v0922-installed-command-contract"
generated_at: "2026-09-11T23:59:41.750973+00:00"
card_status: "ready"
status: "planned"
initial_pvf_lane: "tooling"
planned_pvf_lane: "tooling"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "bounded_local"
validation_resource_profile: "local CPU/disk/Git; isolated candidate and fake remote; no paid operations"
validation_family: "csdlc_contract_integration"
validation_size_split: "focused per source acceptance; no reflexive full workspace suite"
expected_proof_cost: "unknown"
planned_validation_seconds: "unknown"
planned_validation_tokens: "unknown"
issue_goal_ref: "not_created; create issue-bound goal before implementation"
sprint_goal_ref: "issue-866"
goal_metrics_rollup_ref: ".csdlc/evidence/868/goal-metrics.json (planned; absent until execution)"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/868"
  - kind: "stp"
    ref: ".csdlc/issues/868/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/868/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/868/cards/spp.md"
selected_lanes:
  - "tooling; Required deterministic local CPU/Rust/Git contract and installed-public-journey proof; fake authenticated remote transport only. Run focused command-manifest, operational CLI and proof/parity installation tests, plus fixture negatives for omitted descriptor, stale installed binary, help/schema/effect mismatch and forbidden fallback. `cargo test --manifest-path csdlc-v3/Cargo.toml --test command_manifest`, `--test operational_cli_commands`, `--test proof_parity_install_commands`, and focused terminal/remote tests when their contract changes; run Cargo formatting. Classify each new fixture's role, determinism/resources and required issue/SIM-07 release-gate status. Passing text comparison alone is insufficient: actual installed routes and negative operational errors must execute. Concrete baseline commands (retain required new installed scenarios from the acceptance contract; existing suites alone are insufficient): - `cargo test --manifest-path csdlc-v3/Cargo.toml --test command_manifest` - `cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands` - `cargo test --manifest-path csdlc-v3/Cargo.toml --test proof_parity_install_commands` - `cargo fmt --manifest-path csdlc-v3/Cargo.toml --check` - `git diff --check` Re-resolve suite names after merged predecessors; a renamed or absent suite requires a documented VPP update, not a skipped proof."
parallel_groups:
  - "serial within this issue; independent fixtures may parallelize only with isolated state"
validation_commands:
  - "Required deterministic local CPU/Rust/Git contract and installed-public-journey proof; fake authenticated remote transport only. Run focused command-manifest, operational CLI and proof/parity installation tests, plus fixture negatives for omitted descriptor, stale installed binary, help/schema/effect mismatch and forbidden fallback. `cargo test --manifest-path csdlc-v3/Cargo.toml --test command_manifest`, `--test operational_cli_commands`, `--test proof_parity_install_commands`, and focused terminal/remote tests when their contract changes; run Cargo formatting. Classify each new fixture's role, determinism/resources and required issue/SIM-07 release-gate status. Passing text comparison alone is insufficient: actual installed routes and negative operational errors must execute. Concrete baseline commands (retain required new installed scenarios from the acceptance contract; existing suites alone are insufficient): - `cargo test --manifest-path csdlc-v3/Cargo.toml --test command_manifest` - `cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands` - `cargo test --manifest-path csdlc-v3/Cargo.toml --test proof_parity_install_commands` - `cargo fmt --manifest-path csdlc-v3/Cargo.toml --check` - `git diff --check` Re-resolve suite names after merged predecessors; a renamed or absent suite requires a documented VPP update, not a skipped proof."
failure_policy: "Required failures, skipped or zero-test proof block acceptance. Preserve guards; record durable anomalies; repair and rerun affected proof and independent exact-head review. CI evidence is separate from local proof."
notes: "This is one implementation task in the first v0.92.2 C-SDLC simplification sprint. Use native C-SDLC v3, the current selector/receipt guards, an issue-bound FastWork worktree and an issue-bound session goal. Re-resolve current source and active owners before edits. Root main stays inspection-only. Resolve overlap with CSDLC-MAN/#861, CSDLC-DECOMPOSE/#862, CSDLC-REMOTE and other SIM workers before shared-path edits. Preserve all historical evidence bytes. This issue authorizes implementation and isolated candidate proof, not coordinated live writer activation, state conversion, replacement of the active operator binary, real GitHub effects, Runtime/provider shutdown, paid cloud/provider execution or a second live writer. The breaking replacement activates only through the separately authorized transition/pilot after SIM-06/07/08. Installation for proof means an isolated fixture-local candidate built from the exact reviewed source. Never work around an authority or stale-review failure. Wait for accepted merged output of #867; then refresh this plan against that exact source revision and re-resolve path ownership before binding. Preparation only: no implementation, proof success, independent implementation review, publication or live activation is claimed. Preserve existing owners #849/#862/#907 and the older worktree change to csdlc-v3/README.md. Recheck ownership before shared-path edits."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Freeze all 26 current routes plus guarded rollback against SIM-01 baseline; build the descriptor-backed command contract; reconcile installed help, request schemas, effect/result descriptions and source docs; fail closed on operational/construction mismatch; execute installed parity and safety fixtures and retain the same journey corpus.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `tooling`
- Planned PVF lane for execution: `tooling`

## Selected Validation Lanes

- tooling; Required deterministic local CPU/Rust/Git contract and installed-public-journey proof; fake authenticated remote transport only. Run focused command-manifest, operational CLI and proof/parity installation tests, plus fixture negatives for omitted descriptor, stale installed binary, help/schema/effect mismatch and forbidden fallback. `cargo test --manifest-path csdlc-v3/Cargo.toml --test command_manifest`, `--test operational_cli_commands`, `--test proof_parity_install_commands`, and focused terminal/remote tests when their contract changes; run Cargo formatting. Classify each new fixture's role, determinism/resources and required issue/SIM-07 release-gate status. Passing text comparison alone is insufficient: actual installed routes and negative operational errors must execute. Concrete baseline commands (retain required new installed scenarios from the acceptance contract; existing suites alone are insufficient): - `cargo test --manifest-path csdlc-v3/Cargo.toml --test command_manifest` - `cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands` - `cargo test --manifest-path csdlc-v3/Cargo.toml --test proof_parity_install_commands` - `cargo fmt --manifest-path csdlc-v3/Cargo.toml --check` - `git diff --check` Re-resolve suite names after merged predecessors; a renamed or absent suite requires a documented VPP update, not a skipped proof.

## Parallelization Plan

- Parallel groups: serial within this issue; independent fixtures may parallelize only with isolated state
- Validation runtime class: `bounded_local`
- Validation resource profile: `local CPU/disk/Git; isolated candidate and fake remote; no paid operations`
- Validation family: `csdlc_contract_integration`
- Validation size split: `focused per source acceptance; no reflexive full workspace suite`

## Goal Accounting Hooks

- Issue goal ref: `not_created; create issue-bound goal before implementation`
- Sprint goal ref: `issue-866`
- Goal metrics rollup ref: `.csdlc/evidence/868/goal-metrics.json (planned; absent until execution)`

## Proof Cost / Runtime Expectations

- Expected proof cost: `unknown`
- Planned validation seconds: `unknown`
- Planned validation token budget: `unknown`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- Required deterministic local CPU/Rust/Git contract and installed-public-journey proof; fake authenticated remote transport only. Run focused command-manifest, operational CLI and proof/parity installation tests, plus fixture negatives for omitted descriptor, stale installed binary, help/schema/effect mismatch and forbidden fallback. `cargo test --manifest-path csdlc-v3/Cargo.toml --test command_manifest`, `--test operational_cli_commands`, `--test proof_parity_install_commands`, and focused terminal/remote tests when their contract changes; run Cargo formatting. Classify each new fixture's role, determinism/resources and required issue/SIM-07 release-gate status. Passing text comparison alone is insufficient: actual installed routes and negative operational errors must execute. Concrete baseline commands (retain required new installed scenarios from the acceptance contract; existing suites alone are insufficient): - `cargo test --manifest-path csdlc-v3/Cargo.toml --test command_manifest` - `cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands` - `cargo test --manifest-path csdlc-v3/Cargo.toml --test proof_parity_install_commands` - `cargo fmt --manifest-path csdlc-v3/Cargo.toml --check` - `git diff --check` Re-resolve suite names after merged predecessors; a renamed or absent suite requires a documented VPP update, not a skipped proof.

## Failure Semantics

- Required failures, skipped or zero-test proof block acceptance. Preserve guards; record durable anomalies; repair and rerun affected proof and independent exact-head review. CI evidence is separate from local proof.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

This is one implementation task in the first v0.92.2 C-SDLC simplification sprint. Use native C-SDLC v3, the current selector/receipt guards, an issue-bound FastWork worktree and an issue-bound session goal. Re-resolve current source and active owners before edits. Root main stays inspection-only. Resolve overlap with CSDLC-MAN/#861, CSDLC-DECOMPOSE/#862, CSDLC-REMOTE and other SIM workers before shared-path edits. Preserve all historical evidence bytes. This issue authorizes implementation and isolated candidate proof, not coordinated live writer activation, state conversion, replacement of the active operator binary, real GitHub effects, Runtime/provider shutdown, paid cloud/provider execution or a second live writer. The breaking replacement activates only through the separately authorized transition/pilot after SIM-06/07/08. Installation for proof means an isolated fixture-local candidate built from the exact reviewed source. Never work around an authority or stale-review failure. Wait for accepted merged output of #867; then refresh this plan against that exact source revision and re-resolve path ownership before binding. Preparation only: no implementation, proof success, independent implementation review, publication or live activation is claimed. Preserve existing owners #849/#862/#907 and the older worktree change to csdlc-v3/README.md. Recheck ownership before shared-path edits.
