# Structured Output Record

Template: 1.0.0

Issue: 5827

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Repaired WP-10 so continuity construction recomputes caller-visible WP-09 identity-record integrity before token comparisons, crate-private policy establishment requires VerifiedBirthdayEvidence, opaque verified cycles remain chain-bound to one exact authority context, generation overflow fails closed, and signed witnesses are accepted only at the exact generation-bound governed path.

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
- Accept only the exact signed live-kernel witness path evidence/continuity/live-kernel/cycle-{generation}.bin after safe_path checks; reject every arbitrary relabel, including separator, camelCase, acronym, lowercase concatenation, wrong-generation, and superficially safe variants.
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
    "purpose": "Prove real WP-09 authority grounding and nine deterministic stale caller-visible identity-record, token-chain, same-key-ID/different-key authority-context splice, overflow, replay, discontinuity, substitution, copied-state, exact canonical witness-path allowlist, and tamper cases at product revision a554b5598eb4a66277bab43530a184c28f0dbed4.",
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
