# Validation Planning Prompt

Template: 1.0.0

Issue: 298

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/298/design.md

Diagram: .csdlc/prepared/issues/298/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preserved-projection-recovery",
    "proof_role": "Production classify/recover; every recovery-only failpoint including operation-owned temporary-node create/identity, exact-prefix write continuation, completed-write adoption, repeated node/parent fsync, and no-replace node publish; identity/topology/CAS/collision/idempotency; subsequent ordinary commit; initialized/ready/#291 regression; and exact-head support hygiene. AC-8 completion still requires the separate #119 exact-head review lifecycle gate.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 900,
    "budget_tokens": 7000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "preserved_projection_recovery"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate5 preserved_projection_recovery`

## Failure Semantics

Fail closed without deletion on unsafe node type, alias, identity/mount/owner/mode drift, corrupt or mismatched projection, ambiguous lineage, stale tagged CAS/topology, incomplete receipt chain, collision/replacement, validation failure, or stale review; preserve every candidate and attempt.

## Handoff

Retain typed evidence before convergence.
