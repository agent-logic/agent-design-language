# Validation Planning Prompt

Template: 1.0.0

Issue: 191

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/191/design.md

Diagram: .csdlc/prepared/issues/191/diagram.mmd

## Selected Lanes

[
  {
    "lane": "focused-secure-raft-runtime",
    "proof_role": "Run the registered production three-voter encrypted transport, configured-root authority lineage, exact outstanding-request retry, durable dispatch cache, rollback, fault, and restart denominator serially.",
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
    "budget_seconds": 900,
    "budget_tokens": 15000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_runtime_transport",
      "--no-tests=fail",
      "--test-threads=1"
    ],
    "parallel_group": "serial-runtime",
    "defer_reason": null
  },
  {
    "lane": "full-workspace-compatibility-compile",
    "proof_role": "Compile every adl-runtime workspace target so production module registration and the transport/discovery compatibility harnesses cannot drift independently.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 10000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--workspace",
      "--no-run"
    ],
    "parallel_group": "serial-runtime",
    "defer_reason": null
  },
  {
    "lane": "strict-secure-raft-clippy",
    "proof_role": "Reject warnings and API misuse through the normal registered crate surface and focused integration target.",
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
      "distributed_runtime_transport",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "serial-runtime",
    "defer_reason": null
  },
  {
    "lane": "exact-proof-and-review",
    "proof_role": "Bind the registered source, compatibility harnesses, nonzero tests, configured-root topology, retry/dispatch ordering, rollback, strict Clippy, workspace compile, secret/path hygiene, and independent exact-head review.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 5000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/191/validate-proof-receipt.rb"
    ],
    "parallel_group": "serial-proof",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo nextest run --locked --manifest-path adl-runtime/Cargo.toml --test distributed_runtime_transport --no-tests=fail --test-threads=1`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --workspace --no-run`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_runtime_transport -- -D warnings`
- `ruby .csdlc/prepared/issues/191/validate-proof-receipt.rb`

## Failure Semantics

Fail closed on plaintext or unauthenticated transport, identity/domain drift, replay, oversized input, partial durable mutation, rollback/corruption, symlink paths, secret evidence, missing nonzero proof, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
