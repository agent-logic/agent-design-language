# Structured Output Record

Template: 1.0.0

Issue: 208

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Resolved the actionable #208 review findings on current origin/main ancestry. The repaired bridge now exposes only sealed role-specific Runtime capabilities, verifies signed #204 activation decisions in the kernel, durably reconciles five live source participants and every target crash phase, atomically persists channel/journal/certificate succession, applies bounded cancellable effects through descriptor-anchored roots, and emits assertion-bound behavioral receipts. Fresh producer evidence and distinct rereview remain pending; publication and merge remain untouched.

## Artifacts

- adl-runtime-kernel/src/continuity_control.rs
- adl-runtime-kernel/src/ingress.rs
- adl-runtime/src/kernel_continuity_client.rs
- adl-runtime/src/distributed/polis_runtime.rs
- adl-runtime-kernel/tests/kernel_continuity_control.rs
- adl-runtime/tests/kernel_continuity_client.rs
- .csdlc/prepared/issues/208/produce-proof-receipt.rb
- .csdlc/prepared/issues/208/validate-proof-receipt.rb

## Execution

- Replaced the exposed generic client path with production-initialized sealed #204 migration and #210 transfer capabilities stored by the live polis runtime.
- Added durable source prepare, commit, resume, and RecoveryRequired state for the exact admission, recorder prefix, reasoning mutation, governance, and operation-state participants.
- Added restart-reconciled target staging, validation, discard, and signed-decision activation terminals with exact retry behavior across crash windows.
- Unified channel, journal, certificate succession, and retirement persistence and added hard concurrency bounds, deadline/cancellation checks, and descriptor-anchored filesystem safety.
- Replaced unconditional marker output with canonical assertion-bound behavior receipts and real restart, crash, TLS, certificate-role, forgery, and filesystem replacement tests; strengthened producer and validator freshness, ancestry, provenance, squash-safety, and no-LEAK checks.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "kernel_continuity_client",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Prove the repaired Runtime capability and live TLS boundary before immutable evidence production.",
    "outcome": "passed",
    "evidence_ref": "local pre-producer run: 21 passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "kernel_continuity_control",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Prove durable source and target crash reconciliation before immutable evidence production.",
    "outcome": "passed",
    "evidence_ref": "local pre-producer run: 35 passed"
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
    "purpose": "Reject Runtime API and production wiring warnings.",
    "outcome": "passed",
    "evidence_ref": "local pre-producer strict Clippy"
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
    "purpose": "Reject kernel effect and persistence warnings.",
    "outcome": "passed",
    "evidence_ref": "local pre-producer strict Clippy"
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
