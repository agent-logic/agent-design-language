# Structured Output Record

Template: 1.0.0

Issue: 5881

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Removed active claim lifecycle compatibility, normalized 190 mutable tracked current records once, and made repo-local active operator guidance topology-owned.

## Artifacts

- csdlc-v2/src/store.rs
- csdlc-v2/src/lifecycle.rs
- csdlc-v2/tests/gate2.rs
- csdlc-v2/tests/gate10a.rs
- docs/tooling/adl_pr_cycle_skill.md
- docs/tooling/PREP_SCOUT_NEXT_ISSUE_READINESS_LANE.md
- https://github.com/agent-logic/agent-design-language/issues/47

## Execution

- Removed LegacyClaim, LegacyIssueRecord, legacy terminal receipt, verification, and normalization production code.
- Made bootstrap and bind requests reject unknown claim-era fields.
- Normalized 190 pre-existing tracked current issue records while preserving historical evidence packets.
- Updated repo-local active C-SDLC v2 skill and runbook sources to use bound branch/worktree topology.
- Added focused regression guards for claim-free records, production code, requests, and repo-local operator guidance.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib",
      "schema::tests"
    ],
    "purpose": "Prove public schemas and request decoding are claim-free.",
    "outcome": "passed",
    "evidence_ref": "local exact schema lane: 3 passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "topology_migration"
    ],
    "purpose": "Prove one-time topology normalization, serialization, and interrupted recovery.",
    "outcome": "passed",
    "evidence_ref": "local topology migration lane: 9 passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2"
    ],
    "purpose": "Prove claim-bearing requests fail and topology-owned create/bind remains atomic and idempotent.",
    "outcome": "passed",
    "evidence_ref": "local gate2 lane: 1 passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "--test",
      "gate6",
      "--test",
      "gate_finish",
      "--test",
      "gate_cleanup"
    ],
    "purpose": "Prove review, cross-repository publication, finish, and cleanup derive authority without claims.",
    "outcome": "passed",
    "evidence_ref": "local lifecycle lanes: gate5 14, gate6 6, gate_finish 17, gate_cleanup 9 passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate10a"
    ],
    "purpose": "Prove repo-local active skills, runbooks, production decoder, and every tracked current record are claim-free.",
    "outcome": "passed",
    "evidence_ref": "local gate10a lane: 17 passed"
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
    "purpose": "Prove the changed C-SDLC v2 Rust surface is warning-free across all targets.",
    "outcome": "passed",
    "evidence_ref": "local strict clippy: passed"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
