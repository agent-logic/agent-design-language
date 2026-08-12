# Validation Planning Prompt

Template: 1.0.0

Issue: 5911

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5911/design.md

Diagram: .csdlc/prepared/issues/5911/design.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-bind-fastwork-policy",
    "proof_role": "Focused Rust unit tests prove allowed FastWork binding, outside-parent refusal, and missing-policy refusal.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib",
      "fastwork_policy"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "transcript-archive-verification",
    "proof_role": "Issue-owned archive tests prove manifest generation, digest verification, source preservation, and canonical FastWork containment.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 3000,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "adl/tools/test_archive_codex_sessions_to_fastwork.sh"
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

- `cargo test --manifest-path csdlc-v2/Cargo.toml --lib fastwork_policy`
- `bash adl/tools/test_archive_codex_sessions_to_fastwork.sh`

## Failure Semantics

Fail closed on invalid lifecycle state, non-FastWork worktree placement, unavailable storage, checksum mismatch, or any unapproved destructive action.

## Handoff

Retain typed evidence before convergence.
