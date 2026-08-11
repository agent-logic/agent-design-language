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
    "proof_role": "Prove real adl-runtime-kernel binary dispatch, typed completion/rejection, bounded queue pressure, and public contract parity.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 5000,
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
    "proof_role": "Prove principal-and-domain replay isolation, bounded progression, reconnect, maximum value, duplicate, eviction, and unrelated-traffic recovery.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 3500,
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
    "lane": "production-acip-native",
    "proof_role": "Retain exact-head Linux/macOS receipts for production dispatch, replay isolation, pressure/errors, path hygiene, and semantic equivalence, then require fresh review and terminal green delivery.",
    "acceptance_ids": [
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/209/validate-native-receipts.rb",
      ".csdlc/evidence/209/native-platform/linux.json",
      ".csdlc/evidence/209/native-platform/macos.json"
    ],
    "parallel_group": "209-native",
    "defer_reason": "Runs after publication on native GitHub Actions Linux and macOS; merge remains blocked until retained proof and fresh post-native review pass."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo nextest run --manifest-path adl-runtime-kernel/Cargo.toml --test production_acip_wss --no-tests=fail`
- `cargo nextest run --manifest-path adl-runtime-kernel/Cargo.toml --test production_acip_wss --no-tests=fail`
- `ruby .csdlc/prepared/issues/209/validate-native-receipts.rb .csdlc/evidence/209/native-platform/linux.json .csdlc/evidence/209/native-platform/macos.json`

## Failure Semantics

Fail closed on echo-only substitution, missing production dispatch, pressure without typed error, replay-domain ambiguity, max-value poisoning, cross-principal interference, schema/runtime mismatch, stale evidence, or missing exact-head review.

## Handoff

Retain typed evidence before convergence.
