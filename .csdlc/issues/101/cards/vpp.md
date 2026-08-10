# Validation Planning Prompt

Template: 1.0.0

Issue: 101

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/101/design.md

Diagram: .csdlc/prepared/issues/101/diagram.mmd

## Selected Lanes

[
  {
    "lane": "github-route-policy",
    "proof_role": "Prove policy parity, connector-403 classification, and shared token precedence and redaction without network or real credentials.",
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
    "budget_seconds": 180,
    "budget_tokens": 2500,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_github_route_policy"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject malformed tracked changes.",
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
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate_github_route_policy`
- `git diff --check`

## Failure Semantics

Fail closed on missing owners, policy drift, ambiguous 403 classification, stale review, failed focused proof, or any attempted connector/raw-gh fallback.

## Handoff

Retain typed evidence before convergence.
