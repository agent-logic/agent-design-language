# Structured Output Record

Template: 1.0.0

Issue: 5827

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Integrated WP-10 with real Runtime v3 LiveContinuity/CheckpointCoordinator output while preserving exact signed runtime authority, separately binding accepted opaque cycles to verified WP-09 identity context, recomputing caller-visible identity integrity, and failing closed on provenance, path, lineage, authority, and tamper mismatches.

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
- Recompute the complete caller-visible BirthdayIdentityRecord digest before continuity construction and reject stale stable-name, alias, provenance, witness, or other record mutations; record validation inherits the same fail-closed boundary.
- Bind every opaque verified cycle, continuity record, and continuity head to a canonical digest of the trusted Ed25519 key material, selected signer, identity record, generation floor, topology, configuration, service schema, and versioned authority-context schema; reject cross-policy token splicing even when key IDs match.
- Recheck exact identity-record digest, generation order, predecessor linkage, monotonic accepted-through, and unique integrity when constructing a continuity record from opaque verified cycles.
- Use checked generation advancement and reject terminal overflow.
- Adapt exact signed Runtime v3 manifests by requiring provenance runtime-v3-live-shutdown, the CheckpointCoordinator live_kernel service/schema and exact 0000-live_kernel.bin filename, signed runtime predecessor lineage, topology/config identity, and trusted Ed25519 authority; bind the first accepted cycle to the verified WP-09 identity head only inside the opaque authority-context token.
- Prove the positive path with two manifests emitted by real LiveContinuity::checkpoint and reject validly signed provenance/path substitutions plus unsigned manifest tamper without changing continuity.rs or live_continuity.rs.
- Replace the copied-source integration harness and placeholder identity fixture with a crate-internal authority lane that constructs real signed identity-memory and governed private-state evidence.
- Keep the native producer and validator bound to the exact unique nine-test authority inventory and complete WP-09/WP-10 source surface.

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
    "purpose": "Prove two real authoritative LiveContinuity/CheckpointCoordinator cycles integrate with verified WP-09 identity and nine deterministic provenance/path/tamper, stale identity-record, token-chain, same-key-ID/different-key authority-context splice, overflow, replay, discontinuity, substitution, and copied-state cases at product revision 2e99f069abef00078b51aef5eda46edd9b8f1cf3.",
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
    "purpose": "Prove the repaired runtime library authority surface is warning-free under strict Clippy.",
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
