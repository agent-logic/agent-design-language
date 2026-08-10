# Structured Output Record

Template: 1.0.0

Issue: 144

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Repaired WP-13 cognitive profiles with externally provisioned Ed25519 authority, signed canonical policy/evidence/input statements, complete revision replay through genesis, and old-key-governed exact-epoch authority rotation; native Linux/macOS proof remains deferred to publication CI.

## Artifacts

- adl-runtime-kernel/src/cognitive_profile.rs
- adl-runtime-kernel/tests/cognitive_profile.rs
- adl-runtime-kernel/tests/fixtures/cognitive_profile/matrix.json
- docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md
- .csdlc/prepared/issues/144/produce-native-receipt.rb
- .csdlc/prepared/issues/144/validate-native-receipts.rb
- .github/workflows/wp13-authority-repair.yml
- .csdlc/evidence/144/cognitive-profile-authority-v1.log
- .csdlc/evidence/144/local-validation-manifest.json

## Execution

- Add a governed cognitive-profile API that separates provisioned verifying authority from untrusted input and permanently fails closed through the legacy self-authorizing API.
- Bind every signed statement to profile, revision, predecessor, recomputed authority context, canonical input, canonical policy, and canonical evidence digests.
- Rebuild and verify the complete ordered predecessor chain through genesis, including every profile, public projection, authority proof, and exact link.
- Require rotation to be signed by the current old key, advance exactly one epoch, change real key material, and govern the new revision statement with the new key.
- Add fifteen focused positive/adversarial cases plus issue-local native producer, validator, workflow, fixture, feature truth, and retained local proof.

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
    "purpose": "Run the exact nonzero issue-owned cognitive_profile target.",
    "outcome": "passed",
    "evidence_ref": "cognitive-profile-authority.log"
  },
  {
    "command": [
      "ruby",
      "-e",
      "ARGV.each { |path| RubyVM::InstructionSequence.compile_file(path) }",
      ".csdlc/prepared/issues/144/produce-native-receipt.rb",
      ".csdlc/prepared/issues/144/validate-native-receipts.rb"
    ],
    "purpose": "Compile the issue-local Ruby proof scripts.",
    "outcome": "passed",
    "evidence_ref": "cognitive-profile-native-scripts.log"
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
    "purpose": "Run strict Clippy over the exact issue-owned target.",
    "outcome": "passed",
    "evidence_ref": "cognitive-profile-strict-clippy.log"
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
