# Validation Planning Prompt

Template: 1.0.0

Issue: 526

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/526/design.md

Diagram: .csdlc/prepared/issues/526/diagram.mmd

## Selected Lanes

[
  {
    "lane": "tail10-ceremony-denominator",
    "proof_role": "Prove ancestry, zero review blockers, canonical check-only preflight, authorization, exact tag and release identity, notes digest, and retained readback.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/526/validate-tail10.rb"
    ],
    "parallel_group": "ceremony",
    "defer_reason": "Runs after the explicitly authorized ceremony."
  },
  {
    "lane": "tail10-negative",
    "proof_role": "Reject missing authorization, target drift, notes drift, skipped canonical preflight, and incomplete ancestry.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/526/validate-tail10.rb",
      "--negative"
    ],
    "parallel_group": "ceremony-preflight",
    "defer_reason": null
  },
  {
    "lane": "notes-diff-hygiene",
    "proof_role": "Reject malformed release-note changes and unrelated residue.",
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
    "parallel_group": "ceremony-preflight",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/526/validate-tail10.rb`
- `ruby .csdlc/prepared/issues/526/validate-tail10.rb --negative`
- `git diff --check`

## Failure Semantics

Fail closed before mutation on missing ancestry, review, authorization, candidate identity, typed route, or exact readback proof.

## Handoff

Retain typed evidence before convergence.
