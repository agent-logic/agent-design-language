# Structured Output Record

Template: 1.0.0

Issue: 258

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented #258 as the first #203 split slice: sealed raw certificate, lease, and fencing store access behind explicit authority/test access tokens; added authority-bound store adapter facade and expanded published receipt view; updated compile-required fixture callers without touching #203 sibling transport or migration scope.

## Artifacts

- .csdlc/evidence/258/cargo-check-adl-runtime.log
- .csdlc/evidence/258/cargo-test-distributed-identity-lease-authority.log
- .csdlc/evidence/258/cargo-clippy-distributed-identity-lease-authority.log
- .csdlc/evidence/258/cargo-test-distributed-projection-no-run.log
- .csdlc/evidence/258/cargo-test-distributed-resource-weather-no-run.log
- .csdlc/evidence/258/cargo-test-distributed-discovery-no-run.log
- .csdlc/evidence/258/cargo-test-distributed-placement-no-run.log

## Execution

- Added authority store access-token gates for certificate, lease, and fencing raw store APIs.
- Added authority-bound store adapter facade and published receipt view metadata for the #258 boundary slice.
- Updated focused runtime tests and compile-required fixture callers to use explicit test access tokens.

## Validation

[
  {
    "command": [
      "cargo",
      "check",
      "--manifest-path",
      "adl-runtime/Cargo.toml"
    ],
    "purpose": "Compile-check the touched runtime crate after sealing raw authority store APIs.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/cargo-check-adl-runtime.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_identity_lease_authority",
      "--",
      "--nocapture",
      "--test-threads=1"
    ],
    "purpose": "Exercise the focused #258 authority-store boundary guardrails.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/cargo-test-distributed-identity-lease-authority.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_identity_lease_authority",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Strict-lint the focused #258 authority-store boundary test target.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/cargo-clippy-distributed-identity-lease-authority.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_projection",
      "--no-run"
    ],
    "purpose": "Compile-only regression guard for raw-store token fixture fallout in distributed_projection.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/cargo-test-distributed-projection-no-run.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_resource_weather",
      "--no-run"
    ],
    "purpose": "Compile-only regression guard for raw-store token fixture fallout in distributed_resource_weather.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/cargo-test-distributed-resource-weather-no-run.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_discovery",
      "--no-run"
    ],
    "purpose": "Compile-only regression guard for raw-store token fixture fallout in distributed_discovery.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/cargo-test-distributed-discovery-no-run.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_placement",
      "--no-run"
    ],
    "purpose": "Compile-only regression guard for raw-store token fixture fallout in distributed_placement.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/258/cargo-test-distributed-placement-no-run.log"
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
