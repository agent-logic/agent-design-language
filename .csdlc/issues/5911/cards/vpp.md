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
    "lane": "csdlc-v2-standalone-fastwork-policy",
    "proof_role": "The complete standalone C-SDLC v2 suite proves case-insensitive mandatory FastWork enforcement and all linked-worktree fixture compatibility paths changed by this issue.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml"
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

- `cargo test --manifest-path csdlc-v2/Cargo.toml`
- `bash adl/tools/test_archive_codex_sessions_to_fastwork.sh`

## Failure Semantics

Fail closed on invalid lifecycle state, non-FastWork worktree placement, unavailable storage, checksum mismatch, or any unapproved destructive action.

## Handoff

Retain typed evidence before convergence.
