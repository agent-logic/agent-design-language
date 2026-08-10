# Structured Output Record

Template: 1.0.0

Issue: 5826

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the deterministic WP-09 stable-name and identity-root contract with fail-closed provenance, continuity, collision, privacy, path, and serialization boundaries.

## Artifacts

- adl-runtime-kernel/src/birthday_identity.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/tests/birthday_identity.rs
- adl-runtime-kernel/tests/fixtures/birthday_identity
- docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md
- .csdlc/prepared/issues/5826/produce-native-receipt.rb
- .csdlc/prepared/issues/5826/validate-native-receipts.rb
- .github/workflows/wp09-native-birthday-identity.yml

## Execution

- Added canonical birthday identity derivation and retained-record validation.
- Added positive replay proof and table-driven provenance, continuity, collision, privacy, path, and unknown-field rejection cases.
- Added the narrowly issue-specific native macOS/Linux receipt workflow and exact-run provenance validator; native execution remains deferred to GitHub Actions after publication.

## Validation

[
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "birthday_identity",
      "--no-tests=fail",
      "--status-level",
      "all"
    ],
    "purpose": "Prove deterministic stable-name and identity-root behavior plus the bounded negative matrix.",
    "outcome": "passed",
    "evidence_ref": "birthday_identity-runtime-v3.log"
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
