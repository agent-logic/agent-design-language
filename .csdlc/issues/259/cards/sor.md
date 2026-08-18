# Structured Output Record

Template: 1.0.0

Issue: 259

Repository: agent-logic/agent-design-language

Card: sor

Status: complete

## Summary

Bound governed Runtime transport authorization to the terminal #258 authority-store adapter while keeping every raw-store transport constructor test-only and crate-private.

## Artifacts

- adl-runtime/src/distributed/authority_protocol.rs
- adl-runtime/src/distributed/authority_reconciliation.rs
- adl-runtime/src/distributed/transport/core.rs
- adl-runtime/src/distributed/transport/governed/learner_transport/tests.rs
- adl-runtime/src/distributed/transport/governed/polis_runtime.rs
- adl-runtime/tests/distributed_discovery.rs
- adl-runtime/tests/distributed_runtime_transport.rs
- adl-runtime/tests/distributed_transport.rs

## Execution

- Replaced production transport certificate-store fields and constructors with AuthorityBoundCertificateStore-backed authority handles.
- Mapped adapter certificate failures to certificate authorization denial and reconciliation/permit failures to invalid session binding.
- Kept raw-store authorization only in private cfg(test) internals and made dependent integration tests obtain a real published adapter handle through AuthorityStoreAdapterRegistry.
- Updated transport-coupled unit, discovery, and secure Runtime integration fixtures without migrating #260 non-transport callers or touching #203.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--features",
      "internal-test-fixtures",
      "--test",
      "distributed_runtime_transport",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Run the secure Runtime transport integration target through a published adapter fixture.",
    "outcome": "passed",
    "evidence_ref": "dependent-runtime-transport-authority.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Run Git diff hygiene over the issue worktree.",
    "outcome": "passed",
    "evidence_ref": "exact-diff-hygiene.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_transport",
      "--",
      "--nocapture",
      "--test-threads=1"
    ],
    "purpose": "Run the focused governed transport authority target.",
    "outcome": "passed",
    "evidence_ref": "governed-transport-authority.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--features",
      "internal-test-fixtures",
      "--test",
      "distributed_transport",
      "--test",
      "distributed_discovery",
      "--test",
      "distributed_runtime_transport",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict Clippy over the exact changed target denominator.",
    "outcome": "passed",
    "evidence_ref": "runtime-transport-strict-clippy.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_discovery",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Run the directly coupled discovery transport target.",
    "outcome": "passed",
    "evidence_ref": "transport-coupled-discovery.log"
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
