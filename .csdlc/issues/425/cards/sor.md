# Structured Output Record

Template: 1.0.0

Issue: 425

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Repaired PR #428 CI failure by reducing recordless_blocker argument count below strict Clippy threshold.

## Artifacts

- cargo test --manifest-path csdlc-v2/Cargo.toml --test gate_recordless_closeout: 4 passed
- cargo check --manifest-path csdlc-v2/Cargo.toml: passed
- git diff --check: passed
- recordless dry-run: 8 recordless_terminal_eligible, 1 conflicting_historical_publication (#248)
- r6 review P1 fixed: missing/unfetched PR head no longer collapses to projection-absent false
- cargo test --manifest-path csdlc-v2/Cargo.toml --test gate_recordless_closeout: 5 passed
- cargo check --manifest-path csdlc-v2/Cargo.toml: passed
- git diff --check: passed
- recordless dry-run refreshed: 8 recordless_terminal_eligible, 1 conflicting_historical_publication (#248)
- cargo fmt --manifest-path csdlc-v2/Cargo.toml --check: passed
- cargo test --manifest-path csdlc-v2/Cargo.toml --test gate_recordless_closeout: 5 passed
- cargo clippy --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings: passed

## Execution

- csdlc-v2/src/bin/csdlc-finish.rs: add recordless-closeout subcommand while preserving existing finish flags
- csdlc-v2/src/finish.rs: add recordless closeout request/result/receipt types, live classification, no-projection checks, historical-publication conflict detection, and recordless receipt retention
- csdlc-v2/src/lib.rs: export recordless closeout request/result types
- csdlc-v2/tests/gate_recordless_closeout.rs: add focused fail-closed classifier tests for eligible, source-projection, conflicting publication, and PR identity mismatch paths
- .csdlc/evidence/425-v092-residual-dry-run-result.json: retained live classify-only evidence over the nine v0.92 residuals
- csdlc-v2/src/finish.rs: source_projection_at_revision now verifies expected_head_sha resolves to a commit object and returns ReconciliationRequired when missing
- csdlc-v2/tests/gate_recordless_closeout.rs: added negative test for unavailable expected head before projection check
- .csdlc/evidence/425-v092-residual-dry-run-result.json: refreshed live dry-run evidence after the fix
- csdlc-v2/src/finish.rs: grouped recordless_blocker inputs in RecordlessBlockerContext to satisfy clippy::too_many_arguments without changing behavior

## Validation

[]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
