# Structured Output Record

Template: 1.0.0

Issue: 208

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented all eight final-review remediation areas for #208: executable role-specific production continuity ports; independent boot-trusted #204 decision verification; exact durable five-participant prepare/resume reconciliation; bounded predecessor-linked resumable target streaming; assertion-site-only 56/64/12 proof markers; descriptor-relative no-follow inode/link-anchored filesystem effects; absolute deadline and cancellation coverage; and durable certificate-retirement refresh before exact retry. Both focused suites and both strict Clippy lanes pass locally; fresh v3 proof production and distinct exact-head rereview remain pending, and publication/merge remain untouched.

## Artifacts

- adl-runtime-kernel/src/continuity_control.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/tests/kernel_continuity_control.rs
- adl-runtime/src/kernel_continuity_client.rs
- adl-runtime/src/distributed/polis_runtime.rs
- adl-runtime/src/guardian.rs
- adl-runtime/tests/kernel_continuity_client.rs
- .csdlc/prepared/issues/208/produce-proof-receipt.rb
- .csdlc/prepared/issues/208/validate-proof-receipt.rb

## Execution

- Added retained executable #210 source/read/stage/write/verify/cleanup-request ports and #204 resume/activate/discard ports, including VerifiedTransferPossession and an opaque shared cleanup-permit handoff.
- Separated catalog-signing trust from boot-trusted migration-decision verification and bound decisions to root generation, catalog, cleanup permit, possession, route, membership, certificate, boot, and lineage cuts.
- Persisted source intent before admission closure and exact five-participant prepare and resume receipts; ambiguous, empty, or partial durable receipt state now remains closed in RecoveryRequired across restart.
- Added bounded target chunks with index, offset/range, predecessor receipt, per-chunk durable receipt, exact retry, and restart continuation before complete validation.
- Removed unconditional marker emission; each test now supplies only assertion-proved markers and must exactly match the 56-case, 64-boundary, and 12-lifecycle registry.
- Converted source, target, and Guardian journal leaf operations to descriptor-relative no-follow operations with root inode anchoring and single-link checks, plus root/intermediate/symlink/hard-link replacement tests.
- Wrapped client and server semaphore, connection, TLS, frame body, dispatch, read/write, quiesce, and shutdown work in cancellation and bounded absolute-deadline windows.
- Refreshed and persisted certificate retirement before retry lookup and fixed restart reconciliation so an already-resumed source operation is never demoted to committed.

## Validation

[
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "kernel_continuity_client",
      "--no-tests=fail"
    ],
    "purpose": "Prove the Runtime private continuity boundary and exact client denominator after remediation.",
    "outcome": "passed",
    "evidence_ref": "local resolver output: 21 tests run, 21 passed"
  },
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "kernel_continuity_control",
      "--no-tests=fail"
    ],
    "purpose": "Prove durable source/target reconciliation, streaming, decision verification, cleanup lifecycle, and filesystem races.",
    "outcome": "passed",
    "evidence_ref": "local resolver output: 35 tests run, 35 passed"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "--bin",
      "adl-runtime-guardian",
      "--test",
      "kernel_continuity_client",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject Runtime and Guardian warnings across the changed production and test surfaces.",
    "outcome": "passed",
    "evidence_ref": "local resolver output: strict Runtime Clippy passed"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "--bin",
      "adl-runtime-kernel",
      "--test",
      "kernel_continuity_control",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject kernel warnings across the changed effect, persistence, server, and proof surfaces.",
    "outcome": "passed",
    "evidence_ref": "local resolver output: strict kernel Clippy passed"
  }
]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
