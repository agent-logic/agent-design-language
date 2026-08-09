# Structured Output Record

Template: 1.0.0

Issue: 74

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Fixed csdlc-bind so stale same-issue projections without Git topology authority are skipped before strict current-schema decoding.

## Artifacts

- csdlc-v2/tests/gate2.rs
- csdlc-v2/src/lifecycle.rs
- csdlc-v2/tests/gate2.rs

## Execution

- Extended the real csdlc-bind Gate 2 canary with an unrelated legacy projection containing the retired claim field.
- Proved bind leaves the unrelated claim-bearing projection byte-for-byte unchanged.
- Proved the same retired field on the relevant issue remains strict corruption while existing collision checks remain fail closed.
- Moved the no-branch/no-worktree decision ahead of strict IssueRecord deserialization in the topology scan.
- Added a real-binary regression with a second registered worktree containing a same-issue legacy claim projection.
- Kept strict decoding and collision rejection for every projection that declares branch or worktree authority.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2",
      "bind_topology_scan_uses_canonical_record_identity"
    ],
    "purpose": "Prove unrelated claim-bearing legacy records are ignored without mutation while relevant corruption and ownership collisions remain fail closed.",
    "outcome": "passed",
    "evidence_ref": "local:issue-74-gate2:1-passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2"
    ],
    "purpose": "Prove the exact same-issue stale projection succeeds without mutation while relevant corruption and genuine issue, branch, and worktree collisions remain fail closed.",
    "outcome": "passed",
    "evidence_ref": "local:issue-74-regression-gate2:5-passed"
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
    "purpose": "Reject compilation, ownership, error-path, test, and lint regressions across the C-SDLC v2 package.",
    "outcome": "passed",
    "evidence_ref": "local:issue-74-regression-strict-clippy:passed"
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
