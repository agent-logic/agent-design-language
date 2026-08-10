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
    "proof_role": "Run the exact three-voter encrypted transport, authority-derived topology, durable retry cache, external rollback checkpoint, fault and restart denominator through a bounded temporary path harness.",
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
    "budget_tokens": 20000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_runtime_transport",
      "--no-tests=fail"
    ],
    "parallel_group": "serial-runtime",
    "defer_reason": "Deferred only until this issue creates owned target adl-runtime/tests/distributed_runtime_transport.rs, which must compile the unregistered source through #[path = \"../src/distributed/polis_runtime.rs\"] for adl-runtime/src/distributed/polis_runtime.rs; fail closed until both exact owned deliverables exist and the target selects nonzero tests."
  },
  {
    "lane": "strict-secure-raft-clippy",
    "proof_role": "Reject warnings and API misuse in the same exact temporary-path implementation and test surface.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 15000,
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
    "defer_reason": "Deferred only until the same owned target and #[path = \"../src/distributed/polis_runtime.rs\"] source harness exist; fail closed on a missing target, missing source, warnings, or zero proving tests."
  },
  {
    "lane": "exact-proof-and-review",
    "proof_role": "Bind exact source, transport extension, nonzero tests, authority topology, retry, rollback, artifact, negative-case, strict Clippy, secret/path hygiene and independent review.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 10000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/191/validate-proof-receipt.rb"
    ],
    "parallel_group": "serial-proof",
    "defer_reason": "Deferred only until this issue creates owned validator .csdlc/prepared/issues/191/validate-proof-receipt.rb; fail closed until it exists and validates exact current source and nonzero proof."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo nextest run --locked --manifest-path adl-runtime/Cargo.toml --test distributed_runtime_transport --no-tests=fail`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_runtime_transport -- -D warnings`
- `ruby .csdlc/prepared/issues/191/validate-proof-receipt.rb`

## Failure Semantics

Fail closed on plaintext or unauthenticated transport, identity/domain drift, replay, oversized input, partial durable mutation, rollback/corruption, symlink paths, secret evidence, missing nonzero proof, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
