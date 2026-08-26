# Validation Planning Prompt

Template: 1.0.0

Issue: 540

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/540/design.md

Diagram: .csdlc/prepared/issues/540/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-config-origin",
    "proof_role": "Prove Runtime configuration accepts explicit additional_allowed_origins = [\"http://localhost:8000\"] and preserves the canonical HTTPS Observatory origin.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1200,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "configuration"
    ],
    "parallel_group": "runtime-kernel",
    "defer_reason": null
  },
  {
    "lane": "runtime-control-cors",
    "proof_role": "Prove configured allow and default deny for Origin: http://localhost:8000 on Runtime v3 browser CORS routes; test servers bind ephemeral ports only, not 8000.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1200,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "control",
      "observatory_cors",
      "--",
      "--nocapture"
    ],
    "parallel_group": "runtime-kernel",
    "defer_reason": null
  },
  {
    "lane": "runtime-check",
    "proof_role": "Prove all Runtime kernel targets compile after the additive config field and CORS test changes.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 200,
    "budget_tokens": 1200,
    "argv": [
      "cargo",
      "check",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets"
    ],
    "parallel_group": "runtime-kernel",
    "defer_reason": null
  },
  {
    "lane": "runtime-fmt",
    "proof_role": "Prove formatting for touched Runtime kernel files.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 800,
    "argv": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--",
      "--check"
    ],
    "parallel_group": "format",
    "defer_reason": null
  },
  {
    "lane": "runtime-clippy",
    "proof_role": "Prove strict lint cleanliness for the Runtime kernel after the additive config field and focused CORS tests.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 200,
    "budget_tokens": 1200,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "runtime-kernel",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test configuration`
- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test control observatory_cors -- --nocapture`
- `cargo check --locked --manifest-path adl-runtime-kernel/Cargo.toml --all-targets`
- `cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml -- --check`
- `cargo clippy --locked --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings`

## Failure Semantics

Fail closed on any CORS default-open behavior, port-8000 bind/listener introduction, public API/authentication drift, or broad production-ingress scope.

## Handoff

Retain typed evidence before convergence.
