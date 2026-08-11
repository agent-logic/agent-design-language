# Structured Output Record

Template: 1.0.0

Issue: 237

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Capability envelopes now canonically bind the exact verified continuity head and record digest, and governed cognition rejects substitution by a separately valid token; exact local proof is green and publication remains pending.

## Artifacts

- adl-runtime-kernel/src/birthday_continuity.rs
- adl-runtime-kernel/src/capability_envelope.rs
- adl-runtime-kernel/src/cognitive_profile.rs
- adl-runtime-kernel/tests/capability_envelope.rs
- adl-runtime-kernel/tests/fixtures/capability_envelope/authority_tests.rs
- adl-runtime-kernel/tests/fixtures/birthday_continuity/authority_tests.rs
- adl-runtime-kernel/tests/fixtures/cognitive_profile/authority_tests.rs

## Execution

- Require opaque VerifiedBirthdayContinuity on every public authoritative capability and governed-cognition build and validation route.
- Keep raw BirthdayContinuityRecord compatibility primitives crate-private and remove permissive either-or continuity acceptance.
- Bind the capability envelope canonical hash to the exact token continuity head and record digest, and require token-aware capability validation inside governed cognition.
- Prove two independently valid signed continuity tokens sharing identity and predecessor cannot be substituted after downstream cognition is rebuilt and re-signed.

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
    "purpose": "Run the focused public API target (1/1 passed).",
    "outcome": "passed",
    "evidence_ref": "continuity-public-api-target.log"
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
    "purpose": "Run the Runtime library authority suite (79/79 passed).",
    "outcome": "passed",
    "evidence_ref": "continuity-authority-lib.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--locked",
      "--doc"
    ],
    "purpose": "Run compile-fail public-boundary documentation tests (8/8 passed).",
    "outcome": "passed",
    "evidence_ref": "continuity-public-boundary-doc.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--locked",
      "--lib",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict library Clippy.",
    "outcome": "passed",
    "evidence_ref": "continuity-strict-lib-clippy.log"
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
