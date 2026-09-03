# Structured Output Record

Template: 1.0.0

Issue: 505

Repository: agent-logic/agent-design-language

Card: sor

Status: ready

## Summary

Hardened the C-SDLC v3 single-binary remote alias routes and replacement denominator before #505 cutover while preserving C-SDLC v2 as live authority.

## Artifacts

- commit 9e93d5c507d388a38d3ace14e85fd988006ea345
- csdlc-v3/src/commands/remote/mod.rs
- csdlc-v3/src/main.rs
- csdlc-v3/tests/command_manifest.rs
- csdlc-v3/tests/real_issue_canary.rs
- docs/csdlc-v3/full-replacement-denominator.json
- docs/csdlc-v3/v3-command-manifest.json

## Execution

- Implemented pre-cutover top-level v3 bridge aliases for GitHub, review, publication, finish, and cleanup route families without granting v3 operational authority.
- Split publication derivation from terminal delivery so `csdlc publish` accepts a publish-stage evidence envelope without requiring merged PR, closed issue, or cleanup artifacts.
- Made `csdlc finish` fail closed when remote readback derives `OperatorRequired` instead of terminal or checkpoint completion.
- Made `csdlc clean` cleanup-preview-only before cutover and allowed it to consume a cleanup-only evidence envelope.
- Made generic bridge verification parse the full typed bridge evidence set before reporting ready, so schema and identity mismatches fail closed.
- Updated the tracked v3 command manifest and full replacement denominator to reflect 23 visible commands, 21 v2 entrypoints, 17 implemented commands, 2 partial commands, 4 fail-closed commands, and 5 remaining replacement gaps.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "command_manifest"
    ],
    "purpose": "Prove one-binary route table, operation-specific remote alias semantics, minimal publish/cleanup request shapes, identity mismatch rejection, and manifest-denominator consistency.",
    "outcome": "passed",
    "evidence_ref": "exact-head:9e93d5c507d388a38d3ace14e85fd988006ea345:12-passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml"
    ],
    "purpose": "Run the full C-SDLC v3 suite after remote alias hardening.",
    "outcome": "passed",
    "evidence_ref": "exact-head:9e93d5c507d388a38d3ace14e85fd988006ea345:101-passed"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject warnings across all C-SDLC v3 targets.",
    "outcome": "passed",
    "evidence_ref": "exact-head:9e93d5c507d388a38d3ace14e85fd988006ea345:passed"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/505/validate-authority-transition-prep.rb"
    ],
    "purpose": "Prove #505 authority-transition gates and v2-live boundary still hold after command-denominator changes.",
    "outcome": "passed",
    "evidence_ref": "exact-head:9e93d5c507d388a38d3ace14e85fd988006ea345:status-pass"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--all",
      "--",
      "--check"
    ],
    "purpose": "Reject Rust formatting drift after remote alias hardening.",
    "outcome": "passed",
    "evidence_ref": "exact-head:9e93d5c507d388a38d3ace14e85fd988006ea345:passed"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Verify whitespace hygiene for the current #505 worktree diff.",
    "outcome": "passed",
    "evidence_ref": "exact-head:9e93d5c507d388a38d3ace14e85fd988006ea345:passed"
  },
  {
    "command": [
      "csdlc-validate",
      "--root",
      "/Volumes/FastWork/adl-worktrees/adl-issue-505-v3-f-authority-transition-decision-exec",
      "issue",
      "--issue",
      "505"
    ],
    "purpose": "Verify typed C-SDLC v2 issue state remains valid in implemented phase before review and publication.",
    "outcome": "passed",
    "evidence_ref": "exact-head:9e93d5c507d388a38d3ace14e85fd988006ea345:status-pass-generation-21-ready-false"
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
