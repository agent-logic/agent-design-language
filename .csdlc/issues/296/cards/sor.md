# Structured Output Record

Template: 1.0.0

Issue: 296

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Remediated r2 with paired retained authored artifact handles, a functional linked-worktree refresh route, final locked assignment topology/revision checks, and real end-to-end proof.

## Artifacts

- csdlc-v2/src/store.rs
- csdlc-v2/src/review.rs
- csdlc-v2/tests/gate5.rs

## Execution

- Retain design and diagram descriptors, bytes, inode identities, link counts, timestamps, and anchored paths through the immediate canonical commit boundary.
- Permit only the typed recovered refresh operation to reconcile stale authored bytes while retaining projection and audit integrity checks.
- Recheck registered worktree, branch, HEAD, clean substantive revision, approved tuple, and exact scope inside the serialized assignment commit boundary.
- Add real linked-worktree end-to-end CAS, atomic card/history, approval, and reassignment proof plus retained-handle replacement rejection.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib",
      "implemented_authored_design_refresh_retains_handle_identity_until_commit_boundary"
    ],
    "purpose": "Prove a retained paired artifact handle rejects path replacement at the final commit boundary.",
    "outcome": "passed",
    "evidence_ref": "local:r2-retained-handle"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "implemented_authored_design_refresh_end_to_end_is_atomic_and_assignment_gated"
    ],
    "purpose": "Prove linked-worktree recovery, stale CAS, paired refresh, atomic history/cards, approval, and reassignment end to end.",
    "outcome": "passed",
    "evidence_ref": "local:r2-end-to-end"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5"
    ],
    "purpose": "Prove complete review assignment and recovery regression behavior including dirty-tree and exact-revision gates.",
    "outcome": "passed",
    "evidence_ref": "local:r2-gate5"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib"
    ],
    "purpose": "Prove complete library regression including authored artifact race negatives.",
    "outcome": "passed",
    "evidence_ref": "local:r2-lib"
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
    "purpose": "Prove strict all-target lint cleanliness.",
    "outcome": "passed",
    "evidence_ref": "local:r2-clippy"
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
