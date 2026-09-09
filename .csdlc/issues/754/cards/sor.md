# Structured Output Record

Template: 1.0.0

Issue: 754

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Registry1.0.3/1.0.4 compatibility repaired; current/retained route assertions corrected. Full444-test matrix covered by initial unaffected targets plus corrected target rerun; final targeted/format/clippy PVF checks. No release approval or PR753 merge.

## Artifacts

- .csdlc/prepared/issues/754/validation-summary.json

## Execution

- csdlc-v2/src/registry.rs
- csdlc-v2/tests/gate9.rs
- csdlc-v2/tests/gate10a.rs
- csdlc-v2/tests/gate_github_route_policy.rs

## Validation

[
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
    "purpose": "Verify clippy at final source; full matrix retained in validation-summary.json",
    "outcome": "passed",
    "evidence_ref": "clippy.log"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all",
      "--",
      "--check"
    ],
    "purpose": "Verify format at final source; full matrix retained in validation-summary.json",
    "outcome": "passed",
    "evidence_ref": "format.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate9",
      "--test",
      "gate_github_route_policy"
    ],
    "purpose": "Verify registry-focused at final source; full matrix retained in validation-summary.json",
    "outcome": "passed",
    "evidence_ref": "registry-focused.log"
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
