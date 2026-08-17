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
    "lane": "parent-recovery-cleanup-bridge",
    "proof_role": "Prove production recovery validates a completed recovery attempt, emits cleanup-consumable completed recovery receipt and canonical archive manifest authority, and feeds cleanup without test-authored authority.",
    "acceptance_ids": [
      "AC-2",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 7000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "recovery_bridge_emits_cleanup_authority_consumed_by_cleanup",
      "--",
      "--nocapture"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "recovery-regression-matrix",
    "proof_role": "Retain full preserved projection recovery regression coverage after adding the bridge API.",
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
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 7000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "--",
      "--nocapture"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "cleanup-authority-regression",
    "proof_role": "Retain cleanup authority regressions after allowing directory link-count drift only for the same directory identity during child removal.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 7000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_cleanup",
      "--",
      "--nocapture"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "child-300-integrated-matrix-dependency",
    "proof_role": "#300 must separately prove the integrated recovery/cleanup matrix using its own target after #297 is terminal and ancestral; this parent card records the dependency and does not claim #300 proof.",
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
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 7000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "recovery_bridge_emits_cleanup_authority_consumed_by_cleanup",
      "--",
      "--nocapture"
    ],
    "parallel_group": "local",
    "defer_reason": "Dependency record only: #300 remains review-failed/unpublished until it consumes the terminal #297 bridge and mechanically invokes or explicitly enumerates the matrix."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate5 recovery_bridge_emits_cleanup_authority_consumed_by_cleanup -- --nocapture`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate5 -- --nocapture`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate_cleanup -- --nocapture`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate5 recovery_bridge_emits_cleanup_authority_consumed_by_cleanup -- --nocapture`

## Failure Semantics

Fail closed without mutation on unsafe node type, alias, identity drift, corrupt or mismatched projection, ambiguous lineage, stale CAS/topology, incomplete receipt chain, collision/race, validation failure, or stale review; preserve every failure artifact.

## Handoff

Retain typed evidence before convergence.
