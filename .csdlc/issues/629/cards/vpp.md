# Validation Planning Prompt

Template: 1.0.0

Issue: 629

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/629/design.md

Diagram: .csdlc/prepared/issues/629/diagram.mmd

## Selected Lanes

[
  {
    "lane": "629-remote-publication-tests",
    "proof_role": "Prove #629 remote/publication route behavior and authority rejection.",
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
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "real_issue_canary"
    ],
    "parallel_group": "629-rust",
    "defer_reason": null
  },
  {
    "lane": "629-issue-validator",
    "proof_role": "Prove #629 route ownership, non-authority, and no v2 source changes.",
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
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/629/validate-v3-h3-github-publication.sh",
      "all"
    ],
    "parallel_group": "629-focused",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --manifest-path csdlc-v3/Cargo.toml --test real_issue_canary`
- `bash .csdlc/prepared/issues/629/validate-v3-h3-github-publication.sh all`

## Failure Semantics

Fail closed on stale review truth, forged readback, missing closing linkage, credential exposure, raw gh use, v2 fallback, or v3 authority claim.

## Handoff

Retain typed evidence before convergence.
