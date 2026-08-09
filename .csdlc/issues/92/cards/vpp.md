# Validation Planning Prompt

Template: 1.0.0

Issue: 92

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/92/design.md

Diagram: .csdlc/prepared/issues/92/diagram.mmd

## Selected Lanes

[
  {
    "lane": "axum-runtime-tls",
    "proof_role": "Prove shared Axum server TLS startup, DNS and chain validation, HTTPS control behavior, and bounded shutdown.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "control"
    ],
    "parallel_group": "runtime-tls",
    "defer_reason": null
  },
  {
    "lane": "webpki-certificate-policy",
    "proof_role": "Prove deterministic WebPKI acceptance of a directly root-signed server leaf and rejection of expired, not-yet-valid, and clientAuth-only identities.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "tls::tests::server_identity_validation_rejects_time_and_usage_failures"
    ],
    "parallel_group": "runtime-tls",
    "defer_reason": null
  },
  {
    "lane": "observatory-wss",
    "proof_role": "Prove Axum WSS behavior, ordinary server TLS, authentication, rotation, revocation, and ACIP contract parity.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "runtime_api_wss"
    ],
    "parallel_group": "runtime-tls",
    "defer_reason": null
  },
  {
    "lane": "guardian-quinn-mtls",
    "proof_role": "Prove Quinn remains the Guardian transport and shares the Rustls identity and trust policy without weakening mTLS.",
    "acceptance_ids": [
      "AC-2",
      "AC-5",
      "AC-6",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 7000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_transport"
    ],
    "parallel_group": "guardian-tls",
    "defer_reason": null
  },
  {
    "lane": "protocol-mtls",
    "proof_role": "Prove provider and cloud protocol adapters require authenticated Rustls transport and reject unknown server and client authorities.",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "protocol_adapters"
    ],
    "parallel_group": "guardian-tls",
    "defer_reason": null
  },
  {
    "lane": "external-certificate-fixture",
    "proof_role": "Prove lifecycle setup consumes externally provisioned certificate, key, and trust roots without Runtime issuance or host trust mutation.",
    "acceptance_ids": [
      "AC-3",
      "AC-7",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--bin",
      "adl-runtime-lifecycle-soak",
      "init_fixture_uses_externally_provisioned_tls"
    ],
    "parallel_group": "contracts",
    "defer_reason": null
  },
  {
    "lane": "runtime-openapi-contract",
    "proof_role": "Prove the production Axum route inventory and binary ACIP WebSocket contract match the kernel implementation.",
    "acceptance_ids": [
      "AC-1",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "openapi_contract"
    ],
    "parallel_group": "contracts",
    "defer_reason": null
  },
  {
    "lane": "tls-contract-hygiene",
    "proof_role": "Prove the focused diff has no whitespace or conflict-marker defects.",
    "acceptance_ids": [
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "contracts",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test control`
- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --lib tls::tests::server_identity_validation_rejects_time_and_usage_failures`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test runtime_api_wss`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_transport`
- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test protocol_adapters`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --bin adl-runtime-lifecycle-soak init_fixture_uses_externally_provisioned_tls`
- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test openapi_contract`
- `git diff --check`

## Failure Semantics

Fail closed on any production self-signed issuance, host trust mutation, verification bypass, duplicated Axum TLS path, unverified mTLS identity claim, stale contract, or regression in Quinn peer authentication.

## Handoff

Retain typed evidence before convergence.
