# Validation Planning Prompt

Template: 1.0.0

Issue: 5877

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5877/design.md

Diagram: .csdlc/prepared/issues/5877/diagram.mmd

## Selected Lanes

[
  {
    "lane": "exact-child-tests",
    "proof_role": "Exact nextest target distributed_projection validates the issue-owned OpenAPI document and proves one authenticated least-privilege redacted v1 view, coherent-cut consistency, deterministic ordering and identifiers, hard response bounds, denied detail, unsupported-version and malformed-state failure, and exact schema/status/error parity.",
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
      "distributed_projection",
      "--no-tests=fail"
    ],
    "parallel_group": "child",
    "defer_reason": "The issue-owned temporary #[path = \"../src/distributed/projection.rs\"] harness in adl-runtime/tests/distributed_projection.rs will route adl-runtime/src/distributed/projection.rs after both Rust paths are implemented and until integration issue #5878 registers the route and production module; pre-implementation absence is not a validation pass."
  },
  {
    "lane": "exact-revision-proof-receipt",
    "proof_role": "Recompute exact source, OpenAPI, and command bindings, nonzero selected tests, machine-derived negative-case identifiers and results, artifact digests, runner identity, timestamps, and immutable evidence. [preexec_rejection exit=1 diagnostic_sha256=c910bc39219b1f7b467597837b572aecdc324cddfaab06aca75f4887effb13aa]",
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
      ".csdlc/prepared/issues/5877/validate-proof-receipt.rb"
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

- `cargo nextest run --manifest-path adl-runtime/Cargo.toml --test distributed_projection --no-tests=fail`
- `ruby .csdlc/prepared/issues/5877/validate-proof-receipt.rb`

## Failure Semantics

Fail closed on stale dependencies, path overlap, zero tests, invalid evidence, insecure fallback, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
