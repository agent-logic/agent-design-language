# Structured Output Record

Template: 1.0.0

Issue: 5566

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Recognize an exact reserved worktree as issue-local when the current Git branch and canonical checkout path match.

## Artifacts

- csdlc-v2/src/lifecycle.rs
- csdlc-v2/tests/gate2.rs
- #5566
- #5306

## Execution

- Derive issue-local identity from the current branch plus canonical reserved worktree path.
- Preserve exact claim equality and relative path safety.
- Add focused success and mismatch regressions.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2",
      "reserved_worktree"
    ],
    "purpose": "Prove exact existing-worktree activation and mismatched-path rejection.",
    "outcome": "passed",
    "evidence_ref": "Both focused match and mismatch regressions passed independently; cargo fmt --check and strict all-target all-feature Clippy passed."
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
