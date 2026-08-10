# Structured Output Record

Template: 1.0.0

Issue: 5827

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented sealed-authority Birthday continuity across two or more signed bounded runtime cycles with deterministic replay and fail-closed discontinuity semantics.

## Artifacts

- adl-runtime-kernel/src/birthday_continuity.rs
- adl-runtime-kernel/tests/birthday_continuity.rs
- adl-runtime-kernel/tests/fixtures/birthday_continuity/identity_record.json
- .github/workflows/wp10-native-birthday-continuity.yml
- .csdlc/prepared/issues/5827/produce-native-receipt.rb
- .csdlc/prepared/issues/5827/validate-native-receipts.rb

## Execution

- Add a canonical Birthday continuity record whose head binds the accepted Birthday Identity record and ordered signed runtime checkpoint cycles.
- Require crate-private signer, generation, topology, configuration, and service-schema policy before opaque verified-cycle evidence can be constructed.
- Add seven focused replay, authority, tamper, path, copied-state, and discontinuity tests plus an issue-specific exact-head native workflow and receipt validator.

## Validation

[
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "birthday_continuity",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict focused Clippy.",
    "outcome": "passed",
    "evidence_ref": "birthday-continuity-clippy.log"
  },
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "birthday_continuity",
      "--no-tests=fail",
      "--status-level",
      "all"
    ],
    "purpose": "Run the exact issue-owned Birthday continuity integration target.",
    "outcome": "passed",
    "evidence_ref": "birthday-continuity-tests.log"
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
