# Structured Output Record

Template: 1.0.0

Issue: 296

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Remediated r3 refresh and assignment commit gaps with rollback-capable post-swap paired artifact verification, and narrowed proof truth to assertions actually executed.

## Artifacts

- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate5.rs
- .csdlc/issues/296

## Execution

- R3 P1 refresh gap: retain paired authored handles through the store swap, verify after the new projection is installed, and atomically restore the complete prior issue directory when verification fails.
- R3 P1 assignment gap: retain the approved design and diagram pair across final topology, HEAD, substantive revision, and projection commit; rollback assignment projection if the pair changes across commit.
- R3 P2 proof overclaim: assert SPP/VPP parity, pending approval, preserved branch/worktree/transitions/execution, exact old/new audit digests, blocked preapproval assignment, no-op rejection, and unchanged state after failure; no scheduler-race claim.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5"
    ],
    "purpose": "Run linked-worktree lifecycle, exact audit/parity/history, authority, CAS, and review regression proof.",
    "outcome": "passed",
    "evidence_ref": "local:r3-gate5"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib"
    ],
    "purpose": "Run complete library artifact identity and lifecycle regression proof.",
    "outcome": "passed",
    "evidence_ref": "local:r3-lib"
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
    "evidence_ref": "local:r3-clippy"
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
