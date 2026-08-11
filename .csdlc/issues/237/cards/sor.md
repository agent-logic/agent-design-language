# Structured Output Record

Template: 1.0.0

Issue: 237

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented an opaque verified Birthday continuity token and token-consuming capability/cognitive paths; exact review, publication, CI, merge, and closeout remain pending.

## Artifacts

- adl-runtime-kernel/src/birthday_continuity.rs
- adl-runtime-kernel/src/capability_envelope.rs
- adl-runtime-kernel/src/cognitive_profile.rs
- adl-runtime-kernel/tests/fixtures/birthday_continuity/authority_tests.rs

## Execution

- Added VerifiedBirthdayContinuity, constructible only after full canonical record validation over opaque verified Runtime cycles.
- Added capability and governed-cognitive entrypoints that consume the same verified continuity token.
- Added self-consistently rehashed record/head/root/identity-digest substitution rejection while preserving retained authority and privacy tests.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--locked",
      "--test",
      "capability_envelope"
    ],
    "purpose": "Run the focused capability integration suite.",
    "outcome": "passed",
    "evidence_ref": "continuity-capability.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--locked",
      "--lib"
    ],
    "purpose": "Run the Runtime library test suite.",
    "outcome": "passed",
    "evidence_ref": "continuity-cognitive-authority.log"
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
