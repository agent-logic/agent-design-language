# Validation Planning Prompt

Template: 1.0.0

Issue: 209

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/209/design.md

Diagram: .csdlc/prepared/issues/209/diagram.mmd

## Selected Lanes

[
  {
    "lane": "production-acip-wss",
    "proof_role": "Prove real production binary dispatch, typed errors, WSS-observed saturation rollback and exact retry.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "production_acip_wss",
      "--no-tests=fail"
    ],
    "parallel_group": "209-core",
    "defer_reason": null
  },
  {
    "lane": "acip-replay-authority",
    "proof_role": "Prove collision-free typed domains, per-principal capacity, and pending/committed concurrent rollback semantics.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "control::acip_replay_tests"
    ],
    "parallel_group": "209-core",
    "defer_reason": null
  },
  {
    "lane": "acip-contract-parity",
    "proof_role": "Prove the canonical served OpenAPI bearer/binary dispatch contract and separately retained legacy signed-frame admission boundary.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "openapi_contract"
    ],
    "parallel_group": "209-contract",
    "defer_reason": null
  },
  {
    "lane": "legacy-signed-admission",
    "proof_role": "Prove the non-public retained admission path still rejects unsigned control and terminal replay poisoning.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "runtime_api_auth::tests::wss_admission_fails_before_dispatch_for_auth_origin_authority_and_replay",
      "--lib",
      "--",
      "--exact"
    ],
    "parallel_group": "209-contract",
    "defer_reason": null
  },
  {
    "lane": "production-acip-native",
    "proof_role": "Retain exact-head Linux/macOS receipts for production dispatch, replay isolation, WSS pressure/errors, path hygiene, platform-neutral semantic equivalence, and source provenance.",
    "acceptance_ids": [
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/209/validate-native-receipts.rb",
      ".csdlc/evidence/209/native-platform/linux.json",
      ".csdlc/evidence/209/native-platform/macos.json"
    ],
    "parallel_group": "209-native",
    "defer_reason": "Runs after reviewed publication on native GitHub Actions Linux and macOS; merge remains blocked until retained proof and fresh post-native review pass."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo nextest run --manifest-path adl-runtime-kernel/Cargo.toml --test production_acip_wss --no-tests=fail`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --lib control::acip_replay_tests`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test openapi_contract`
- `cargo test --manifest-path adl-runtime/Cargo.toml runtime_api_auth::tests::wss_admission_fails_before_dispatch_for_auth_origin_authority_and_replay --lib -- --exact`
- `ruby .csdlc/prepared/issues/209/validate-native-receipts.rb .csdlc/evidence/209/native-platform/linux.json .csdlc/evidence/209/native-platform/macos.json`

## Failure Semantics

Fail closed on echo-only substitution, missing production dispatch, pressure without typed error, replay-domain ambiguity, max-value poisoning, cross-principal interference, schema/runtime mismatch, stale evidence, or missing exact-head review.

## Handoff

Retain typed evidence before convergence.
