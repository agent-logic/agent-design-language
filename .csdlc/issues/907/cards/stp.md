---
issue_card_schema: adl.issue.v1
wp: "CSDLC-REMOTE"
slug: "v0922-remote-command-decomposition"
title: "[v0.92.2][CSDLC-REMOTE] Decompose the remote C-SDLC command owner"
labels:
  - "track:roadmap"
issue_number: 907
generated_at: "2026-09-16T00:27:06.103043+00:00"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "0.92.2"
required_outcome_type:
  - "implementation_and_executed_proof"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/907"
canonical_files: []
demo_required: true
demo_names: []
issue_graph_notes:
  - "#849 CSDLC-MERGE is accepted via merged PR #952 at merge commit b70ea9b9a12f642b46c71284f8df744ebaca2e7f. #862 CSDLC-DECOMPOSE is accepted via merged PR #1007 at merge commit 0b27711fb2678aa4ed59eddd42360122f45b6780, and native terminal finish succeeded. Both explicit prerequisites are satisfied."
pr_start:
  enabled: true
  slug: "v0922-remote-command-decomposition"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-16T00:27:06.103043+00:00

# Structured Task Prompt

## Summary

The remote command owner is decomposed into cohesive production modules while authority, credentials, GitHub operations, publication, receipts and reconciliation retain public contracts.

Dependencies: CSDLC-DECOMPOSE, CSDLC-MERGE. Numeric identities are attached during native creation.

## Goal

The remote command owner is decomposed into cohesive production modules while authority, credentials, GitHub operations, publication, receipts and reconciliation retain public contracts.

Dependencies: CSDLC-DECOMPOSE, CSDLC-MERGE. Numeric identities are attached during native creation.

## Required Outcome

The remote command owner is decomposed into cohesive production modules while authority, credentials, GitHub operations, publication, receipts and reconciliation retain public contracts.

Dependencies: CSDLC-DECOMPOSE, CSDLC-MERGE. Numeric identities are attached during native creation.

## Deliverables

Retain CSDLC-DECOMPOSE/#862 (local owner) and CSDLC-MERGE/#849 as accepted merged prerequisites. Re-resolve actual source after their changes and coordinate SIM command-owner/install boundaries. This task completes remote owner decomposition only, not local refactoring again.

Own `csdlc-v3/src/commands/remote/mod.rs` and cohesive new sibling modules beneath `commands/remote/`, preserving existing `merge.rs`, `tests.rs` and `tests/merge_cases.rs`. Inventory responsibilities: remote request/admission validation, authority/credential resolution, typed GitHub mutations, publication/review/readback, durable intent/idempotency, receipts and authenticated reconciliation. Keep one clear owner per responsibility and a thin stable entrypoint, with explicit dependency direction.

The remote command owner is decomposed into cohesive production modules while authority, credentials, GitHub operations, publication, receipts and reconciliation retain public contracts.

Dependencies: CSDLC-DECOMPOSE, CSDLC-MERGE. Numeric identities are attached during native creation.

## Acceptance Criteria

1. Extract the complete bounded remote responsibilities into cohesive production modules, with every existing route still using them. No replacement god module, parallel implementation, command-domain cycle or partially migrated route qualifies.
2. Preserve public CLI, schemas/serialized bytes, fields/defaults, error/status codes, artifact paths, authority and review checks, operation/intent/receipt digests, idempotency and fail-closed behavior. Preserve #849 closing/part-of linkage merge guard and current remote uncertainty recovery exactly.
3. Execute focused existing `csdlc-v3/tests/remote_publication_commands.rs`, operational CLI and remote merge-case suites as applicable, with golden serialized contracts and negative authority/review/base/head/linkage/corruption cases. Controlled fake remote transport must prove each affected mutation/readback and crash/reconciliation path; no live writes required by these tests.
4. Record before/after recursive source and responsibility inventory, route-to-owner mapping and dependency checks. Confirm thin dispatch and absence of cycles independently; facade line counts alone prove neither simplification nor preserved behavior.
5. Focused proof and required CI pass on the independently reviewed candidate; update bounded internal ownership/recovery docs. Build isolated candidate binaries, never replace another session's stable operational writer as a side effect of tests.

PVF: deterministic local C-SDLC contract/integration, controlled fake authenticated remote transport, bounded CPU/disk, required milestone support gate. No lifecycle feature/schema redesign, weakened guard, local-owner scope, broad cleanup or live remote mutation. Stop for overlapping ownership, unstable baseline, public/serialized drift, missing route proof or replacement god module.

acceptance: `remote_thin_dispatch`, `remote_regression_parity`, `before_after_responsibility_inventory`, `thin_entrypoints`, `no_command_domain_cycles`, `public_contract_compatible`, `decomposition_inventory_complete`.

pvf: `remote_thin_dispatch`, `remote_regression_parity`, `before_after_responsibility_inventory`, `public_contract_drift_rejected`, `replacement_god_module_rejected`, `domain_cycle_rejected`, `native_v3_regression`, `serialized_contract_goldens`, `module_dependency_check`, `before_after_inventory`.

stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `behavior_change`, `replacement_god_module`, `weakened_guard`.

non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `new_lifecycle_features`, `schema_redesign`, `unrelated_cleanup`.

Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`.

## Repo Inputs

Full live source contract, retained without dropping requirements:

# [v0.92.2][CSDLC-REMOTE] Decompose the remote C-SDLC command owner

## One complete result

The remote command owner is decomposed into cohesive production modules while authority, credentials, GitHub operations, publication, receipts and reconciliation retain public contracts.

Dependencies: CSDLC-DECOMPOSE, CSDLC-MERGE. Numeric identities are attached during native creation.

## Existing dependencies and source

Retain CSDLC-DECOMPOSE/#862 (local owner) and CSDLC-MERGE/#849 as accepted merged prerequisites. Re-resolve actual source after their changes and coordinate SIM command-owner/install boundaries. This task completes remote owner decomposition only, not local refactoring again.

Own `csdlc-v3/src/commands/remote/mod.rs` and cohesive new sibling modules beneath `commands/remote/`, preserving existing `merge.rs`, `tests.rs` and `tests/merge_cases.rs`. Inventory responsibilities: remote request/admission validation, authority/credential resolution, typed GitHub mutations, publication/review/readback, durable intent/idempotency, receipts and authenticated reconciliation. Keep one clear owner per responsibility and a thin stable entrypoint, with explicit dependency direction.

## Executed acceptance

1. Extract the complete bounded remote responsibilities into cohesive production modules, with every existing route still using them. No replacement god module, parallel implementation, command-domain cycle or partially migrated route qualifies.
2. Preserve public CLI, schemas/serialized bytes, fields/defaults, error/status codes, artifact paths, authority and review checks, operation/intent/receipt digests, idempotency and fail-closed behavior. Preserve #849 closing/part-of linkage merge guard and current remote uncertainty recovery exactly.
3. Execute focused existing `csdlc-v3/tests/remote_publication_commands.rs`, operational CLI and remote merge-case suites as applicable, with golden serialized contracts and negative authority/review/base/head/linkage/corruption cases. Controlled fake remote transport must prove each affected mutation/readback and crash/reconciliation path; no live writes required by these tests.
4. Record before/after recursive source and responsibility inventory, route-to-owner mapping and dependency checks. Confirm thin dispatch and absence of cycles independently; facade line counts alone prove neither simplification nor preserved behavior.
5. Focused proof and required CI pass on the independently reviewed candidate; update bounded internal ownership/recovery docs. Build isolated candidate binaries, never replace another session's stable operational writer as a side effect of tests.

PVF: deterministic local C-SDLC contract/integration, controlled fake authenticated remote transport, bounded CPU/disk, required milestone support gate. No lifecycle feature/schema redesign, weakened guard, local-owner scope, broad cleanup or live remote mutation. Stop for overlapping ownership, unstable baseline, public/serialized drift, missing route proof or replacement god module.

## Global startup and proof boundary

All 69 milestone issues must be created and reviewed before any implementation begins, as directed by the operator. Creation/review batches add no execution dependencies beyond that global startup gate and each task's declared prerequisites. Use native C-SDLC v3, current authority/readiness, a bound FastWork worktree and issue-bound goal. Reconcile actual owners before shared-path edits. Preserve repository/provider credentials and inspected source; stdout carries machine output and stderr redacted human diagnostics, including tested compatibility logging when relevant.

Deliver one complete result with required tests, failure handling and documentation; no schema/scaffold/unused library/zero-test or authored-packet-only completion. Classify every new proof case by lane, role, determinism, resources and release gate; distinguish local fixture proof, actual hardware/provider runs and required CI. Required proof not executed remains not proved, even when issue creation succeeds. No implementation, paid execution, activation or remote mutation is performed by this issue-creation batch.

## Inherited obligation ledger

acceptance: `remote_thin_dispatch`, `remote_regression_parity`, `before_after_responsibility_inventory`, `thin_entrypoints`, `no_command_domain_cycles`, `public_contract_compatible`, `decomposition_inventory_complete`.

pvf: `remote_thin_dispatch`, `remote_regression_parity`, `before_after_responsibility_inventory`, `public_contract_drift_rejected`, `replacement_god_module_rejected`, `domain_cycle_rejected`, `native_v3_regression`, `serialized_contract_goldens`, `module_dependency_check`, `before_after_inventory`.

stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `behavior_change`, `replacement_god_module`, `weakened_guard`.

non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `new_lifecycle_features`, `schema_redesign`, `unrelated_cleanup`.

Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`.


## Canonical execution links

Planning owner: #864. Creation/review batch: 7; this grouping adds no execution gate.
Execution prerequisite: #862 (CSDLC-DECOMPOSE); accepted output is required before dependent execution.
Execution prerequisite: #849 (CSDLC-MERGE); accepted output is required before dependent execution.

Reviewed creation source: `d955fd1bdbe7f79433dbcf1ea8426536ec8274a7`. This issue records a complete task; creation does not claim execution or acceptance.


<!-- csdlc-v3-operation:a30789e7521ab1ef30c480efdd96d36fdc2c72e34fb466ed7b3f5cf2cd359b88 -->

## Dependencies

#849 CSDLC-MERGE is accepted via merged PR #952 at merge commit b70ea9b9a12f642b46c71284f8df744ebaca2e7f. #862 CSDLC-DECOMPOSE is accepted via merged PR #1007 at merge commit 0b27711fb2678aa4ed59eddd42360122f45b6780, and native terminal finish succeeded. Both explicit prerequisites are satisfied.

## Target Files / Surfaces

Retain CSDLC-DECOMPOSE/#862 (local owner) and CSDLC-MERGE/#849 as accepted merged prerequisites. Re-resolve actual source after their changes and coordinate SIM command-owner/install boundaries. This task completes remote owner decomposition only, not local refactoring again.

Own `csdlc-v3/src/commands/remote/mod.rs` and cohesive new sibling modules beneath `commands/remote/`, preserving existing `merge.rs`, `tests.rs` and `tests/merge_cases.rs`. Inventory responsibilities: remote request/admission validation, authority/credential resolution, typed GitHub mutations, publication/review/readback, durable intent/idempotency, receipts and authenticated reconciliation. Keep one clear owner per responsibility and a thin stable entrypoint, with explicit dependency direction.

## Validation Plan

Planned deterministic local tooling/contract/integration proof, bounded local CPU/Git/Rust/disk, isolated primary and linked worktree fixtures plus controlled fake authenticated remote transport where a local boundary consumes remote or terminal facts. Preserve exact CLI/result/serialized bytes, request/receipt digests, durable paths, authority checks, errors, idempotency and fail-closed behavior. Record each route and negative scenario with nonzero denominator; facade line counts or a replacement god module are not acceptance.

Current source-grounded planned commands: `cargo test --manifest-path csdlc-v3/Cargo.toml --test remote_publication_commands`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test terminal_cleanup_cutover_commands`; `cargo fmt --manifest-path csdlc-v3/Cargo.toml --check`. Enumerate registered tests after predecessors land; preserve serialized-contract goldens and add focused meaningful missing boundary cases. These existing suites are inputs, not proof that every extraction is covered; retain a complete route-to-test inventory and run applicable internal remote merge cases after #849. No zero-test filtered invocation qualifies.

Record before/after recursive source/responsibility inventory, one owner per responsibility and module dependency direction; test both admitted and denied routes, mutation recovery and unchanged outputs against the exact candidate. Required CI and independent review are separate from local proof. Build isolated candidate binaries and never replace another session stable operational writer during tests.

`git diff --check` accompanies focused proof. Future proving commands are planned only; no Rust tests or product execution run during this preparation. Install the issue candidate through the approved installer only at execution with current provenance; do not replace a shared binary during preparation.

## Demo Expectations

Planned deterministic local tooling/contract/integration proof, bounded local CPU/Git/Rust/disk, isolated primary and linked worktree fixtures plus controlled fake authenticated remote transport where a local boundary consumes remote or terminal facts. Preserve exact CLI/result/serialized bytes, request/receipt digests, durable paths, authority checks, errors, idempotency and fail-closed behavior. Record each route and negative scenario with nonzero denominator; facade line counts or a replacement god module are not acceptance.

Current source-grounded planned commands: `cargo test --manifest-path csdlc-v3/Cargo.toml --test remote_publication_commands`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test terminal_cleanup_cutover_commands`; `cargo fmt --manifest-path csdlc-v3/Cargo.toml --check`. Enumerate registered tests after predecessors land; preserve serialized-contract goldens and add focused meaningful missing boundary cases. These existing suites are inputs, not proof that every extraction is covered; retain a complete route-to-test inventory and run applicable internal remote merge cases after #849. No zero-test filtered invocation qualifies.

Record before/after recursive source/responsibility inventory, one owner per responsibility and module dependency direction; test both admitted and denied routes, mutation recovery and unchanged outputs against the exact candidate. Required CI and independent review are separate from local proof. Build isolated candidate binaries and never replace another session stable operational writer during tests.

`git diff --check` accompanies focused proof. Future proving commands are planned only; no Rust tests or product execution run during this preparation. Install the issue candidate through the approved installer only at execution with current provenance; do not replace a shared binary during preparation.

## Non-goals

acceptance: `remote_thin_dispatch`, `remote_regression_parity`, `before_after_responsibility_inventory`, `thin_entrypoints`, `no_command_domain_cycles`, `public_contract_compatible`, `decomposition_inventory_complete`.

pvf: `remote_thin_dispatch`, `remote_regression_parity`, `before_after_responsibility_inventory`, `public_contract_drift_rejected`, `replacement_god_module_rejected`, `domain_cycle_rejected`, `native_v3_regression`, `serialized_contract_goldens`, `module_dependency_check`, `before_after_inventory`.

stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `behavior_change`, `replacement_god_module`, `weakened_guard`.

non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `new_lifecycle_features`, `schema_redesign`, `unrelated_cleanup`.

Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`.

All 69 milestone issues must be created and reviewed before any implementation begins, as directed by the operator. Creation/review batches add no execution dependencies beyond that global startup gate and each task's declared prerequisites. Use native C-SDLC v3, current authority/readiness, a bound FastWork worktree and issue-bound goal. Reconcile actual owners before shared-path edits. Preserve repository/provider credentials and inspected source; stdout carries machine output and stderr redacted human diagnostics, including tested compatibility logging when relevant.

Deliver one complete result with required tests, failure handling and documentation; no schema/scaffold/unused library/zero-test or authored-packet-only completion. Classify every new proof case by lane, role, determinism, resources and release gate; distinguish local fixture proof, actual hardware/provider runs and required CI. Required proof not executed remains not proved, even when issue creation succeeds. No implementation, paid execution, activation or remote mutation is performed by this issue-creation batch.

## Issue-Graph Notes

#849 CSDLC-MERGE is accepted via merged PR #952 at merge commit b70ea9b9a12f642b46c71284f8df744ebaca2e7f. #862 CSDLC-DECOMPOSE is accepted via merged PR #1007 at merge commit 0b27711fb2678aa4ed59eddd42360122f45b6780, and native terminal finish succeeded. Both explicit prerequisites are satisfied.

## Notes

#849 CSDLC-MERGE is accepted via merged PR #952 at merge commit b70ea9b9a12f642b46c71284f8df744ebaca2e7f. #862 CSDLC-DECOMPOSE is accepted via merged PR #1007 at merge commit 0b27711fb2678aa4ed59eddd42360122f45b6780, and native terminal finish succeeded. Current origin/main ced79b611d319bf93392985ff0ec3b790314795e contains both prerequisites. No branch, worktree, or PR exists for #907. Live re-inventory records remote/mod.rs at 4,046 lines plus existing intent.rs, merge.rs and merge_linkage.rs owners; the closed #975 worktree has no scoped modifications. Dependencies and paths are clear for native bind. Implementation, proof execution, review, publication, merge, stable-binary replacement and live remote mutation remain unstarted. Sprint 7 coordination is #933.

## Tooling Notes

Use current native v3 authority. Both prerequisites are accepted and scoped ownership is clear; bind natively, then create the issue-bound goal before implementation. Create an issue-bound Sprint #933 / child #907 goal only before implementation.
