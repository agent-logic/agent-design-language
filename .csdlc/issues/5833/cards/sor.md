# Structured Output Record

Template: 1.0.0

Issue: 5833

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented and proved WP-15 opaque-authority exact-candidate witnesses and deterministic path-redacted citizen receipts without manufacturing birth authority.

## Artifacts

- adl-runtime-kernel/src/birth_witness.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/tests/birth_witness.rs
- adl-runtime-kernel/tests/fixtures/birth_witness/authority_tests.rs
- adl-runtime-kernel/tests/fixtures/birth_witness/matrix.json
- docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md
- .csdlc/prepared/issues/5833/design.md
- .csdlc/prepared/issues/5833/produce-native-receipt.rb
- .csdlc/prepared/issues/5833/validate-native-receipts.rb
- .github/workflows/wp15-native-birth-witness.yml
- .csdlc/evidence/5833/local-validation-manifest.json
- .csdlc/evidence/5833/birth-witness-authority-runtime-v3.log
- .csdlc/evidence/5833/birth-witness-public-boundary.log
- .csdlc/evidence/5833/birth-witness-compile-fail.log
- .csdlc/evidence/5833/birth-witness-strict-clippy.log
- .csdlc/evidence/5833/birth-witness-native-scripts.log

## Execution

- Replaced caller-provisionable roots with a non-serializable opaque runtime-established policy; external authority establishment now fails to compile.
- Required exact canonical witness identity equality and executable all-case negative fixtures, closing case-variant equivocation and inert-matrix gaps.
- Replaced original evidence paths in the receipt with deterministic kind-and-digest tokens and rejected secret-bearing path syntax before projection.
- Added hidden-file opt-in to both exact native uploads while preserving four-file producer isolation and success-only exact eight-file aggregate retention.
- Updated native proof to the exact 13-test crate-internal authority lane with a separate one-test public serialization boundary.

## Validation

[
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "-E",
      "test(/^birth_witness::authority_tests::/)",
      "--no-tests=fail",
      "--status-level",
      "all"
    ],
    "purpose": "Prove the exact 13-test opaque authority, canonical identity, executable negative matrix, receipt redaction, signature, freshness, substitution, and no-premature-claim surface.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5833/birth-witness-authority-runtime-v3.log"
  },
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "birth_witness",
      "--no-tests=fail",
      "--status-level",
      "all"
    ],
    "purpose": "Prove the public packet serialization boundary rejects undeclared private fields without exposing authority establishment.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5833/birth-witness-public-boundary.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--doc",
      "birth_witness"
    ],
    "purpose": "Prove external callers cannot establish or nominate the opaque trusted witness root.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5833/birth-witness-compile-fail.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Require the authority-bearing library and public integration target to compile without warnings.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5833/birth-witness-strict-clippy.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5833/validate-native-receipts.rb",
      "--self-test"
    ],
    "purpose": "Prove log path hygiene, exact 13-test inventory scaffolding, both hidden-upload opt-ins, Ruby syntax, and workflow YAML before native CI.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5833/birth-witness-native-scripts.log"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
