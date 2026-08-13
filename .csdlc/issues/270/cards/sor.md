# Structured Output Record

Template: 1.0.0

Issue: 270

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented trusted Layer 8 recipient-acknowledgement Runtime API protocol.

## Artifacts

- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/tests/openapi_contract.rs
- docs/api/runtime-v3/v1/openapi.json
- .csdlc/prepared/issues/270/validate_preparation_bundle.py
- .csdlc/prepared/issues/270/readiness-packet.md
- .csdlc/evidence/270

## Execution

- Added the served Runtime Core recipient-acknowledgement route that verifies sender-signed requests and recipient-signed acknowledgements before returning delivery status.
- Bound acknowledgement responses to signed credential generations and redacted correlation by returning only a BLAKE3 correlation hash.
- Added focused regressions for verified delivery, tampered credential-generation refusal before side effects, and served-route response redaction.
- Updated Runtime Core OpenAPI contract and route-inventory proof for the new acknowledgement endpoint.

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Run git diff hygiene check.",
    "outcome": "passed",
    "evidence_ref": "diff-hygiene.log"
  },
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/270/validate_preparation_bundle.py"
    ],
    "purpose": "Run the issue-owned preparation validator after #112 and #265 terminal caches are present.",
    "outcome": "passed",
    "evidence_ref": "issue-270-preparation-validator.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "recipient_ack",
      "--",
      "--nocapture"
    ],
    "purpose": "Run the focused #270 Runtime recipient acknowledgement regressions.",
    "outcome": "passed",
    "evidence_ref": "runtime-ack-api-focused.log"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--check"
    ],
    "purpose": "Run rustfmt check for adl-runtime-kernel.",
    "outcome": "passed",
    "evidence_ref": "runtime-kernel-fmt.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict Clippy for adl-runtime-kernel.",
    "outcome": "passed",
    "evidence_ref": "runtime-kernel-strict-clippy.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "openapi_contract",
      "--",
      "--nocapture"
    ],
    "purpose": "Run the Runtime Core OpenAPI route-inventory contract tests.",
    "outcome": "passed",
    "evidence_ref": "runtime-openapi-contract.log"
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
