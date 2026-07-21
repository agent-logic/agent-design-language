# Validation Planning Prompt

Template: 1.0.0

Issue: 5350

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5350/design.md

Diagram: .csdlc/prepared/issues/5350/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Validate six cards, design, inventory, exact dependency direction, preparation-only claim, budgets, PVF, no-deferral contract, and root/product safety",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5350/validate-preparation.rb"
    ],
    "parallel_group": "preparation",
    "defer_reason": null
  },
  {
    "lane": "subject-and-corpus-verification",
    "proof_role": "PLANNED-UNIMPLEMENTED: after typed implementation, verify both exact subjects, terminal dependency receipts/ancestry, corpus bundle, evidence envelopes, portable stream hashes, and command policy",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5350/validate-parity.sh",
      "subjects"
    ],
    "parallel_group": "parity-local",
    "defer_reason": "Execute only after every named dependency is merged and typed terminal; the fail-closed runner stub must first be replaced through typed implementation"
  },
  {
    "lane": "exact-shadow-comparison",
    "proof_role": "PLANNED-UNIMPLEMENTED: after typed implementation, run and compare every corpus case, behavior, repetition, equivalence group, and difference group with only reviewed normalization",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 8000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5350/validate-parity.sh",
      "compare"
    ],
    "parallel_group": "parity-local",
    "defer_reason": "Execute only after every named dependency is merged and typed terminal; the fail-closed runner stub must first be replaced through typed implementation"
  },
  {
    "lane": "runtime-workcell-overlay",
    "proof_role": "PLANNED-UNIMPLEMENTED: after typed implementation, verify exact terminal Runtime ten-group and WP-10A live evidence, reject non-live credit, and preserve #5361 downstream direction",
    "acceptance_ids": [
      "AC-7",
      "AC-8",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5350/validate-parity.sh",
      "overlays"
    ],
    "parallel_group": "parity-overlay",
    "defer_reason": "Execute only after every named dependency is merged and typed terminal; the fail-closed runner stub must first be replaced through typed implementation"
  },
  {
    "lane": "parity-complete",
    "proof_role": "PLANNED-UNIMPLEMENTED: after typed implementation, enforce zero blockers/unclassified rows, strict lint, exact COTS and scope, 1500/2000 LoC budgets, 120-test ceiling, deterministic rerun, no network, and two exact-revision reviews",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 12000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5350/validate-parity.sh",
      "complete"
    ],
    "parallel_group": "parity-final",
    "defer_reason": "Execute only after every named dependency is merged and typed terminal; the fail-closed runner stub must first be replaced through typed implementation"
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/5350/validate-preparation.rb`
- `bash .csdlc/prepared/issues/5350/validate-parity.sh subjects`
- `bash .csdlc/prepared/issues/5350/validate-parity.sh compare`
- `bash .csdlc/prepared/issues/5350/validate-parity.sh overlays`
- `bash .csdlc/prepared/issues/5350/validate-parity.sh complete`

## Failure Semantics

Fail closed without parity credit, publication, acceptance, soak, cutover, or deletion on identity drift, invalid corpus/evidence, unknown command or normalization, missing case/group/overlay, unclassified mismatch, non-live Runtime credit, absent WP-10A proof, forbidden dependency, budget failure, or deferred acceptance proof.

## Handoff

Retain typed evidence before convergence.
