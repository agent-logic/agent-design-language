# [v0.92.2][PLAT-RUST] Complete one selected production Rust responsibility refactor

## One complete result

One selected production Rust responsibility is fully extracted or simplified while preserving observable behavior, with recursive source accounting and focused regressions.

Dependencies: WP-01. Numeric identities are attached during native creation.

## Exact responsibility and ownership

Select process-status argument parsing/validation from `adl/src/cli/process_cmd.rs`: `ParsedStatus`, `parse_status_args`, `take_value`, `parse_pid`, `parse_port`, `validate_loopback_host`. The real caller is `real_process_status`. Move this coherent responsibility to proposed `adl/src/cli/process_cmd/args.rs` while simplifying redundant target-selection representation and control flow. Preserve the rest of process probing/output.

The inspected baseline has 482 lines and four separate optional targets followed by counting and a repeated selection chain ending in `expect`. Record exact baseline hash and recursive source/function/branch inventory before editing. A registered worktree ending `.worktrees/adl-process-status-fanout` has a dirty modification to this exact file. Preserve those bytes, identify its owner and resolve overlap before any implementation. Never take over, reset, cherry-pick or assume abandoned work from this issue's creation. Selection is concrete; execution ownership remains a prerequisite.

## Executed acceptance

1. Deliver the complete pure parser used by production `real_process_status`, preserving accepted/rejected argv, repeated options and option order, error text/order, defaults, mutually exclusive target semantics, zero PID/port rejection and loopback-only host rules. Keep syscall/network probes and output schema unchanged.
2. Reduce duplicated target-selection logic/control-flow complexity with an explicit before/after measure and reviewed rationale. Count recursive source totals and explain additions. A smaller parent facade, moved lines or more tests alone is not reduction; no arbitrary net-line claim substitutes for the actual simplification.
3. Exercise installed `adl process status` via existing `adl/tests/cli_smoke/process_status.rs` and focused parser cases, including malformed/missing/duplicate/conflicting flags and boundary values. Test safety denials without broad process scans or unsafe network targets. Preserve existing public outputs on all supported platforms.
4. Focused regressions, diff/format checks and required CI pass at independent exact-head review. No design-only refactor proposal, unused helper or compilation-only evidence closes the issue. Update narrowly affected process-status documentation if necessary.

PVF: deterministic local parser/CLI behavior-preservation contract, small CPU and controlled local process fixtures, required milestone support gate. No broad Rust rewrite, process-control feature changes, unrelated cleanup or other-owner takeover. Stop on unresolved dirty ownership, behavior drift, measurement without simplification or missing/failed proof.

## Global startup and proof boundary

All 69 milestone issues must be created and reviewed before any implementation begins, as directed by the operator. Creation/review batches add no execution dependencies beyond that global startup gate and each task's declared prerequisites. Use native C-SDLC v3, current authority/readiness, a bound FastWork worktree and issue-bound goal. Reconcile actual owners before shared-path edits. Preserve repository/provider credentials and inspected source; stdout carries machine output and stderr redacted human diagnostics, including tested compatibility logging when relevant.

Deliver one complete result with required tests, failure handling and documentation; no schema/scaffold/unused library/zero-test or authored-packet-only completion. Classify every new proof case by lane, role, determinism, resources and release gate; distinguish local fixture proof, actual hardware/provider runs and required CI. Required proof not executed remains not proved, even when issue creation succeeds. No implementation, paid execution, activation or remote mutation is performed by this issue-creation batch.

## Inherited obligation ledger

acceptance: `bounded_surface`, `behavior_preserved`, `measurable_reduction`, `exact_module_and_invariant_selected_before_creation`, `production_callers_preserved`, `recursive_before_after_inventory`.

pvf: `focused_regression`, `before_after_measurement`, `exact_module_and_invariant_selected_before_creation`, `production_callers_preserved`, `recursive_before_after_inventory`, `unselected_surface_rejected`, `facade_only_reduction_claim_rejected`.

stop_conditions: `scope_sprawl`, `behavior_redesign`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `production_responsibility_unselected`.

non_goals: `repo_wide_rewrite`.

Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`.
