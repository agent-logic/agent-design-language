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
    "lane": "implemented-authored-design-refresh-end-to-end",
    "proof_role": "Deterministic linked-worktree integration proof for assignment, typed recovery, stale CAS rejection, paired refresh, atomic card/history update, exact tuple approval, and reassignment across AC-1 through AC-8.",
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
      "gate5",
      "implemented_authored_design_refresh_end_to_end_is_atomic_and_assignment_gated"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "implemented-authored-design-refresh-retained-handles",
    "proof_role": "Deterministic unit proof that paired authored artifact handles retain inode/path identity through the final commit-boundary verification and reject replacement.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib",
      "implemented_authored_design_refresh_retains_handle_identity_until_commit_boundary"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-gate5-review-assignment-regression",
    "proof_role": "Deterministic complete Gate 5 integration suite covering clean/dirty assignment, linked-worktree topology, exact revision, recovery, atomic history, and review authority regressions; no claim of scheduler-level race injection.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 7000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "issue-294-terminal-ancestry-observation",
    "proof_role": "Deferred live observation that issue 296 is terminal and ancestral before issue 294 resumes; not implementation proof.",
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
    "defer_reason": "Run only after issue 296 is terminal; until then issue 294 remains blocked and AC-9 is not green."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate5 implemented_authored_design_refresh_end_to_end_is_atomic_and_assignment_gated`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --lib implemented_authored_design_refresh_retains_handle_identity_until_commit_boundary`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate5`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test card_identity issue_294_waits_for_terminal_ancestral_296`

## Failure Semantics

Fail closed without mutation on wrong phase or card, absent or stale recovery provenance, active authority truth, stale CAS, unsafe or drifted artifacts, partial failure, no-op refresh, validation failure, or stale review.

## Handoff

Retain typed evidence before convergence.
