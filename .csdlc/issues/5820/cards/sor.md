# Structured Output Record

Template: 1.0.0

Issue: 5820

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Consolidated Runtime v3 under Guardian ownership and proved sustained bounded recovery, durable continuity, authenticated HTTPS/WSS, exact artifact identity, and complete task-owned AWS teardown on native macOS and Linux at product revision 0dfd6c26084b20528e805aa570b2d6b930c42b81. Native Windows remains explicitly blocked and is not claimed as proved.

## Artifacts

- adl-runtime/src/bin/adl-runtime-guardian.rs
- adl-runtime/src/bin/adl-runtime-lifecycle-soak.rs
- adl-runtime/src/guardian.rs
- adl-runtime/tests/runtime_guardian_lifecycle.rs
- adl-runtime-kernel/src/config.rs
- adl/tools/validate_v092_runtime_guardian_lifecycle.sh
- adl/tools/validate_v092_runtime_native_receipts.rb
- adl/tools/run_aws_spot_remote_validation_lane.sh
- .csdlc/evidence/5820/runtime-native-receipts.json
- .csdlc/evidence/5820/native/macos
- .csdlc/evidence/5820/native/linux
- .csdlc/evidence/5820/native/linux/volume-deletion-receipt.json
- .csdlc/evidence/5820/native/windows/blocker.json

## Execution

- Hardened Guardian startup, child ownership, bounded restart, and failure classification around the authoritative runtime init contract.
- Added a production lifecycle soak that forces one kernel failure and proves restart, durable generation continuity, authenticated Observatory HTTPS/WSS, clean logging, and clean shutdown.
- Made suite eligibility explicit and reconciled acceptance counters so preflight runs cannot be mistaken for sustained acceptance proof.
- Added digest-complete native receipt validation for named production Guardian and kernel artifacts, command logs, runtime init, runner provenance, and HTTPS/WSS transcripts.
- Removed non-portable process-substitution from the AWS Spot validation wrapper while retaining direct stdout and stderr evidence.
- Recorded tooling-only AWS tail/finalizer defects as new-repository issue #26 and the native receipt denominator defect as new-repository issue #27 without weakening runtime acceptance.
- Deleted the task-owned EC2 instance and temporary EBS cache volume after Linux proof; no task-owned compute remains.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--bin",
      "adl-runtime-lifecycle-soak",
      "--no-default-features"
    ],
    "purpose": "Prove restart policy, marker safety, nonce pairing, cleanup, and lifecycle invariants across the focused unit surface.",
    "outcome": "passed",
    "evidence_ref": "10 focused lifecycle tests passed at the final product revision"
  },
  {
    "command": [
      "bash",
      "adl/tools/validate_v092_runtime_guardian_lifecycle.sh",
      "--suite",
      "stress_100x10s"
    ],
    "purpose": "Prove 100 sustained macOS windows using the production Guardian and kernel with one forced failure, durable continuity, authenticated HTTPS/WSS, and clean shutdown.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5820/native/macos/lifecycle-component-report.json; 100/100 windows, 1954 completed cycles, generation 1954, one restart"
  },
  {
    "command": [
      "bash",
      "adl/tools/run_aws_spot_remote_validation_lane.sh",
      "--",
      "bash",
      "adl/tools/validate_v092_runtime_guardian_lifecycle.sh",
      "--suite",
      "stress_100x10s"
    ],
    "purpose": "Prove 100 sustained Linux Spot windows at the exact product revision and verify task-owned compute teardown.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5820/native/linux/lifecycle-component-report.json; .csdlc/evidence/5820/native/linux/aws-summary.json; .csdlc/evidence/5820/native/linux/volume-deletion-receipt.json; 100/100 windows, 3177 completed cycles, generation 3177, one restart; instance terminated and temporary volume deletion verified by InvalidVolume.NotFound"
  },
  {
    "command": [
      "ruby",
      "adl/tools/validate_v092_runtime_native_receipts.rb",
      ".csdlc/evidence/5820/runtime-native-receipts.json"
    ],
    "purpose": "Verify digest-complete macOS and Linux production artifacts, logs, transcripts, runtime init, and runner provenance against the exact product proof revision.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5820/runtime-native-receipts.json"
  },
  {
    "command": [
      "native-windows-runner"
    ],
    "purpose": "Retain the explicit fail-closed Windows platform boundary without claiming native proof that was not run.",
    "outcome": "blocked",
    "evidence_ref": ".csdlc/evidence/5820/native/windows/blocker.json"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_run_aws_spot_remote_validation_lane.sh"
    ],
    "purpose": "Prove the AWS wrapper retains stdout and stderr without relying on non-portable process substitution.",
    "outcome": "passed",
    "evidence_ref": "focused AWS wrapper regression passed"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_validate_v092_runtime_native_receipts.sh"
    ],
    "purpose": "Prove exact-final-head receipt closure while rejecting any post-proof runtime product change.",
    "outcome": "passed",
    "evidence_ref": "final-head native receipt closure passed"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace damage before exact-head review.",
    "outcome": "passed",
    "evidence_ref": "final worktree hygiene check"
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
