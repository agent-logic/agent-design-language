# Validation Planning Prompt

Template: 1.0.0

Issue: 617

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/617/design.md

Diagram: .csdlc/prepared/issues/617/diagram.mmd

## Selected Lanes

[
  {
    "lane": "canonical-agent-name",
    "proof_role": "Prove authoritative dynamic and Shepherd canonical names, field distinction, roster/detail parity, additive serialization, and OpenAPI agreement with nonzero focused tests.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 7000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/617/validate-canonical-agent-name.sh"
    ],
    "parallel_group": "runtime-api",
    "defer_reason": "The issue-owned wrapper exists; its named canonical_name cases are issue #617 implementation deliverables."
  },
  {
    "lane": "rust-format",
    "proof_role": "Prove formatting for the Runtime kernel workspace.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--check"
    ],
    "parallel_group": "hygiene",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Prove exact branch diff whitespace hygiene.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
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

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/617/validate-canonical-agent-name.sh`
- `cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --check`
- `git diff --check`

## Failure Semantics

Fail closed on inferred identity, field aliasing, roster/detail drift, schema drift, zero-test validation, scope expansion, or stale review.

## Handoff

Retain typed evidence before convergence.
