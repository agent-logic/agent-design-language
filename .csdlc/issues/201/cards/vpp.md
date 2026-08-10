# Validation Planning Prompt

Template: 1.0.0

Issue: 201

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/201/design.md

Diagram: .csdlc/prepared/issues/201/diagram.mmd

## Selected Lanes

[
  {
    "lane": "committed-authority-protocol",
    "proof_role": "Prove real three-voter committed intent/finalize, opaque endorsements, deterministic time, durable retry/checkpoint recovery, legacy closure, and operation-token boundary.",
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
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 20000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_authority_protocol",
      "--no-tests=fail"
    ],
    "parallel_group": "201-runtime",
    "defer_reason": "Deferred until PR #197 is merged and this issue creates adl-runtime/tests/distributed_authority_protocol.rs and adl-runtime/src/distributed/authority_protocol.rs; fail closed on a missing target, missing source, or zero tests."
  },
  {
    "lane": "committed-authority-protocol-clippy",
    "proof_role": "Reject warnings and API misuse across the same bounded core protocol surface.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 10000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_authority_protocol",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "201-runtime",
    "defer_reason": "Deferred until the owned focused target exists; fail closed on warnings, missing target, or missing source."
  },
  {
    "lane": "committed-authority-protocol-producer",
    "proof_role": "Produce machine-derived execution and negative-case artifacts only from the exact clean protected source and named nonzero denominator.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 10000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/201/produce-proof-receipt.rb"
    ],
    "parallel_group": "201-proof",
    "defer_reason": "Deferred until this issue creates exact owned producer .csdlc/prepared/issues/201/produce-proof-receipt.rb; fail closed while absent and on dirty protected source, zero tests, incomplete case denominator, or nonzero command status."
  },
  {
    "lane": "committed-authority-protocol-receipt",
    "proof_role": "Bind exact protected source, commands, nonzero case denominator, strict Clippy, machine cases, immutable evidence introduction, review, and squash-merge-safe validation.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 5000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/201/validate-proof-receipt.rb"
    ],
    "parallel_group": "201-proof",
    "defer_reason": "Deferred until this issue creates .csdlc/prepared/issues/201/validate-proof-receipt.rb and post-finalize immutable evidence; fail closed until both exist and bind exact reviewed source."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo nextest run --locked --manifest-path adl-runtime/Cargo.toml --test distributed_authority_protocol --no-tests=fail`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_authority_protocol -- -D warnings`
- `ruby .csdlc/prepared/issues/201/produce-proof-receipt.rb`
- `ruby .csdlc/prepared/issues/201/validate-proof-receipt.rb`

## Failure Semantics

Fail closed on missing or invalid quorum endorsements, signer unavailability, stale membership, domain/index/time mismatch, incomplete protocol checkpoint, rollback, retry conflict, legacy direct authority, corruption, unsafe paths, zero-test proof, source drift, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
