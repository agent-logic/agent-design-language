# Structured Output Record

Template: 1.0.0

Issue: 5865

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Validate the single-provider transport adapter with race-free authorization fixtures.

## Artifacts

- .csdlc/evidence/5865/execution-proof.json
- .csdlc/evidence/5865/distributed-transport.stdout.log
- .csdlc/evidence/5865/negative-cases.json

## Execution

- Use one AWS-LC Rustls provider.
- Remove the Unix-second authorization-boundary race from positive transport fixtures.

## Validation

[
  {
    "command": [
      "/Users/daniel/.cargo/bin/cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_transport",
      "--no-tests=fail"
    ],
    "purpose": "Run focused WP-04.03 transport tests.",
    "outcome": "passed",
    "evidence_ref": "exact-child-tests.log"
  },
  {
    "command": [
      "/usr/bin/ruby",
      ".csdlc/prepared/issues/5865/validate-proof-receipt.rb"
    ],
    "purpose": "Run the WP-04.03 proof receipt validator.",
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
