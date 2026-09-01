# Structured Output Record

Template: 1.0.0

Issue: 589

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented and proved simple ordered Wuji Runtime v3 startup, reload rollback, Shepherd-gated readiness, bounded CloudWatch health, and governed SSM recovery.

## Artifacts

- adl/src/cli/csm_runtime_v3_cmd.rs
- adl-runtime/src/bin/adl-runtime-guardian.rs
- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/observability.rs
- infra/aws/csm-runtime-health

## Execution

- Current Runtime generation stops and its current HTTPS endpoint disappears before a candidate generation starts.
- Guardian ordinary startup no longer creates a separate continuity client.
- Readiness requires a fresh Shepherd admission lease.
- Health export is allowlisted and emits the active canonical config hash.
- CloudWatch missing-heartbeat recovery invokes bounded CSM recovery through SSM.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "--bin",
      "adl",
      "csm_runtime_v3"
    ],
    "purpose": "CSM lifecycle and transaction regression proof",
    "outcome": "passed",
    "evidence_ref": "final focused run: 11 passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "assembly"
    ],
    "purpose": "Writer recovery proof",
    "outcome": "passed",
    "evidence_ref": "final focused run: 10 passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "control"
    ],
    "purpose": "Shepherd-gated readiness proof",
    "outcome": "passed",
    "evidence_ref": "final focused run: 28 passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "observability"
    ],
    "purpose": "Health-only export and log recovery proof",
    "outcome": "passed",
    "evidence_ref": "final focused run: 32 passed"
  },
  {
    "command": [
      "csm",
      "runtime-v3",
      "reload",
      "--init",
      ".adl/runtime-v3/live/runtime-init.toml"
    ],
    "purpose": "Live ordered service lifecycle proof",
    "outcome": "passed",
    "evidence_ref": "live Wuji HTTPS readiness, observability readiness, and one healthy Shepherd"
  },
  {
    "command": [
      "aws",
      "ssm",
      "list-command-invocations",
      "--profile",
      "agent-logic-admin"
    ],
    "purpose": "CloudWatch-to-SSM recovery proof",
    "outcome": "passed",
    "evidence_ref": "alarm recovery command 9ce0b157-5862-4622-a90d-a39c9f7ff1c4 succeeded"
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
