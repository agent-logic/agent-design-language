# Validation Planning Prompt

Template: 1.0.0

Issue: 5881

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5881/design.md

Diagram: .csdlc/prepared/issues/5881/diagram.mmd

## Selected Lanes

[
  {
    "lane": "canonical-schema",
    "proof_role": "Prove canonical requests, records, schemas, and CLI surfaces contain no claim lifecycle state.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 240,
    "budget_tokens": 1800,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "schema"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  },
  {
    "lane": "record-normalization-retirement",
    "proof_role": "Prove current claim-bearing records normalize once with topology and audit truth preserved, then prove claim-specific production structs and logic are absent.",
    "acceptance_ids": [
      "AC-3",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 420,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "topology_migration"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  },
  {
    "lane": "bind-concurrency-recovery",
    "proof_role": "Prove atomic and idempotent same-issue bind, different-issue concurrency, and interrupted-bind recovery without claims; extend gate2 with the required interrupted-transaction case.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-7",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 480,
    "budget_tokens": 3600,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2"
    ],
    "parallel_group": "lifecycle",
    "defer_reason": null
  },
  {
    "lane": "review-to-cleanup",
    "proof_role": "Prove exact-head review, qualified cross-repository publication, finish, and cleanup derive authority from bound topology without claim IDs.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4200,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "--test",
      "gate6",
      "--test",
      "gate_finish",
      "--test",
      "gate_cleanup"
    ],
    "parallel_group": "lifecycle",
    "defer_reason": null
  },
  {
    "lane": "operator-contract",
    "proof_role": "Prove current skills, policies, and runbooks use only claim-free binding while historical evidence remains untouched.",
    "acceptance_ids": [
      "AC-2",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1400,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate10a"
    ],
    "parallel_group": "contracts",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml schema`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test topology_migration`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate2`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate5 --test gate6 --test gate_finish --test gate_cleanup`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate10a`

## Failure Semantics

Fail closed on duplicate authoritative topology, active claim state, manual historical-record repair, weakened review/finish truth, compatibility routing, or historical evidence mutation.

## Handoff

Retain typed evidence before convergence.
