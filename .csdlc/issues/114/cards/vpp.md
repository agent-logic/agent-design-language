# Validation Planning Prompt

Template: 1.0.0

Issue: 114

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/114/design.md

Diagram: .csdlc/prepared/issues/114/diagram.mmd

## Selected Lanes

[
  {
    "lane": "conversation-history-exact-denominator",
    "proof_role": "After #111 and #112 are terminal and ancestral, validate one exact candidate aggregate against history-proof-receipt-schema.v2.json. AC-1 through AC-9 coverage may be emitted only when the canonical 42 unique ordered cases all pass and six separate, ordered, nonduplicate receipts for the Rust store target, Runtime API target, real Runtime-backed browser validator, strict Clippy, diff hygiene, and fresh independent review each bind the same resolved candidate SHA, are structurally complete, and pass. Test selection, an aggregate status, a stale review, or any missing receipt is insufficient.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 21600,
    "budget_tokens": 100000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/114/validate-history-proof.rb",
      "--repo",
      ".",
      "--candidate-sha",
      "HEAD",
      "--manifest",
      ".csdlc/prepared/issues/114/history-proof-cases.json",
      "--schema",
      ".csdlc/prepared/issues/114/history-proof-receipt-schema.v2.json",
      "--results",
      ".csdlc/evidence/114/conversation-history-proof-receipts.v2.json"
    ],
    "parallel_group": "114-history-proof",
    "defer_reason": "Deferred until #111 and #112 are terminal through merged PRs ancestral to the selected #114 execution base and all six issue-owned exact-candidate receipts exist. The validator currently fails closed on absent product evidence; preparation records no product implementation, AC pass, dependency completion, review pass, or #83 mutation claim."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 21600

Tokens: 100000

## Commands

- `ruby .csdlc/prepared/issues/114/validate-history-proof.rb --repo . --candidate-sha HEAD --manifest .csdlc/prepared/issues/114/history-proof-cases.json --schema .csdlc/prepared/issues/114/history-proof-receipt-schema.v2.json --results .csdlc/evidence/114/conversation-history-proof-receipts.v2.json`

## Failure Semantics

Fail closed on nonterminal or nonancestral dependencies, stale authority, unauthorized read, cursor or sequence drift, duplicate conflict, terminal rewrite, forbidden-field exposure, unbounded search/export, retention or deletion ambiguity, partial write, disk-full, reply loss, unknown or lossy migration, unsafe rollback, corruption, receipt-chain break, residue, zero-test selection, denominator drift, or unresolved review finding.

## Handoff

Retain typed evidence before convergence.
