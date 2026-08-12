# Structured Output Record

Template: 1.0.0

Issue: 5912

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented and locally proved Runtime-owned birth-witness trust loading, service provisioning, canonical receipt staging, and fail-closed validation without exposing caller-nominated authority construction.

## Artifacts

- adl-runtime-kernel/src/birth_witness.rs
- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/tests/birth_witness.rs
- adl-runtime-kernel/tests/configuration.rs
- adl-runtime-kernel/tests/support/runtime_init.rs
- infra/runtime-v3/runtime-init.toml
- .csdlc/prepared/issues/5912/validate-runtime-birth-witness.sh
- .csdlc/evidence/5912/runtime-birth-witness-production-path.log
- .csdlc/evidence/5912/runtime-birth-witness-clippy.log

## Execution

- Made direct authority construction and RuntimeBirthWitnessService provisioning crate-private.
- Added an opaque trust object loaded only from the manifest path in validated Runtime credential initialization.
- Loaded the trust object during the production adl-runtime-kernel serve bootstrap.
- Proved the external production receipt path and all thirteen retained authority, privacy, canonicalization, and rejection cases.

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/5912/validate-runtime-birth-witness.sh"
    ],
    "purpose": "Prove validated Runtime-init trust loading, canonical production emission, fail-closed preparation, and all thirteen retained security regressions.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5912/runtime-birth-witness-production-path.log"
  },
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
    "purpose": "Prove warning-free production and test targets for the Runtime kernel.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5912/runtime-birth-witness-clippy.log"
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
