# Structured Output Record

Template: 1.0.0

Issue: 5871

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented bounded, canonical, signed capability advertisement evidence with durable replay defense and exact live certificate authorization.

## Artifacts

- .csdlc/evidence/5871/execution-proof.json
- .csdlc/evidence/5871/negative-cases.json
- .csdlc/evidence/5871/exact-child-tests.log
- .csdlc/evidence/5871/exact-revision-proof-receipt.log

## Execution

- Added an unregistered distributed capability advertisement module with domain-separated Ed25519 signing, canonical JCS encoding, deterministic sorting and deduplication, exact certificate binding, durable bounded replay high-water state, and evidence-only projections.
- Added a bounded path-based integration target with twelve focused tests covering valid projection, durable replay across expiry and restart, hard policy maxima, and all required fail-closed security and resource cases.

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
      "distributed_capability_advertisement",
      "--no-tests=fail"
    ],
    "purpose": "Validate canonical signatures, exact live certificate authorization, durable replay, absolute resource bounds, and fail-closed negative cases.",
    "outcome": "passed",
    "evidence_ref": "pvf-exact-child-tests.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5871/validate-proof-receipt.rb"
    ],
    "purpose": "Prove immutable source and evidence introduction bindings for issue 5871.",
    "outcome": "passed",
    "evidence_ref": "pvf-exact-revision-proof-receipt.log"
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
