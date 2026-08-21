# Structured Output Record

Template: 1.0.0

Issue: 461

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Make lifecycle TLS authority config-only, snapshot validated opened-file bytes into Runtime-owned state, redact external paths, and prove real HTTPS/WSS Guardian lifecycle and recovered Vector operation.

## Artifacts

- adl-runtime/src/bin/adl-runtime-lifecycle-soak.rs
- adl-runtime-kernel/src/observability.rs
- adl-runtime-kernel/tests/observability.rs
- adl/tools/validate_v092_runtime_guardian_lifecycle.sh
- .csdlc/evidence/461/runtime-guardian-config-owned-tls.log

## Execution

- Removed lifecycle TLS command inputs and load certificate, private-key, trust-root, and private-continuity TLS authority only from Runtime configuration.
- Opened and identity-checked configured TLS files once, captured bytes from the validated handles, and copied them into protected Runtime-owned snapshots before downstream use.
- Removed configured external paths from PEM and Guardian diagnostics and added adversarial path-substitution coverage.
- Updated the Guardian harness to generate config-owned API and private-continuity TLS, exercise 50 HTTPS and 50 authenticated WSS connections, dependency degradation, Vector recovery, and Runtime restart.
- Classified automatically recovered Vector child exits as warnings while preserving errors for recovery persistence or restart failure.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/validate_v092_runtime_guardian_lifecycle.sh",
      "--suite",
      "preflight_1x"
    ],
    "purpose": "Execute the real config-owned TLS Guardian lifecycle with HTTPS, WSS, continuity TLS, Vector recovery, and restart.",
    "outcome": "passed",
    "evidence_ref": "runtime-guardian-config-owned-tls.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "observability",
      "runtime_vector_pipeline_restarts_child_after_sustained_master_log_stagnation",
      "--",
      "--exact",
      "--nocapture"
    ],
    "purpose": "Prove a recovered Vector restart is warning-classified and retains service continuity.",
    "outcome": "passed",
    "evidence_ref": "runtime-kernel-vector-recovery.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--bin",
      "adl-runtime-lifecycle-soak",
      "configured_tls",
      "--",
      "--nocapture"
    ],
    "purpose": "Prove config-owned TLS permission, symlink, and identity-substitution denial.",
    "outcome": "passed",
    "evidence_ref": "runtime-tls-configured-files.log"
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
    "purpose": "Run focused lifecycle-soak Clippy with warnings denied.",
    "outcome": "passed",
    "evidence_ref": "runtime-lifecycle-soak-clippy.log"
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
    "purpose": "Run Runtime-kernel Clippy with warnings denied.",
    "outcome": "passed",
    "evidence_ref": "runtime-kernel-clippy.log"
  },
  {
    "command": [
      "bash",
      "-n",
      "adl/tools/validate_v092_runtime_guardian_lifecycle.sh",
      "adl/tools/run_runtime_v3_operational_proof.sh"
    ],
    "purpose": "Validate the lifecycle launch scripts syntactically.",
    "outcome": "passed",
    "evidence_ref": "tls-launch-shell-syntax.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Verify diff whitespace hygiene.",
    "outcome": "passed",
    "evidence_ref": "diff-hygiene.log"
  }
]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
