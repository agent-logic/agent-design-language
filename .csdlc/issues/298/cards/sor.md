# Structured Output Record

Template: 1.0.0

Issue: 298

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implement anchored classification and deterministic resumable recovery for preserved failed issue projections.

## Artifacts

- csdlc-v2/src/projection_recovery.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/bin/csdlc-issue.rs
- csdlc-v2/tests/gate5.rs
- csdlc-v2/src/projection_recovery.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/bin/csdlc-issue.rs
- csdlc-v2/tests/gate5.rs

## Execution

- Added typed classify/recover contracts and CLI/schema exports; cleanup remains excluded for #299
- Added retained descriptor-relative no-follow traversal with fstat/fstatfs identity, mount, ownership, permission, type, and link validation
- Added immutable hash-linked main/per-node receipt ledgers and exact-prefix temporary-node create/write/fsync/no-replace-publish restart
- Added archive, candidate verification, atomic exchange/no-replace install, displacement, canonical verification, and final idempotent recovery
- Blocked ordinary commits before complete recovery and proved later ordinary commit after recovery
- Added deterministic 21-boundary failpoints plus lineage, replacement, symlink, hardlink, mode, topology, initialized/ready, and #291-compatible regressions
- Cross-mount enforcement is implemented with retained-handle fstatfs mount identity; dynamic bind-mount injection was not available locally and is not claimed
- Typed tagged-CAS classification and failed-operation lineage
- Retained descriptor-relative no-follow per-node identity and mount authority
- Immutable main and per-node recovery ledgers with deterministic restart
- Recovery-owned candidate construction, atomic install, displacement, and canonical verification
- CLI/schema/store integration and focused recovery-only regression proof

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib"
    ],
    "purpose": "Run complete library unit suite.",
    "outcome": "passed",
    "evidence_ref": "csdlc-v2-lib.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict all-target Clippy.",
    "outcome": "passed",
    "evidence_ref": "csdlc-v2-strict-clippy.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "preserved_projection_recovery"
    ],
    "purpose": "Run focused gate5 recovery tests.",
    "outcome": "passed",
    "evidence_ref": "preserved-projection-recovery.log"
  }
]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
