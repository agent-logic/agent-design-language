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
- Replaced duplicate hand-built Guardian proof launch logic with the existing production lifecycle runner.

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
    "evidence_ref": "23 passed at substantive head c57892d0a"
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
    "evidence_ref": "6 passed before review remediation; the Observatory TLS surface was unchanged by remediation"
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
    "evidence_ref": "13 passed at substantive head c57892d0a"
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
    "evidence_ref": "14 passed at substantive head c57892d0a"
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
    "evidence_ref": "5 passed at substantive head c57892d0a"
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
    "evidence_ref": "1 passed at substantive head c57892d0a with a fixed 2030 verification time"
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
    "evidence_ref": "1 passed at substantive head c57892d0a"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "openapi_contract"
    ],
    "purpose": "Production Axum route and binary ACIP OpenAPI parity",
    "outcome": "passed",
    "evidence_ref": "6 passed after review remediation"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Focused diff hygiene",
    "outcome": "passed",
    "evidence_ref": "git diff --check passed after review remediation"
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
