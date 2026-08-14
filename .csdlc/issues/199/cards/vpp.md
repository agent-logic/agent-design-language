# Validation Planning Prompt

Template: 1.0.0

Issue: 199

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/199/design.md

Diagram: .csdlc/prepared/issues/199/diagram.mmd

## Selected Lanes

[
  {
    "lane": "governed-membership-transition",
    "proof_role": "Prove exactly twelve behavior-specific public membership contracts without name substitution; production lanes separately bind governed receipts, authorized target maps, current-operation membership history, crash recovery, and strict Clippy.",
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
    "budget_tokens": 12000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_membership_transition",
      "--",
      "--nocapture",
      "--test-threads=1"
    ],
    "parallel_group": "199-runtime",
    "defer_reason": null
  },
  {
    "lane": "governed-membership-saga-unit",
    "proof_role": "Prove seven coordinator unit cases including exact authorized stable-map/target binding, current-operation log-index history, durable phase recovery, exact retry, conflicting operation and receipt denial, and single publication.",
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
    "budget_seconds": 600,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "distributed::membership_coordinator::tests",
      "--",
      "--nocapture",
      "--test-threads=1"
    ],
    "parallel_group": "199-runtime",
    "defer_reason": null
  },
  {
    "lane": "governed-membership-admission-receipt",
    "proof_role": "Prove the real fourth-node admission receipt, exact-current observation, and mismatch denial.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "distributed::transport::governed::learner_transport::tests::real_four_node_learner_replication",
      "--",
      "--exact",
      "--nocapture",
      "--test-threads=1"
    ],
    "parallel_group": "199-runtime",
    "defer_reason": null
  },
  {
    "lane": "governed-membership-exclusion-receipt",
    "proof_role": "Prove exact pending-exclusion receipt observation and mismatch denial.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "distributed::transport::governed::learner_transport::tests::excluded_node_recovery_learner",
      "--",
      "--exact",
      "--nocapture",
      "--test-threads=1"
    ],
    "parallel_group": "199-runtime",
    "defer_reason": null
  },
  {
    "lane": "governed-membership-history",
    "proof_role": "Prove same-batch joint and uniform OpenRaft membership history survives restart.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "distributed::transport::governed::polis_runtime::authority_consensus_tests::membership_history_retains_joint_and_uniform_entries_from_one_apply_batch",
      "--",
      "--exact",
      "--nocapture",
      "--test-threads=1"
    ],
    "parallel_group": "199-runtime",
    "defer_reason": null
  },
  {
    "lane": "governed-membership-lib-clippy",
    "proof_role": "Reject warnings and API misuse across the production library.",
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
      "--lib",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "199-runtime",
    "defer_reason": null
  },
  {
    "lane": "governed-membership-target-clippy",
    "proof_role": "Reject warnings and API misuse across the exact public transition target.",
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
      "distributed_membership_transition",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "199-runtime",
    "defer_reason": null
  },
  {
    "lane": "governed-membership-proof-producer",
    "proof_role": "Produce retained .csdlc/evidence/199/v19/execution-proof.json with schema adl.issue199.governed_membership_transition_proof.v19, exact command streams, twelve behavior cases, nine production assertions, protected-source ancestry, and cleanliness.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1500,
    "budget_tokens": 8000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/199/produce-proof-receipt.rb"
    ],
    "parallel_group": "199-proof",
    "defer_reason": null
  },
  {
    "lane": "governed-membership-proof-validator",
    "proof_role": "Validate immutable v19 evidence, exact twelve/nine denominators, protected-source and evidence drift, and current-main ancestry.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/199/validate-proof-receipt.rb"
    ],
    "parallel_group": "199-proof",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_membership_transition -- --nocapture --test-threads=1`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --lib distributed::membership_coordinator::tests -- --nocapture --test-threads=1`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --lib distributed::transport::governed::learner_transport::tests::real_four_node_learner_replication -- --exact --nocapture --test-threads=1`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --lib distributed::transport::governed::learner_transport::tests::excluded_node_recovery_learner -- --exact --nocapture --test-threads=1`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --lib distributed::transport::governed::polis_runtime::authority_consensus_tests::membership_history_retains_joint_and_uniform_entries_from_one_apply_batch -- --exact --nocapture --test-threads=1`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --lib -- -D warnings`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_membership_transition -- -D warnings`
- `ruby .csdlc/prepared/issues/199/produce-proof-receipt.rb`
- `ruby .csdlc/prepared/issues/199/validate-proof-receipt.rb`

## Failure Semantics

Fail closed on missing or invalid #201 token, authority-cut drift, learner lag/divergence, joint/final mismatch, stale or removed voter authority, unstable Raft ids, incomplete parity, checkpoint ambiguity, rollback, retry conflict, corruption, unsafe paths, zero-test proof, source drift, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
