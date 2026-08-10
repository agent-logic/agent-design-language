# Validation Planning Prompt

Template: 1.0.0

Issue: 5874

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5874/design.md

Diagram: .csdlc/prepared/issues/5874/diagram.mmd

## Selected Lanes

[
  {
    "lane": "exact-child-tests",
    "proof_role": "Exact nonzero nextest target proves authenticated digest-bound redacted snapshot catalogs and transfer manifests, including certificate authorization, schema versioning, replay, incomplete transfer, corruption, oversize, and fenced-authority fail-closed cases.",
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
      "distributed_snapshot_catalog",
      "--no-tests=fail"
    ],
    "parallel_group": "child",
    "defer_reason": "The issue-owned temporary #[path = \"../src/distributed/snapshot_catalog.rs\"] harness in adl-runtime/tests/distributed_snapshot_catalog.rs will route adl-runtime/src/distributed/snapshot_catalog.rs after both owned paths are implemented and until integration issue #5878 registers the production module; their pre-implementation absence is not a validation pass."
  },
  {
    "lane": "exact-revision-proof-receipt",
    "proof_role": "Recompute source, command, nonzero test, artifact, machine-derived negative-case, runner, and receipt bindings. [preexec_rejection exit=1 diagnostic_sha256=946415ba5922dcd2b1aa02885207ed7d5ebdf557ea1299b5acdd805c6866d3f6]",
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
      ".csdlc/prepared/issues/5874/validate-proof-receipt.rb"
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

- `cargo nextest run --manifest-path adl-runtime/Cargo.toml --test distributed_snapshot_catalog --no-tests=fail`
- `ruby .csdlc/prepared/issues/5874/validate-proof-receipt.rb`

## Failure Semantics

Fail closed on stale dependencies, path overlap, zero tests, invalid evidence, insecure fallback, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
