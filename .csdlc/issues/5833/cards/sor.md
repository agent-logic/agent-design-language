# Structured Output Record

Template: 1.0.0

Issue: 5833

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented and locally proved WP-15 exact-candidate signed witness sets and deterministic redacted citizen receipts without manufacturing birth authority.

## Artifacts

- adl-runtime-kernel/src/birth_witness.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/tests/birth_witness.rs
- adl-runtime-kernel/tests/fixtures/birth_witness/matrix.json
- docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md
- .csdlc/prepared/issues/5833/produce-native-receipt.rb
- .csdlc/prepared/issues/5833/validate-native-receipts.rb
- .github/workflows/wp15-native-birth-witness.yml
- .csdlc/evidence/5833/local-validation-manifest.json
- .csdlc/evidence/5833/birth-witness-runtime-v3.log
- .csdlc/evidence/5833/birth-witness-strict-clippy.log
- .csdlc/evidence/5833/birth-witness-native-scripts.log

## Execution

- Added a canonical Runtime v3 birth-witness packet bound to an accepted BirthdayCandidate, exact reviewed evidence digest, provisioned four-role roster, current generation, and Ed25519 attestations.
- Added deterministic accepted/rejected witness dispositions and a citizen-facing receipt whose birth status remains not_claimed with fixed non-authority caveats.
- Added 12 focused positive and fail-closed tests for ordering, missing, duplicate, substitution, stale, forged, roster, packet, privacy, path, identifier, unknown-field, and premature-claim boundaries.
- Added a narrow issue-specific native macOS/Linux workflow with normalized structured logs, exact inventory validation, disjoint producer fragments, and success-only exact aggregate retention.

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
      "birth_witness",
      "--no-tests=fail",
      "--status-level",
      "all"
    ],
    "purpose": "Prove canonical signed witness binding, deterministic receipt derivation, privacy, anti-substitution, freshness, and no-premature-claim boundaries.",
    "outcome": "passed",
    "evidence_ref": "birth_witness-runtime-v3.log"
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
