# Validation Planning Prompt

Template: 1.0.0

Issue: 294

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/294/design.md

Diagram: .csdlc/prepared/issues/294/diagram.mmd

## Selected Lanes

[
  {
    "lane": "initialized-design-envelope-recovery",
    "proof_role": "Focused unit and linked-worktree proof for AC-1 through AC-9",
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
    "budget_seconds": 600,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "card_identity"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "issue-292-terminal-ancestry-gate",
    "proof_role": "Local dependency-gate fixture; live csdlc-finish terminal receipt, typed issue read, and origin/main ancestry observation remain deferred to closeout",
    "acceptance_ids": [
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "card_identity",
      "issue_292_waits_for_terminal_ancestral_294"
    ],
    "parallel_group": "local",
    "defer_reason": "All three live observations can exist only after merge/finish, outside this session authority"
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test card_identity`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test card_identity issue_292_waits_for_terminal_ancestral_294`

## Failure Semantics

Fail closed without mutation on stale CAS, wrong phase, unsafe path, artifact drift, insufficient reviewer proof, audit-history mismatch, validation failure, or stale review.

## Handoff

Retain typed evidence before convergence.
