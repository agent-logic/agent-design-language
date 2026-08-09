# Structured Output Record

Template: 1.0.0

Issue: 5863

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented stable node and Guardian identity with authenticated fail-closed enrollment and hardened durable-state validation.

## Artifacts

- .csdlc/evidence/5863/execution-proof.json
- .csdlc/evidence/5863/negative-cases.json
- .csdlc/evidence/5863/distributed-identity.stdout.log
- .csdlc/evidence/5863/distributed-identity.stderr.log

## Execution

- adl-runtime/src/distributed/identity.rs
- adl-runtime/tests/distributed_identity.rs

## Validation

[
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_identity",
      "--no-tests=fail"
    ],
    "purpose": "Prove identity, enrollment, replay, bounded-resource, durable-corruption, and path-safety behavior",
    "outcome": "passed",
    "evidence_ref": "exact-child-tests.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5863/validate-proof-receipt.rb"
    ],
    "purpose": "Fail closed on stale or tampered WP-04.01 proof",
    "outcome": "passed",
    "evidence_ref": "exact-revision-proof-receipt.log"
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
