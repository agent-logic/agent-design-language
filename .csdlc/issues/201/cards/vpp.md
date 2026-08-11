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
    "lane": "committed-authority-contract-47",
    "proof_role": "Prove the exact ordered 47-case authority protocol contract with no production-selectable bypass.",
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
    "budget_tokens": 15000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "-E",
      "test(/^distributed::authority_protocol::contract_tests::/)",
      "--no-tests=fail"
    ],
    "parallel_group": "201-runtime",
    "defer_reason": null
  },
  {
    "lane": "production-three-voter-openraft",
    "proof_role": "Exercise production PrepareAuthority, FinalizeAuthority, actual apply IDs, trusted route custody, pending publication, and snapshot install/restart with exact custody/finalization verification plus injection and tamper denial through the production state machine.",
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
    "budget_tokens": 10000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "distributed::polis_runtime::authority_consensus_tests::real_three_voter_authority_prepare_finalize_uses_applied_log_ids",
      "--",
      "--exact",
      "--nocapture"
    ],
    "parallel_group": "201-runtime",
    "defer_reason": null
  },
  {
    "lane": "committed-authority-production-clippy",
    "proof_role": "Reject warnings and API misuse across the production library without cfg(test)-only authority helpers.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 8000,
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
    "parallel_group": "201-runtime",
    "defer_reason": null
  },
  {
    "lane": "committed-authority-proof-producer",
    "proof_role": "Produce one v6 packet binding protected source, strict Clippy, exact 47/47, real three-voter OpenRaft, and snapshot trust-boundary results.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 10000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/201/produce-proof-receipt.rb"
    ],
    "parallel_group": "201-proof",
    "defer_reason": null
  },
  {
    "lane": "committed-authority-proof-validator",
    "proof_role": "Require ancestry whenever the source object exists; allow exact protected-tree fallback only when it is genuinely unavailable, proving available-divergent rejection and depth-one pass.",
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
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo nextest run --locked --manifest-path adl-runtime/Cargo.toml --lib -E test(/^distributed::authority_protocol::contract_tests::/) --no-tests=fail`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --lib distributed::polis_runtime::authority_consensus_tests::real_three_voter_authority_prepare_finalize_uses_applied_log_ids -- --exact --nocapture`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --lib -- -D warnings`
- `ruby .csdlc/prepared/issues/201/produce-proof-receipt.rb`
- `ruby .csdlc/prepared/issues/201/validate-proof-receipt.rb`

## Failure Semantics

Fail closed on missing or invalid quorum endorsements, signer unavailability, stale membership, domain/index/time mismatch, incomplete protocol checkpoint, rollback, retry conflict, legacy direct authority, corruption, unsafe paths, zero-test proof, source drift, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
