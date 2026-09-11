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
