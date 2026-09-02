# Structured Output Record

Template: 1.0.0

Issue: 629

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented V3-H.3 GitHub, PR-state, review, and publication routes under the single non-authoritative csdlc v3 binary.

## Artifacts

- csdlc-v3/src/commands/remote/mod.rs
- csdlc-v3/src/main.rs
- csdlc-v3/tests/command_manifest.rs
- csdlc-v3/tests/remote_publication_commands.rs
- docs/csdlc-v3/v3-command-manifest.json
- .csdlc/prepared/issues/629/design.md
- .csdlc/prepared/issues/629/diagram.mmd
- .csdlc/prepared/issues/629/validate-v3-h3-github-publication.sh
- .csdlc/issues/629

## Execution

- Implemented #629-owned remote/publication routes github, github-issue, github-pr, pr-state, publish, and review.
- Kept every #629 route read-only and non-authoritative before #505 cutover.
- Added typed route planning that blocks missing review truth, stale reviewed head, missing closing linkage, caller-forged PR readback, missing GitHub readback linkage, self-review, and missing exact review revisions.
- Added credential-name redaction in route reports.
- Updated the v3 command manifest so #629 routes are implemented but not live authority.
- Added focused command-manifest and remote-publication tests.

## Validation

[
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
    "purpose": "Run strict Clippy across csdlc-v3 targets.",
    "outcome": "passed",
    "evidence_ref": "629-clippy.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v3/Cargo.toml"
    ],
    "purpose": "Run all C-SDLC v3 tests.",
    "outcome": "passed",
    "evidence_ref": "629-full-v3-regression.log"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/629/validate-v3-h3-github-publication.sh",
      "all"
    ],
    "purpose": "Run the #629 issue-owned validator.",
    "outcome": "passed",
    "evidence_ref": "629-issue-validator.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "remote_publication_commands"
    ],
    "purpose": "Run focused #629 integration tests.",
    "outcome": "passed",
    "evidence_ref": "629-remote-publication-tests.log"
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
