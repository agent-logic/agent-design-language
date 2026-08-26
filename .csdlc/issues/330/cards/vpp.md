# Validation Planning Prompt

Template: 1.0.0

Issue: 330

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/330/design.md

Diagram: .csdlc/prepared/issues/330/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-330-preparation-bundle",
    "proof_role": "Prove #330 issue-local cards and prepared validation bundle exist before binding.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/330/validate_preparation_bundle.py"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "issue-330-focused-regressions",
    "proof_role": "After bind, prove cleaned recovery validation and cleanup final-receipt race zero mutation fail closed under exact production authority.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 2500,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "issue_330_bridge_cleanup_defect",
      "--",
      "--nocapture"
    ],
    "parallel_group": "local",
    "defer_reason": "The issue-owned regression target csdlc-v2/tests/issue_330_bridge_cleanup_defect.rs is created only after typed bind; fail closed if absent at implementation validation."
  },
  {
    "lane": "existing-recovery-authority-regression",
    "proof_role": "Retain existing preserved-recovery validation proof while #330 adds the post-cleanup validation repair.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 2500,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "preserved_projection_recovery",
      "--",
      "--nocapture"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "existing-cleanup-authority-regression",
    "proof_role": "Retain existing cleanup authority and final-chain regression proof while #330 tightens bridge-fed race behavior.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 2500,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "archived_projection_cleanup",
      "--",
      "--nocapture"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-strict-clippy",
    "proof_role": "Reject warning regressions in the changed C-SDLC v2 crate.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 1500,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `python3 .csdlc/prepared/issues/330/validate_preparation_bundle.py`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test issue_330_bridge_cleanup_defect -- --nocapture`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate5 preserved_projection_recovery -- --nocapture`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test archived_projection_cleanup -- --nocapture`
- `cargo clippy --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings`

## Failure Semantics

Fail closed on stale topology, owned-path collision, validation failure, stale review, or any need to widen beyond the named production boundary.

## Handoff

Retain typed evidence before convergence.
