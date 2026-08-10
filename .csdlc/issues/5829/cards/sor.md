# Structured Output Record

Template: 1.0.0

Issue: 5829

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the deterministic WP-12 Runtime v3 capability envelope with exact evidence binding, explicit capability grants and denials, bounded resource limits, canonical ordering, and fail-closed privacy and provenance validation.

## Artifacts

- adl-runtime-kernel/src/capability_envelope.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/tests/capability_envelope.rs
- adl-runtime-kernel/tests/fixtures/capability_envelope/matrix.json
- docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md
- .csdlc/prepared/issues/5829/produce-native-receipt.rb
- .csdlc/prepared/issues/5829/validate-native-receipts.rb
- .csdlc/evidence/5829/dependency-verification.json
- .github/workflows/wp12-native-capability-envelope.yml

## Execution

- Added a versioned canonical capability envelope bound to accepted WP-08 birthday evidence and digest-valid WP-09 identity evidence.
- Separated untrusted input from provisioned provider/model/tool/skill policy and rejected unknown, stale, colliding, escalating, or undeclared capabilities.
- Required explicit grants, denials, recurrence and resource ceilings, provenance, unsupported claims, and canonical deterministic ordering.
- Added a focused 13-test integration target with a 33-case negative matrix covering stale provenance, escalation, omissions, secrets, private and host paths, collisions, and reconstructed packet forgery.
- Added issue-local native producer and validator plus a narrow WP-12 macOS/Linux workflow with disjoint fragments and success-only exact aggregate retention.

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
      "capability_envelope",
      "--no-tests=fail",
      "--status-level",
      "all"
    ],
    "purpose": "Prove deterministic canonicalization, evidence and provenance binding, explicit grants/denials/limits, policy separation, and fail-closed privacy boundaries.",
    "outcome": "passed",
    "evidence_ref": "capability_envelope-runtime-v3.log"
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
