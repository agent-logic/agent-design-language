# Structured Output Record

Template: 1.0.0

Issue: 544

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the ADL primary-checkout bootstrap guard before initialization writes, added focused regression coverage, and documented isolated staging bootstrap.

## Artifacts

- csdlc-v2/src/lifecycle.rs
- csdlc-v2/tests/primary_checkout_bootstrap_guard.rs
- docs/onboarding.md
- csdlc-v2/README.md

## Execution

- csdlc-v2/src/lifecycle.rs rejects ADL bootstrap from the Git topology primary checkout before binding locks, authored artifacts, issue records, prepared state, or lock files are created.
- csdlc-v2/tests/primary_checkout_bootstrap_guard.rs proves primary rejection with zero issue residue, non-primary staging success, idempotent staging initialization, and operator doc wording.
- csdlc-v2/src/lifecycle.rs unit tests cover missing primary topology, topology probe failure, and Git common-dir mismatch as fail-closed UnsafeCheckout cases.
- docs/onboarding.md and csdlc-v2/README.md state that the primary checkout is inspection-only and ADL bootstrap uses an isolated staging checkout.

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
