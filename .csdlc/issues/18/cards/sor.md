# Structured Output Record

Template: 1.0.0

Issue: 18

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Made split C-SDLC GitHub machine-readable output tolerate downstream pipe closure without panic output.

## Artifacts

- csdlc-v2/src/output.rs
- csdlc-v2/src/lib.rs
- csdlc-v2/src/bin/csdlc-github-issue.rs
- csdlc-v2/src/bin/csdlc-github-pr.rs
- csdlc-v2/tests/gate_github_actions.rs
- docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md

## Execution

- Added a shared machine-readable JSON stdout writer that accepts only BrokenPipe as normal termination.
- Routed split GitHub issue and pull-request schema, success, and error payloads through the shared writer.
- Added process-level regression coverage that closes stdout early and rejects panic contamination.
- Documented the machine-output termination contract.

## Validation

[
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--target-dir",
      "../adl-builds/csdlc-v2-issue-18",
      "--lib",
      "--bin",
      "csdlc-github-issue",
      "--bin",
      "csdlc-github-pr",
      "--test",
      "gate_github_actions",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict lint on the changed C-SDLC v2 output surfaces.",
    "outcome": "passed",
    "evidence_ref": "csdlc-github-broken-pipe-clippy.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--target-dir",
      "../adl-builds/csdlc-v2-issue-18",
      "--lib",
      "--test",
      "gate_github_actions"
    ],
    "purpose": "Run the focused C-SDLC v2 GitHub action and shared output regression surfaces.",
    "outcome": "passed",
    "evidence_ref": "csdlc-github-broken-pipe-focused.log"
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
