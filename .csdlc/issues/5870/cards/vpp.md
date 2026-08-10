# Validation Planning Prompt

Template: 1.0.0

Issue: 5870

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5870/design.md

Diagram: .csdlc/prepared/issues/5870/diagram.mmd

## Selected Lanes

[
  {
    "lane": "exact-child-tests",
    "proof_role": "Exact nextest target distributed_fencing proves quorum fence and revoke without old-holder activation possession, strict committed next-epoch transitions, portable nondecreasing recovery floors, fresh current AuthorityMembership, exact operation allowlisting, atomic immediately durable fence and replay receipts, path and symlink denial, restart and rollback safety, hard capacity behavior, and absent-current-membership denial.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_fencing",
      "--no-tests=fail"
    ],
    "parallel_group": "child",
    "defer_reason": "The issue-owned temporary harness in adl-runtime/tests/distributed_fencing.rs will import read-only adl-runtime/src/distributed/lease.rs with #[path = \"../src/distributed/lease.rs\"] and owned adl-runtime/src/distributed/fencing.rs with #[path = \"../src/distributed/fencing.rs\"] until #5878 registers the production module; after implementation any missing target remains a hard failure."
  },
  {
    "lane": "exact-revision-proof-receipt",
    "proof_role": "Recompute exact source, command, nonzero test, artifact, runner, and receipt bindings and require exact name/result parity for sixteen future ADL_ISSUE_5870_NEGATIVE_CASE_V1 machine markers derived from executed output; no marker or case result is claimed during preparation. [preexec_rejection exit=1 diagnostic_sha256=9ef02b09994a1fae1e9ebdbb42a437d856c45baf388783c7171b422dc9a00978]",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5870/validate-proof-receipt.rb"
    ],
    "parallel_group": "receipt",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo nextest run --manifest-path adl-runtime/Cargo.toml --test distributed_fencing --no-tests=fail`
- `ruby .csdlc/prepared/issues/5870/validate-proof-receipt.rb`

## Failure Semantics

Fail closed on stale dependencies, path overlap, zero tests, invalid evidence, insecure fallback, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
