# Structured Output Record

Template: 1.0.0

Issue: 144

Repository: agent-logic/agent-design-language

Card: sor

Status: complete

## Summary

Repaired WP-13 cognitive profiles with a runtime-owned opaque Ed25519 authority policy, pinned canonical policy/evidence digests, complete genesis replay, and old-key-governed exact-epoch rotation; migrated both the corrective and merged generic WP-13 native proof routes to the exact fifteen-test authority lane. Replacement native Linux/macOS execution remains deferred to publication CI.

## Artifacts

- adl-runtime-kernel/src/cognitive_profile.rs
- adl-runtime-kernel/tests/cognitive_profile.rs
- adl-runtime-kernel/tests/fixtures/cognitive_profile/authority_tests.rs
- docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md
- .csdlc/prepared/issues/144/produce-native-receipt.rb
- .csdlc/prepared/issues/144/validate-native-receipts.rb
- .github/workflows/wp13-authority-repair.yml
- .csdlc/prepared/issues/5830/produce-native-receipt.rb
- .csdlc/prepared/issues/5830/validate-native-receipts.rb
- .github/workflows/wp13-native-cognitive-profile.yml
- .csdlc/evidence/144/cognitive-profile-authority-v1.log
- .csdlc/evidence/144/cognitive-profile-public-integration.log
- .csdlc/evidence/144/cognitive-profile-compile-fail.log
- .csdlc/evidence/144/cognitive-profile-native-scripts.log
- .csdlc/evidence/144/local-validation-manifest.json

## Execution

- Replace the caller-constructible authority root with an opaque runtime-owned cognitive authority policy whose private state can be established only inside the runtime crate.
- Pin canonical policy and evidence digests in the opaque authority policy and require every current and historical profile input to match those trusted pins.
- Bind every signed statement to profile, revision, predecessor, recomputed authority context, canonical input, canonical policy, and canonical evidence digests.
- Rebuild and verify the complete ordered predecessor chain through genesis, including every profile, public projection, authority proof, and exact link.
- Require rotation to be signed by the current old key, advance exactly one epoch, change real key material, and govern the new revision statement with the new key.
- Run both the corrective #144 and merged generic WP-13 native proof routes against the exact filtered fifteen-test crate-internal authority inventory while preserving their distinct workflow, issue, artifact, and provenance bindings.

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
      "--no-tests=fail",
      "--status-level",
      "all",
      "-E",
      "test(/^cognitive_profile::authority_tests::/)"
    ],
    "purpose": "Run the exact nonzero crate-internal opaque cognitive authority lane used by both native routes.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/144/cognitive-profile-authority-v1.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "cognitive_profile"
    ],
    "purpose": "Prove the public serialization and fail-closed boundary without exposing authority establishment.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/144/cognitive-profile-public-integration.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--doc",
      "cognitive_profile"
    ],
    "purpose": "Prove external callers cannot establish the opaque cognitive authority policy.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/144/cognitive-profile-compile-fail.log"
  },
  {
    "command": [
      "ruby",
      "-e",
      "ARGV.each { |path| RubyVM::InstructionSequence.compile_file(path) }",
      ".csdlc/prepared/issues/144/produce-native-receipt.rb",
      ".csdlc/prepared/issues/144/validate-native-receipts.rb",
      ".csdlc/prepared/issues/5830/produce-native-receipt.rb",
      ".csdlc/prepared/issues/5830/validate-native-receipts.rb"
    ],
    "purpose": "Compile and self-test both distinct WP-13 native proof script pairs and validate both workflow YAML files.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/144/cognitive-profile-native-scripts.log"
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
