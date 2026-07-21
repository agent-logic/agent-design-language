# Structured Output Record

Template: 1.0.0

Issue: 5591

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented canonical Runtime v3 ingress, deterministic execution and replay continuity, and graceful terminal serialization without Runtime v2 or AWS.

## Artifacts

- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/ingress.rs
- adl-runtime-kernel/tests/assembly.rs
- adl-runtime-kernel/tests/control.rs

## Execution

- Route signed Execute capability through bounded canonical Runtime v3 components and deterministic results
- Persist signed checkpoints and restore fresh-runtime sequence and duplicate-safe replay continuity
- Close admission and drain accepted work before terminal checkpoint serialization
- Keep governed provider and cloud-bridge operations behind permit-authoritative operational APIs
- Prevent pressure recovery from reopening admission after a signed terminal action is reserved
- Keep unsupported and failed dispatch retries deterministic without advancing accepted state

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "assembly",
      "--test",
      "control",
      "--test",
      "operations"
    ],
    "purpose": "Prove governed routing, terminal-pressure race safety, delayed-work checkpoint ordering, deterministic retries, and focused Runtime v3 regressions; assembly 4, control 21, operations 9, ingress atomic admission, and focused process pressure/signed shutdown yielded 36 unique passing tests",
    "outcome": "passed",
    "evidence_ref": "subagent:019f8277-0050-7db0-a96b-05593df9c703@6f19349e6d6227c362f5d73dce2c977aab41c1db:36-focused-tests:/Volumes/FastWork/adl-5591-rereview-target"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove the Parity-A implementation and test targets are warning-free under strict Clippy",
    "outcome": "passed",
    "evidence_ref": "owner:019f8189-8205-7dd3-bbe2-f7f4dddd098a@6f19349e6d6227c362f5d73dce2c977aab41c1db:/Volumes/FastWork/adl-5591/runtime-target"
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
