# Validation Planning Prompt

Template: 1.0.0

Issue: 5896

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5896/design.md

Diagram: .csdlc/prepared/issues/5896/diagram.mmd

## Selected Lanes

[
  {
    "lane": "migration-contract",
    "proof_role": "Prove classification, atomicity, preservation, failure behavior, and idempotence through focused Rust tests.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "topology_migration"
    ],
    "parallel_group": "migration",
    "defer_reason": null
  },
  {
    "lane": "cohort-proof",
    "proof_role": "Prove the exact before and after cohort counts, dispositions, second-run no-op, current doctor results, and issue 5844 bindability.",
    "acceptance_ids": [
      "AC-1",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5896/validate-migration.rb"
    ],
    "parallel_group": "cohort",
    "defer_reason": "Validator is produced with the implementation and runs against the final migrated cohort."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace errors in migration code, tests, records, and evidence.",
    "acceptance_ids": [
      "AC-5",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "hygiene",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 21600

Tokens: 100000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test topology_migration`
- `ruby .csdlc/prepared/issues/5896/validate-migration.rb`
- `git diff --check`

## Failure Semantics

Fail before any mutation on invalid digest, missing issue state, ambiguous topology, partial topology, or classification error; retain a complete diagnostic report.

## Handoff

Retain typed evidence before convergence.
