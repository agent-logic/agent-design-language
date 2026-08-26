# Structured Output Record

Template: 1.0.0

Issue: 544

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Resolved review finding REV-544-P1-PRIMARY-SUBDIR-BYPASS by comparing the invocation Git top-level to the primary worktree and adding a primary-subdirectory zero-residue regression test.

## Artifacts

- csdlc-v2/src/lifecycle.rs
- csdlc-v2/tests/primary_checkout_bootstrap_guard.rs
- docs/onboarding.md
- csdlc-v2/README.md
- csdlc-v2/src/lifecycle.rs
- csdlc-v2/tests/primary_checkout_bootstrap_guard.rs

## Execution

- csdlc-v2/src/lifecycle.rs rejects ADL bootstrap from the Git topology primary checkout before binding locks, authored artifacts, issue records, prepared state, or lock files are created.
- csdlc-v2/tests/primary_checkout_bootstrap_guard.rs proves primary rejection with zero issue residue, non-primary staging success, idempotent staging initialization, and operator doc wording.
- csdlc-v2/src/lifecycle.rs unit tests cover missing primary topology, topology probe failure, and Git common-dir mismatch as fail-closed UnsafeCheckout cases.
- docs/onboarding.md and csdlc-v2/README.md state that the primary checkout is inspection-only and ADL bootstrap uses an isolated staging checkout.
- reject_primary_checkout_bootstrap now resolves `git rev-parse --path-format=absolute --show-toplevel` before comparing to the topology primary checkout.
- initialize_native_json runs the primary-checkout guard before native registry validation so primary subdirectory invocations fail before reads or writes.
- primary_checkout_bootstrap_guard.rs now covers primary subdirectory rejection with zero issue residue.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib",
      "lifecycle"
    ],
    "purpose": "Focused lifecycle unit proof for fail-closed topology ambiguity and unchanged FastWork bind policy.",
    "outcome": "passed",
    "evidence_ref": "local output: 9 passed, 0 failed, 80 filtered out"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "primary_checkout_bootstrap_guard"
    ],
    "purpose": "Focused integration proof for primary rejection, zero residue, non-primary staging success, idempotence, and docs contract.",
    "outcome": "passed",
    "evidence_ref": "local output: 3 passed, 0 failed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "primary_checkout_bootstrap_guard"
    ],
    "purpose": "Focused integration proof after review remediation, including primary subdirectory rejection and zero residue.",
    "outcome": "passed",
    "evidence_ref": "local output after remediation: 4 passed, 0 failed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib",
      "lifecycle"
    ],
    "purpose": "Focused lifecycle unit proof after design recovery and reapproval.",
    "outcome": "passed",
    "evidence_ref": "local output after design recovery: 9 passed, 0 failed, 80 filtered out"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "primary_checkout_bootstrap_guard"
    ],
    "purpose": "Focused integration proof after design recovery and reapproval, including primary subdirectory rejection and zero residue.",
    "outcome": "passed",
    "evidence_ref": "local output after design recovery: 4 passed, 0 failed"
  }
]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
