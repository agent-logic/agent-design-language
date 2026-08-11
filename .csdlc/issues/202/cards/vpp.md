# Validation Planning Prompt

Template: 1.0.0

Issue: 202

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/202/design.md

Diagram: .csdlc/prepared/issues/202/diagram.mmd

## Selected Lanes

[
  {
    "lane": "authorized-learner-transport",
    "proof_role": "Prove the unchanged exact thirty-six semantic learner cases through forty-two passing private runner tests, including distinct learner-owned production authority, production recovery enforcement, exact removal deadline and target-membership binding, cache-first retry, boot custody, durable instance and peer pins, and real-effect races, with exactly thirty-one named subassertions.",
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
    "budget_seconds": 1500,
    "budget_tokens": 18000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "distributed::transport::governed::learner_transport::tests",
      "--",
      "--nocapture",
      "--test-threads=1"
    ],
    "parallel_group": "202-runtime",
    "defer_reason": "Fail closed unless all 42 runner tests pass, all 36 semantic cases remain represented, and the exact 31 named subassertions occur once."
  },
  {
    "lane": "authorized-learner-transport-public-boundary",
    "proof_role": "Prove the separate exact thirteen-test public canonical Membership artifact boundary without exposing governed construction or mutation authority.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_authorized_learner_transport",
      "--",
      "--test-threads=1"
    ],
    "parallel_group": "202-runtime",
    "defer_reason": "Fail closed on any count other than 13, any failure, or any public route around the sealed adapter."
  },
  {
    "lane": "distributed-transport-standalone-compile",
    "proof_role": "Compile the unchanged standalone distributed_transport target against the dependency-free transport core.",
    "acceptance_ids": [
      "AC-2",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_transport",
      "--no-run"
    ],
    "parallel_group": "202-compile",
    "defer_reason": "Fail closed if the historical standalone transport target no longer compiles."
  },
  {
    "lane": "distributed-discovery-standalone-compile",
    "proof_role": "Compile the unchanged standalone distributed_discovery target against the dependency-free transport core.",
    "acceptance_ids": [
      "AC-2",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_discovery",
      "--no-run"
    ],
    "parallel_group": "202-compile",
    "defer_reason": "Fail closed if the historical standalone discovery target no longer compiles."
  },
  {
    "lane": "authorized-learner-runtime-integration-compile",
    "proof_role": "Compile the full distributed runtime transport target against both voter-owned and learner-owned governed authority factories.",
    "acceptance_ids": [
      "AC-4",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_runtime_transport",
      "--no-run"
    ],
    "parallel_group": "202-compile",
    "defer_reason": "Fail closed if runtime integration bypasses factory ownership or does not compile."
  },
  {
    "lane": "authorized-learner-transport-clippy-lib",
    "proof_role": "Reject warnings and API misuse across the production library boundary.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 7000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "202-lint",
    "defer_reason": "Fail closed on any production library warning."
  },
  {
    "lane": "authorized-learner-transport-clippy-public",
    "proof_role": "Reject warnings and API misuse across the exact public learner artifact target.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_authorized_learner_transport",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "202-lint",
    "defer_reason": "Fail closed on any public-target warning."
  },
  {
    "lane": "authorized-learner-transport-producer",
    "proof_role": "Produce v9 exact-source evidence binding a fully clean worktree, exact current origin/main ancestry, all protected paths, 36 semantic versus 42 runner plus 13 public and 31 subassertions, three integration compiles, the exact route-rotation regression, the coverage-impact contract, and strict Clippy.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 2400,
    "budget_tokens": 10000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/202/produce-proof-receipt.rb"
    ],
    "parallel_group": "202-proof",
    "defer_reason": "Fail closed on any dirty or untracked path outside v9 output, stale origin/main, API-boundary drift, count mismatch, or failed command."
  },
  {
    "lane": "authorized-learner-transport-receipt",
    "proof_role": "Validate immutable v9 exact-source evidence, protected-path parity, and exact current origin/main ancestry for fresh independent review.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/202/validate-proof-receipt.rb"
    ],
    "parallel_group": "202-proof",
    "defer_reason": "Fail closed until exact source, denominators, commands, immutable evidence introduction, protected paths, and exact current origin/main agree."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --lib distributed::transport::governed::learner_transport::tests -- --nocapture --test-threads=1`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_authorized_learner_transport -- --test-threads=1`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_transport --no-run`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_discovery --no-run`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_runtime_transport --no-run`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --lib -- -D warnings`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_authorized_learner_transport -- -D warnings`
- `ruby .csdlc/prepared/issues/202/produce-proof-receipt.rb`
- `ruby .csdlc/prepared/issues/202/validate-proof-receipt.rb`

## Failure Semantics

Fail closed on missing/invalid token, voter-cut drift, learner role escalation, wrong identity/cert/boot/address, exclusion bypass, stale connection, replay conflict, checkpoint ambiguity, rollback, corruption, capacity, unsafe paths, zero-test proof, source drift, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
