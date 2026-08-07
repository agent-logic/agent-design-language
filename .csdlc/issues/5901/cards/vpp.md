# Validation Planning Prompt

Template: 1.0.0

Issue: 5901

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5901/design.md

Diagram: .csdlc/prepared/issues/5901/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-readiness-focused",
    "proof_role": "Focused Rust tests prove safe future-path admission, malformed path rejection, and exact-digest initialized/ready planning repair without widening non-planning edits or bound behavior.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "wp04-wave-preflight",
    "proof_role": "Exact typed doctor sweep and canonical wave preflight prove all sixteen packets are structurally bind-ready and claim-free.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5862/validate-implementation-wave.rb",
      "--preflight"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "exact-scope-guard",
    "proof_role": "Exact-base changed-path allowlist rejects Guardian product changes, child binding, and unexpected lifecycle mutation.",
    "acceptance_ids": [
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5901/validate-scope.rb"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "wp04-terminal-fixtures",
    "proof_role": "Fixture-backed terminal execution proves valid derived envelopes and rejects malformed, digest, head, merge, linkage, and ancestry failures.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5901/test-implementation-wave.rb"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate2`
- `ruby .csdlc/prepared/issues/5862/validate-implementation-wave.rb --preflight`
- `ruby .csdlc/prepared/issues/5901/validate-scope.rb`
- `ruby .csdlc/prepared/issues/5901/test-implementation-wave.rb`

## Failure Semantics

Fail closed on containment ambiguity, record digest drift, claim restoration, product-path mutation, or any child doctor failure.

## Handoff

Retain typed evidence before convergence.
