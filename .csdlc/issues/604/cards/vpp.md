# Validation Planning Prompt

Template: 1.0.0

Issue: 604

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/604/design.md

Diagram: .csdlc/prepared/issues/604/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-v2-publish-ready-tests",
    "proof_role": "Prove typed ready and reconcile-ready publication behavior, rejection paths, and recovery semantics.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "publication_ready"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-github-route-tests",
    "proof_role": "Prove GitHub route owner boundaries and publication command inventory remain aligned.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1500,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_github_route_policy"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Validate whitespace hygiene for tracked #604 changes.",
    "acceptance_ids": [
      "AC-1",
      "AC-5",
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

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test publication_ready`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate_github_route_policy`
- `git diff --check`

## Failure Semantics

Fail closed on missing typed ready command, caller-forgeable ready truth, stale CAS, identity drift, raw GitHub fallback, unresolved review findings, red checks, or PR publication without Closes #604.

## Handoff

Retain typed evidence before convergence.
