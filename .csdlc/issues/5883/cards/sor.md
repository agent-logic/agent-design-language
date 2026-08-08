# Structured Output Record

Template: 1.0.0

Issue: 5883

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Retired csdlc-init so csdlc-issue create is the sole typed creation route, and repaired the PVF concurrency proof exposed by exact-head CI.

## Artifacts

- csdlc-v2/src/operator.rs
- csdlc-v2/tests/gate10a.rs
- csdlc-v2/tests/gate4.rs
- docs/tooling/adl_pr_cycle_skill.md

## Execution

- Deleted the csdlc-init Cargo target and source binary.
- Removed csdlc-init from installer, skill, coexistence, and proof authority and reject its reappearance.
- Updated active skills, runbooks, adapters, architecture contracts, and contributor guidance to csdlc-issue create while preserving historical evidence.
- Replaced the timing-threshold concurrency assertion with a deterministic peer barrier that fails if independent lanes are serialized.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2"
    ],
    "purpose": "Prove the real csdlc-issue create/validate/doctor/bind lifecycle.",
    "outcome": "passed",
    "evidence_ref": "exact-head:gate2:1-passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate10a"
    ],
    "purpose": "Prove the retired binary is absent, reappearance fails closed, and active guidance uses csdlc-issue create.",
    "outcome": "passed",
    "evidence_ref": "exact-head:gate10a:19-passed"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove the changed Rust surface is warning-free across all targets.",
    "outcome": "passed",
    "evidence_ref": "exact-head:clippy:passed"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_install_adl_pr_cycle_skill.sh"
    ],
    "purpose": "Prove the installed compatibility skill source teaches csdlc-issue create.",
    "outcome": "passed",
    "evidence_ref": "exact-head:adl-pr-cycle-skill:passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate4"
    ],
    "purpose": "Prove PVF concurrency, convergence, cancellation, timeout, and evidence behavior without scheduler timing assumptions.",
    "outcome": "passed",
    "evidence_ref": "exact-head:gate4:17-passed"
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
