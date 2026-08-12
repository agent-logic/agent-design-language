# Structured Output Record

Template: 1.0.0

Issue: 5912

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Added a Runtime-owned service that provisions opaque birth-witness policy, builds and validates packets, and emits canonical caveated receipts through a production path.

## Artifacts

- adl-runtime-kernel/src/birth_witness.rs
- adl-runtime-kernel/tests/birth_witness.rs
- .csdlc/prepared/issues/5912/validate-runtime-birth-witness.sh
- .csdlc/evidence/5912

## Execution

- Added public trusted Runtime authority configuration and an opaque RuntimeBirthWitnessService.
- Added one build-validate-emit path that emits only canonical receipt bytes after successful packet validation.
- Added an external integration test that provisions the production service, signs witnesses, and verifies exact receipt emission.

## Validation

[
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject warnings or invalid production and test API usage.",
    "outcome": "passed",
    "evidence_ref": "runtime-birth-witness-clippy.log"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/5912/validate-runtime-birth-witness.sh"
    ],
    "purpose": "Run the exact production-path integration test with a positive test count.",
    "outcome": "passed",
    "evidence_ref": "runtime-birth-witness-production-path.log"
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
