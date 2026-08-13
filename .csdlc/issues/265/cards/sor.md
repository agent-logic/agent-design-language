# Structured Output Record

Template: 1.0.0

Issue: 265

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented Layer 8 authority enforcement at Runtime kernel conversation ingress.

## Artifacts

- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- .csdlc/prepared/issues/265/validate_preparation_bundle.py
- .csdlc/prepared/issues/265/readiness-packet.md
- .csdlc/evidence/265

## Execution

- Added a ControlService Layer 8 authority gate that signs and verifies ingress requests, authorizes Contact and Continue actions, and refuses unauthorized requests before conversation session or turn side effects.
- Added production runtime startup wiring for optional Layer 8 authority/signing profiles, including sender key-byte identity binding, recipient Polis validation, and fail-closed incomplete or invalid configuration.
- Added focused runtime-kernel regressions proving refused ingress leaves no conversation session and authorized ingress proceeds to dispatch.

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Run git diff hygiene check.",
    "outcome": "passed",
    "evidence_ref": "diff-hygiene.log"
  },
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/265/validate_preparation_bundle.py"
    ],
    "purpose": "Run the issue-owned preparation validator after #112 terminal cache is present.",
    "outcome": "passed",
    "evidence_ref": "issue-265-preparation-validator.log"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--check"
    ],
    "purpose": "Run rustfmt check for adl-runtime-kernel.",
    "outcome": "passed",
    "evidence_ref": "runtime-kernel-fmt.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "layer8_ingress",
      "--",
      "--nocapture"
    ],
    "purpose": "Run the focused #265 runtime kernel ingress regressions.",
    "outcome": "passed",
    "evidence_ref": "runtime-kernel-layer8-ingress-focused.log"
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
    "purpose": "Run strict Clippy for adl-runtime-kernel.",
    "outcome": "passed",
    "evidence_ref": "runtime-kernel-strict-clippy.log"
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
