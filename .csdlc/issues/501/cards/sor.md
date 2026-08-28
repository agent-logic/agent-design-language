# Structured Output Record

Template: 1.0.0

Issue: 501

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the non-authoritative V3-B foundation slice with explicit repository context, deterministic state/projection replay, a read-only machine-output command, and focused proof for retained requirements #164 through #167. C-SDLC v2 remains the sole operational authority.

## Artifacts

- csdlc-v3/src/repository/mod.rs
- csdlc-v3/src/application/mod.rs
- csdlc-v3/src/bin/csdlc-v3-foundation.rs
- csdlc-v3/tests/foundation.rs
- csdlc-v3/src/lib.rs

## Execution

- Added explicit RepositoryContext discovery and required-contract path checks.
- Added deterministic FoundationState projection replay with stable machine JSON output.
- Added read-only csdlc-v3-foundation CLI requiring --repo-root instead of ambient cwd authority.
- Added focused foundation tests for repository context, deterministic projection replay, retained requirements #164-#167, and the three-minute issue-start readiness projection.
- Fixed one strict-clippy finding in the existing V3-A test surface.

## Validation

[
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--check"
    ],
    "purpose": "Verify Rust formatting for the V3-B crate surface.",
    "outcome": "passed",
    "evidence_ref": "local command output, 2026-08-28, issue #501 worktree"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "foundation"
    ],
    "purpose": "Prove repository context, deterministic projection replay, retained requirements #164-#167, and three-minute issue-start projection.",
    "outcome": "passed",
    "evidence_ref": "4 tests passed locally, 2026-08-28, issue #501 worktree"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject strict Rust warnings across library, binary, and test targets.",
    "outcome": "passed",
    "evidence_ref": "local command output, 2026-08-28, issue #501 worktree"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace and malformed diff defects.",
    "outcome": "passed",
    "evidence_ref": "local command output, 2026-08-28, issue #501 worktree"
  },
  {
    "command": [
      "cargo",
      "run",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--bin",
      "csdlc-v3-foundation",
      "--",
      "--repo-root",
      "/Volumes/FastWork/adl-worktrees/adl-issue-501-v3-b-csdlc-v3-foundation"
    ],
    "purpose": "Smoke test the read-only foundation command and explicit repository-root argument.",
    "outcome": "passed",
    "evidence_ref": "machine JSON output schema csdlc.v3.foundation.v1, 2026-08-28, issue #501 worktree"
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

- Bind #501 in an issue worktree only after doctor passes.
