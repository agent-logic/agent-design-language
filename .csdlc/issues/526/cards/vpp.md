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
    "lane": "ceremony-preflight",
    "proof_role": "Run canonical release ceremony tests and the safe check-only v0.92.1 command against the exact clean main candidate and merge-based gate.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 360,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/526/validate-ceremony-preflight.sh"
    ],
    "parallel_group": "ceremony-preflight",
    "defer_reason": "Runs after the gate file and exact candidate are frozen."
  },
  {
    "lane": "tail10-live-readback",
    "proof_role": "Prove live remote tag and published GitHub release identity plus receipt, notes, review-revision, ordering, and ancestry parity.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/526/validate-tail10.rb",
      "receipt"
    ],
    "parallel_group": "ceremony",
    "defer_reason": "Runs after explicitly authorized mutation."
  },
  {
    "lane": "tail10-adversarial",
    "proof_role": "Reject unsafe flags, malformed argv, stale review binding, ordering violations, tag drift, and incomplete merge gates.",
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

- `bash .csdlc/prepared/issues/526/validate-ceremony-preflight.sh`
- `ruby .csdlc/prepared/issues/526/validate-tail10.rb receipt`
- `ruby .csdlc/prepared/issues/526/validate-tail10.rb --negative`
- `git diff --check`

## Failure Semantics

Fail closed before mutation on missing ancestry, review, authorization, candidate identity, typed route, or exact readback proof.

## Handoff

Retain typed evidence before convergence.
