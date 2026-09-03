# Structured Output Record

Template: 1.0.0

Issue: 659

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented configurable, backward-compatible Runtime v3 service convergence with generous validated defaults and distinct listener-open and authenticated-readiness gates.

## Artifacts

- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/tests/configuration.rs
- adl/src/cli/csm_runtime_v3_cmd.rs
- .csdlc/prepared/issues/659/validate-runtime-convergence.sh

## Execution

- Added serde-defaulted stop, unload, listener, and readiness convergence limits with 5/5/5/15-minute defaults and inclusive 1-second to 1-hour bounds.
- Replaced all five fixed 15-second service-control waits while preserving service-manager ownership and interrupted-reload recovery.
- Separated TCP listener availability from owned authenticated /v1/ready convergence and added exact-stage deadline diagnostics.
- Bound launchctl and systemctl convergence probes to the same remaining stage budget and reject success observed after expiry.
- Added focused configuration, slow-success, true-expiry, and listener-versus-readiness tests without restarting the live Runtime.

## Validation

[
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "--bin",
      "adl",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Issue 659 strict Clippy validation",
    "outcome": "passed",
    "evidence_ref": "runtime-convergence-clippy.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "configuration",
      "service_convergence_"
    ],
    "purpose": "Issue 659 Runtime init configuration validation",
    "outcome": "passed",
    "evidence_ref": "runtime-convergence-config.log"
  },
  {
    "command": [
      "/Volumes/FastWork/adl-worktrees/adl-issue-659-runtime-configurable-convergence/.csdlc/prepared/issues/659/validate-runtime-convergence.sh"
    ],
    "purpose": "Issue 659 convergence contract validation",
    "outcome": "passed",
    "evidence_ref": "runtime-convergence-contract.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Issue 659 diff hygiene",
    "outcome": "passed",
    "evidence_ref": "runtime-convergence-diff.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "--bin",
      "adl",
      "convergence_"
    ],
    "purpose": "Issue 659 CSM service-control validation",
    "outcome": "passed",
    "evidence_ref": "runtime-convergence-service-control.log"
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
