# Structured Output Record

Template: 1.0.0

Issue: 589

Repository: agent-logic/agent-design-language

Card: sor

Status: ready

## Summary

Implemented and proved simple ordered Wuji Runtime v3 startup, reload rollback, Shepherd-gated readiness, bounded CloudWatch health, governed SSM recovery, and safe transaction cleanup.

## Artifacts

- adl/src/cli/csm_runtime_v3_cmd.rs
- adl/tools/check_coverage_impact.sh
- adl/tools/test_check_coverage_impact.sh
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
- CSM failure-path tests prove transaction cleanup never deletes a pre-existing destination, and coverage-impact routing selects the focused CSM lifecycle tests.

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
    "purpose": "CSM lifecycle, failure-path, and transaction regression proof",
    "outcome": "passed",
    "evidence_ref": "final focused run: 20 passed"
  },
  {
    "command": [
      "cargo",
      "llvm-cov",
      "--manifest-path",
      "adl/Cargo.toml",
      "--bin",
      "adl",
      "--json",
      "--summary-only",
      "--",
      "csm_runtime_v3"
    ],
    "purpose": "Focused changed-source line coverage proof",
    "outcome": "passed",
    "evidence_ref": "csm_runtime_v3_cmd.rs: 686/851 lines, 80.61 percent"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_check_coverage_impact.sh"
    ],
    "purpose": "Coverage-impact mapping contract proof",
    "outcome": "passed",
    "evidence_ref": "focused mapping selects cli::csm_runtime_v3_cmd::tests"
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
    "evidence_ref": "governed Wuji SSM recovery command succeeded"
  }
]

## Integration

pr_open

## Publication

Publication: draft

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
