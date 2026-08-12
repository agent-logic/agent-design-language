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
