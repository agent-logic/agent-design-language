# Structured Output Record

Template: 1.0.0

Issue: 5826

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Repaired WP-09 so external callers cannot establish self-consistent attacker trust roots: the runtime provisions an opaque BirthdayAuthorityPolicy capability internally, and construction requires canonical signed identity-memory/private-state evidence under that capability.

## Artifacts

- adl-runtime-kernel/src/birthday_identity.rs
- adl-runtime-kernel/tests/fixtures/birthday_identity/authority_tests.rs
- adl-runtime-kernel/tests/fixtures/birthday_identity/authority_recipe.json
- .csdlc/evidence/5826/birthday_identity-runtime-v3.log
- .csdlc/evidence/5826/local-validation-manifest.json
- .csdlc/prepared/issues/5826/produce-native-receipt.rb
- .csdlc/prepared/issues/5826/validate-native-receipts.rb
- .github/workflows/wp09-native-birthday-identity.yml

## Execution

- Made BirthdayAuthorityPolicy establishment crate-private while retaining the opaque policy as the runtime-provisioned capability required by evidence verification.
- Moved the authority proof behind the crate boundary and retained a compile-fail external policy-establishment test.
- Preserved signed IdentityBinding, MemoryCheckpoint, accepted PrivateStateRecord lineage, governed projection, signer/generation, tamper, raw-private, provenance, and continuity negatives.
- Updated native receipt production to bind the internal authority test, identity_memory.rs, private_state.rs, complete passed-test inventory, and semantic output.

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
      "test(/^birthday_identity::authority_tests::/)",
      "--no-tests=fail",
      "--status-level",
      "all"
    ],
    "purpose": "Prove canonical runtime authority construction and the complete fail-closed security/privacy matrix with a nonzero test count.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5826/birthday_identity-runtime-v3.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--doc",
      "birthday_identity"
    ],
    "purpose": "Prove external code cannot access the crate-private BirthdayAuthorityPolicy establishment function.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5826/local-validation-manifest.json"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove the repaired Runtime v3 authority surface is warning-free under strict Clippy.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5826/local-validation-manifest.json"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5826/validate-native-receipts.rb",
      ".csdlc/evidence/5826/native-platform/macos.json",
      ".csdlc/evidence/5826/native-platform/linux.json"
    ],
    "purpose": "Prove exact-head native macOS/Linux internal authority tests, matching source manifests, seven passing tests per platform, GitHub Actions provenance, and byte-identical semantic output.",
    "outcome": "passed",
    "evidence_ref": "https://github.com/agent-logic/agent-design-language/actions/runs/31365422176"
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
