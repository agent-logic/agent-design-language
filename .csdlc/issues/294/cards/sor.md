# Structured Output Record

Template: 1.0.0

Issue: 294

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Added a typed initialized design-envelope recovery that relocates authored artifacts, records complete provenance, invalidates approval, and rejects unsafe bootstrap paths.

## Artifacts

- csdlc-v2/src/store.rs
- csdlc-v2/src/bin/csdlc-edit.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/tests/card_identity.rs

## Execution

- Expose a typed recover-initialized-design-envelope command and public schema.
- CAS-guard initialized/unbound recovery, copy artifacts to distinct safe paths, refresh SPP/VPP bindings, append old/new provenance, and require fresh design approval.
- Reject .git, issue-control, duplicate, absolute, and traversal authored paths during bootstrap/recovery.
- Add focused positive, bootstrap-negative, and dependency-gate tests.

## Validation

[
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
    "purpose": "Run strict all-target Clippy",
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
      "card_identity"
    ],
    "purpose": "Run focused card identity integration tests",
    "outcome": "passed",
    "evidence_ref": "initialized-design-envelope-recovery.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "card_identity"
    ],
    "purpose": "Prove initialized design-envelope recovery, iterative pending authored-design refresh, unsafe .git pre-bind rejection, actual failpoint restart, owned cleanup, and #292 terminal-gate fixture.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/294/card-identity-18-exact-head.log"
  },
  {
    "command": [
      "cargo",
      "check",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml"
    ],
    "purpose": "Prove the recovered csdlc-v2 implementation compiles after the iterative pending authored-design refresh owner fix.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/294/csdlc-v2-cargo-check-exact-head.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove the recovered csdlc-v2 implementation is warning-free across all targets after the iterative pending authored-design refresh owner fix.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/294/csdlc-v2-strict-clippy-exact-head.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "card_identity"
    ],
    "purpose": "Prove initialized design-envelope recovery, unsafe .git pre-bind rejection, actual failpoint restart, owned cleanup, and #292 terminal gate fixture at exact current head.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/294/card-identity-18-exact-head.log"
  },
  {
    "command": [
      "cargo",
      "check",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml"
    ],
    "purpose": "Prove the recovered csdlc-v2 implementation compiles at exact current head after initialized design-envelope recovery changes.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/294/csdlc-v2-cargo-check-exact-head.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove the recovered csdlc-v2 implementation is warning-free across all targets at exact current head after initialized design-envelope recovery changes.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/294/csdlc-v2-strict-clippy-exact-head.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "card_identity"
    ],
    "purpose": "Prove initialized design-envelope recovery and the Linux-stable owned-inode replacement fixture after PR #385 CI failure remediation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/294/card-identity-18-exact-head.log"
  },
  {
    "command": [
      "cargo",
      "check",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml"
    ],
    "purpose": "Prove csdlc-v2 still compiles after PR #385 Linux-stable fixture remediation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/294/csdlc-v2-cargo-check-exact-head.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove csdlc-v2 remains warning-free after PR #385 Linux-stable fixture remediation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/294/csdlc-v2-strict-clippy-exact-head.log"
  }
]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
