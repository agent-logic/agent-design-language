# Validation Planning Prompt

Template: 1.0.0

Issue: 297

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/297/design.md

Diagram: .csdlc/prepared/issues/297/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preserved-projection-recovery",
    "proof_role": "Real typed classify/recover/cleanup and later ordinary commit proof; exact tagged-CAS permissions; anchored no-follow per-node identity with device and platform mount identity, uid/gid/mode/link policy; pre-mutation intent and exact post-state adoption; every recovery and cleanup receipt, rename/exchange, file fsync, parent fsync, regular-file unlink, empty-directory rmdir, partial-tree restart, collision/replacement, type-matched placeholder counterpart creation, original-placeholder removal, counterpart capture/removal, and both absent-after-removal restart-adoption windows; corruption/ambiguity/topology/CAS negatives; initialized/ready and #291 regression for AC-1 through AC-9",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
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

Fail closed without mutation on unsafe node type, alias, identity drift, corrupt or mismatched projection, ambiguous lineage, stale CAS/topology, incomplete receipt chain, collision/race, validation failure, or stale review; preserve every failure artifact.

## Handoff

Retain typed evidence before convergence.
