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
    "lane": "layer8-production-conversation-boundary",
    "proof_role": "Prove authenticated WSS authority executes before session or turn reservation and provider dispatch, including bounded refusal and duplicate idempotency on the merged #111 production path.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 12000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "conversation_sessions",
      "--no-fail-fast"
    ],
    "parallel_group": "112-product-required",
    "defer_reason": null
  },
  {
    "lane": "layer8-authority-contract",
    "proof_role": "Prove principal derivation, action-specific authority, signed human-agent and agent-agent messages, recipient-signed acknowledgement binding, signature and rotation negatives, replay defense, restart integrity, and redacted audit.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 16000,
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
    "parallel_group": "112-product-required",
    "defer_reason": null
  },
  {
    "lane": "layer8-runtime-api-integration",
    "proof_role": "Prove the narrow CSM API adapter invokes delivery only after the same Runtime authority grant and never invokes it after refusal.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 16000,
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
    "parallel_group": "112-product-required",
    "defer_reason": null
  },
  {
    "lane": "layer8-observatory-ui",
    "proof_role": "Run the actual HTML Observatory in a real local browser and prove authorized, refused, stale or revoked, and disclosure-safe authority presentation without provider, cloud, or soak work.",
    "acceptance_ids": [
      "AC-3",
      "AC-5",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      "adl/tools/validate_layer8_authority_observatory_ui.sh"
    ],
    "parallel_group": "112-product-required",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test conversation_sessions --no-fail-fast`
- `cargo nextest run --locked --manifest-path adl-runtime/Cargo.toml --test layer8_authority --no-tests=fail --status-level all`
- `cargo nextest run --locked --manifest-path adl/Cargo.toml --test layer8_authority_runtime_api --no-tests=fail --status-level all`
- `bash adl/tools/validate_layer8_authority_observatory_ui.sh`

## Failure Semantics

Fail closed on serial-gate or ownership drift, unauthenticated or stale identity, capability mismatch, recipient substitution or widening, cross-Polis action, replay, revocation, expiry, policy uncertainty, audit discontinuity or write failure, forbidden-field leakage, zero-test selection, preparation-as-product-proof substitution, or unresolved exact-head findings.

## Handoff

Retain typed evidence before convergence.
