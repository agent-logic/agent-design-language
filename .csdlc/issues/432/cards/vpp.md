# Validation Planning Prompt

Template: 1.0.0

Issue: 432

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/432/design.md

Diagram: .csdlc/prepared/issues/432/diagram.mmd

## Selected Lanes

[
  {
    "lane": "adl-boundary",
    "proof_role": "Prove zero tracked paths and zero active authoritative dependencies.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      "adl/tools/test_check_no_tracked_adl.sh"
    ],
    "parallel_group": "boundary",
    "defer_reason": null
  },
  {
    "lane": "worktree-policy",
    "proof_role": "Run the exact nonzero fastwork_policy module to prove relocated policy resolution and allowed, rejected, required, and bound-topology behavior.",
    "acceptance_ids": [
      "AC-4",
      "AC-7"
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
      "--lib",
      "lifecycle::fastwork_policy_tests"
    ],
    "parallel_group": "rust",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject patch whitespace errors.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "hygiene",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 21600

Tokens: 100000

## Commands

- `bash adl/tools/test_check_no_tracked_adl.sh`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --lib lifecycle::fastwork_policy_tests`
- `git diff --check`

## Failure Semantics

Fail closed on any tracked .adl path, active .adl authority reference, sensitive promotion, policy drift, or failed fresh-checkout proof.

## Handoff

Retain typed evidence before convergence.
