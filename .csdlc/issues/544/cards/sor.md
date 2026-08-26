# Structured Output Record

Template: 1.0.0

Issue: 544

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Adjusted initialized_decomposition_recovery fixtures to build their synthetic repository under a linked non-primary worktree so bootstrap creation remains compatible with the primary-checkout guard.

## Artifacts

- csdlc-v2/src/lifecycle.rs
- csdlc-v2/tests/primary_checkout_bootstrap_guard.rs
- docs/onboarding.md
- csdlc-v2/README.md
- csdlc-v2/src/lifecycle.rs
- csdlc-v2/tests/primary_checkout_bootstrap_guard.rs
- csdlc-v2/tests/gate2.rs
- csdlc-v2/tests/initialized_decomposition_recovery.rs

## Execution

- csdlc-v2/src/lifecycle.rs rejects ADL bootstrap from the Git topology primary checkout before binding locks, authored artifacts, issue records, prepared state, or lock files are created.
- csdlc-v2/tests/primary_checkout_bootstrap_guard.rs proves primary rejection with zero issue residue, non-primary staging success, idempotent staging initialization, and operator doc wording.
- csdlc-v2/src/lifecycle.rs unit tests cover missing primary topology, topology probe failure, and Git common-dir mismatch as fail-closed UnsafeCheckout cases.
- docs/onboarding.md and csdlc-v2/README.md state that the primary checkout is inspection-only and ADL bootstrap uses an isolated staging checkout.
- reject_primary_checkout_bootstrap now resolves `git rev-parse --path-format=absolute --show-toplevel` before comparing to the topology primary checkout.
- initialize_native_json runs the primary-checkout guard before native registry validation so primary subdirectory invocations fail before reads or writes.
- primary_checkout_bootstrap_guard.rs now covers primary subdirectory rejection with zero issue residue.
- Added a gate2 test helper that detaches the fixture primary checkout and checks out main in a linked worktree used as the test repository root.
- Updated focused and manual gate2 fixtures that bootstrap ADL issue records to use sibling primary checkouts plus linked non-primary worktrees.
- Kept production guard behavior unchanged; the fix is limited to test fixtures that previously initialized from their topology primary checkout.
- Changed fixture_repo in csdlc-v2/tests/initialized_decomposition_recovery.rs to create a sibling primary fixture repository and add the test root as a linked main worktree.
- Moved fixture file population into write_fixture_files so the primary fixture can be committed before the linked worktree is created.
- Kept production primary-checkout guard behavior unchanged; the fix is limited to a test fixture that previously inherited the Actions primary checkout topology.

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
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2"
    ],
    "purpose": "Targeted reproduction for the csdlc-v2-standalone failure caused by gate2 primary-checkout bootstrap fixtures.",
    "outcome": "passed",
    "evidence_ref": "local output after janitor fix: 14 passed, 0 failed"
  },
  {
    "command": [
      "cargo fmt --manifest-path csdlc-v2/Cargo.toml --all -- --check",
      "cargo clippy --locked --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings"
    ],
    "purpose": "Workflow-equivalent format and strict Clippy proof for the fixture-only janitor change.",
    "outcome": "passed",
    "evidence_ref": "local output after janitor fix: fmt --check passed; clippy finished without warnings"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "initialized_decomposition_recovery"
    ],
    "purpose": "Targeted reproduction for the csdlc-v2-standalone failure caused by initialized_decomposition_recovery bootstrapping inside the Actions primary checkout.",
    "outcome": "passed",
    "evidence_ref": "local output after janitor fix: 2 passed, 0 failed"
  },
  {
    "command": [
      "cargo fmt --manifest-path csdlc-v2/Cargo.toml --all -- --check",
      "cargo clippy --locked --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings"
    ],
    "purpose": "Workflow-equivalent format and strict Clippy proof for the second fixture-only janitor change.",
    "outcome": "passed",
    "evidence_ref": "local output after initialized_decomposition_recovery fixture fix: fmt --check passed; clippy finished without warnings"
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
