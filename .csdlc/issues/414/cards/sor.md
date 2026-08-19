# Structured Output Record

Template: 1.0.0

Issue: 414

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Integrate existing Runtime-v2, CSM capsule, LiveContinuity, Ollama-provider, retained-volume, and IMDSv2 authorities into fail-closed resident Shepherd Spot continuity.

## Artifacts

- adl/src/resident_shepherd_spot_continuity.rs
- adl/src/bin/adl_resident_shepherd_continuity.rs
- adl/src/runtime_v2/agent_lifecycle_state.rs
- adl/src/runtime_v2/citizen.rs
- adl/src/runtime_v2/contracts.rs
- adl/src/runtime_v2/snapshot.rs
- tools/aws_remote_validation/scripts/remote_validation_runner.sh
- tools/aws_remote_validation/src/aws_remote_validation.rs
- adl/tools/issue414_spot_dehydrate_callback.sh
- adl/tools/issue414_restore_and_admit.sh
- adl/tools/issue414_s3_linux_bootstrap.py
- adl/tools/test_issue414_s3_linux_bootstrap.py
- .csdlc/evidence/414/EVIDENCE_CLASSIFICATION.json

## Execution

- Orchestrate complete-population dehydration and exact restore for distinct resident agents without introducing a second lifecycle, capsule, or model-manager authority.
- Make Spot interruption callbacks deadline-bounded and immune to cancellation when the normal validation command completes.
- Bind retained volume, population, model route/config, executing source, and Linux bootstrap artifacts into validated receipts while deferring paid r7 qualification to issue 268.

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Patch hygiene.",
    "outcome": "passed",
    "evidence_ref": "diff-hygiene.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "live_continuity"
    ],
    "purpose": "Kernel continuity compatibility tests.",
    "outcome": "passed",
    "evidence_ref": "live-continuity-compatibility.log"
  },
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/414/validate_preparation_bundle.py"
    ],
    "purpose": "Issue 414 preparation contract.",
    "outcome": "passed",
    "evidence_ref": "preparation-contract.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "resident_shepherd_spot_continuity",
      "--lib"
    ],
    "purpose": "Six focused resident continuity tests.",
    "outcome": "passed",
    "evidence_ref": "resident-continuity-focused.log"
  },
  {
    "command": [
      "python3",
      "adl/tools/test_issue414_s3_linux_bootstrap.py"
    ],
    "purpose": "S3 bootstrap contract tests without AWS.",
    "outcome": "passed",
    "evidence_ref": "s3-bootstrap-contract.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "--bin",
      "adl_resident_shepherd_continuity",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Strict Clippy.",
    "outcome": "passed",
    "evidence_ref": "strict-clippy.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "tools/aws_remote_validation/Cargo.toml",
      "validation_completion_cannot_cancel_an_accepted_spot_transaction",
      "--bin",
      "adl-aws-remote-validation"
    ],
    "purpose": "AWS runner watcher regression.",
    "outcome": "passed",
    "evidence_ref": "watcher-race-regression.log"
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
