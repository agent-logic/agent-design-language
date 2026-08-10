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
    "proof_role": "Prove policy parity, connector-403 classification, installer ownership, and shared token precedence and redaction without network or real credentials.",
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
    "proof_role": "Reject malformed tracked changes across the exact issue base and corrected implementation head.",
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
      "--check",
      "0608764d902b02eb2965002168ae210059866e8e",
      "33fad0d3bc70c8701a811670d8254bdef374289b"
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
- `git diff --check 0608764d902b02eb2965002168ae210059866e8e 33fad0d3bc70c8701a811670d8254bdef374289b`

## Failure Semantics

Fail closed on missing owners, policy drift, ambiguous 403 classification, stale review, failed focused proof, or any attempted connector/raw-gh fallback.

## Handoff

Retain typed evidence before convergence.
