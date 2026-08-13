# Structured Output Record

Template: 1.0.0

Issue: 327

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Remove the unreachable sunset v1 tooling helper and prove extant ADL compatibility binaries remain fail closed.

## Artifacts

- adl/src/cli/mod.rs
- adl/tests/issue_327_removed_tooling.rs

## Execution

- Remove private uncalled real_tooling helper that caused strict dead_code failure.
- Add focused extant-binary regression for adl and adl-review tooling routes.
- Preserve v1 sunset, typed v2 authority, and all #259 surfaces.

## Validation

[
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict all-target Clippy.",
    "outcome": "passed",
    "evidence_ref": "adl-strict-clippy.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "--test",
      "issue_327_removed_tooling"
    ],
    "purpose": "Run the exact issue-owned regression.",
    "outcome": "passed",
    "evidence_ref": "issue-327-routing-regression.log"
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
