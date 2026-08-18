# Structured Output Record

Template: 1.0.0

Issue: 270

Repository: agent-logic/agent-design-language

Card: sor

Status: complete

## Summary

Implemented trusted Layer 8 recipient-acknowledgement Runtime API protocol, then remediated the P1 exact-review finding by validating recipient acknowledgement payload semantics before reporting delivery.

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
- Validated the recipient acknowledgement payload itself after signature verification: accepted delivery is required, refused delivery remains refused, and unrelated or recipient-mismatched payloads fail before delivered state.
- Added focused regressions for verified delivery, tampered credential-generation refusal before side effects, served-route response redaction, recipient-signed delivery refusal, and unrelated signed acknowledgement payload refusal.
- Updated Runtime Core OpenAPI contract and route-inventory proof for the new acknowledgement endpoint.

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Run git diff hygiene check after #270 P1 acknowledgement-payload remediation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/270/diff-hygiene.log"
  },
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/270/validate_preparation_bundle.py"
    ],
    "purpose": "Run the issue-owned preparation validator after #112 and #265 terminal caches are present and the validator accepts bound/implemented phases.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/270/issue-270-preparation-validator.log"
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
    "purpose": "Run the focused #270 Runtime recipient acknowledgement regressions including the P1 fix cases for recipient-signed refusal and unrelated signed acknowledgement payload refusal.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/270/runtime-ack-api-focused.log"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--check"
    ],
    "purpose": "Run rustfmt check for adl-runtime-kernel after #270 P1 remediation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/270/runtime-kernel-fmt.log"
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
    "purpose": "Run strict Clippy for adl-runtime-kernel after #270 P1 remediation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/270/runtime-kernel-strict-clippy.log"
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
    "purpose": "Run the Runtime Core OpenAPI route-inventory contract tests after #270 API route integration.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/270/runtime-openapi-contract.log"
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
