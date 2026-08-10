# Structured Output Record

Template: 1.0.0

Issue: 5827

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Repaired WP-10 so crate-private continuity policy establishment requires WP-09 VerifiedBirthdayEvidence, opaque verified cycles remain chain-bound after verification, and generation overflow fails closed.

## Artifacts

- adl-runtime-kernel/src/birthday_continuity.rs
- adl-runtime-kernel/src/birthday_identity.rs
- adl-runtime-kernel/tests/fixtures/birthday_continuity/authority_tests.rs
- adl-runtime-kernel/tests/fixtures/birthday_identity/authority_tests.rs
- .csdlc/evidence/5827/local-validation-manifest.json
- .csdlc/prepared/issues/5827/produce-native-receipt.rb
- .csdlc/prepared/issues/5827/validate-native-receipts.rb
- .github/workflows/wp10-native-birthday-continuity.yml

## Execution

- Require validate_birthday_identity_record against opaque WP-09 VerifiedBirthdayEvidence before the runtime continuity policy can accept an identity record.
- Recheck exact identity-record digest, generation order, predecessor linkage, monotonic accepted-through, and unique integrity when constructing a continuity record from opaque verified cycles.
- Use checked generation advancement and reject terminal overflow.
- Replace the copied-source integration harness and placeholder identity fixture with a crate-internal authority lane that constructs real signed identity-memory and governed private-state evidence.
- Update the native producer and validator to bind the internal nine-test authority lane and complete WP-09/WP-10 source manifest.

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
      "test(/^birthday_continuity::authority_tests::/)",
      "--no-tests=fail",
      "--status-level",
      "all"
    ],
    "purpose": "Prove real WP-09 authority grounding and nine deterministic token-chain, overflow, replay, discontinuity, substitution, copied-state, path, and tamper cases.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5827/local-validation-manifest.json"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--doc",
      "birthday_continuity"
    ],
    "purpose": "Prove external callers cannot establish a continuity authority policy.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5827/local-validation-manifest.json"
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
    "purpose": "Prove the runtime library authority surface is warning-free under strict Clippy.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5827/local-validation-manifest.json"
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
