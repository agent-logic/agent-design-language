# Validation Planning Prompt

Template: 1.0.0

Issue: 32

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/32/design.md

Diagram: .csdlc/prepared/issues/32/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runner-preflight-rust-contracts",
    "proof_role": "Prove typed capacity, policy, dispatch, stale-ref, and redaction classifications.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "runner_preflight"
    ],
    "parallel_group": "focused-contracts",
    "defer_reason": null
  },
  {
    "lane": "runner-preflight-schema",
    "proof_role": "Prove the typed request and result are present in the public schema bundle.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "schema::tests"
    ],
    "parallel_group": "focused-contracts",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject malformed tracked changes before review.",
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
    "parallel_group": "focused-contracts",
    "defer_reason": null
  },
  {
    "lane": "github-runner-live-canary",
    "proof_role": "Prove group 3 is repository-scoped and unrestricted, the hosted label is visible, and the PR job receives that runner and terminates.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "run",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--bin",
      "csdlc-github",
      "--",
      "runner-preflight",
      "--request",
      ".csdlc/prepared/issues/32/live-preflight.json"
    ],
    "parallel_group": "post-publication",
    "defer_reason": "Requires live organization settings and the published issue #32 PR head."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml runner_preflight`
- `cargo test --manifest-path csdlc-v2/Cargo.toml schema::tests`
- `git diff --check`
- `cargo run --manifest-path csdlc-v2/Cargo.toml --bin csdlc-github -- runner-preflight --request .csdlc/prepared/issues/32/live-preflight.json`

## Failure Semantics

Fail closed with typed indeterminate or ineligible diagnostics; never infer eligibility from runner Ready alone.

## Handoff

Retain typed evidence before convergence.
