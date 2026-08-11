# Validation Planning Prompt

Template: 1.0.0

Issue: 112

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/112/design.md

Diagram: .csdlc/prepared/issues/112/diagram.mmd

## Selected Lanes

[
  {
    "lane": "layer8-authority-contract",
    "proof_role": "The exact issue-owned adl-runtime integration-test target proves principal derivation, action-specific capability and policy intersection, fail-closed refusal, replay defense, restart integrity, redacted tamper-evident audit, and nonzero test selection.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 20000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "layer8_authority",
      "--no-tests=fail",
      "--status-level",
      "all"
    ],
    "parallel_group": "112-product-serial-01",
    "defer_reason": "Deferred until #83 and #111 are merged and ancestral. The issue-owned temporary #[path = \"../src/layer8_authority.rs\"] harness in adl-runtime/tests/layer8_authority.rs routes adl-runtime/src/layer8_authority.rs until module registration and fails closed until the exact nonzero target exists."
  },
  {
    "lane": "layer8-runtime-api-integration",
    "proof_role": "The exact issue-owned adl integration-test target proves authorization and durable audit occur before sequence reservation, provider execution, and delivery while preserving bounded role projections and nonzero test selection.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 20000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "--test",
      "layer8_authority_runtime_api",
      "--no-tests=fail",
      "--status-level",
      "all"
    ],
    "parallel_group": "112-product-serial-02",
    "defer_reason": "Deferred until #83 and #111 are merged and ancestral and issue execution creates adl/tests/layer8_authority_runtime_api.rs; fail closed until the exact nonzero integration target exists."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo nextest run --locked --manifest-path adl-runtime/Cargo.toml --test layer8_authority --no-tests=fail --status-level all`
- `cargo nextest run --locked --manifest-path adl/Cargo.toml --test layer8_authority_runtime_api --no-tests=fail --status-level all`

## Failure Semantics

Fail closed on serial-gate or ownership drift, unauthenticated or stale identity, capability mismatch, recipient substitution or widening, cross-Polis action, replay, revocation, expiry, policy uncertainty, audit discontinuity or write failure, forbidden-field leakage, zero-test selection, preparation-as-product-proof substitution, or unresolved exact-head findings.

## Handoff

Retain typed evidence before convergence.
