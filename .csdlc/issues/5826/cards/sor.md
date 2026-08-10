# Structured Output Record

Template: 1.0.0

Issue: 5826

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Repaired WP-09 so Birthday Identity construction requires an opaque verified-evidence capability minted only under an established runtime authority policy and canonical signed identity-memory/private-state evidence.

## Artifacts

- adl-runtime-kernel/src/birthday_identity.rs
- adl-runtime-kernel/tests/birthday_identity.rs
- adl-runtime-kernel/tests/fixtures/birthday_identity/authority_recipe.json
- .csdlc/evidence/5826/birthday_identity-runtime-v3.log
- .csdlc/evidence/5826/local-validation-manifest.json
- .csdlc/prepared/issues/5826/produce-native-receipt.rb
- .csdlc/prepared/issues/5826/validate-native-receipts.rb
- .github/workflows/wp09-native-birthday-identity.yml

## Execution

- Bound identity origin and continuity to a signed IdentityBinding and signed MemoryCheckpoint verified through the established runtime trust policy.
- Bound privacy claims to an accepted signed PrivateStateRecord lineage and governed projection; removed caller-controlled visibility and redaction booleans.
- Added fail-closed authority, signer/generation, projection-tamper, raw-private, invented-provenance, and wrong-continuity proof.
- Expanded native receipts to bind identity_memory.rs, private_state.rs, the complete passed-test inventory, and byte-identical semantic output.

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
    "purpose": "Prove real signed authority construction and the complete fail-closed security/privacy matrix with a nonzero test count.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5826/birthday_identity-runtime-v3.log"
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
    "purpose": "Require exact-head native macOS/Linux authority manifests, machine-derived passed-test inventory, matching semantic output, and GitHub Actions provenance before review.",
    "outcome": "passed",
    "evidence_ref": "https://github.com/agent-logic/agent-design-language/actions/workflows/wp09-native-birthday-identity.yml"
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
