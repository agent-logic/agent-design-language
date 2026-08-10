# Validation Planning Prompt

Template: 1.0.0

Issue: 5878

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5878/design.md

Diagram: .csdlc/prepared/issues/5878/diagram.mmd

## Selected Lanes

[
  {
    "lane": "exact-child-tests",
    "proof_role": "Run exact nextest target distributed_guardian with nonzero enforcement to prove complete production registration, deterministic coherent integration, authenticated API and WSS continuity, OpenAPI parity, hard bounds, redaction, partitions, fencing, migration, recovery, shutdown, disable, and rollback behavior.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_guardian",
      "--no-tests=fail"
    ],
    "parallel_group": "child",
    "defer_reason": "The issue-owned integration target adl-runtime/tests/distributed_guardian.rs is created only after the two owned registration paths are implemented and all fifteen child contracts are terminal; pre-implementation absence is not a validation pass."
  },
  {
    "lane": "production-distributed-guardian",
    "proof_role": "Launch production Guardians and kernels from the exact protected source revision and retain bounded redacted logs and artifacts proving authenticated API and WSS, partition, fencing, migration, recovery, rollback or disable, clean shutdown, and no insecure fallback.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "bash",
      "adl/tools/validate_v092_distributed_guardian.sh"
    ],
    "parallel_group": "integration",
    "defer_reason": "The issue-owned production validator adl/tools/validate_v092_distributed_guardian.sh is created during implementation after registration and the integration target exists; pre-implementation absence is not proof or a pass."
  },
  {
    "lane": "native-distributed-receipts",
    "proof_role": "Recompute macOS, Linux, and Windows receipts exactly once each from actual production command logs and artifacts, distinct run identifiers and runner identities, exact protected source revision, nonzero successful production execution, bounded redacted outputs, and digest-verified evidence; reject missing, duplicate, synthetic, or self-attested receipts.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "ruby",
      "adl/tools/validate_v092_distributed_native_receipts.rb"
    ],
    "parallel_group": "native",
    "defer_reason": "The issue-owned native receipt validator adl/tools/validate_v092_distributed_native_receipts.rb is created during implementation and can verify receipts only after actual platform runs; pre-implementation absence is not proof or a pass."
  },
  {
    "lane": "exact-revision-proof-receipt",
    "proof_role": "Reject self-attestation by recomputing exact source and command bindings, nonzero integration results, machine-derived negative cases, bounded logs and artifacts, and exact macOS, Linux, and Windows receipt digests and runner identities. [preexec_rejection exit=1 diagnostic_sha256=34eb48e9c92d52048105794654f1350509c44ade82aa5b504df449b7efa23cc6]",
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
      ".csdlc/prepared/issues/5878/validate-proof-receipt.rb"
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

- `cargo nextest run --manifest-path adl-runtime/Cargo.toml --test distributed_guardian --no-tests=fail`
- `bash adl/tools/validate_v092_distributed_guardian.sh`
- `ruby adl/tools/validate_v092_distributed_native_receipts.rb`
- `ruby .csdlc/prepared/issues/5878/validate-proof-receipt.rb`

## Failure Semantics

Fail closed on stale dependencies, path overlap, zero tests, invalid evidence, insecure fallback, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
