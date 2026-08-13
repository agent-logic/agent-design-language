# Structured Output Record

Template: 1.0.0

Issue: 299

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the initial #299 exact-authority archived-projection cleanup engine in a new isolated module with a focused test harness. The implementation validates cached #298 terminal merge authority and ancestry, creates an immutable cleanup ledger outside the private delete namespace, captures receipt-owned regular files and empty directories through atomic exchange with operation-owned placeholders, removes only captured private inodes with type-correct unlink/rmdir, fsyncs parent boundaries, validates placeholder identity before disposal, preserves third-party replacements, and resumes idempotently from recorded receipts.

## Artifacts

- csdlc-v2/src/projection_cleanup.rs
- csdlc-v2/tests/archived_projection_cleanup.rs

## Execution

- Added csdlc-v2/src/projection_cleanup.rs with typed cleanup request/result structures, terminal envelope and ancestry gate, immutable receipt ledger, safe relative path checks, exact identity capture, atomic exchange, type-correct removal, placeholder disposal, and replay handling.
- Added csdlc-v2/tests/archived_projection_cleanup.rs as the focused #[path = "../src/projection_cleanup.rs"] harness required by the pre-bind validator.
- Proved terminal gate fail-closed before mutation, exact file and empty-directory cleanup, idempotent repeat, replacement-inode preservation before capture, symlink and non-empty-directory rejection, resume after capture, and public replacement preservation before placeholder disposal.
- Kept initial implementation ownership to csdlc-v2/src/projection_cleanup.rs and csdlc-v2/tests/archived_projection_cleanup.rs; did not edit projection_recovery.rs, store.rs, or gate5.rs.

## Validation

[]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
