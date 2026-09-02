# Validation Planning Prompt

Template: 1.0.0

Issue: 627

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/627/design.md

Diagram: .csdlc/prepared/issues/627/diagram.mmd

## Selected Lanes

[
  {
    "lane": "627-denominator-manifest",
    "proof_role": "Prove the machine-readable denominator covers all 21 v2 binaries and identifies the 19 sprint replacement routes.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/627/validate-v3-h1-command-denominator.sh",
      "manifest"
    ],
    "parallel_group": "627-focused",
    "defer_reason": "Runs after the issue-owned validator and manifest are implemented."
  },
  {
    "lane": "627-cli-help",
    "proof_role": "Prove one `csdlc` binary exposes the manifest command surface through stable help or semantic command listing.",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "command_manifest"
    ],
    "parallel_group": "627-focused",
    "defer_reason": "Runs after command manifest tests are implemented."
  },
  {
    "lane": "627-fail-closed",
    "proof_role": "Prove unimplemented live-authority routes fail closed without invoking v2, raw gh, or shell wrappers.",
    "acceptance_ids": [
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "command_manifest",
      "fail_closed"
    ],
    "parallel_group": "627-focused",
    "defer_reason": "Runs after fail-closed route handling is implemented."
  },
  {
    "lane": "627-no-v2-source-change",
    "proof_role": "Reject C-SDLC v2 source changes in this issue's diff.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 400,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/627/validate-v3-h1-command-denominator.sh",
      "no-v2-source-change"
    ],
    "parallel_group": "627-focused",
    "defer_reason": "Runs after implementation diff exists."
  },
  {
    "lane": "627-diff-hygiene",
    "proof_role": "Reject whitespace and conflict artifacts in the bounded issue diff.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 400,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "627-focused",
    "defer_reason": "Runs after implementation changes exist."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/627/validate-v3-h1-command-denominator.sh manifest`
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test command_manifest`
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test command_manifest fail_closed`
- `bash .csdlc/prepared/issues/627/validate-v3-h1-command-denominator.sh no-v2-source-change`
- `git diff --check`

## Failure Semantics

Fail closed on denominator drift, missing route ownership, hidden v2/raw-gh/v1 fallback, live v3 authority claims before #505, or any C-SDLC v2 source change.

## Handoff

Retain typed evidence before convergence.
