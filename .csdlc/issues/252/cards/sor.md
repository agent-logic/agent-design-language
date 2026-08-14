# Structured Output Record

Template: 1.0.0

Issue: 252

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Serialized process-heavy Guardian invocations only in test builds so the directly included parity target cannot exhaust hosted lease, pipe, or child resources and misclassify unrelated scenarios as SpawnFailed.

## Artifacts

- adl-runtime/src/guardian.rs
- .csdlc/evidence/252

## Execution

- Added a cfg(test) Tokio mutex around Guardian invocation in the included Guardian module.
- Preserved production concurrency and genuine missing-program SpawnFailed behavior.
- Repeated the complete Guardian test namespace ten times with eight requested test threads.

## Validation

[
  {
    "command": [
      "/Users/daniel/.cargo/bin/cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "parity_b_live_kernel"
    ],
    "purpose": "Prove all Guardian parity tests, including both hosted failures and genuine missing-program SpawnFailed.",
    "outcome": "passed",
    "evidence_ref": "guardian-parity-focused.log"
  },
  {
    "command": [
      "/Users/daniel/.cargo/bin/cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml"
    ],
    "purpose": "Run the complete Runtime-kernel test boundary used by hosted Runtime focused tests.",
    "outcome": "passed",
    "evidence_ref": "runtime-required-local.log"
  },
  {
    "command": [
      "/Users/daniel/.cargo/bin/cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove warning-free Runtime-kernel production and test targets.",
    "outcome": "passed",
    "evidence_ref": "runtime-strict-clippy.log"
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
