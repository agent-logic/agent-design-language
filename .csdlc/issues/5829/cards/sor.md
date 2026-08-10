# Structured Output Record

Template: 1.0.0

Issue: 5829

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented and proved the deterministic WP-12 Runtime v3 capability envelope with exact evidence binding, provisioned policy separation, explicit grants and denials, bounded limits, canonical identity, and fail-closed privacy and provenance validation on native Linux and macOS.

## Artifacts

- adl-runtime-kernel/src/capability_envelope.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/tests/capability_envelope.rs
- adl-runtime-kernel/tests/fixtures/capability_envelope/matrix.json
- docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md
- .csdlc/prepared/issues/5829/produce-native-receipt.rb
- .csdlc/prepared/issues/5829/validate-native-receipts.rb
- .github/workflows/wp12-native-capability-envelope.yml
- .csdlc/evidence/5829/dependency-verification.json
- .csdlc/evidence/5829/capability_envelope-runtime-v3.log
- .csdlc/evidence/5829/native-validation-manifest.json
- .csdlc/evidence/5829/native-platform/linux.json
- .csdlc/evidence/5829/native-platform/linux-nextest.log
- .csdlc/evidence/5829/native-platform/linux-semantic.json
- .csdlc/evidence/5829/native-platform/linux-source-manifest.json
- .csdlc/evidence/5829/native-platform/macos.json
- .csdlc/evidence/5829/native-platform/macos-nextest.log
- .csdlc/evidence/5829/native-platform/macos-semantic.json
- .csdlc/evidence/5829/native-platform/macos-source-manifest.json
- .csdlc/evidence/5829/native-platform/independent-validator.log

## Execution

- Added a versioned canonical capability envelope bound to accepted WP-08 birthday evidence and digest-valid WP-09 identity evidence.
- Separated untrusted input from provisioned provider/model/tool/skill policy and rejected unknown, stale, colliding, escalating, or undeclared capabilities.
- Required explicit grants, denials, recurrence and resource ceilings, provenance, unsupported claims, and canonical deterministic ordering.
- Prevented rejected secret-like attacker values from appearing in serialized or debug diagnostics by retaining only stable SHA-256 fingerprints.
- Enforced lexical cross-platform repository paths and independent provider plus provider-scoped model case-fold collision boundaries.
- Added a focused 13-test target with a 33-case negative matrix and retained equivalent exact-head Linux x86_64 and macOS arm64 receipts from run 31391052361.

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
    "purpose": "Prove the exact 13-test canonicalization, evidence/provenance, policy separation, grants/denials/limits, privacy, path, collision, and reconstruction boundary.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5829/capability_envelope-runtime-v3.log"
  },
  {
    "command": [
      "github-actions",
      "wp12-native-capability-envelope",
      "run",
      "31391052361",
      "attempt",
      "1"
    ],
    "purpose": "Run the exact 13-test WP-12 inventory on native Linux x86_64 and macOS arm64 at PR head b2dcfd3075371d98b384b62d1443353ea73c48fc and require identical semantic output.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5829/native-validation-manifest.json"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5829/validate-native-receipts.rb",
      ".csdlc/evidence/5829/native-platform/linux.json",
      ".csdlc/evidence/5829/native-platform/macos.json"
    ],
    "purpose": "Independently revalidate retained GitHub Actions receipts in a detached exact-head checkout, including workflow/run provenance, source manifests, exact 13-test inventories, file digests, and semantic equivalence.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5829/native-platform/independent-validator.log"
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
