# Validation Planning Prompt

Template: 1.0.0

Issue: 510

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/510/design.md

Diagram: .csdlc/prepared/issues/510/diagram.mmd

## Selected Lanes

[
  {
    "lane": "valid-reload",
    "proof_role": "Prove a valid file update atomically replaces the active configuration.",
    "acceptance_ids": [
      "AC-1"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/510/validate-valid-reload.rb"
    ],
    "parallel_group": "hot-reload",
    "defer_reason": null
  },
  {
    "lane": "invalid-retention",
    "proof_role": "Prove invalid update content is rejected and the last-known-good configuration remains active.",
    "acceptance_ids": [
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/510/validate-invalid-retention.rb"
    ],
    "parallel_group": "hot-reload",
    "defer_reason": null
  },
  {
    "lane": "debounce",
    "proof_role": "Prove file-event bursts are debounced before reload evaluation.",
    "acceptance_ids": [
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/510/validate-debounce.rb"
    ],
    "parallel_group": "hot-reload",
    "defer_reason": null
  },
  {
    "lane": "concurrent-read",
    "proof_role": "Prove concurrent readers observe complete configurations only.",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/510/validate-concurrent-read.rb"
    ],
    "parallel_group": "hot-reload",
    "defer_reason": null
  },
  {
    "lane": "watcher-shutdown",
    "proof_role": "Prove the reload watcher exits cleanly when shutdown is requested.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/510/validate-watcher-shutdown.rb"
    ],
    "parallel_group": "hot-reload",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/510/validate-valid-reload.rb`
- `ruby .csdlc/prepared/issues/510/validate-invalid-retention.rb`
- `ruby .csdlc/prepared/issues/510/validate-debounce.rb`
- `ruby .csdlc/prepared/issues/510/validate-concurrent-read.rb`
- `ruby .csdlc/prepared/issues/510/validate-watcher-shutdown.rb`

## Failure Semantics

Fail closed on stale typed state, invalid reload activation, partial snapshots, missing debounce proof, watcher task leaks, stale exact-head review, ambiguous publication, or merge attempts.

## Handoff

Retain typed evidence before convergence.
