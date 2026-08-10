# Structured Output Record

Template: 1.0.0

Issue: 5830

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented and proved the deterministic WP-13 Runtime v3 cognitive-profile contract, including authority-bound evidence, canonical revision lineage, governed nonclaims, unique profile fields, bounded projections, privacy, explicit unsupported-inference boundaries, and exact-head native Linux/macOS semantic equivalence.

## Artifacts

- adl-runtime-kernel/src/cognitive_profile.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/tests/cognitive_profile.rs
- adl-runtime-kernel/tests/fixtures/cognitive_profile/matrix.json
- docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md
- .csdlc/prepared/issues/5830/produce-native-receipt.rb
- .csdlc/prepared/issues/5830/validate-native-receipts.rb
- .github/workflows/wp13-native-cognitive-profile.yml
- .csdlc/evidence/5830/cognitive-profile-runtime-v3.log
- .csdlc/evidence/5830/cognitive-profile-strict-clippy.log
- .csdlc/evidence/5830/cognitive-profile-native-scripts.log
- .csdlc/evidence/5830/review-fix-validation.json
- .csdlc/evidence/5830/native-platform/linux.json
- .csdlc/evidence/5830/native-platform/linux-nextest.log
- .csdlc/evidence/5830/native-platform/linux-semantic.json
- .csdlc/evidence/5830/native-platform/linux-source-manifest.json
- .csdlc/evidence/5830/native-platform/macos.json
- .csdlc/evidence/5830/native-platform/macos-nextest.log
- .csdlc/evidence/5830/native-platform/macos-semantic.json
- .csdlc/evidence/5830/native-platform/macos-source-manifest.json
- .csdlc/evidence/5830/native-platform/independent-validator.log
- .csdlc/evidence/5830/native-validation-manifest.json

## Execution

- Added full cognitive-profile construction and reconstruction validation over birthday, identity, continuity, and capability authorities.
- Bound seven evidence categories, exact revision digests, update actor and reason, predecessor lineage, privacy policy, canonical fields, and bounded internal/public projections.
- Rejected stale, missing, duplicate, colliding, or forbidden evidence; authority substitution; unexplained revision changes; secret and host paths; private-evidence public derivation; and unsupported status or personhood inferences.
- Rejected ungoverned extra nonclaims, recomputed complete predecessor profile and public-projection integrity, bound predecessor continuity and canonical history shape, and rejected case-fold duplicate field keys before revision-delta calculation.
- Added and executed the exact 11-test matrix on native Linux and macOS through the issue-specific WP-13 workflow; retained and independently validated the eight-file receipt packet with identical semantic output.

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
      "cognitive_profile",
      "--no-tests=fail",
      "--status-level",
      "all"
    ],
    "purpose": "Prove the exact 11-test profile and all review-fix regressions at product revision 0e2f1de8766c2495db8d5433bb8d5475cfe29712.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5830/review-fix-validation.json"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "cognitive_profile",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove strict lint cleanliness at the reviewed product revision.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5830/review-fix-validation.json"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5830/validate-native-receipts.rb",
      ".csdlc/evidence/5830/native-platform/linux.json",
      ".csdlc/evidence/5830/native-platform/macos.json"
    ],
    "purpose": "Independently bind exact candidate head 60197504fa86566c8c4b1983d3be09f89b595e94 to native Linux and macOS 11-test receipts from run 31400973911, exact source manifests, runner provenance, artifact custody, and identical semantic SHA dd8cb6901d88b1412ed89715ccf053f26eb95c47e69be2174b447e14ba08b736.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5830/native-validation-manifest.json"
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
