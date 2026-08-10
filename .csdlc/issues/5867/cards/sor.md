# Structured Output Record

Template: 1.0.0

Issue: 5867

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented deterministic committed membership epochs, bounded replay, duplicate active-voter control-key denial, and trusted-commitment snapshot recovery.

## Artifacts

- adl-runtime/src/distributed/membership.rs
- adl-runtime/tests/distributed_membership.rs
- .csdlc/evidence/5867/execution-proof.json
- .csdlc/evidence/5867/negative-cases.json

## Execution

- Add deterministic committed membership event application with exact duplicate idempotence and stale, gap, conflict, domain, capacity, and shape denial.
- Preserve one effective Guardian control key per active voter through promotion, removal, snapshot, and replay.
- Require an externally trusted snapshot commitment and reject policies whose valid worst-case state exceeds the fixed snapshot budget.

## Validation

[
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_membership",
      "--no-tests=fail"
    ],
    "purpose": "Run the exact issue-owned distributed membership target.",
    "outcome": "passed",
    "evidence_ref": "exact-child-tests.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5867/validate-proof-receipt.rb"
    ],
    "purpose": "Validate the retained WP-04.05 two-revision receipt.",
    "outcome": "passed",
    "evidence_ref": "exact-revision-proof-receipt.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_membership",
      "--",
      "-D",
      "warnings",
      "-A",
      "clippy::absurd_extreme_comparisons"
    ],
    "purpose": "Run strict Clippy for the issue-owned membership surface.",
    "outcome": "passed",
    "evidence_ref": "strict-focused-clippy.log"
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
