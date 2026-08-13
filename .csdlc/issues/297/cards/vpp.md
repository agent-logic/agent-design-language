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
    "proof_role": "Prove the parent integration bridge with targets available in this #297 worktree: production recovery authority must connect to cleanup-consumable completed recovery receipt and canonical archive manifest authority, with terminal/canonical/archive binding and no test-authored authority.",
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
      "preserved_projection_recovery"
    ],
    "parallel_group": "local",
    "defer_reason": "Deferred until #297 bridge implementation or an explicitly split child lands; current #300 Noether r1 found no production bridge for cleanup to consume."
  },
  {
    "lane": "cleanup-authority-regression",
    "proof_role": "Retain cleanup authority regressions available in this #297 worktree while making clear they do not substitute for #300's later integrated matrix unless #300 mechanically invokes or enumerates them.",
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
      "gate_cleanup"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "child-300-integrated-matrix-dependency",
    "proof_role": "#300 must separately prove the integrated recovery/cleanup matrix using its own target after the bridge lands; this parent card records the dependency and does not claim local proof from a missing #300 test file.",
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
      "preserved_projection_recovery"
    ],
    "parallel_group": "local",
    "defer_reason": "Dependency record only: #300 remains review-failed/unpublished until it proves the bridge-fed integration target and either mechanically invokes or explicitly enumerates the matrix."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate5 preserved_projection_recovery`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate_cleanup`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate5 preserved_projection_recovery`

## Failure Semantics

Fail closed without mutation on unsafe node type, alias, identity drift, corrupt or mismatched projection, ambiguous lineage, stale CAS/topology, incomplete receipt chain, collision/race, validation failure, or stale review; preserve every failure artifact.

## Handoff

Retain typed evidence before convergence.
