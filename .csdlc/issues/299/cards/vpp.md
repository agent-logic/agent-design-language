# Validation Planning Prompt

Template: 1.0.0

Issue: 299

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/299/design.md

Diagram: .csdlc/prepared/issues/299/diagram.mmd

## Selected Lanes

[
  {
    "lane": "archived-projection-cleanup-focused",
    "proof_role": "Prove exact capture, type-correct removal, restart, idempotence, and sentinel preservation for AC-1 through AC-7, including terminal gate, recovery receipt load, cleanup namespace creation, capture intent, exchange, capture receipt, removal intent, unlink/rmdir, parent fsync, placeholder disposal, final receipt, and completed-repeat restart boundaries.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
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
      "csdlc-v2/Cargo.toml",
      "--test",
      "archived_projection_cleanup",
      "--no-tests=fail"
    ],
    "parallel_group": "local",
    "defer_reason": "Deferred until implementation creates csdlc-v2/tests/archived_projection_cleanup.rs with #[path = \"../src/projection_cleanup.rs\"] coverage for csdlc-v2/src/projection_cleanup.rs and enumerates the #299 cleanup restart cuts: terminal gate/read, recovery receipt load, cleanup namespace creation, capture intent, exchange, capture receipt, removal intent, unlink/rmdir, parent fsync, placeholder disposal, final cleanup receipt, and completed repeat."
  },
  {
    "lane": "csdlc-v2-strict-clippy",
    "proof_role": "Reject warning and target regressions in the cleanup implementation.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--workspace",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo nextest run --manifest-path csdlc-v2/Cargo.toml --test archived_projection_cleanup --no-tests=fail`
- `cargo clippy --manifest-path csdlc-v2/Cargo.toml --workspace --all-targets -- -D warnings`

## Failure Semantics

Fail closed on missing #298 terminal ancestry, receipt/canonical/archive mismatch, unsafe topology, symlink/special node, unsupported type, replacement, non-empty directory, mount/owner/mode drift, parent identity drift, stale generation/digest, or unresolved review finding.

## Handoff

Retain typed evidence before convergence.
