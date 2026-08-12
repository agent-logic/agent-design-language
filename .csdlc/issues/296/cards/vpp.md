# Validation Planning Prompt

Template: 1.0.0

Issue: 296

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/296/design.md

Diagram: .csdlc/prepared/issues/296/diagram.mmd

## Selected Lanes

[
  {
    "lane": "implemented-authored-design-refresh",
    "proof_role": "Deterministic typed lifecycle, CAS, artifact safety, paired revalidation, approval invalidation, append-only history, and regression proof",
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
    "budget_seconds": 600,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "card_identity",
      "implemented_authored_design_refresh"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "implemented-authored-design-refresh-linked-worktree",
    "proof_role": "Deterministic canonical registered-worktree, Git-common lock, wrong-worktree, alias, and concurrent linked-worktree negative proof",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-6",
      "AC-7",
      "AC-8"
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
      "card_identity",
      "implemented_authored_design_refresh_linked_worktree"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "issue-294-terminal-ancestry-observation",
    "proof_role": "Deferred live observation that #296 is terminal and ancestral before #294 resumes; not implementation validation proof",
    "acceptance_ids": [
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "card_identity",
      "issue_294_waits_for_terminal_ancestral_296"
    ],
    "parallel_group": "post-terminal",
    "defer_reason": "Run only after #296 is terminal; until then #294 remains blocked and AC-9 is not green."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test card_identity implemented_authored_design_refresh`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test card_identity implemented_authored_design_refresh_linked_worktree`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test card_identity issue_294_waits_for_terminal_ancestral_296`

## Failure Semantics

Fail closed without mutation on wrong phase or card, absent or stale recovery provenance, active authority truth, stale CAS, unsafe or drifted artifacts, partial failure, no-op refresh, validation failure, or stale review.

## Handoff

Retain typed evidence before convergence.
