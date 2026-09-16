# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0907
Run ID: issue-0907
Version: 0.92.2
Title: [v0.92.2][CSDLC-REMOTE] Decompose the remote C-SDLC command owner
Branch: codex/907-v0922-remote-command-decomposition
Card Status: ready
Generated: 2026-09-16T00:27:06.103043+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/907
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/907
- Docs: AGENTS.md; docs/csdlc-v3/CURRENT_AUTHORITY.md; docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md; docs/milestones/v0.92.2/SPRINT_v0.92.2.md; docs/milestones/v0.92.2/cognitive-sdlc/C_SDLC_V3_SIMPLIFICATION_PLAN.md; csdlc-v3/AGENTS.md; csdlc-v3/README.md
- Other: none

## Agent Execution Rules
- This issue is not started yet; do not assume a branch or worktree already exists.
- Do not use v1 wrappers; bind execution with native v3 `csdlc bind` only if execution later becomes necessary.
- Do not delete or recreate cards.
- Do not switch branches unless explicitly instructed.
- Do not work on `main`.
- Only modify files required for the issue.
- Use repository-relative paths; avoid absolute host paths.
- Write the output record to the paired local task bundle `sor.md` path.
- If repository state is unexpected, stop and ask before attempting repository repair.

## Lifecycle Semantics
- Lifecycle stage: `SIP`
- Activation state: active after issue-intent review.
- Next stage: `STP`, where the selected task or solution is made explicit.
- Downstream planning path: `STP -> SPP -> VPP -> SRP -> SOR` once execution planning becomes concrete.
- Legacy compatibility: older references may call this an input card, but new issue work should treat it as the Structured Issue Prompt.

## Prompt Spec
```yaml
prompt_schema: adl.v1
actor:
  role: execution_agent
  name: codex
model:
  id: gpt-5-codex
  determinism_mode: stable
inputs:
  sections:
    - goal
    - required_outcome
    - acceptance_criteria
    - inputs
    - target_files_surfaces
    - validation_plan
    - demo_proof_requirements
    - constraints_policies
    - system_invariants
    - reviewer_checklist
    - non_goals_out_of_scope
    - notes_risks
    - instructions_to_agent
outputs:
  output_card: .csdlc/issues/907/cards/sor.md
  summary_style: concise_structured
constraints:
  include_system_invariants: true
  include_reviewer_checklist: true
  disallow_secrets: true
  disallow_absolute_host_paths: true
automation_hints:
  source_issue_prompt_required: true
  target_files_surfaces_recommended: true
  validation_plan_required: true
  required_outcome_type_supported: true
review_surfaces:
  - card_review_checklist.v1
  - card_review_output.v1
  - card_reviewer_gpt.v1.1
```

## Execution
- Agent:
- Provider:
- Tools allowed:
- Sandbox / approvals:
- Source issue-prompt slug: v0922-remote-command-decomposition
- Required outcome type: implementation_and_executed_proof
- Demo required: true

## Goal

The remote command owner is decomposed into cohesive production modules while authority, credentials, GitHub operations, publication, receipts and reconciliation retain public contracts.

Dependencies: CSDLC-DECOMPOSE, CSDLC-MERGE. Numeric identities are attached during native creation.

## Required Outcome

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

## Inputs

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

## Target Files / Surfaces

Retain CSDLC-DECOMPOSE/#862 (local owner) and CSDLC-MERGE/#849 as accepted merged prerequisites. Re-resolve actual source after their changes and coordinate SIM command-owner/install boundaries. This task completes remote owner decomposition only, not local refactoring again.

Own `csdlc-v3/src/commands/remote/mod.rs` and cohesive new sibling modules beneath `commands/remote/`, preserving existing `merge.rs`, `tests.rs` and `tests/merge_cases.rs`. Inventory responsibilities: remote request/admission validation, authority/credential resolution, typed GitHub mutations, publication/review/readback, durable intent/idempotency, receipts and authenticated reconciliation. Keep one clear owner per responsibility and a thin stable entrypoint, with explicit dependency direction.

## Validation Plan

Planned deterministic local tooling/contract/integration proof, bounded local CPU/Git/Rust/disk, isolated primary and linked worktree fixtures plus controlled fake authenticated remote transport where a local boundary consumes remote or terminal facts. Preserve exact CLI/result/serialized bytes, request/receipt digests, durable paths, authority checks, errors, idempotency and fail-closed behavior. Record each route and negative scenario with nonzero denominator; facade line counts or a replacement god module are not acceptance.

Current source-grounded planned commands: `cargo test --manifest-path csdlc-v3/Cargo.toml --test remote_publication_commands`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test terminal_cleanup_cutover_commands`; `cargo fmt --manifest-path csdlc-v3/Cargo.toml --check`. Enumerate registered tests after predecessors land; preserve serialized-contract goldens and add focused meaningful missing boundary cases. These existing suites are inputs, not proof that every extraction is covered; retain a complete route-to-test inventory and run applicable internal remote merge cases after #849. No zero-test filtered invocation qualifies.

Record before/after recursive source/responsibility inventory, one owner per responsibility and module dependency direction; test both admitted and denied routes, mutation recovery and unchanged outputs against the exact candidate. Required CI and independent review are separate from local proof. Build isolated candidate binaries and never replace another session stable operational writer during tests.

`git diff --check` accompanies focused proof. Future proving commands are planned only; no Rust tests or product execution run during this preparation. Install the issue candidate through the approved installer only at execution with current provenance; do not replace a shared binary during preparation.

## Demo / Proof Requirements

Planned deterministic local tooling/contract/integration proof, bounded local CPU/Git/Rust/disk, isolated primary and linked worktree fixtures plus controlled fake authenticated remote transport where a local boundary consumes remote or terminal facts. Preserve exact CLI/result/serialized bytes, request/receipt digests, durable paths, authority checks, errors, idempotency and fail-closed behavior. Record each route and negative scenario with nonzero denominator; facade line counts or a replacement god module are not acceptance.

Current source-grounded planned commands: `cargo test --manifest-path csdlc-v3/Cargo.toml --test remote_publication_commands`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test terminal_cleanup_cutover_commands`; `cargo fmt --manifest-path csdlc-v3/Cargo.toml --check`. Enumerate registered tests after predecessors land; preserve serialized-contract goldens and add focused meaningful missing boundary cases. These existing suites are inputs, not proof that every extraction is covered; retain a complete route-to-test inventory and run applicable internal remote merge cases after #849. No zero-test filtered invocation qualifies.

Record before/after recursive source/responsibility inventory, one owner per responsibility and module dependency direction; test both admitted and denied routes, mutation recovery and unchanged outputs against the exact candidate. Required CI and independent review are separate from local proof. Build isolated candidate binaries and never replace another session stable operational writer during tests.

`git diff --check` accompanies focused proof. Future proving commands are planned only; no Rust tests or product execution run during this preparation. Install the issue candidate through the approved installer only at execution with current provenance; do not replace a shared binary during preparation.

## Constraints / Policies

- Follow `AGENTS.md`.
- Use authenticated native C-SDLC v3 for lifecycle routing. Retained typed v2 requires explicit issue-scoped rollback or remediation approval.
- Edit cards only with editor skills.
- Work only in the bound issue worktree after native v3 `csdlc bind`.
- Keep validation focused on the touched surface.

## System Invariants (must remain true)

- Deterministic execution for identical inputs.
- No hidden state or undeclared side effects.
- Artifacts remain replay-compatible with the replay runner.
- Trace artifacts contain no secrets, prompts, tool arguments, or absolute host paths.
- Artifact schema changes are explicit and approved.

## Reviewer Checklist (machine-readable hints)
```yaml
determinism_required: true
network_allowed: false
artifact_schema_change: false
replay_required: true
security_sensitive: true
ci_validation_required: true
```

## Card Automation Hooks (prompt generation)
- Prompt source fields:
  - Goal
  - Required Outcome
  - Acceptance Criteria
  - Inputs
  - Target Files / Surfaces
  - Validation Plan
  - Demo / Proof Requirements
  - Constraints / Policies
  - System Invariants
  - Reviewer Checklist
- Generation requirements:
  - Deterministic output for identical SIP content
  - No secrets, tokens, or absolute host paths in generated prompt text
  - Preserve traceability back to the source issue prompt
  - Preserve explicit required-outcome and demo/proof requirements

## Non-goals / Out of scope

acceptance: `remote_thin_dispatch`, `remote_regression_parity`, `before_after_responsibility_inventory`, `thin_entrypoints`, `no_command_domain_cycles`, `public_contract_compatible`, `decomposition_inventory_complete`.

pvf: `remote_thin_dispatch`, `remote_regression_parity`, `before_after_responsibility_inventory`, `public_contract_drift_rejected`, `replacement_god_module_rejected`, `domain_cycle_rejected`, `native_v3_regression`, `serialized_contract_goldens`, `module_dependency_check`, `before_after_inventory`.

stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `behavior_change`, `replacement_god_module`, `weakened_guard`.

non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `new_lifecycle_features`, `schema_redesign`, `unrelated_cleanup`.

Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`.

All 69 milestone issues must be created and reviewed before any implementation begins, as directed by the operator. Creation/review batches add no execution dependencies beyond that global startup gate and each task's declared prerequisites. Use native C-SDLC v3, current authority/readiness, a bound FastWork worktree and issue-bound goal. Reconcile actual owners before shared-path edits. Preserve repository/provider credentials and inspected source; stdout carries machine output and stderr redacted human diagnostics, including tested compatibility logging when relevant.

Deliver one complete result with required tests, failure handling and documentation; no schema/scaffold/unused library/zero-test or authored-packet-only completion. Classify every new proof case by lane, role, determinism, resources and release gate; distinguish local fixture proof, actual hardware/provider runs and required CI. Required proof not executed remains not proved, even when issue creation succeeds. No implementation, paid execution, activation or remote mutation is performed by this issue-creation batch.

## Notes / Risks

#849 CSDLC-MERGE is accepted via merged PR #952 at merge commit b70ea9b9a12f642b46c71284f8df744ebaca2e7f. #862 CSDLC-DECOMPOSE is accepted via merged PR #1007 at merge commit 0b27711fb2678aa4ed59eddd42360122f45b6780, and native terminal finish succeeded. Current origin/main ced79b611d319bf93392985ff0ec3b790314795e contains both prerequisites. No branch, worktree, or PR exists for #907. Live re-inventory records remote/mod.rs at 4,046 lines plus existing intent.rs, merge.rs and merge_linkage.rs owners; the closed #975 worktree has no scoped modifications. Dependencies and paths are clear for native bind. Implementation, proof execution, review, publication, merge, stable-binary replacement and live remote mutation remain unstarted. Sprint 7 coordination is #933.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
