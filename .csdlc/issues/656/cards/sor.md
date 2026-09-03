# Structured Output Record

Template: 1.0.0

Issue: 656

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented immutable matched Runtime v3 generations with one receipt, atomic current and previous references, rollback, platform and schema checks, and CSM preflight before start or reload mutation.

## Artifacts

- adl/tools/install_runtime_v3_generation.sh
- adl/tools/runtime_v3_generation.py
- adl/src/cli/csm_runtime_v3_cmd.rs
- adl/tools/test_runtime_v3_generation_install.sh
- adl/tests/csm_runtime_v3_generation.rs

## Execution

- Added a single Runtime v3 generation command for install, verify, and rollback.
- Bound CSM, Guardian, and kernel hashes plus source revision, platform, build profile, and Runtime-init schema in one receipt.
- Required installed CSM, kernel, and launchd or systemd Guardian paths to resolve through the same current generation.
- Moved current and candidate generation verification ahead of interrupted-transaction reconciliation and service stop paths.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_runtime_v3_generation_install.sh"
    ],
    "purpose": "Prove matched immutable generation install, verification, atomic activation, tamper rejection, contained rollback, and current-reference preservation.",
    "outcome": "passed",
    "evidence_ref": "git:8e5a47ad48af6536b700a18a16d7c06b0fea76da; runtime v3 generation installer PASS"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "--bin",
      "adl",
      "preflight_"
    ],
    "purpose": "Prove hash, executable-mode, direct-generation, launchd, and systemd mismatches fail before service mutation.",
    "outcome": "passed",
    "evidence_ref": "git:8e5a47ad48af6536b700a18a16d7c06b0fea76da; 6 passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "--test",
      "csm_runtime_v3_generation"
    ],
    "purpose": "Prove the public installer rejects a mixed generation and preserves the current generation reference.",
    "outcome": "passed",
    "evidence_ref": "git:8e5a47ad48af6536b700a18a16d7c06b0fea76da; 1 passed"
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "8e5a47ad48af6536b700a18a16d7c06b0fea76da^",
      "8e5a47ad48af6536b700a18a16d7c06b0fea76da"
    ],
    "purpose": "Reject malformed whitespace in the exact substantive remediation commit after Rust formatting passed.",
    "outcome": "passed",
    "evidence_ref": "git:8e5a47ad48af6536b700a18a16d7c06b0fea76da; cargo fmt check and exact commit diff check passed"
  },
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
    "purpose": "Prove the hosted-CI repair is warning-free across ADL targets.",
    "outcome": "passed",
    "evidence_ref": "git:235befd2e2f38378c01208a993add590a79de799; strict Clippy passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "--bin",
      "adl",
      "preflight_"
    ],
    "purpose": "Prove generation preflight behavior remains correct and the test fixture completes without coverage-only timeouts.",
    "outcome": "passed",
    "evidence_ref": "git:235befd2e2f38378c01208a993add590a79de799; 6 passed in 0.01s; cargo fmt and git diff check passed"
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
