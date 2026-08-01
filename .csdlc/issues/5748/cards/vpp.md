# Validation Planning Prompt

Template: 1.0.0

Issue: 5748

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5748/design.md

Diagram: .csdlc/prepared/issues/5748/diagram.mmd

## Selected Lanes

[
  {
    "lane": "terminal-inventory-and-full-receipt-integrity",
    "proof_role": "Verify exact-head owner-binary provenance, the retained live 90/10/1 universe, canonical terminal projections, full retained receipts, authored artifacts, and pinned fail-closed exception identity.",
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
    "resource_profile": "small",
    "budget_seconds": 900,
    "budget_tokens": 9000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5748/validate-final-inventory.sh"
    ],
    "parallel_group": "local-inventory",
    "defer_reason": null
  },
  {
    "lane": "inventory-path-guard-regression",
    "proof_role": "Prove final, parent-component, and dangling symlinks fail closed in the aggregate validator.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5748/validate-final-inventory.sh",
      "--self-test-path-guards"
    ],
    "parallel_group": "local-fast",
    "defer_reason": null
  },
  {
    "lane": "terminal-receipt-doctor-regression",
    "proof_role": "Prove doctor rejects tampered receipt digests, authored-artifact drift, and symlinked receipts.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate7_lifecycle",
      "no_pr_closeout_produces_doctor_valid_terminal_state"
    ],
    "parallel_group": "local-fast",
    "defer_reason": null
  },
  {
    "lane": "aggregate-diff-hygiene",
    "proof_role": "Reject whitespace errors across the complete origin/main aggregate change.",
    "acceptance_ids": [
      "AC-4",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check",
      "origin/main..HEAD"
    ],
    "parallel_group": "local-fast",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `bash .csdlc/prepared/issues/5748/validate-final-inventory.sh`
- `bash .csdlc/prepared/issues/5748/validate-final-inventory.sh --self-test-path-guards`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate7_lifecycle no_pr_closeout_produces_doctor_valid_terminal_state`
- `git diff --check origin/main..HEAD`

## Failure Semantics

Fail closed on missing receipt, stale identity, unsupported disposition correction, dirty-worktree conflict, doctor failure, receipt mismatch, or any forbidden route.

## Handoff

Retain typed evidence before convergence.
