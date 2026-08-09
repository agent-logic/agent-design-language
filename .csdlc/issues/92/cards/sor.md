# Structured Output Record

Template: 1.0.0

Issue: 92

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Removed Runtime-owned local PKI and unified Runtime transport configuration around one Rustls policy, Axum HTTP/WSS, and Quinn Guardian QUIC, with externally provisioned certificate ownership and focused fail-closed proof.

## Artifacts

- adl-runtime-kernel/src/tls.rs
- infra/runtime-v3/runtime-init.toml
- docs/api/runtime-v3/v1/openapi.json
- docs/api/runtime-v3/v1/acip.openapi.json
- adl-runtime/tests/support/tls-fixtures/
- .csdlc/prepared/issues/92/design.md

## Execution

- Deleted production local self-signed issuance, bootstrap CLI, reissue, and trust-store mutation paths.
- Routed Runtime HTTP/WSS through shared Axum/Rustls configuration with pre-bind WebPKI chain, validity, usage, and DNS verification.
- Reused the shared Rustls policy for Quinn Guardian mTLS and protocol-adapter mTLS without changing Guardian authorization.
- Aligned Runtime init, OpenAPI, proof scripts, HTML Observatory, Unity, and architecture documentation with externally provisioned certificate ownership.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "control"
    ],
    "purpose": "Axum HTTPS control, shared startup validation, and bounded shutdown",
    "outcome": "passed",
    "evidence_ref": "23 passed at final implementation head"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "observatory"
    ],
    "purpose": "Observatory WSS authentication and revocation",
    "outcome": "passed",
    "evidence_ref": "6 passed at final implementation head"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "protocol_adapters"
    ],
    "purpose": "Protocol-adapter mutual TLS and authority rejection",
    "outcome": "passed",
    "evidence_ref": "13 passed at final implementation head"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_transport"
    ],
    "purpose": "Guardian Quinn mutual TLS and identity binding",
    "outcome": "passed",
    "evidence_ref": "14 passed at final implementation head"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "runtime_api_wss"
    ],
    "purpose": "Runtime WSS, ACIP contract, and certificate rejection matrix",
    "outcome": "passed",
    "evidence_ref": "5 passed at final implementation head"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "tls::tests::server_identity_validation_rejects_time_and_usage_failures"
    ],
    "purpose": "Expired, not-yet-valid, and unsuitable server-auth certificate rejection",
    "outcome": "passed",
    "evidence_ref": "1 passed at final implementation head 9acff0a70"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--bin",
      "adl-runtime-lifecycle-soak",
      "init_fixture_uses_externally_provisioned_tls"
    ],
    "purpose": "External certificate lifecycle fixture",
    "outcome": "passed",
    "evidence_ref": "1 passed at final implementation head"
  },
  {
    "command": [
      "cargo",
      "check",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--all-targets"
    ],
    "purpose": "All-target compile and contract hygiene",
    "outcome": "passed",
    "evidence_ref": "runtime and kernel all-target checks plus JSON, JavaScript, shell, and diff hygiene passed"
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
