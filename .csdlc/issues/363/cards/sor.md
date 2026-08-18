# Structured Output Record

Template: 1.0.0

Issue: 363

Repository: agent-logic/agent-design-language

Card: sor

Status: complete

## Summary

Narrow Implemented SPP summary recovery to the exact approved typed repair epoch.

## Artifacts

- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate5.rs

## Execution

- Split SPP summary epoch authorization from immediate-only SIP required-outcome recovery
- Allow only approve_design affected_areas plan_steps and validation_lanes
- Prove exact #274 chain exact recovery provenance second correction unknown operation and SIP denial

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
      "gate5"
    ],
    "purpose": "Prove full recovery and review gate matrix including all R1 closures.",
    "outcome": "passed",
    "evidence_ref": "58 passed at source aabed8b81"
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
    "purpose": "Strict warnings proof.",
    "outcome": "passed",
    "evidence_ref": "PASS at source aabed8b81"
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "dae957c435b73d87af1f36d4e15fb088f6fd055b...HEAD"
    ],
    "purpose": "Diff hygiene.",
    "outcome": "passed",
    "evidence_ref": "no output at source aabed8b81"
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
