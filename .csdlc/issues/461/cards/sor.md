# Structured Output Record

Template: 1.0.0

Issue: 461

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Remove lifecycle TLS command inputs, make Runtime init configuration the sole TLS path authority, enforce fail-closed path and private-key permission validation, and update Guardian and operational proof launchers to consume config-owned TLS.

## Artifacts

- adl-runtime/src/bin/adl-runtime-lifecycle-soak.rs
- adl/tools/validate_v092_runtime_guardian_lifecycle.sh
- adl/tools/run_runtime_v3_operational_proof.sh

## Execution

- Removed certificate-chain, private-key, and trust-root arguments from the lifecycle soak parser and production fixture.
- Validated configured TLS paths as absolute canonical regular non-symlink files, required distinct paths, and denied group/other private-key access.
- Updated the Guardian fixture to create protected TLS fixtures and write their paths into its generated Runtime config.
- Removed TLS environment and argv forwarding from the operational proof launcher.
- Added regression tests for removed CLI flags, config-owned fixture startup, unsafe permissions, and symlink denial.

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "purpose": "Run Git diff whitespace hygiene.",
    "outcome": "passed",
    "evidence_ref": "diff-hygiene.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--bin",
      "adl-runtime-lifecycle-soak",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run focused Clippy with warnings denied.",
    "outcome": "passed",
    "evidence_ref": "runtime-lifecycle-soak-clippy.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--bin",
      "adl-runtime-lifecycle-soak",
      "--",
      "--nocapture"
    ],
    "purpose": "Run all lifecycle soak binary unit tests.",
    "outcome": "passed",
    "evidence_ref": "runtime-lifecycle-soak-tests.log"
  },
  {
    "command": [
      "bash",
      "-n",
      "adl/tools/validate_v092_runtime_guardian_lifecycle.sh",
      "adl/tools/run_runtime_v3_operational_proof.sh"
    ],
    "purpose": "Run Bash syntax validation.",
    "outcome": "passed",
    "evidence_ref": "tls-launch-shell-syntax.log"
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
