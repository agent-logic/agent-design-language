# Structured Output Record

Template: 1.0.0

Issue: 723

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Restored install approval binding and normalized current typed doctor output for proof and shadow fixtures.

## Artifacts

- csdlc-v3/src/commands/proof.rs
- csdlc-v3/tests/proof_parity_install_commands.rs
- csdlc-v3/tests/real_issue_canary.rs

## Execution

- Added fail-closed #505 install approval exact-head and digest validation
- Normalized current operational v3 doctor output alongside local-preparation output
- Made real issue fixtures resolve the canonical primary repository root

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "proof_parity_install_commands"
    ],
    "purpose": "Prove install approval, retained receipt, shadow comparison, and bounded soak behavior",
    "outcome": "passed",
    "evidence_ref": "local exact-head output: 6 passed, 0 failed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "real_issue_canary"
    ],
    "purpose": "Prove current operational doctor schema and primary-root fixture normalization",
    "outcome": "passed",
    "evidence_ref": "local exact-head output: 6 passed, 0 failed"
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
