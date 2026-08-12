# Structured Output Record

Template: 1.0.0

Issue: 298

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the non-destructive #298 classification and recovery engine with descriptor-anchored manifests, tagged CAS and lineage, immutable receipts, per-node candidate construction, atomic installation, deterministic restart, and ordinary-commit release proof.

## Artifacts

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

## Validation

[]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
