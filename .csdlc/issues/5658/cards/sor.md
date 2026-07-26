# Structured Output Record

Template: 1.0.0

Issue: 5658

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented bound-worktree lifecycle materialization for typed C-SDLC v2 bind and added a regression proving the primary checkout does not advance to bound.

## Artifacts

- csdlc-v2/src/lifecycle.rs
- csdlc-v2/tests/gate7_lifecycle.rs

## Execution

- Added lifecycle materialization helpers that copy issue records, cards, prepared artifacts, and evidence roots into the target worktree before committing a non-local bind transition.
- Changed bind commit routing so issue-local binds still commit in place, while non-local binds commit through a Store rooted at the declared worktree.
- Added a gate7 regression that bootstraps on primary main, binds a new issue worktree with absent ignored .csdlc state, and verifies the target worktree owns the bound record while primary remains initialized.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate7_lifecycle"
    ],
    "purpose": "Prove bound-worktree lifecycle materialization and fail-closed primary-main behavior.",
    "outcome": "passed",
    "evidence_ref": "csdlc-lifecycle-focused.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Verify changed files have no whitespace or patch hygiene errors.",
    "outcome": "passed",
    "evidence_ref": "diff-hygiene.log"
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
