# Structured Output Record

Template: 1.0.0

Issue: 200

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Rebased issue #200 onto exact merged-main 5e25dccebde3bdd608e3ecb80d3d60a0c40e3a90 after issue #208 changed the shared protected path adl-runtime/src/distributed/polis_runtime.rs. Git reconciled the histories cleanly; the current #200 delta on that file remains the bounded 24-line sealed reconciliation bridge atop #208's continuity work. The immutable v5 receipt at source 3c827d4f6abd909e4ab3b4225cf8e9c8827e8ef6 binds 36/36 cases, 13/13 subassertions, strict Clippy, portable fixture policy, and the exact post-#208 protected digests. Full Runtime remains 264/264. Fresh independent exact-head review is pending; PR #231 is stale and nothing is merged or closed.

## Artifacts

- adl-runtime/src/distributed/polis_runtime.rs
- .csdlc/prepared/issues/200/produce-proof-receipt.rb
- .csdlc/prepared/issues/200/validate-proof-receipt.rb
- .csdlc/evidence/200/v5/execution-proof.json

## Execution

- Synchronized the complete #200 history onto exact main 5e25dcceb after #208 merged.
- Reconciled the shared polis_runtime.rs path without conflict, preserving #208 continuity behavior and the bounded #200 sealed reconciliation bridge.
- Advanced immutable proof evidence from v4 to v5 so protected digests bind the exact post-#208 main ancestry.

## Validation

[
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "--test",
      "distributed_authority_reconciliation",
      "--no-tests=fail",
      "-E",
      "test(/authority_reconciliation/)"
    ],
    "purpose": "Prove the exact post-#208 unit-plus-integration denominator: 36 tests run and 36 passed.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/200/v5/execution-proof.json"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "--test",
      "distributed_authority_reconciliation",
      "authority_reconciliation",
      "--",
      "--nocapture",
      "--test-threads=1"
    ],
    "purpose": "Bind exactly 36 case markers and 13 required subassertion markers after main synchronization.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/200/v5/execution-proof.json"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "--test",
      "distributed_authority_reconciliation",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove the reconciled production and test target remains warning-free.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/200/v5/execution-proof.json"
  },
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "--no-tests=fail"
    ],
    "purpose": "Prove the full post-#208 Runtime library: 264 tests run and 264 passed.",
    "outcome": "passed",
    "evidence_ref": "adl-runtime/src/distributed/polis_runtime.rs"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/200/validate-proof-receipt.rb"
    ],
    "purpose": "Prove exact cases, subassertions, portability, immutable v5 proof, protected digests, and current-main ancestry.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/200/v5/execution-proof.json"
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
