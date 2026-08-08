# Structured Output Record

Template: 1.0.0

Issue: 5864

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented purpose-separated distributed certificates with bounded rotation, revocation, expiry, atomic compromise fencing, and durable fail-closed authority.

## Artifacts

- .csdlc/evidence/5864/execution-proof.json
- .csdlc/evidence/5864/negative-cases.json
- .csdlc/evidence/5864/distributed-certificates.stdout.log
- .csdlc/evidence/5864/distributed-certificates.stderr.log

## Execution

- Added five non-interchangeable certificate purposes with Ed25519 root verification and bounded validity.
- Added monotonic rotation with bounded overlap and one identity per quorum vote.
- Added durable revocation, atomic identity compromise fencing, restart validation, and resource ceilings.
- Added exact positive, negative, recovery, corruption, and capacity regressions.

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
      "distributed_certificates",
      "--no-tests=fail"
    ],
    "purpose": "Run the exact distributed certificate regression target with nonzero selection.",
    "outcome": "passed",
    "evidence_ref": "exact-child-tests.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5864/validate-proof-receipt.rb"
    ],
    "purpose": "Fail closed on stale or tampered WP-04.02 proof.",
    "outcome": "passed",
    "evidence_ref": "exact-revision-proof-receipt.log"
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
